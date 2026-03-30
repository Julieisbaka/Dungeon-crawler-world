use dungeon_crawler_world::console::{
    inspect_save, looks_like_number, parse_command_line, ConsoleCommandContext, ConsoleRegistry,
    HighlightedLine, LineType, TokenKind,
};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

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

mod parse_command_tests {
    use super::*;

    #[test]
    fn test_parse_command_line_with_quotes() {
        let parsed = parse_command_line("echo \"hello world\" 123").unwrap();
        assert_eq!(parsed.name, "echo");
        assert_eq!(parsed.args, vec!["hello world", "123"]);
    }

    #[test]
    fn test_parse_command_line_empty() {
        assert!(parse_command_line("   ").is_none());
    }
}

mod registry_tests {
    use super::*;

    struct MockContext {
        cleared: bool,
        previews: Vec<String>,
        current_save: Option<String>,
        saves_root: PathBuf,
        skills: Result<Vec<(String, i8)>, String>,
        settings: Vec<(String, String)>,
    }

    impl MockContext {
        fn new() -> Self {
            Self {
                cleared: false,
                previews: vec!["skills".to_string(), "grid".to_string()],
                current_save: None,
                saves_root: unique_temp_dir("console_tests_empty"),
                skills: Ok(vec![
                    ("Breathing".to_string(), 4),
                    ("Walking".to_string(), 5),
                ]),
                settings: vec![
                    ("show_console".to_string(), "true".to_string()),
                    ("log_to_console".to_string(), "false".to_string()),
                ],
            }
        }
    }

