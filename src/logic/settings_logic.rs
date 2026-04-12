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
    /// Target FPS cap (0 = unlimited).
    pub target_fps: u32,
    /// VSync mode (Off, On, or Adaptive).
    pub vsync_mode: VsyncMode,
    /// Show a simple FPS counter overlay for all users.
    pub show_fps_counter: bool,
    /// GPU power preference. Takes effect on next application start.
    pub power_preference: PowerPreference,
}

/// Controls vertical synchronisation behaviour.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum VsyncMode {
    /// No VSync — frames are presented as fast as possible; may exhibit tearing.
    Off = 0,
    /// Standard VSync — frames are synchronised to the display refresh rate (no tearing).
    On = 1,
    /// Adaptive VSync — syncs to the display when possible; allows tearing when the
    /// frame rate falls below the refresh rate to avoid stutter (FifoRelaxed).
    Adaptive = 2,
}

impl Default for VsyncMode {
    fn default() -> Self {
        VsyncMode::On
    }
}

/// GPU power preference used when selecting a graphics adapter.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PowerPreference {
    /// Let the driver/OS decide.
    Default = 0,
    /// Prefer the integrated / low-power GPU.
    LowPower = 1,
    /// Prefer the discrete / high-performance GPU.
    HighPerformance = 2,
}

impl Default for PowerPreference {
    fn default() -> Self {
        PowerPreference::Default
    }
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
            target_fps: 0,
            vsync_mode: VsyncMode::On,
            show_fps_counter: false,
            power_preference: PowerPreference::Default,
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
