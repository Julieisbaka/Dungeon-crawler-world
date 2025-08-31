use log::{Log, Metadata, Record};
use std::sync::{Mutex, OnceLock};
use std::sync::mpsc::{Sender, channel, Receiver};

static LOGGER: Logger = Logger;
static LOG_SENDER: OnceLock<Mutex<Option<Sender<String>>>> = OnceLock::new();

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

pub fn init_logger() -> (Sender<String>, Receiver<String>) {
    let (tx, rx) = channel();
    (&LOG_SENDER).get_or_init(|| -> Mutex<Option<Sender<String>>> { Mutex::new(Some((&tx).clone())) });
    log::set_logger(&LOGGER).map(|()| log::set_max_level(log::LevelFilter::Trace)).ok();
    (tx, rx)
}
