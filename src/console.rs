use std::fs;
use std::path::Path;

use crate::player::Player;
use egui::{TextBuffer, TextEdit, Ui};

/// Token kind for syntax highlighting
#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    Quote,
    Word,
    Number,
    Command,
    Space,
}

/// A single token with its text and kind
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub text: String,
    pub kind: TokenKind,
}

/// Quick check if a string looks like a number (avoids full parsing overhead)
pub fn looks_like_number(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut chars = s.chars().peekable();
    if matches!(chars.peek(), Some('+' | '-')) {
        chars.next();
    }

    let mut has_digit = false;
    let mut has_dot = false;
    for c in chars {
        if c.is_ascii_digit() {
            has_digit = true;
        } else if c == '.' && !has_dot {
            has_dot = true;
        } else if c == 'e' || c == 'E' {
            return s.parse::<f64>().is_ok();
        } else {
            return false;
        }
    }
    has_digit
}

/// Line type for fast prefix-based highlighting
#[derive(Clone, Debug, PartialEq)]
pub enum LineType {
    Error,
    Warning,
    Help,
    HelpDetail,
    Normal,
}

/// A cached highlighted line with pre-computed tokens
#[derive(Clone, Debug)]
pub struct HighlightedLine {
    pub line_type: LineType,
    pub raw: String,
    /// Tokens for Normal lines, None for other line types
    pub tokens: Option<Vec<Token>>,
}

