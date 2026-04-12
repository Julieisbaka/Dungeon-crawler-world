use crate::logic::settings_logic::{LogVerbosity, PowerPreference, Settings, SettingsResult, VsyncMode};
use egui::Ui;

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
    let mut result = SettingsResult::default();
    
    ui.horizontal(|ui: &mut Ui| {
        if ui.checkbox(
            &mut settings.show_save_creation_date,
            "Show save creation date in saves menu",
        ).changed() {
            settings.save();
        }
    });

    ui.horizontal(|ui: &mut Ui| {
        ui.label("Fog:");
        egui::ComboBox::from_id_salt("fog_combo")
            .selected_text(match settings.fog {
                0 => "No fog",
                1 => "Fast fog",
                2 => "Default fog",
                3 => "Fancy fog",
                _ => "Unknown",
            })
            .show_ui(ui, |ui: &mut Ui| {
                if ui.selectable_value(&mut settings.fog, 0, "No fog").changed() {
                    settings.save();
                }
                if ui.selectable_value(&mut settings.fog, 1, "Fast fog").changed() {
                    settings.save();
                }
                if ui.selectable_value(&mut settings.fog, 2, "Default fog").changed() {
                    settings.save();
                }
                if ui.selectable_value(&mut settings.fog, 3, "Fancy fog").changed() {
                    settings.save();
                }
            });
    });

    ui.horizontal(|ui: &mut Ui| {
        ui.label("Lighting:");
        egui::ComboBox::from_id_salt("lighting_combo")
            .selected_text(match settings.lighting {
                0 => "No dynamic lighting",
                1 => "Non-shader lighting",
                2 => "Simple shader lighting",
                3 => "Normal shader lighting",
                4 => "Fancy shader lighting",
                5 => "Highest quality",
                _ => "Unknown",
            })
            .show_ui(ui, |ui: &mut Ui| {
                if ui.selectable_value(&mut settings.lighting, 0, "No dynamic lighting").changed() {
                    settings.save();
                }
                if ui.selectable_value(&mut settings.lighting, 1, "Non-shader lighting").changed() {
                    settings.save();
                }
                if ui.selectable_value(&mut settings.lighting, 2, "Simple shader lighting").changed() {
                    settings.save();
                }
                if ui.selectable_value(&mut settings.lighting, 3, "Normal shader lighting").changed() {
                    settings.save();
                }
                if ui.selectable_value(&mut settings.lighting, 4, "Fancy shader lighting").changed() {
                    settings.save();
                }
                if ui.selectable_value(&mut settings.lighting, 5, "Highest quality").changed() {
                    settings.save();
                }
            });
    });

    ui.horizontal(|ui: &mut Ui| {
        ui.label("Physically based sound:");
        if ui.checkbox(&mut settings.sound, "Enable").changed() {
            settings.save();
        }
    });

    ui.separator();
    ui.heading("Performance");
    ui.add_space(4.0);

    ui.horizontal(|ui: &mut Ui| {
        ui.label("VSync:");
        egui::ComboBox::from_id_salt("vsync_mode_combo")
            .selected_text(match settings.vsync_mode {
                VsyncMode::Off => "Off",
                VsyncMode::On => "On",
                VsyncMode::Adaptive => "Adaptive",
            })
            .show_ui(ui, |ui: &mut Ui| {
                if ui
                    .selectable_value(&mut settings.vsync_mode, VsyncMode::Off, "Off")
                    .changed()
                {
                    settings.save();
                }
                if ui
                    .selectable_value(&mut settings.vsync_mode, VsyncMode::On, "On")
                    .changed()
                {
                    settings.save();
                }
                if ui
                    .selectable_value(
                        &mut settings.vsync_mode,
                        VsyncMode::Adaptive,
                        "Adaptive",
                    )
                    .changed()
                {
                    settings.save();
                }
            });
        ui.label(
            egui::RichText::new("Adaptive allows tearing below the display refresh rate to reduce stutter")
                .small()
                .color(egui::Color32::GRAY),
        );
    });

    ui.horizontal(|ui: &mut Ui| {
        ui.label("FPS cap:");
        egui::ComboBox::from_id_salt("target_fps_combo")
            .selected_text(match settings.target_fps {
                0 => "Unlimited".to_string(),
                n => format!("{} FPS", n),
            })
            .show_ui(ui, |ui: &mut Ui| {
                for &fps in &[0u32, 15, 20, 24, 30, 45, 60, 75, 90, 100, 120, 144, 165, 200, 240, 360] {
                    let label = if fps == 0 {
                        "Unlimited".to_string()
                    } else {
                        format!("{} FPS", fps)
                    };
                    if ui.selectable_value(&mut settings.target_fps, fps, label).changed() {
                        settings.save();
                    }
                }
            });
    });

    ui.horizontal(|ui: &mut Ui| {
        if ui.checkbox(&mut settings.show_fps_counter, "Show FPS counter").changed() {
            settings.save();
        }
    });

    ui.horizontal(|ui: &mut Ui| {
        ui.label("GPU power preference:");
        egui::ComboBox::from_id_salt("power_preference_combo")
            .selected_text(match settings.power_preference {
                PowerPreference::Default => "Default",
                PowerPreference::LowPower => "Low power",
                PowerPreference::HighPerformance => "High performance",
            })
            .show_ui(ui, |ui: &mut Ui| {
                if ui
                    .selectable_value(
                        &mut settings.power_preference,
                        PowerPreference::Default,
                        "Default",
                    )
                    .changed()
                {
                    settings.save();
                }
                if ui
                    .selectable_value(
                        &mut settings.power_preference,
                        PowerPreference::LowPower,
                        "Low power",
                    )
                    .changed()
                {
                    settings.save();
                }
                if ui
                    .selectable_value(
                        &mut settings.power_preference,
                        PowerPreference::HighPerformance,
                        "High performance",
                    )
                    .changed()
                {
                    settings.save();
                }
            });
        ui.label(
            egui::RichText::new("(takes effect on restart)")
                .small()
                .color(egui::Color32::GRAY),
        );
    });

    if dev_mode_available {
        ui.separator();

        if ui.checkbox(&mut settings.developer_mode, "Developer Mode").changed() {
            settings.save();
        }

        if settings.developer_mode {
            ui.group(|ui: &mut Ui| {
                ui.heading("Developer Options");
                if ui.checkbox(&mut settings.verbose_logging, "Verbose Logging").changed() {
                    settings.save();
                }
                if ui.checkbox(&mut settings.show_console, "In-game Console").changed() {
                    settings.save();
                }
                if ui.checkbox(&mut settings.show_fps_graph, "FPS Graph").changed() {
                    settings.save();
                }
                if ui.checkbox(&mut settings.log_to_console, "Log to in-game Console").changed() {
                    settings.save();
                }
                ui.horizontal(|ui: &mut Ui| {
                    ui.label("Console max lines:");
                    let mut lines = settings.console_max_lines as u16;
                    if ui.add(egui::DragValue::new(&mut lines).range(50..=2000)).changed() {
                        settings.console_max_lines = lines as usize;
                        settings.save();
                    }
                });
                ui.horizontal(|ui: &mut Ui| {
                    ui.label("Log verbosity:");
                    let mut verbosity = settings.log_verbosity;
                    egui::ComboBox::from_id_salt("log_verbosity_combo")
                        .selected_text(match verbosity {
                            LogVerbosity::Error => "Error",
                            LogVerbosity::Warn => "Warn",
                            LogVerbosity::Info => "Info",
                            LogVerbosity::Debug => "Debug",
                            LogVerbosity::Trace => "Trace",
                        })
                        .show_ui(ui, |ui: &mut Ui| {
                            for v in [
                                LogVerbosity::Error,
                                LogVerbosity::Warn,
                                LogVerbosity::Info,
                                LogVerbosity::Debug,
                                LogVerbosity::Trace,
                            ] {
                                let label = match v {
                                    LogVerbosity::Error => "Error",
                                    LogVerbosity::Warn => "Warn",
                                    LogVerbosity::Info => "Info",
                                    LogVerbosity::Debug => "Debug",
                                    LogVerbosity::Trace => "Trace",
                                };
                                if ui.selectable_value(&mut verbosity, v, label).changed() {
                                    settings.log_verbosity = v;
                                    settings.save();
                                }
                            }
                        });
                });
            });
        }
    }

    ui.separator();
    if ui.checkbox(&mut settings.fullscreen, "Fullscreen").changed() {
        settings.save();
    }
    ui.add_space(8.0);

    ui.with_layout(
        egui::Layout::left_to_right(egui::Align::Center),
        |ui: &mut Ui| {
            if ui.add_sized([100.0, 28.0], egui::Button::new("Save")).clicked() {
                settings.save();
                result.request_save = true;
            }
            ui.add_space(8.0);
            if ui.add_sized([100.0, 28.0], egui::Button::new("Back")).clicked() {
                result.request_back = true;
            }
        },
    );

    result
}
