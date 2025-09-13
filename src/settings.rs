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
    pub fullscreen: bool,
    pub console_max_lines: usize,
    /// Show save creation date in saves menu
    pub show_save_creation_date: bool,
}

const SETTINGS_FILE: &str = "setting.json";

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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::path::PathBuf;

    fn test_settings_file_path(temp_dir: &TempDir) -> PathBuf {
        temp_dir.path().join("test_settings.json")
    }

    #[test]
    fn test_settings_default_values() {
        let settings = Settings::default_inner();
        
        assert_eq!(settings.fog, 2);
        assert_eq!(settings.lighting, 3);
        assert_eq!(settings.sound, true);
        assert_eq!(settings.developer_mode, false);
        assert_eq!(settings.verbose_logging, false);
        assert_eq!(settings.show_console, false);
        assert_eq!(settings.show_fps_graph, false);
        assert_eq!(settings.fullscreen, false);
        assert_eq!(settings.log_to_console, false);
        assert_eq!(settings.console_max_lines, 300);
        assert_eq!(settings.show_save_creation_date, true);
    }

    #[test]
    fn test_settings_serialization() {
        let settings = Settings::default_inner();
        
        // Test serialization
        let json = serde_json::to_string(&settings).unwrap();
        assert!(json.contains("fog"));
        assert!(json.contains("lighting"));
        assert!(json.contains("sound"));
        
        // Test deserialization
        let deserialized: Settings = serde_json::from_str(&json).unwrap();
        assert_eq!(settings, deserialized);
    }

    #[test]
    fn test_settings_pretty_serialization() {
        let settings = Settings::default_inner();
        
        let pretty_json = serde_json::to_string_pretty(&settings).unwrap();
        assert!(pretty_json.contains("{\n"));
        assert!(pretty_json.contains("  \"fog\""));
        assert!(pretty_json.len() > serde_json::to_string(&settings).unwrap().len());
    }

    #[test]
    fn test_settings_partial_deserialization() {
        // Test that settings can be loaded even with missing fields (using defaults)
        let partial_json = r#"{"fog": 5, "sound": false}"#;
        
        let settings: Settings = serde_json::from_str(partial_json).unwrap();
        assert_eq!(settings.fog, 5);
        assert_eq!(settings.sound, false);
        // Other fields should have default values
        assert_eq!(settings.lighting, 3);
        assert_eq!(settings.developer_mode, false);
    }

    #[test]
    fn test_settings_with_all_features_enabled() {
        let mut settings = Settings::default_inner();
        settings.developer_mode = true;
        settings.verbose_logging = true;
        settings.show_console = true;
        settings.show_fps_graph = true;
        settings.fullscreen = true;
        settings.log_to_console = true;
        
        let json = serde_json::to_string(&settings).unwrap();
        let deserialized: Settings = serde_json::from_str(&json).unwrap();
        
        assert_eq!(deserialized.developer_mode, true);
        assert_eq!(deserialized.verbose_logging, true);
        assert_eq!(deserialized.show_console, true);
        assert_eq!(deserialized.show_fps_graph, true);
        assert_eq!(deserialized.fullscreen, true);
        assert_eq!(deserialized.log_to_console, true);
    }

    #[test]
    fn test_settings_fog_lighting_ranges() {
        let mut settings = Settings::default_inner();
        
        // Test extreme values for fog and lighting
        settings.fog = -10;
        settings.lighting = 100;
        
        let json = serde_json::to_string(&settings).unwrap();
        let deserialized: Settings = serde_json::from_str(&json).unwrap();
        
        assert_eq!(deserialized.fog, -10);
        assert_eq!(deserialized.lighting, 100);
    }

    #[test]
    fn test_settings_console_max_lines() {
        let mut settings = Settings::default_inner();
        settings.console_max_lines = 1000;
        
        let json = serde_json::to_string(&settings).unwrap();
        let deserialized: Settings = serde_json::from_str(&json).unwrap();
        
        assert_eq!(deserialized.console_max_lines, 1000);
    }

    #[test]
    fn test_settings_result_default() {
        let result = SettingsResult::default();
        assert_eq!(result.request_save, false);
        assert_eq!(result.request_back, false);
    }

    #[test]
    fn test_invalid_json_fallback() {
        // Test that invalid JSON falls back to defaults
        let invalid_json = r#"{"fog": "not_a_number"}"#;
        
        let result = serde_json::from_str::<Settings>(invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_settings_clone() {
        let settings1 = Settings::default_inner();
        let settings2 = settings1.clone();
        
        assert_eq!(settings1, settings2);
        assert_eq!(settings1.fog, settings2.fog);
        assert_eq!(settings1.sound, settings2.sound);
    }

    #[test]
    fn test_settings_debug() {
        let settings = Settings::default_inner();
        let debug_str = format!("{:?}", settings);
        
        assert!(debug_str.contains("Settings"));
        assert!(debug_str.contains("fog"));
        assert!(debug_str.contains("sound"));
    }
}
