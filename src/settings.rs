use egui::Ui;
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
    /// Current version of the game
    pub current_version: String,
    /// Latest available version from GitHub
    pub latest_version: Option<String>,
    /// Whether to check for updates on startup
    pub check_updates_on_startup: bool,
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
                if let Ok(settings) = serde_json::from_str::<Settings>(&**&data) {
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
            current_version: env!("CARGO_PKG_VERSION").to_string(),
            latest_version: None,
            check_updates_on_startup: true,
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
    pub request_update: bool,
}

impl Default for SettingsResult {
    /// Returns a new `SettingsResult` with all fields set to false.
    fn default() -> Self {
        Self {
            request_save: false,
            request_back: false,
            request_update: false,
        }
    }
}

/// Renders the settings UI, allowing the user to modify and save settings.
///
/// # Arguments
/// * `ui` - The egui UI to render into.
/// * `settings` - The mutable settings object to edit.
/// * `dev_mode_available` - Whether developer mode options should be shown.
///
/// # Returns
/// * `SettingsResult` - Indicates if the user requested to save or go back.
pub fn settings_ui(
    ui: &mut Ui,
    settings: &mut Settings,
    dev_mode_available: bool,
) -> SettingsResult {
    let mut result: SettingsResult = SettingsResult::default();
    // Only show heading once in the settings menu (handled in main.rs)
    ui.horizontal(|ui: &mut Ui| {
        if (&ui.checkbox(
            &mut (*settings).show_save_creation_date,
            "Show save creation date in saves menu",
        ))
            .changed()
        {
            settings.save();
        }
    });

    ui.horizontal(|ui: &mut Ui| {
        ui.label("Fog:");
        egui::ComboBox::from_id_salt("fog_combo")
            .selected_text(match (*settings).fog {
                0 => "No fog",
                1 => "Fast fog",
                2 => "Default fog",
                3 => "Fancy fog",
                _ => "Unknown",
            })
            .show_ui(ui, |ui: &mut Ui| {
                if (&ui.selectable_value(&mut (*settings).fog, 0, "No fog")).changed() {
                    settings.save();
                }
                if (&ui.selectable_value(&mut (*settings).fog, 1, "Fast fog")).changed() {
                    settings.save();
                }
                if (&ui.selectable_value(&mut (*settings).fog, 2, "Default fog")).changed() {
                    settings.save();
                }
                if (&ui.selectable_value(&mut (*settings).fog, 3, "Fancy fog")).changed() {
                    settings.save();
                }
            });
    });

    ui.horizontal(|ui: &mut Ui| {
        ui.label("Lighting:");
        egui::ComboBox::from_id_salt("lighting_combo")
            .selected_text(match (*settings).lighting {
                0 => "No dynamic lighting",
                1 => "Non-shader lighting",
                2 => "Simple shader lighting",
                3 => "Normal shader lighting",
                4 => "Fancy shader lighting",
                5 => "Highest quality",
                _ => "Unknown",
            })
            .show_ui(ui, |ui: &mut Ui| {
                if (&ui.selectable_value(&mut (*settings).lighting, 0, "No dynamic lighting"))
                    .changed()
                {
                    settings.save();
                }
                if (&ui.selectable_value(&mut (*settings).lighting, 1, "Non-shader lighting"))
                    .changed()
                {
                    settings.save();
                }
                if (&ui.selectable_value(&mut (*settings).lighting, 2, "Simple shader lighting"))
                    .changed()
                {
                    settings.save();
                }
                if (&ui.selectable_value(&mut (*settings).lighting, 3, "Normal shader lighting"))
                    .changed()
                {
                    settings.save();
                }
                if (&ui.selectable_value(&mut (*settings).lighting, 4, "Fancy shader lighting"))
                    .changed()
                {
                    settings.save();
                }
                if (&ui.selectable_value(&mut (*settings).lighting, 5, "Highest quality")).changed()
                {
                    settings.save();
                }
            });
    });

    ui.horizontal(|ui: &mut Ui| {
        ui.label("Physically based sound:");
        if (&ui.checkbox(&mut (*settings).sound, "Enable")).changed() {
            settings.save();
        }
    });

    // UPDATE SECTION
    ui.separator();
    ui.heading("Updates");
    
    ui.horizontal(|ui: &mut Ui| {
        ui.label("Check for updates on startup:");
        if (&ui.checkbox(&mut (*settings).check_updates_on_startup, "Enable")).changed() {
            settings.save();
        }
    });
    
    ui.horizontal(|ui: &mut Ui| {
        ui.label("Current version:");
        ui.label(&(*settings).current_version);
    });
    
    if let Some(latest_version) = &(*settings).latest_version {
        ui.horizontal(|ui: &mut Ui| {
            ui.label("Latest version:");
            ui.label(latest_version);
        });
        
        ui.horizontal(|ui: &mut Ui| {
            if (&ui.add_sized([120.0, 28.0], egui::Button::new("Update Now"))).clicked() {
                result.request_update = true;
            }
            ui.label("⚠ A newer version is available!");
        });
    } else {
        ui.horizontal(|ui: &mut Ui| {
            ui.label("✓ You have the latest version");
        });
    }

    if dev_mode_available {
        ui.separator();

        if (&ui.checkbox(&mut (*settings).developer_mode, "Developer Mode")).changed() {
            settings.save();
        }

        if (*settings).developer_mode {
            ui.group(|ui: &mut Ui| {
                ui.heading("Developer Options");
                if (&ui.checkbox(&mut (*settings).verbose_logging, "Verbose Logging")).changed() {
                    settings.save();
                }
                if (&ui.checkbox(&mut (*settings).show_console, "In-game Console")).changed() {
                    settings.save();
                }
                if (&ui.checkbox(&mut (*settings).show_fps_graph, "FPS Graph")).changed() {
                    settings.save();
                }
                if (&ui.checkbox(&mut (*settings).log_to_console, "Log to in-game Console"))
                    .changed()
                {
                    settings.save();
                }
                ui.horizontal(|ui: &mut Ui| {
                    ui.label("Console max lines:");
                    let mut lines: u16 = (*settings).console_max_lines as u16;
                    if (&ui.add(egui::DragValue::new(&mut lines).range(50..=2000))).changed() {
                        (*settings).console_max_lines = lines as usize;
                        settings.save();
                    }
                });
                ui.horizontal(|ui: &mut Ui| {
                    ui.label("Log verbosity:");
                    let mut verbosity: LogVerbosity = (*settings).log_verbosity;
                    egui::ComboBox::from_id_salt("log_verbosity_combo")
                        .selected_text(match verbosity {
                            LogVerbosity::Error => "Error",
                            LogVerbosity::Warn => "Warn",
                            LogVerbosity::Info => "Info",
                            LogVerbosity::Debug => "Debug",
                            LogVerbosity::Trace => "Trace",
                        })
                        .show_ui(ui, |ui| {
                            for v in [
                                LogVerbosity::Error,
                                LogVerbosity::Warn,
                                LogVerbosity::Info,
                                LogVerbosity::Debug,
                                LogVerbosity::Trace,
                            ] {
                                let label: &str = match v {
                                    LogVerbosity::Error => "Error",
                                    LogVerbosity::Warn => "Warn",
                                    LogVerbosity::Info => "Info",
                                    LogVerbosity::Debug => "Debug",
                                    LogVerbosity::Trace => "Trace",
                                };
                                if (&ui.selectable_value(&mut verbosity, v, label)).changed() {
                                    (*settings).log_verbosity = v;
                                    settings.save();
                                }
                            }
                        });
                });
            });
        }
    }

    ui.separator();
    if (&ui.checkbox(&mut (*settings).fullscreen, "Fullscreen")).changed() {
        settings.save();
    }
    ui.add_space(8.0);

    ui.with_layout(
        egui::Layout::left_to_right(egui::Align::Center),
        |ui: &mut Ui| {
            if (&ui.add_sized([100.0, 28.0], egui::Button::new("Save"))).clicked() {
                settings.save();
                result.request_save = true;
            }
            ui.add_space(8.0);
            if (&ui.add_sized([100.0, 28.0], egui::Button::new("Back"))).clicked() {
                result.request_back = true;
            }
        },
    );

    result
}
