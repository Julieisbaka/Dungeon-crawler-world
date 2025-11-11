// Max lines is now a runtime setting, not a constant.
use egui::{TextBuffer, TextEdit, Ui};

#[derive(Default)]
pub struct ConsoleState {
    input: String,
    log: Vec<String>,
    scroll_to_end: bool,
    pending: Vec<String>,
    last_command: Option<String>,
    dirty: bool, // Tracks if the console state has changed
}

impl ConsoleState {
    /// Clears the console log and scrolls to the end.
    pub fn clear(&mut self) {
        (&mut (*self).log).clear();
        (*self).scroll_to_end = true;
        (*self).dirty = true;
    }

    /// Appends a line to the console log and scrolls to the end.
    fn push_line<S: Into<String>>(&mut self, s: S) {
        (&mut (*self).log).push(s.into());
        (*self).scroll_to_end = true;
        (*self).dirty = true;
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
        let trimmed: &str = cmd.trim();
        // Split command into head and tail for argument parsing
        let mut parts: std::str::SplitN<'_, char> = trimmed.splitn(2, ' ');
        let head: &str = (&mut parts).next().unwrap_or("");
        let tail: &str = (&mut parts).next().unwrap_or("");
        match head {
            "help" => {
                self.push_line("Available commands:");
                self.push_line("  help  - show this message");
                self.push_line("  clear - clear the console output");
                self.push_line("  log <message> - log a message to the console");
                self.push_line("  invoke <ui> - open a preview window for a UI (e.g., skills, new_save, saves, settings, console, quit)");
            }
            "clear" => {
                self.clear();
            }
            "log" => {
                if tail.trim().is_empty() {
                    self.push_line("Usage: log <message>");
                } else {
                    self.log_line(tail.trim());
                }
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

    /// Mark the console as dirty if the input changes
    pub fn set_input(&mut self, new_input: String) {
        if (*self).input != new_input {
            (*self).input = new_input;
            (*self).dirty = true;
        }
    }

    /// Returns whether the console state is dirty (has changed since last redraw)
    pub fn is_dirty(&self) -> bool {
        (*self).dirty
    }

    /// Clears the dirty flag (call after redraw)
    pub fn clear_dirty(&mut self) {
        (*self).dirty = false;
    }
}

/// Renders the console UI, including the log output and input field.
///
/// # Arguments
/// * `ui` - The egui UI to render into.
/// * `state` - The mutable state of the console.
pub fn console_ui(ui: &mut Ui, state: &mut ConsoleState, max_lines: usize) {
    // Log output area (scrollable)
    egui::ScrollArea::vertical()
        .auto_shrink([false; 2])
        .stick_to_bottom(true)
        .show(ui, |ui: &mut Ui| {
            let log_len: usize = (&(*state).log).len();
            let start: usize = log_len.saturating_sub(max_lines);
            for line in (&(&(*state).log)[start..]).iter() {
                // Advanced syntax highlighting
                let line: &str = line.as_str();
                if line.starts_with("Unknown command") || line.starts_with("Error") {
                    ui.label(egui::RichText::new(line).color(egui::Color32::RED).strong());
                    continue;
                }
                if line.starts_with("Warning") {
                    ui.label(
                        egui::RichText::new(line)
                            .color(egui::Color32::YELLOW)
                            .strong(),
                    );
                    continue;
                }
                if line.starts_with("Available commands:")
                    || line.starts_with("Type 'help'")
                    || line.starts_with("Usage:")
                {
                    ui.label(egui::RichText::new(line).color(egui::Color32::LIGHT_BLUE));
                    continue;
                }
                if line.starts_with("  ") {
                    // Command help lines
                    ui.label(egui::RichText::new(line).color(egui::Color32::from_rgb(0, 200, 255)));
                    continue;
                }
                // Optimized tokenization for advanced highlighting
                ui.horizontal(|ui: &mut Ui| {
                    let mut is_first: bool = true;
                    let mut in_quotes: bool = false;
                    let mut token_start: usize = 0;
                    
                    for (i, c) in line.char_indices() {
                        if c == '"' {
                            if !in_quotes {
                                // Start of quoted string - capture opening quote
                                if i > token_start {
                                    // Render accumulated word before the quote
                                    let token = &line[token_start..i];
                                    let mut text = egui::RichText::new(token);
                                    if is_first {
                                        text = text.color(egui::Color32::from_rgb(0, 200, 255)).strong();
                                        is_first = false;
                                    } else if token.parse::<f64>().is_ok() {
                                        text = text.color(egui::Color32::YELLOW);
                                    }
                                    ui.label(text);
                                }
                                token_start = i;
                            } else if i > token_start {
                                // End of quoted string
                                let token = &line[token_start..i + 1];
                                ui.label(egui::RichText::new(token).color(egui::Color32::GREEN));
                                token_start = i + c.len_utf8();
                            }
                            in_quotes = !in_quotes;
                        } else if !in_quotes && c.is_whitespace() {
                            if i > token_start {
                                // Render accumulated word
                                let token = &line[token_start..i];
                                let mut text = egui::RichText::new(token);
                                if is_first {
                                    text = text.color(egui::Color32::from_rgb(0, 200, 255)).strong();
                                    is_first = false;
                                } else if token.parse::<f64>().is_ok() {
                                    text = text.color(egui::Color32::YELLOW);
                                }
                                ui.label(text);
                            }
                            token_start = i + c.len_utf8();
                        }
                    }
                    
                    // Render final token after loop completes
                    if token_start < line.len() {
                        let token = &line[token_start..];
                        if !token.is_empty() {
                            let mut text = egui::RichText::new(token);
                            if in_quotes {
                                text = text.color(egui::Color32::GREEN);
                            } else if is_first {
                                text = text.color(egui::Color32::from_rgb(0, 200, 255)).strong();
                            } else if token.parse::<f64>().is_ok() {
                                text = text.color(egui::Color32::YELLOW);
                            }
                            ui.label(text);
                        }
                    }
                });
            }
            if (*state).scroll_to_end {
                ui.scroll_to_cursor(Some(egui::Align::BOTTOM));
                (*state).scroll_to_end = false;
            }
        });

    // Input field and action buttons (not scrollable)
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
}
