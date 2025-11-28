use dungeon_crawler_world::console::{
    looks_like_number, HighlightedLine, LineType, Token, TokenKind,
};

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
