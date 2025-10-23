use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub fog: i8,
    pub lighting: i8,
    pub sound: bool,
    pub developer_mode: bool,
    pub verbose_logging: bool,
    pub show_console: bool,
    pub show_fps_graph: bool,
    pub log_to_console: bool,
    pub log_verbosity: LogVerbosity,
    pub fullscreen: bool,
    pub console_max_lines: usize,
    /// Show save creation date in saves menu
    pub show_save_creation_date: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LogVerbosity {
    Error = 0,
    Warn = 1,
    Info = 2,
    Debug = 3,
    Trace = 4,
}

impl Default for LogVerbosity {
    fn default() -> Self {
        LogVerbosity::Info
    }
}

const SETTINGS_FILE: &str = "settings.json";

impl Settings {
    /// Saves the current settings to a JSON file on disk.
    ///
    /// The file path is determined by the constant `SETTINGS_FILE`.
    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(SETTINGS_FILE, json);
        }
    }

    /// Loads settings from the JSON file on disk, or returns defaults if loading fails.
    ///
    /// # Returns
    /// * `Settings` - The loaded or default settings.
    pub fn load() -> Self {
        if Path::new(SETTINGS_FILE).exists() {
            if let Ok(data) = fs::read_to_string(SETTINGS_FILE) {
                if let Ok(settings) = serde_json::from_str::<Settings>(&data) {
                    return settings;
                }
            }
        }
        Settings::default_inner()
    }

    /// Returns the default settings values (used if loading fails).
    fn default_inner() -> Self {
        Self {
            fog: 2,
            lighting: 3,
            sound: true,
            developer_mode: false,
            verbose_logging: false,
            show_console: false,
            show_fps_graph: false,
            fullscreen: false,
            log_to_console: false,
            log_verbosity: LogVerbosity::Info,
            console_max_lines: 300,
            show_save_creation_date: true,
        }
    }
}

impl Default for Settings {
    /// Returns the default settings by loading from disk or using defaults.
    fn default() -> Self {
        Settings::load()
    }
}

pub struct SettingsResult {
    pub request_save: bool,
    pub request_back: bool,
}

impl Default for SettingsResult {
    /// Returns a new `SettingsResult` with all fields set to false.
    fn default() -> Self {
        Self {
            request_save: false,
            request_back: false,
        }
    }
}
