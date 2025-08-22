use egui::{TextEdit, Ui};

#[derive(Default)]
pub struct ConsoleState {
    input: String,
    log: Vec<String>,
    scroll_to_end: bool,
    pending: Vec<String>,
}

impl ConsoleState {
    /// Clears the console log and scrolls to the end.
    pub fn clear(&mut self) {
        (*self).log.clear();
        (*self).scroll_to_end = true;
    }

    /// Appends a line to the console log and scrolls to the end.
    fn push_line<S: Into<String>>(&mut self, s: S) {
        (*self).log.push(s.into());
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
                self.push_line("  invoke <ui> - open a preview window for a UI (e.g., skills, new_save, saves, settings, console)");
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
pub fn console_ui(ui: &mut Ui, state: &mut ConsoleState) {
    ui.vertical(|ui: &mut Ui| {
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .stick_to_bottom(true)
            .show(ui, |ui: &mut Ui| {
                for line in &(*state).log {
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
            TextEdit::singleline(&mut (*state).input)
                .hint_text("Enter command...")
                .desired_width(f32::INFINITY),
        );
        // Ensure the widget has a reasonable fixed height so the window doesn't flicker/resize
        ui.add_space(4.0);
        let pressed_enter: bool = input_resp.lost_focus()
            && ui.input(|i: &egui::InputState| i.key_pressed(egui::Key::Enter));
        ui.horizontal(|ui: &mut Ui| {
            if ui
                .add_sized([64.0, 24.0], egui::Button::new("Run"))
                .clicked()
                || pressed_enter
            {
                let cmd: String = (*state).input.clone();
                // Queue the command for external handling in the main loop
                (&mut (*state).pending).push(cmd);
                (&mut (*state).input).clear();
            }
            if ui
                .add_sized([64.0, 24.0], egui::Button::new("Clear"))
                .clicked()
            {
                state.clear();
            }
        });
    });
}
