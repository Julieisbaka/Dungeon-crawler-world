// Max lines is now a runtime setting, not a constant.
use egui::{TextBuffer, TextEdit, Ui};

/// Maximum number of log lines to keep in memory to prevent unbounded growth
const MAX_LOG_LINES: usize = 1000;

#[derive(Default)]
pub struct ConsoleState {
    input: String,
    log: Vec<String>,
    scroll_to_end: bool,
    pending: Vec<String>,
    last_command: Option<String>,
}

impl ConsoleState {
    /// Clears the console log and scrolls to the end.
    pub fn clear(&mut self) {
        (&mut (*self).log).clear();
        (*self).scroll_to_end = true;
    }

    /// Appends a line to the console log and scrolls to the end.
    fn push_line<S: Into<String>>(&mut self, s: S) {
        (&mut (*self).log).push(s.into());
        
        // Trim log to prevent unbounded growth and maintain performance
        if (*self).log.len() > MAX_LOG_LINES {
            let excess = (*self).log.len() - MAX_LOG_LINES;
            (*self).log.drain(0..excess);
        }
        
        (*self).scroll_to_end = true;
    }

    /// Allows external systems to log messages to the console.
    ///
    /// # Arguments
    /// * `s` - The message to log.
    pub fn log_line<S: Into<String>>(&mut self, s: S) {
        self.push_line(s);
    }

    /// Runs a console command, handling built-in commands like 'help' and 'clear'.
    ///
    /// # Arguments
    /// * `cmd` - The command string to execute.
    pub fn run_command(&mut self, cmd: &str) {
        let trimmed = cmd.trim();
        match trimmed {
            "help" => {
                self.push_line("Available commands:");
                self.push_line("  help  - show this message");
                self.push_line("  clear - clear the console output");
                self.push_line("  invoke <ui> - open a preview window for a UI (e.g., skills, new_save, saves, settings, console, quit)");
            }
            "clear" => {
                self.clear();
            }
            "" => {}
            other => {
                self.push_line(format!("Unknown command: {}", other));
                self.push_line("Type 'help' for a list of commands.");
            }
        }
    }

    /// Drains and returns pending commands submitted by the user in the UI.
    ///
    /// # Returns
    /// A vector of pending command strings.
    pub fn take_pending(&mut self) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        std::mem::swap(&mut out, &mut (*self).pending);
        out
    }
}

/// Renders the console UI, including the log output and input field.
///
/// # Arguments
/// * `ui` - The egui UI to render into.
/// * `state` - The mutable state of the console.
pub fn console_ui(ui: &mut Ui, state: &mut ConsoleState, max_lines: usize) {
    ui.vertical(|ui: &mut Ui| {
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .stick_to_bottom(true)
            .show(ui, |ui: &mut Ui| {
                let log_len: usize = (&(*state).log).len();
                let start: usize = log_len.saturating_sub(max_lines);
                for line in (&(&(*state).log)[start..]).iter() {
                    ui.label(line);
                }
                if (*state).scroll_to_end {
                    ui.scroll_to_cursor(Some(egui::Align::BOTTOM));
                    (*state).scroll_to_end = false;
                }
            });

        // Stable-size input field and action buttons
        // Use a visible fixed height to prevent hover-based reflow/resizing
        let input_resp: egui::Response = ui.add(
            TextEdit::singleline(&mut (*state).input as &mut dyn TextBuffer)
                .hint_text("Enter command...")
                .desired_width(f32::INFINITY),
        );
        // Ensure the widget has a reasonable fixed height so the window doesn't flicker/resize
        ui.add_space(4.0);
        let pressed_enter: bool = (&input_resp).lost_focus()
            && ui.input(|i: &egui::InputState| -> bool { i.key_pressed(egui::Key::Enter) });
        ui.horizontal(|ui: &mut Ui| {
            if (&ui.add_sized([64.0, 24.0], egui::Button::new("Run"))).clicked() || pressed_enter {
                let cmd = std::mem::take(&mut (*state).input);
                if !cmd.trim().is_empty() {
                    (*state).last_command = Some(cmd.clone());
                }
                // Queue the command for external handling in the main loop
                (&mut (*state).pending).push(cmd);
            }
            if (&ui.add_sized([64.0, 24.0], egui::Button::new("Clear"))).clicked() {
                state.clear();
            }
        });

        // Up arrow recall: if input is focused and up is pressed, recall last command
        let input_focused: bool = (&input_resp).has_focus();
        let up_pressed: bool =
            ui.input(|i: &egui::InputState| -> bool { i.key_pressed(egui::Key::ArrowUp) });
        if input_focused && up_pressed {
            if let Some(cmd) = &(*state).last_command {
                (*state).input = cmd.clone();
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_trimming() {
        let mut console = ConsoleState::default();
        
        // Add more than MAX_LOG_LINES entries
        for i in 0..MAX_LOG_LINES + 100 {
            console.log_line(format!("Test line {}", i));
        }
        
        // Verify log is trimmed to MAX_LOG_LINES
        assert_eq!(console.log.len(), MAX_LOG_LINES);
        
        // Verify the oldest entries were removed (should start from line 100)
        assert_eq!(console.log[0], "Test line 100");
        assert_eq!(console.log[MAX_LOG_LINES - 1], format!("Test line {}", MAX_LOG_LINES + 99));
    }

    #[test]
    fn test_log_no_trimming_when_under_limit() {
        let mut console = ConsoleState::default();
        
        // Add fewer than MAX_LOG_LINES entries
        let test_lines = 50;
        for i in 0..test_lines {
            console.log_line(format!("Test line {}", i));
        }
        
        // Verify all lines are kept
        assert_eq!(console.log.len(), test_lines);
        assert_eq!(console.log[0], "Test line 0");
        assert_eq!(console.log[test_lines - 1], "Test line 49");
    }
}