    impl Drop for MockContext {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.saves_root);
        }
    }

    impl ConsoleCommandContext for MockContext {
        fn clear_console(&mut self) {
            self.cleared = true;
        }

        fn open_preview(&mut self, name: &str) -> Result<(), String> {
            if self.previews.iter().any(|preview| preview == name) {
                Ok(())
            } else {
                Err(format!("Unknown preview: {}", name))
            }
        }

        fn list_previews(&self) -> Vec<String> {
            self.previews.clone()
        }

        fn current_save(&self) -> Option<String> {
            self.current_save.clone()
        }

        fn saves_root(&self) -> &Path {
            &self.saves_root
        }

        fn read_owned_skills(&self) -> Result<Vec<(String, i8)>, String> {
            self.skills.clone()
        }

        fn get_setting_value(&self, key: &str) -> Result<String, String> {
            self.settings
                .iter()
                .find(|(name, _)| name == key)
                .map(|(_, value)| value.clone())
                .ok_or_else(|| format!("Unknown setting: {}", key))
        }

        fn list_setting_values(&self) -> Vec<(String, String)> {
            self.settings.clone()
        }

        fn set_setting_value(&mut self, key: &str, value: &str) -> Result<String, String> {
            if key == "show_console" {
                if value != "true" && value != "false" {
                    return Err(format!("Invalid boolean value: {}", value));
                }
                self.settings
                    .retain(|(existing, _)| existing != "show_console");
                self.settings
                    .push(("show_console".to_string(), value.to_string()));
                Ok(format!("Set show_console = {}", value))
            } else {
                Err(format!("Unknown setting: {}", key))
            }
        }

        fn regenerate_grid_preview(&mut self) -> Result<String, String> {
            Ok("Regenerated grid preview.".to_string())
        }

        fn reset_grid_preview(&mut self) -> Result<String, String> {
            Ok("Reset grid preview view.".to_string())
        }
    }

    #[test]
    fn test_help_lists_registry_commands() {
        let registry = ConsoleRegistry::new();
        let mut context = MockContext::new();

        let lines = registry.execute("help", &mut context);

        assert_eq!(lines[0], "Available commands:");
        assert!(lines.iter().any(|line| line.contains("help [command]")));
        assert!(lines
            .iter()
            .any(|line| line.contains("save.inspect [name]")));
        assert!(lines
            .iter()
            .any(|line| line.contains("settings.set <key> <value>")));
    }

    #[test]
    fn test_help_command_specific_usage() {
        let registry = ConsoleRegistry::new();
        let mut context = MockContext::new();

        let lines = registry.execute("help save.inspect", &mut context);

        assert_eq!(lines[0], "Usage: save.inspect [name]");
        assert!(lines.iter().any(|line| line.contains("compact summary")));
    }

    #[test]
    fn test_unknown_command_standardized_error() {
        let registry = ConsoleRegistry::new();
        let mut context = MockContext::new();

        let lines = registry.execute("wat", &mut context);

        assert_eq!(lines[0], "Error: Unknown command: wat");
        assert_eq!(lines[1], "Type 'help' for a list of commands.");
    }

    #[test]
    fn test_echo_preserves_spaced_and_quoted_arguments() {
        let registry = ConsoleRegistry::new();
        let mut context = MockContext::new();

        let lines = registry.execute("echo one \"two words\" three", &mut context);

        assert_eq!(lines, vec!["one two words three"]);
    }

    #[test]
    fn test_command_aliases_resolve_correctly() {
        let registry = ConsoleRegistry::new();
        let mut context = MockContext::new();

        let help_lines = registry.execute("commands", &mut context);
        let echo_lines = registry.execute("log alias still works", &mut context);

        assert_eq!(help_lines[0], "Available commands:");
        assert_eq!(echo_lines, vec!["alias still works"]);
    }

    #[test]
    fn test_invalid_arity_returns_usage() {
        let registry = ConsoleRegistry::new();
        let mut context = MockContext::new();

        let lines = registry.execute("settings.set only_key", &mut context);

        assert_eq!(lines, vec!["Usage: settings.set <key> <value>"]);
    }

    #[test]
    fn test_ui_list_returns_previews() {
        let registry = ConsoleRegistry::new();
        let mut context = MockContext::new();

        let lines = registry.execute("ui.list", &mut context);

        assert_eq!(lines[0], "Known preview UIs:");
        assert!(lines.iter().any(|line| line == "  skills"));
        assert!(lines.iter().any(|line| line == "  grid"));
    }

    #[test]
    fn test_settings_get_and_set() {
        let registry = ConsoleRegistry::new();
        let mut context = MockContext::new();

        let get_lines = registry.execute("settings.get show_console", &mut context);
        let set_lines = registry.execute("settings.set show_console false", &mut context);
        let invalid_lines = registry.execute("settings.set show_console maybe", &mut context);

        assert_eq!(get_lines, vec!["show_console = true"]);
        assert_eq!(set_lines, vec!["Set show_console = false"]);
        assert_eq!(invalid_lines, vec!["Error: Invalid boolean value: maybe"]);
    }

    #[test]
    fn test_save_current_handles_none() {
        let registry = ConsoleRegistry::new();
        let mut context = MockContext::new();

        let lines = registry.execute("save.current", &mut context);

        assert_eq!(lines, vec!["No current save selected."]);
    }

    #[test]
    fn test_clear_dispatches_through_registry() {
        let registry = ConsoleRegistry::new();
        let mut context = MockContext::new();

        let lines = registry.execute("clear", &mut context);

        assert!(context.cleared);
        assert!(lines.is_empty());
    }
}

mod save_inspection_tests {
    use super::*;

    #[test]
    fn test_save_inspect_handles_missing_save() {
        let root = unique_temp_dir("console_tests_missing_save");

        let result = inspect_save(&root, Some("missing"), None);

        assert_eq!(result.unwrap_err(), "Save not found: missing");
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn test_save_inspect_handles_malformed_json() {
        let root = unique_temp_dir("console_tests_bad_json");
        let save_dir = root.join("broken_save");
        fs::create_dir_all(&save_dir).unwrap();
        fs::write(save_dir.join("save.json"), "{ bad json").unwrap();
        fs::write(save_dir.join("player.json"), "{}").unwrap();

        let result = inspect_save(&root, Some("broken_save"), None);

        assert!(result.unwrap_err().starts_with("Failed to parse"));
        let _ = fs::remove_dir_all(root);
    }
}

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// Removes `path` if it exists; returns `Ok(())` for NotFound, propagates any other error.
fn try_clean_dir(path: &std::path::Path) -> std::io::Result<()> {
    match fs::remove_dir_all(path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e),
    }
}