impl HighlightedLine {
    /// Parse and highlight a line, caching the result
    pub fn new(line: String) -> Self {
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
            LineType::HelpDetail
        } else {
            LineType::Normal
        };

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
    pub fn tokenize(line: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut in_quotes = false;
        let mut current = String::new();
        let mut whitespace = String::new();
        let mut is_first_word = true;

        for c in line.chars() {
            if c == '"' {
                if !in_quotes {
                    // Entering a quoted segment: flush any pending whitespace and
                    // any accumulated unquoted token, then open the quote.
                    if !whitespace.is_empty() {
                        tokens.push(Token {
                            text: std::mem::take(&mut whitespace),
                            kind: TokenKind::Space,
                        });
                    }
                    if !current.is_empty() {
                        let kind = if is_first_word {
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
                    // The opening quote always takes the first-word slot, whether or
                    // not there was accumulated text before it. This prevents an
                    // unterminated opening quote from being misclassified as a Command.
                    is_first_word = false;
                    in_quotes = true;
                    current.push(c);
                } else {
                    // Exiting a quoted segment: close the token and emit it.
                    current.push(c);
                    in_quotes = false;
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
                whitespace.push(c);
            } else {
                if !whitespace.is_empty() {
                    tokens.push(Token {
                        text: std::mem::take(&mut whitespace),
                        kind: TokenKind::Space,
                    });
                }
                current.push(c);
            }
        }

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
    dirty: bool,
}

impl ConsoleState {
    pub fn clear(&mut self) {
        self.log.clear();
        self.scroll_to_end = true;
        self.dirty = true;
    }

    fn push_line<S: Into<String>>(&mut self, s: S) {
        self.log.push(HighlightedLine::new(s.into()));
        self.scroll_to_end = true;
        self.dirty = true;
    }

    pub fn log_line<S: Into<String>>(&mut self, s: S) {
        self.push_line(s);
    }

    pub fn log_lines<I, S>(&mut self, lines: I)
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for line in lines {
            self.push_line(line);
        }
    }

    pub fn take_pending(&mut self) -> Vec<String> {
        std::mem::take(&mut self.pending)
    }

    pub fn set_input(&mut self, new_input: String) {
        if self.input != new_input {
            self.input = new_input;
            self.dirty = true;
        }
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParsedCommand {
    pub raw: String,
    pub name: String,
    pub args: Vec<String>,
}

pub fn parse_command_line(input: &str) -> Option<ParsedCommand> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for ch in trimmed.chars() {
        match ch {
            '"' => {
                in_quotes = !in_quotes;
            }
            c if c.is_whitespace() && !in_quotes => {
                if !current.is_empty() {
                    parts.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(ch),
        }
    }

    if !current.is_empty() {
        parts.push(current);
    }

    let name = parts.first()?.to_lowercase();
    let args = parts.into_iter().skip(1).collect();
    Some(ParsedCommand {
        raw: trimmed.to_string(),
        name,
        args,
    })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SaveListEntry {
    pub folder_name: String,
    pub save_name: String,
    pub difficulty: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SaveInspection {
    pub folder_name: String,
    pub save_name: String,
    pub difficulty: Option<String>,
    pub current_floor: u32,
    pub level: u32,
    pub coins: i32,
}

pub trait ConsoleCommandContext {
    fn clear_console(&mut self);
    fn open_preview(&mut self, name: &str) -> Result<(), String>;
    fn list_previews(&self) -> Vec<String>;
    fn current_save(&self) -> Option<String>;
    fn saves_root(&self) -> &Path;
    fn read_owned_skills(&self) -> Result<Vec<(String, i8)>, String>;
    fn get_setting_value(&self, key: &str) -> Result<String, String>;
    fn list_setting_values(&self) -> Vec<(String, String)>;
    fn set_setting_value(&mut self, key: &str, value: &str) -> Result<String, String>;
    fn regenerate_grid_preview(&mut self) -> Result<String, String>;
    fn reset_grid_preview(&mut self) -> Result<String, String>;
}

type CommandHandler = fn(
    &ConsoleRegistry,
    &ParsedCommand,
    &mut dyn ConsoleCommandContext,
) -> Result<Vec<String>, String>;

pub struct ConsoleCommand {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub usage: &'static str,
    pub description: &'static str,
    pub dev_only: bool,
    handler: CommandHandler,
}

pub struct ConsoleRegistry {
    commands: Vec<ConsoleCommand>,
}

impl ConsoleRegistry {
    pub fn new() -> Self {
        Self {
            commands: vec![
                ConsoleCommand {
                    name: "help",
                    aliases: &["commands"],
                    usage: "help [command]",
                    description: "show all commands or detailed help for one command",
                    dev_only: true,
                    handler: help_command,
                },
                ConsoleCommand {
                    name: "clear",
                    aliases: &[],
                    usage: "clear",
                    description: "clear the console output",
                    dev_only: true,
                    handler: clear_command,
                },
                ConsoleCommand {
                    name: "echo",
                    aliases: &["log"],
                    usage: "echo <message>",
                    description: "write a message to the console",
                    dev_only: true,
                    handler: echo_command,
                },
                ConsoleCommand {
                    name: "invoke",
                    aliases: &[],
                    usage: "invoke <ui>",
                    description: "open a preview window for a known UI",
                    dev_only: true,
                    handler: invoke_command,
                },
                ConsoleCommand {
                    name: "ui.list",
                    aliases: &[],
                    usage: "ui.list",
                    description: "list known preview UIs",
                    dev_only: true,
                    handler: ui_list_command,
                },
                ConsoleCommand {
                    name: "save.current",
                    aliases: &[],
                    usage: "save.current",
                    description: "show the currently selected save",
                    dev_only: true,
                    handler: save_current_command,
                },
                ConsoleCommand {
                    name: "save.list",
                    aliases: &[],
                    usage: "save.list",
                    description: "list saves with summary metadata",
                    dev_only: true,
                    handler: save_list_command,
                },
                ConsoleCommand {
                    name: "save.inspect",
                    aliases: &[],
                    usage: "save.inspect [name]",
                    description: "show a compact summary for a save",
                    dev_only: true,
                    handler: save_inspect_command,
                },
                ConsoleCommand {
                    name: "skills.list",
                    aliases: &[],
                    usage: "skills.list",
                    description: "list owned skills for the current save",
                    dev_only: true,
                    handler: skills_list_command,
                },
                ConsoleCommand {
                    name: "settings.get",
                    aliases: &[],
                    usage: "settings.get [key]",
                    description: "show console-related settings",
                    dev_only: true,
                    handler: settings_get_command,
                },
                ConsoleCommand {
                    name: "settings.set",
                    aliases: &[],
                    usage: "settings.set <key> <value>",
                    description: "change a whitelisted console setting",
                    dev_only: true,
                    handler: settings_set_command,
                },
                ConsoleCommand {
                    name: "grid.regen",
                    aliases: &[],
                    usage: "grid.regen",
                    description: "regenerate the open grid preview",
                    dev_only: true,
                    handler: grid_regen_command,
                },
                ConsoleCommand {
                    name: "grid.reset_view",
                    aliases: &[],
                    usage: "grid.reset_view",
                    description: "reset zoom and pan on the open grid preview",
                    dev_only: true,
                    handler: grid_reset_view_command,
                },
            ],
        }
    }

    pub fn commands(&self) -> &[ConsoleCommand] {
        &self.commands
    }

    pub fn execute(&self, line: &str, context: &mut dyn ConsoleCommandContext) -> Vec<String> {
        let Some(parsed) = parse_command_line(line) else {
            return Vec::new();
        };

        let Some(command) = self.find_command(&parsed.name) else {
            return vec![
                format!("Error: Unknown command: {}", parsed.name),
                "Type 'help' for a list of commands.".to_string(),
            ];
        };

        match (command.handler)(self, &parsed, context) {
            Ok(lines) => lines,
            Err(error) if error.starts_with("Usage:") || error.starts_with("Error:") => {
                vec![error]
            }
            Err(error) => vec![format!("Error: {}", error)],
        }
    }

    pub fn render_help_overview(&self) -> Vec<String> {
        let mut lines = vec!["Available commands:".to_string()];
        for command in &self.commands {
            lines.push(format!("  {} - {}", command.usage, command.description));
        }
        lines
    }

    pub fn render_command_help(&self, command_name: &str) -> Result<Vec<String>, String> {
        let command = self
            .find_command(command_name)
            .ok_or_else(|| format!("Unknown command: {}", command_name))?;
        let mut lines = vec![
            format!("Usage: {}", command.usage),
            format!("  {} - {}", command.name, command.description),
        ];
        if !command.aliases.is_empty() {
            lines.push(format!("Aliases: {}", command.aliases.join(", ")));
        }
        if command.dev_only {
            lines.push("Developer-only command.".to_string());
        }
        Ok(lines)
    }

    fn find_command(&self, name: &str) -> Option<&ConsoleCommand> {
        self.commands.iter().find(|command| {
            command.name.eq_ignore_ascii_case(name)
                || command
                    .aliases
                    .iter()
                    .any(|alias| alias.eq_ignore_ascii_case(name))
        })
    }
}

impl Default for ConsoleRegistry {
    fn default() -> Self {
        Self::new()
    }
}

fn help_command(
    registry: &ConsoleRegistry,
    parsed: &ParsedCommand,
    _context: &mut dyn ConsoleCommandContext,
) -> Result<Vec<String>, String> {
    match parsed.args.len() {
        0 => Ok(registry.render_help_overview()),
        1 => registry.render_command_help(&parsed.args[0]),
        _ => Err("Usage: help [command]".to_string()),
    }
}

fn clear_command(
    _registry: &ConsoleRegistry,
    parsed: &ParsedCommand,
    context: &mut dyn ConsoleCommandContext,
) -> Result<Vec<String>, String> {
    if !parsed.args.is_empty() {
        return Err("Usage: clear".to_string());
    }
    context.clear_console();
    Ok(Vec::new())
}

fn echo_command(
    _registry: &ConsoleRegistry,
    parsed: &ParsedCommand,
    _context: &mut dyn ConsoleCommandContext,
) -> Result<Vec<String>, String> {
    if parsed.args.is_empty() {
        return Err("Usage: echo <message>".to_string());
    }
    Ok(vec![parsed.args.join(" ")])
}

fn invoke_command(
    _registry: &ConsoleRegistry,
    parsed: &ParsedCommand,
    context: &mut dyn ConsoleCommandContext,
) -> Result<Vec<String>, String> {
    if parsed.args.is_empty() {
        return Err("Usage: invoke <ui>".to_string());
    }
    let name = parsed.args.join(" ");
    context.open_preview(&name)?;
    Ok(vec![format!("Invoked UI preview: {}", name)])
}

fn ui_list_command(
    _registry: &ConsoleRegistry,
    parsed: &ParsedCommand,
    context: &mut dyn ConsoleCommandContext,
) -> Result<Vec<String>, String> {
    if !parsed.args.is_empty() {
        return Err("Usage: ui.list".to_string());
    }
    let previews = context.list_previews();
    let mut lines = vec!["Known preview UIs:".to_string()];
    for preview in previews {
        lines.push(format!("  {}", preview));
    }
    Ok(lines)
}

fn save_current_command(
    _registry: &ConsoleRegistry,
    parsed: &ParsedCommand,
    context: &mut dyn ConsoleCommandContext,
) -> Result<Vec<String>, String> {
    if !parsed.args.is_empty() {
        return Err("Usage: save.current".to_string());
    }
    Ok(match context.current_save() {
        Some(save) => vec![format!("Current save: {}", save)],
        None => vec!["No current save selected.".to_string()],
    })
}

fn save_list_command(
    _registry: &ConsoleRegistry,
    parsed: &ParsedCommand,
    context: &mut dyn ConsoleCommandContext,
) -> Result<Vec<String>, String> {
    if !parsed.args.is_empty() {
        return Err("Usage: save.list".to_string());
    }
    let entries = read_save_list(context.saves_root())?;
    if entries.is_empty() {
        return Ok(vec!["No saves found.".to_string()]);
    }

    let mut lines = vec!["Saves:".to_string()];
    for entry in entries {
        let mut details = Vec::new();
        if let Some(difficulty) = entry.difficulty {
            details.push(format!("Difficulty: {}", difficulty));
        }
        if let Some(created_at) = entry.created_at {
            details.push(created_at);
        }
        if details.is_empty() {
            lines.push(format!("  {}", entry.folder_name));
        } else {
            lines.push(format!("  {} - {}", entry.folder_name, details.join(" | ")));
        }
    }
    Ok(lines)
}

fn save_inspect_command(
    _registry: &ConsoleRegistry,
    parsed: &ParsedCommand,
    context: &mut dyn ConsoleCommandContext,
) -> Result<Vec<String>, String> {
    if parsed.args.len() > 1 {
        return Err("Usage: save.inspect [name]".to_string());
    }
    let requested = parsed.args.first().map(String::as_str);
    let inspection = inspect_save(context.saves_root(), requested, context.current_save())?;
    let mut lines = vec![
        format!("Save: {}", inspection.save_name),
        format!("Folder: {}", inspection.folder_name),
    ];
    if let Some(difficulty) = inspection.difficulty {
        lines.push(format!("Difficulty: {}", difficulty));
    }
    lines.push(format!("Current floor: {}", inspection.current_floor));
    lines.push(format!("Level: {}", inspection.level));
    lines.push(format!("Coins: {}", inspection.coins));
    Ok(lines)
}

fn skills_list_command(
    _registry: &ConsoleRegistry,
    parsed: &ParsedCommand,
    context: &mut dyn ConsoleCommandContext,
) -> Result<Vec<String>, String> {
    if !parsed.args.is_empty() {
        return Err("Usage: skills.list".to_string());
    }
    let skills = context.read_owned_skills()?;
    if skills.is_empty() {
        return Ok(vec!["No owned skills found.".to_string()]);
    }

    let mut lines = vec!["Owned skills:".to_string()];
    for (name, level) in skills {
        lines.push(format!("  {} - level {}", name, level));
    }
    Ok(lines)
}

fn settings_get_command(
    _registry: &ConsoleRegistry,
    parsed: &ParsedCommand,
    context: &mut dyn ConsoleCommandContext,
) -> Result<Vec<String>, String> {
    match parsed.args.len() {
        0 => {
            let mut lines = vec!["Settings:".to_string()];
            for (key, value) in context.list_setting_values() {
                lines.push(format!("{} = {}", key, value));
            }
            Ok(lines)
        }
        1 => Ok(vec![format!(
            "{} = {}",
            parsed.args[0],
            context.get_setting_value(&parsed.args[0])?
        )]),
        _ => Err("Usage: settings.get [key]".to_string()),
    }
}

fn settings_set_command(
    _registry: &ConsoleRegistry,
    parsed: &ParsedCommand,
    context: &mut dyn ConsoleCommandContext,
) -> Result<Vec<String>, String> {
    if parsed.args.len() != 2 {
        return Err("Usage: settings.set <key> <value>".to_string());
    }
    let message = context.set_setting_value(&parsed.args[0], &parsed.args[1])?;
    Ok(vec![message])
}

fn grid_regen_command(
    _registry: &ConsoleRegistry,
    parsed: &ParsedCommand,
    context: &mut dyn ConsoleCommandContext,
) -> Result<Vec<String>, String> {
    if !parsed.args.is_empty() {
        return Err("Usage: grid.regen".to_string());
    }
    Ok(vec![context.regenerate_grid_preview()?])
}

fn grid_reset_view_command(
    _registry: &ConsoleRegistry,
    parsed: &ParsedCommand,
    context: &mut dyn ConsoleCommandContext,
) -> Result<Vec<String>, String> {
    if !parsed.args.is_empty() {
        return Err("Usage: grid.reset_view".to_string());
    }
    Ok(vec![context.reset_grid_preview()?])
}

pub fn read_save_list(root: &Path) -> Result<Vec<SaveListEntry>, String> {
    let mut entries = Vec::new();
    if !root.exists() {
        return Ok(entries);
    }

    let read_dir = fs::read_dir(root).map_err(|error| {
        format!(
            "Failed to read saves directory '{}': {}",
            root.display(),
            error
        )
    })?;

    for entry in read_dir.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let Some(folder_name) = path
            .file_name()
            .map(|value| value.to_string_lossy().to_string())
        else {
            continue;
        };

        let save_json = read_json_file(&path.join("save.json")).ok();
        let difficulty = save_json
            .as_ref()
            .and_then(|value| value.get("difficulty"))
            .and_then(|value| value.as_str())
            .map(ToOwned::to_owned);
        let created_at = save_json
            .as_ref()
            .and_then(|value| value.get("created_at"))
            .and_then(|value| value.as_str())
            .and_then(format_created_at);

        entries.push(SaveListEntry {
            folder_name: folder_name.clone(),
            save_name: folder_name.replace('_', " "),
            difficulty,
            created_at,
        });
    }

    entries.sort_by(|left, right| left.folder_name.cmp(&right.folder_name));
    Ok(entries)
}

pub fn inspect_save(
    root: &Path,
    requested_name: Option<&str>,
    current_save: Option<String>,
) -> Result<SaveInspection, String> {
    let folder_name = match requested_name {
        Some(name) => {
            resolve_save_folder(root, name).ok_or_else(|| format!("Save not found: {}", name))?
        }
        None => current_save.ok_or_else(|| "No current save selected.".to_string())?,
    };

    let save_dir = root.join(&folder_name);
    let save_json = read_json_file(&save_dir.join("save.json"))?;
    let player_json = fs::read_to_string(save_dir.join("player.json")).map_err(|error| {
        format!(
            "Failed to read player.json for '{}': {}",
            folder_name, error
        )
    })?;
    let player: Player = serde_json::from_str(&player_json).map_err(|error| {
        format!(
            "Failed to parse player.json for '{}': {}",
            folder_name, error
        )
    })?;

    let save_name = save_json
        .get("save_name")
        .and_then(|value| value.as_str())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| folder_name.replace('_', " "));
    let difficulty = save_json
        .get("difficulty")
        .and_then(|value| value.as_str())
        .map(ToOwned::to_owned);

    Ok(SaveInspection {
        folder_name,
        save_name,
        difficulty,
        current_floor: player.current_floor,
        level: player.level,
        coins: player.coins,
    })
}

pub fn read_player_skills_from_path(
    root: &Path,
    current_save: Option<String>,
) -> Result<Vec<(String, i8)>, String> {
    let save_name = current_save.ok_or_else(|| "No current save selected.".to_string())?;
    let player_path = root.join(save_name).join("player.json");
    let content = fs::read_to_string(&player_path)
        .map_err(|error| format!("Failed to read '{}': {}", player_path.display(), error))?;
    let player: Player = serde_json::from_str(&content)
        .map_err(|error| format!("Failed to parse '{}': {}", player_path.display(), error))?;
    let mut skills: Vec<(String, i8)> = player.skills.into_iter().collect();
    skills.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(skills)
}

fn resolve_save_folder(root: &Path, requested_name: &str) -> Option<String> {
    let direct = root.join(requested_name);
    if direct.is_dir() {
        return Some(requested_name.to_string());
    }

    let underscored = requested_name.replace(' ', "_");
    let fallback = root.join(&underscored);
    if fallback.is_dir() {
        return Some(underscored);
    }
    None
}

fn read_json_file(path: &Path) -> Result<serde_json::Value, String> {
    let content = fs::read_to_string(path)
        .map_err(|error| format!("Failed to read '{}': {}", path.display(), error))?;
    serde_json::from_str(&content)
        .map_err(|error| format!("Failed to parse '{}': {}", path.display(), error))
}

fn format_created_at(value: &str) -> Option<String> {
    chrono::DateTime::parse_from_rfc3339(value)
        .ok()
        .map(|datetime| format!("Created: {}", datetime.format("%Y-%m-%d %H:%M")))
}

/// Renders the console UI, including the log output and input field.
pub fn console_ui(ui: &mut Ui, state: &mut ConsoleState, max_lines: usize) {
    egui::ScrollArea::vertical()
        .auto_shrink([false; 2])
        .stick_to_bottom(true)
        .show(ui, |ui: &mut Ui| {
            let log_len = state.log.len();
            let start = log_len.saturating_sub(max_lines);
            for highlighted_line in &state.log[start..] {
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
                        if let Some(tokens) = &highlighted_line.tokens {
                            ui.horizontal(|ui: &mut Ui| {
                                for token in tokens {
                                    let text = match token.kind {
                                        TokenKind::Quote => egui::RichText::new(&token.text)
                                            .color(egui::Color32::GREEN),
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

    let input_resp = ui.add(
        TextEdit::singleline(&mut state.input as &mut dyn TextBuffer)
            .hint_text("Enter command...")
            .desired_width(f32::INFINITY),
    );
    ui.add_space(4.0);
    let pressed_enter =
        input_resp.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter));
    ui.horizontal(|ui: &mut Ui| {
        if ui
            .add_sized([64.0, 24.0], egui::Button::new("Run"))
            .clicked()
            || pressed_enter
        {
            let cmd = state.input.clone();
            if !cmd.trim().is_empty() {
                state.last_command = Some(cmd.clone());
            }
            state.pending.push(cmd);
            state.input.clear();
        }
        if ui
            .add_sized([64.0, 24.0], egui::Button::new("Clear"))
            .clicked()
        {
            state.clear();
        }
    });

    let input_focused = input_resp.has_focus();
    let up_pressed = ui.input(|input| input.key_pressed(egui::Key::ArrowUp));
    if input_focused && up_pressed {
        if let Some(cmd) = &state.last_command {
            state.input = cmd.clone();
        }
    }
}
