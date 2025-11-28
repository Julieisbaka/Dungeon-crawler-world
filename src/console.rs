// Max lines is now a runtime setting, not a constant.
use egui::{TextBuffer, TextEdit, Ui};

/// Token kind for syntax highlighting
#[derive(Clone, Debug, PartialEq)]
enum TokenKind {
    Quote,
    Word,
    Number,
    Command,
    Space,
}

/// A single token with its text and kind
#[derive(Clone, Debug, PartialEq)]
struct Token {
    text: String,
    kind: TokenKind,
}

/// Quick check if a string looks like a number (avoids full parsing overhead)
fn looks_like_number(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut chars = s.chars().peekable();
    // Optional sign
    if matches!(chars.peek(), Some('+' | '-')) {
        chars.next();
    }
    // Must have at least one digit
    let mut has_digit = false;
    let mut has_dot = false;
    for c in chars {
        if c.is_ascii_digit() {
            has_digit = true;
        } else if c == '.' && !has_dot {
            has_dot = true;
        } else if c == 'e' || c == 'E' {
            // Scientific notation - fall back to full parse
            return s.parse::<f64>().is_ok();
        } else {
            return false;
        }
    }
    has_digit
}

/// Line type for fast prefix-based highlighting
#[derive(Clone, Debug, PartialEq)]
enum LineType {
    Error,
    Warning,
    Help,
    HelpDetail,
    Normal,
}

/// A cached highlighted line with pre-computed tokens
#[derive(Clone, Debug)]
struct HighlightedLine {
    line_type: LineType,
    raw: String,
    /// Tokens for Normal lines, None for other line types
    tokens: Option<Vec<Token>>,
}

impl HighlightedLine {
    /// Parse and highlight a line, caching the result
    fn new(line: String) -> Self {
        let line_type = if line.starts_with("Unknown command") || line.starts_with("Error") {
            LineType::Error
        } else if line.starts_with("Warning") {
            LineType::Warning
        } else if line.starts_with("Available commands:")
            || line.starts_with("Type 'help'")
            || line.starts_with("Usage:")
        {
            LineType::Help
        } else if line.starts_with("  ") && line.contains(" - ") {
            // Help detail lines have format "  <command> - <description>"
            LineType::HelpDetail
        } else {
            LineType::Normal
        };

        // Only tokenize normal lines (others are rendered as single colored text)
        let tokens = if line_type == LineType::Normal {
            Some(Self::tokenize(&line))
        } else {
            None
        };

        Self {
            line_type,
            raw: line,
            tokens,
        }
    }

    /// Tokenize a line into colored tokens
    fn tokenize(line: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut in_quotes = false;
        let mut current = String::new();
        let mut whitespace = String::new();
        let mut is_first_word = true;

        for c in line.chars() {
            if c == '"' {
                // Flush any accumulated whitespace before the quote
                if !whitespace.is_empty() {
                    tokens.push(Token {
                        text: std::mem::take(&mut whitespace),
                        kind: TokenKind::Space,
                    });
                }
                in_quotes = !in_quotes;
                current.push(c);
                if !in_quotes {
                    tokens.push(Token {
                        text: std::mem::take(&mut current),
                        kind: TokenKind::Quote,
                    });
                }
                continue;
            }
            if in_quotes {
                current.push(c);
                continue;
            }
            if c.is_whitespace() {
                if !current.is_empty() {
                    let kind = if is_first_word {
                        is_first_word = false;
                        TokenKind::Command
                    } else if looks_like_number(&current) {
                        TokenKind::Number
                    } else {
                        TokenKind::Word
                    };
                    tokens.push(Token {
                        text: std::mem::take(&mut current),
                        kind,
                    });
                }
                // Accumulate consecutive whitespace
                whitespace.push(c);
            } else {
                // Flush accumulated whitespace before non-whitespace
                if !whitespace.is_empty() {
                    tokens.push(Token {
                        text: std::mem::take(&mut whitespace),
                        kind: TokenKind::Space,
                    });
                }
                current.push(c);
            }
        }
        // Flush remaining tokens
        if !whitespace.is_empty() {
            tokens.push(Token {
                text: whitespace,
                kind: TokenKind::Space,
            });
        }
        if !current.is_empty() {
            let kind = if in_quotes {
                TokenKind::Quote
            } else if is_first_word {
                TokenKind::Command
            } else if looks_like_number(&current) {
                TokenKind::Number
            } else {
                TokenKind::Word
            };
            tokens.push(Token {
                text: current,
                kind,
            });
        }
        tokens
    }
}