/// Creates `path` and all ancestors; wraps `fs::create_dir_all` so the
/// result can be inspected in tests.
fn try_create_dir(path: &std::path::Path) -> std::io::Result<()> {
    fs::create_dir_all(path)
}

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!("{}_{}", prefix, id));
    if let Err(e) = try_clean_dir(&path) {
        panic!(
            "Failed to remove existing temporary directory {:?}: {}",
            path, e
        );
    }
    try_create_dir(&path).unwrap_or_else(|e| {
        panic!("Failed to create temporary directory {:?}: {}", path, e)
    });
    path
}

mod try_create_dir_tests {
    use super::*;

    #[test]
    fn returns_ok_for_new_path() {
        let path = std::env::temp_dir().join("try_create_dir_ok_test");
        let _ = fs::remove_dir_all(&path);
        let result = try_create_dir(&path);
        let _ = fs::remove_dir_all(&path);
        assert!(result.is_ok(), "Expected Ok for a path that can be created");
    }

    #[test]
    #[cfg(unix)]
    fn returns_err_when_parent_is_not_writable() {
        use std::os::unix::fs::PermissionsExt;

        let base = std::env::temp_dir().join("try_create_dir_perm_test");
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).unwrap();

        // Remove write permission on the parent so mkdir inside it fails.
        let mut perms = fs::metadata(&base).unwrap().permissions();
        perms.set_mode(0o555);
        fs::set_permissions(&base, perms).unwrap();

        let child = base.join("child");
        let result = try_create_dir(&child);

        // Restore permissions before asserting so cleanup always runs.
        let mut perms = fs::metadata(&base).unwrap().permissions();
        perms.set_mode(0o755);
        let _ = fs::set_permissions(&base, perms);
        let _ = fs::remove_dir_all(&base);

        assert!(
            result.is_err(),
            "Expected Err when parent directory is not writable"
        );
        assert_ne!(
            result.unwrap_err().kind(),
            std::io::ErrorKind::NotFound,
            "Error should not be NotFound"
        );
    }
}

mod try_clean_dir_tests {
    use super::*;

    #[test]
    fn returns_ok_when_path_does_not_exist() {
        let path = std::env::temp_dir().join("try_clean_dir_not_found_test");
        // Ensure it really doesn't exist.
        let _ = fs::remove_dir_all(&path);
        assert!(try_clean_dir(&path).is_ok());
    }

    #[test]
    #[cfg(unix)]
    fn returns_err_on_permission_denied() {
        use std::os::unix::fs::PermissionsExt;

        let dir = std::env::temp_dir().join("try_clean_dir_perm_denied_test");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        // Create a file inside; removing it requires write permission on the parent.
        fs::write(dir.join("blocker"), "x").unwrap();
        // Remove write permission from the directory so removal of its contents fails.
        let mut perms = fs::metadata(&dir).unwrap().permissions();
        perms.set_mode(0o555);
        fs::set_permissions(&dir, perms).unwrap();

        let result = try_clean_dir(&dir);

        // Restore permissions before asserting so cleanup always runs.
        let mut perms = fs::metadata(&dir).unwrap().permissions();
        perms.set_mode(0o755);
        let _ = fs::set_permissions(&dir, perms);
        let _ = fs::remove_dir_all(&dir);

        assert!(
            result.is_err(),
            "Expected Err for permission-denied removal"
        );
        assert_ne!(
            result.unwrap_err().kind(),
            std::io::ErrorKind::NotFound,
            "Error should not be NotFound"
        );
    }
}
