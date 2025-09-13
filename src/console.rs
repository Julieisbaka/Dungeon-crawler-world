// Max lines is now a runtime setting, not a constant.
use egui::{TextBuffer, TextEdit, Ui};

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
                let cmd: String = (&(*state).input).clone();
                if !(&*cmd).trim().is_empty() {
                    (*state).last_command = Some((&cmd).clone());
                }
                // Queue the command for external handling in the main loop
                (&mut (*state).pending).push(cmd);
                (&mut (*state).input).clear();
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
    fn test_console_state_default() {
        let state = ConsoleState::default();
        assert!(state.input.is_empty());
        assert!(state.log.is_empty());
        assert_eq!(state.scroll_to_end, false);
        assert!(state.pending.is_empty());
        assert!(state.last_command.is_none());
    }

    #[test]
    fn test_log_line() {
        let mut state = ConsoleState::default();
        
        state.log_line("Test message");
        assert_eq!(state.log.len(), 1);
        assert_eq!(state.log[0], "Test message");
        assert!(state.scroll_to_end);
    }

    #[test]
    fn test_clear() {
        let mut state = ConsoleState::default();
        
        state.log_line("Message 1");
        state.log_line("Message 2");
        assert_eq!(state.log.len(), 2);
        
        state.clear();
        assert!(state.log.is_empty());
        assert!(state.scroll_to_end);
    }

    #[test]
    fn test_run_command_help() {
        let mut state = ConsoleState::default();
        
        state.run_command("help");
        
        assert!(state.log.len() > 0);
        assert!(state.log[0].contains("Available commands"));
        assert!(state.log.iter().any(|line| line.contains("help")));
        assert!(state.log.iter().any(|line| line.contains("clear")));
        assert!(state.log.iter().any(|line| line.contains("invoke")));
    }

    #[test]
    fn test_run_command_clear() {
        let mut state = ConsoleState::default();
        
        state.log_line("Initial message");
        assert_eq!(state.log.len(), 1);
        
        state.run_command("clear");
        assert!(state.log.is_empty());
    }

    #[test]
    fn test_run_command_empty() {
        let mut state = ConsoleState::default();
        
        state.run_command("");
        state.run_command("   ");
        
        // Empty commands should not produce output
        assert!(state.log.is_empty());
    }

    #[test]
    fn test_run_command_unknown() {
        let mut state = ConsoleState::default();
        
        state.run_command("unknown_command");
        
        assert!(state.log.len() >= 2);
        assert!(state.log.iter().any(|line| line.contains("Unknown command")));
        assert!(state.log.iter().any(|line| line.contains("help")));
    }

    #[test]
    fn test_take_pending() {
        let mut state = ConsoleState::default();
        
        // Simulate adding pending commands (this would normally happen in the UI)
        state.pending.push("command1".to_string());
        state.pending.push("command2".to_string());
        
        let pending = state.take_pending();
        assert_eq!(pending.len(), 2);
        assert_eq!(pending[0], "command1");
        assert_eq!(pending[1], "command2");
        
        // After taking, pending should be empty
        assert!(state.pending.is_empty());
        
        // Taking again should return empty vector
        let pending2 = state.take_pending();
        assert!(pending2.is_empty());
    }

    #[test]
    fn test_command_trimming() {
        let mut state = ConsoleState::default();
        
        state.run_command("  help  ");
        assert!(state.log.iter().any(|line| line.contains("Available commands")));
        
        state.log.clear();
        state.run_command("  clear  ");
        assert!(state.log.is_empty());
    }
}
