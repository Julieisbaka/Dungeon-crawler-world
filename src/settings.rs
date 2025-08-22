use egui::Ui;
use serde::{Serialize, Deserialize};
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
	pub fullscreen: bool,
}

const SETTINGS_FILE: &str = "setting.json";

impl Settings {
    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(SETTINGS_FILE, json);
        }
    }

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
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings::load()
    }
}

pub struct SettingsResult {
	pub request_save: bool,
	pub request_back: bool,
}

impl Default for SettingsResult { fn default() -> Self { Self { request_save: false, request_back: false } } }

pub fn settings_ui(ui: &mut Ui, settings: &mut Settings, dev_mode_available: bool) -> SettingsResult {
	let mut result: SettingsResult = SettingsResult::default();
	// Only show heading once in the settings menu (handled in main.rs)

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
				if ui.selectable_value(&mut (*settings).fog, 0, "No fog").changed() { settings.save(); }
				if ui.selectable_value(&mut (*settings).fog, 1, "Fast fog").changed() { settings.save(); }
				if ui.selectable_value(&mut (*settings).fog, 2, "Default fog").changed() { settings.save(); }
				if ui.selectable_value(&mut (*settings).fog, 3, "Fancy fog").changed() { settings.save(); }
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
				if ui.selectable_value(&mut (*settings).lighting, 0, "No dynamic lighting").changed() { settings.save(); }
				if ui.selectable_value(&mut (*settings).lighting, 1, "Non-shader lighting").changed() { settings.save(); }
				if ui.selectable_value(&mut (*settings).lighting, 2, "Simple shader lighting").changed() { settings.save(); }
				if ui.selectable_value(&mut (*settings).lighting, 3, "Normal shader lighting").changed() { settings.save(); }
				if ui.selectable_value(&mut (*settings).lighting, 4, "Fancy shader lighting").changed() { settings.save(); }
				if ui.selectable_value(&mut (*settings).lighting, 5, "Highest quality").changed() { settings.save(); }
			});
	});

	ui.horizontal(|ui: &mut Ui| {
		ui.label("Physically based sound:");
		if ui.checkbox(&mut (*settings).sound, "Enable").changed() { settings.save(); }
	});

	if dev_mode_available {
		ui.separator();

		if ui.checkbox(&mut (*settings).developer_mode, "Developer Mode").changed() { settings.save(); }

		if (*settings).developer_mode {
			ui.group(|ui: &mut Ui| {
				ui.heading("Developer Options");
				if ui.checkbox(&mut (*settings).verbose_logging, "Verbose Logging").changed() { settings.save(); }
				if ui.checkbox(&mut (*settings).show_console, "In-game Console").changed() { settings.save(); }
				if ui.checkbox(&mut (*settings).show_fps_graph, "FPS Graph").changed() { settings.save(); }
			});
		}
	}

	ui.separator();
	if ui.checkbox(&mut (*settings).fullscreen, "Fullscreen").changed() { settings.save(); }
	ui.add_space(8.0);

	ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui: &mut Ui| {
		if ui.add_sized([100.0, 28.0], egui::Button::new("Save")).clicked() {
			settings.save();
			result.request_save = true;
		}
		ui.add_space(8.0);
		if ui.add_sized([100.0, 28.0], egui::Button::new("Back")).clicked() {
			result.request_back = true;
		}
	});

	result
}
