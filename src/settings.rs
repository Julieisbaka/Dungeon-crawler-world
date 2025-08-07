use egui::{Ui};

#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
	pub fog: i32,
	pub lighting: i32,
	pub sound: bool,
	pub developer_mode: bool,
	pub verbose_logging: bool,
	pub show_console: bool,
	pub show_fps_graph: bool,
}

impl Default for Settings {
	fn default() -> Self {
		Self {
			fog: 2,
			lighting: 3,
			sound: true,
			developer_mode: false,
			verbose_logging: false,
			show_console: false,
			show_fps_graph: false,
		}
	}
}

pub fn settings_ui(ui: &mut Ui, settings: &mut Settings, dev_mode_available: bool) {
	ui.heading("Settings");

	ui.horizontal(|ui| {
		ui.label("Fog:");
		egui::ComboBox::from_id_salt("fog_combo")
			.selected_text(match (*settings).fog {
				0 => "No fog",
				1 => "Fast fog",
				2 => "Default fog",
				3 => "Fancy fog",
				_ => "Unknown",
			})
			.show_ui(ui, |ui| {
				ui.selectable_value(&mut (*settings).fog, 0, "No fog");
				ui.selectable_value(&mut (*settings).fog, 1, "Fast fog");
				ui.selectable_value(&mut (*settings).fog, 2, "Default fog");
				ui.selectable_value(&mut (*settings).fog, 3, "Fancy fog");
			});
	});

	ui.horizontal(|ui| {
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
			.show_ui(ui, |ui| {
				ui.selectable_value(&mut (*settings).lighting, 0, "No dynamic lighting");
				ui.selectable_value(&mut (*settings).lighting, 1, "Non-shader lighting");
				ui.selectable_value(&mut (*settings).lighting, 2, "Simple shader lighting");
				ui.selectable_value(&mut (*settings).lighting, 3, "Normal shader lighting");
				ui.selectable_value(&mut (*settings).lighting, 4, "Fancy shader lighting");
				ui.selectable_value(&mut (*settings).lighting, 5, "Highest quality");
			});
	});

	ui.horizontal(|ui| {
		ui.label("Physically based sound:");
		ui.checkbox(&mut (*settings).sound, "Enable");
	});

	if dev_mode_available {
		ui.separator();

		ui.checkbox(&mut settings.developer_mode, "Developer Mode");

		if settings.developer_mode {
			ui.group(|ui| {
				ui.heading("Developer Options");
				ui.checkbox(&mut settings.verbose_logging, "Verbose Logging");
				ui.checkbox(&mut settings.show_console, "In-game Console");
				ui.checkbox(&mut settings.show_fps_graph, "FPS Graph");
			});
		}
	}
}