#[derive(Default)]
pub struct ConsoleState {
    input: String,
    log: Vec<HighlightedLine>,
    scroll_to_end: bool,
    pending: Vec<String>,
    last_command: Option<String>,
    dirty: bool, // Tracks if the console state has changed
}

impl ConsoleState {
    /// Clears the console log and scrolls to the end.
    pub fn clear(&mut self) {
        self.log.clear();
        self.scroll_to_end = true;
        self.dirty = true;
    }

    /// Appends a line to the console log and scrolls to the end.
    fn push_line<S: Into<String>>(&mut self, s: S) {
        self.log.push(HighlightedLine::new(s.into()));
        self.scroll_to_end = true;
        self.dirty = true;
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
        std::mem::take(&mut self.pending)
    }

    /// Mark the console as dirty if the input changes
    pub fn set_input(&mut self, new_input: String) {
        if self.input != new_input {
            self.input = new_input;
            self.dirty = true;
        }
    }

    /// Returns whether the console state is dirty (has changed since last redraw)
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Clears the dirty flag (call after redraw)
    pub fn clear_dirty(&mut self) {
        self.dirty = false;
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
            let log_len = state.log.len();
            let start = log_len.saturating_sub(max_lines);
            for highlighted_line in &state.log[start..] {
                // Use pre-computed line type for fast rendering
                match highlighted_line.line_type {
                    LineType::Error => {
                        ui.label(
                            egui::RichText::new(&highlighted_line.raw)
                                .color(egui::Color32::RED)
                                .strong(),
                        );
                    }
                    LineType::Warning => {
                        ui.label(
                            egui::RichText::new(&highlighted_line.raw)
                                .color(egui::Color32::YELLOW)
                                .strong(),
                        );
                    }
                    LineType::Help => {
                        ui.label(
                            egui::RichText::new(&highlighted_line.raw)
                                .color(egui::Color32::LIGHT_BLUE),
                        );
                    }
                    LineType::HelpDetail => {
                        ui.label(
                            egui::RichText::new(&highlighted_line.raw)
                                .color(egui::Color32::from_rgb(0, 200, 255)),
                        );
                    }
                    LineType::Normal => {
                        // Render pre-computed tokens with color
                        if let Some(tokens) = &highlighted_line.tokens {
                            ui.horizontal(|ui: &mut Ui| {
                                for token in tokens {
                                    let text = match token.kind {
                                        TokenKind::Quote => {
                                            egui::RichText::new(&token.text)
                                                .color(egui::Color32::GREEN)
                                        }
                                        TokenKind::Command => egui::RichText::new(&token.text)
                                            .color(egui::Color32::from_rgb(0, 200, 255))
                                            .strong(),
                                        TokenKind::Number => egui::RichText::new(&token.text)
                                            .color(egui::Color32::YELLOW),
                                        TokenKind::Word | TokenKind::Space => {
                                            egui::RichText::new(&token.text)
                                        }
                                    };
                                    ui.label(text);
                                }
                            });
                        } else {
                            // Fallback: render raw text if tokens are missing
                            ui.label(&highlighted_line.raw);
                        }
                    }
                }
            }
            if state.scroll_to_end {
                ui.scroll_to_cursor(Some(egui::Align::BOTTOM));
                state.scroll_to_end = false;
            }
        });

    // Input field and action buttons (not scrollable)
    // Use a visible fixed height to prevent hover-based reflow/resizing
    let input_resp = ui.add(
        TextEdit::singleline(&mut state.input as &mut dyn TextBuffer)
            .hint_text("Enter command...")
            .desired_width(f32::INFINITY),
    );
    // Ensure the widget has a reasonable fixed height so the window doesn't flicker/resize
    ui.add_space(4.0);
    let pressed_enter =
        input_resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
    ui.horizontal(|ui: &mut Ui| {
        if ui.add_sized([64.0, 24.0], egui::Button::new("Run")).clicked() || pressed_enter {
            let cmd = state.input.clone();
            if !cmd.trim().is_empty() {
                state.last_command = Some(cmd.clone());
            }
            // Queue the command for external handling in the main loop
            state.pending.push(cmd);
            state.input.clear();
        }
        if ui.add_sized([64.0, 24.0], egui::Button::new("Clear")).clicked() {
            state.clear();
        }
    });

    // Up arrow recall: if input is focused and up is pressed, recall last command
    let input_focused = input_resp.has_focus();
    let up_pressed = ui.input(|i| i.key_pressed(egui::Key::ArrowUp));
    if input_focused && up_pressed {
        if let Some(cmd) = &state.last_command {
            state.input = cmd.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for looks_like_number()
    mod looks_like_number_tests {
        use super::*;

        #[test]
        fn test_valid_integers() {
            assert!(looks_like_number("123"));
            assert!(looks_like_number("0"));
            assert!(looks_like_number("999999"));
        }

        #[test]
        fn test_valid_decimals() {
            assert!(looks_like_number("45.67"));
            assert!(looks_like_number("0.5"));
            assert!(looks_like_number(".5"));
            assert!(looks_like_number("123."));
        }

        #[test]
        fn test_valid_signed_numbers() {
            assert!(looks_like_number("-89"));
            assert!(looks_like_number("+1.5"));
            assert!(looks_like_number("-0.25"));
            assert!(looks_like_number("+100"));
        }

        #[test]
        fn test_scientific_notation() {
            assert!(looks_like_number("1e5"));
            assert!(looks_like_number("2.5E-3"));
            assert!(looks_like_number("1.0e10"));
            assert!(looks_like_number("5E+2"));
        }

        #[test]
        fn test_invalid_inputs() {
            assert!(!looks_like_number(""));
            assert!(!looks_like_number("."));
            assert!(!looks_like_number("+"));
            assert!(!looks_like_number("-"));
            assert!(!looks_like_number("abc"));
            assert!(!looks_like_number("12.34.56"));
            assert!(!looks_like_number("12abc"));
            assert!(!looks_like_number("hello123"));
        }

        #[test]
        fn test_edge_cases() {
            assert!(!looks_like_number("+-5"));
            assert!(!looks_like_number("--5"));
            assert!(looks_like_number("+.5"));
            assert!(looks_like_number("-.5"));
        }
    }

    // Tests for tokenize()
    mod tokenize_tests {
        use super::*;

        #[test]
        fn test_simple_command() {
            let tokens = HighlightedLine::tokenize("help");
            assert_eq!(tokens.len(), 1);
            assert_eq!(tokens[0].text, "help");
            assert_eq!(tokens[0].kind, TokenKind::Command);
        }

        #[test]
        fn test_command_with_args() {
            let tokens = HighlightedLine::tokenize("log hello world");
            assert_eq!(tokens.len(), 5);
            assert_eq!(tokens[0].text, "log");
            assert_eq!(tokens[0].kind, TokenKind::Command);
            assert_eq!(tokens[1].text, " ");
            assert_eq!(tokens[1].kind, TokenKind::Space);
            assert_eq!(tokens[2].text, "hello");
            assert_eq!(tokens[2].kind, TokenKind::Word);
            assert_eq!(tokens[3].text, " ");
            assert_eq!(tokens[3].kind, TokenKind::Space);
            assert_eq!(tokens[4].text, "world");
            assert_eq!(tokens[4].kind, TokenKind::Word);
        }

        #[test]
        fn test_quoted_argument() {
            let tokens = HighlightedLine::tokenize("log \"hello world\"");
            assert_eq!(tokens.len(), 3);
            assert_eq!(tokens[0].text, "log");
            assert_eq!(tokens[0].kind, TokenKind::Command);
            assert_eq!(tokens[1].text, " ");
            assert_eq!(tokens[1].kind, TokenKind::Space);
            assert_eq!(tokens[2].text, "\"hello world\"");
            assert_eq!(tokens[2].kind, TokenKind::Quote);
        }

        #[test]
        fn test_number_argument() {
            let tokens = HighlightedLine::tokenize("command 123");
            assert_eq!(tokens.len(), 3);
            assert_eq!(tokens[0].text, "command");
            assert_eq!(tokens[0].kind, TokenKind::Command);
            assert_eq!(tokens[1].text, " ");
            assert_eq!(tokens[1].kind, TokenKind::Space);
            assert_eq!(tokens[2].text, "123");
            assert_eq!(tokens[2].kind, TokenKind::Number);
        }

        #[test]
        fn test_mixed_scenario() {
            let tokens = HighlightedLine::tokenize("command \"quoted arg\" 123 word");
            assert_eq!(tokens.len(), 7);
            assert_eq!(tokens[0].text, "command");
            assert_eq!(tokens[0].kind, TokenKind::Command);
            assert_eq!(tokens[1].text, " ");
            assert_eq!(tokens[1].kind, TokenKind::Space);
            assert_eq!(tokens[2].text, "\"quoted arg\"");
            assert_eq!(tokens[2].kind, TokenKind::Quote);
            assert_eq!(tokens[3].text, " ");
            assert_eq!(tokens[3].kind, TokenKind::Space);
            assert_eq!(tokens[4].text, "123");
            assert_eq!(tokens[4].kind, TokenKind::Number);
            assert_eq!(tokens[5].text, " ");
            assert_eq!(tokens[5].kind, TokenKind::Space);
            assert_eq!(tokens[6].text, "word");
            assert_eq!(tokens[6].kind, TokenKind::Word);
        }

        #[test]
        fn test_unclosed_quote() {
            // Unclosed quote should treat remaining text as Quote token
            let tokens = HighlightedLine::tokenize("command \"hello world");
            assert_eq!(tokens.len(), 3);
            assert_eq!(tokens[0].text, "command");
            assert_eq!(tokens[0].kind, TokenKind::Command);
            assert_eq!(tokens[1].text, " ");
            assert_eq!(tokens[1].kind, TokenKind::Space);
            assert_eq!(tokens[2].text, "\"hello world");
            assert_eq!(tokens[2].kind, TokenKind::Quote);
        }

        #[test]
        fn test_empty_string() {
            let tokens = HighlightedLine::tokenize("");
            assert!(tokens.is_empty());
        }

        #[test]
        fn test_whitespace_only() {
            let tokens = HighlightedLine::tokenize("   ");
            assert_eq!(tokens.len(), 1);
            assert_eq!(tokens[0].text, "   ");
            assert_eq!(tokens[0].kind, TokenKind::Space);
        }

        #[test]
        fn test_consecutive_quotes() {
            let tokens = HighlightedLine::tokenize("\"\"");
            assert_eq!(tokens.len(), 1);
            assert_eq!(tokens[0].text, "\"\"");
            assert_eq!(tokens[0].kind, TokenKind::Quote);
        }

        #[test]
        fn test_quote_mid_word() {
            // Quote mid-word starts a new quote context
            let tokens = HighlightedLine::tokenize("wo\"rd\"");
            assert_eq!(tokens.len(), 2);
            assert_eq!(tokens[0].text, "wo");
            assert_eq!(tokens[0].kind, TokenKind::Command);
            assert_eq!(tokens[1].text, "\"rd\"");
            assert_eq!(tokens[1].kind, TokenKind::Quote);
        }

        #[test]
        fn test_multiple_spaces() {
            let tokens = HighlightedLine::tokenize("command   arg");
            assert_eq!(tokens.len(), 3);
            assert_eq!(tokens[0].text, "command");
            assert_eq!(tokens[0].kind, TokenKind::Command);
            assert_eq!(tokens[1].text, "   ");
            assert_eq!(tokens[1].kind, TokenKind::Space);
            assert_eq!(tokens[2].text, "arg");
            assert_eq!(tokens[2].kind, TokenKind::Word);
        }
    }

    // Tests for HighlightedLine line type detection
    mod line_type_tests {
        use super::*;

        #[test]
        fn test_error_detection() {
            let line = HighlightedLine::new("Unknown command: foo".to_string());
            assert_eq!(line.line_type, LineType::Error);
            assert!(line.tokens.is_none());

            let line = HighlightedLine::new("Error: something went wrong".to_string());
            assert_eq!(line.line_type, LineType::Error);
        }

        #[test]
        fn test_warning_detection() {
            let line = HighlightedLine::new("Warning: be careful".to_string());
            assert_eq!(line.line_type, LineType::Warning);
            assert!(line.tokens.is_none());
        }

        #[test]
        fn test_help_detection() {
            let line = HighlightedLine::new("Available commands:".to_string());
            assert_eq!(line.line_type, LineType::Help);

            let line = HighlightedLine::new("Type 'help' for more info".to_string());
            assert_eq!(line.line_type, LineType::Help);

            let line = HighlightedLine::new("Usage: command <arg>".to_string());
            assert_eq!(line.line_type, LineType::Help);
        }

        #[test]
        fn test_help_detail_detection() {
            // Help detail lines have format "  <command> - <description>"
            let line = HighlightedLine::new("  help  - show this message".to_string());
            assert_eq!(line.line_type, LineType::HelpDetail);
            assert!(line.tokens.is_none());

            let line = HighlightedLine::new("  clear - clear the console output".to_string());
            assert_eq!(line.line_type, LineType::HelpDetail);
        }

        #[test]
        fn test_indented_normal_lines_not_help_detail() {
            // Lines starting with spaces but without " - " separator should be Normal
            let line = HighlightedLine::new("  indented text".to_string());
            assert_eq!(line.line_type, LineType::Normal);
            assert!(line.tokens.is_some());

            let line = HighlightedLine::new("  normal command with spaces".to_string());
            assert_eq!(line.line_type, LineType::Normal);
            assert!(line.tokens.is_some());
        }

        #[test]
        fn test_normal_detection() {
            let line = HighlightedLine::new("hello world".to_string());
            assert_eq!(line.line_type, LineType::Normal);
            assert!(line.tokens.is_some());
        }
    }
}
