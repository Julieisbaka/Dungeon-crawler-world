use log::{Log, Metadata, Record};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Mutex, OnceLock};

static LOGGER: Logger = Logger;
static LOG_SENDER: OnceLock<Mutex<Option<Sender<String>>>> = OnceLock::new();

/// Custom logger implementation that sends log messages to a channel.
/// 
/// This logger allows the application to capture log messages and display
/// them in the in-game console or other UI components.
pub struct Logger;

impl Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        // Enable all levels; filtering is done by log crate
        true
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        if let Some(sender_mutex) = (&LOG_SENDER).get() {
            if let Some(sender) = &*sender_mutex.lock().unwrap() {
                let msg: String = format!("[{}] {}", record.level(), record.args());
                let _ = sender.send(msg);
            }
        }
    }

    fn flush(&self) {}
}

/// Initializes the custom logger system.
/// 
/// This function sets up a logger that sends messages to a channel, allowing
/// the application to display log messages in the UI. It returns both ends
/// of the channel for sending and receiving log messages.
/// 
/// # Returns
/// 
/// A tuple containing:
/// - `Sender<String>` - For sending additional messages to the log
/// - `Receiver<String>` - For receiving log messages to display in UI
pub fn init_logger() -> (Sender<String>, Receiver<String>) {
    let (tx, rx) = channel();
    (&LOG_SENDER)
        .get_or_init(|| -> Mutex<Option<Sender<String>>> { Mutex::new(Some((&tx).clone())) });
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(log::LevelFilter::Trace))
        .ok();
    (tx, rx)
}
