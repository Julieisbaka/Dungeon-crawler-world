use eframe::{App, Frame, NativeOptions};
use egui::{Color32, RichText, Style, Visuals, Ui};

#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
	pub fog: i32,
	pub lighting: i32,
	pub sound: bool,
}

impl Default for Settings {
	fn default() -> Self {
		Self {
			fog: 2,
			lighting: 3,
			sound: true,
		}
	}
}

pub fn settings_ui(ui: &mut Ui, settings: &mut Settings) {
	ui.heading("Settings");

	ui.horizontal(|ui| {
		ui.label("Fog:");
		egui::ComboBox::from_id_source("fog_combo")
			.selected_text(match settings.fog {
				0 => "No fog",
				1 => "Fast fog",
				2 => "Default fog",
				3 => "Fancy fog",
				_ => "Unknown",
			})
			.show_ui(ui, |ui| {
				ui.selectable_value(&mut settings.fog, 0, "No fog");
				ui.selectable_value(&mut settings.fog, 1, "Fast fog");
				ui.selectable_value(&mut settings.fog, 2, "Default fog");
				ui.selectable_value(&mut settings.fog, 3, "Fancy fog");
			});
	});

	ui.horizontal(|ui| {
		ui.label("Lighting:");
		egui::ComboBox::from_id_source("lighting_combo")
			.selected_text(match settings.lighting {
				0 => "No dynamic lighting",
				1 => "Non-shader lighting",
				2 => "Simple shader lighting",
				3 => "Normal shader lighting",
				4 => "Fancy shader lighting",
				5 => "Highest quality",
				_ => "Unknown",
			})
			.show_ui(ui, |ui| {
				ui.selectable_value(&mut settings.lighting, 0, "No dynamic lighting");
				ui.selectable_value(&mut settings.lighting, 1, "Non-shader lighting");
				ui.selectable_value(&mut settings.lighting, 2, "Simple shader lighting");
				ui.selectable_value(&mut settings.lighting, 3, "Normal shader lighting");
				ui.selectable_value(&mut settings.lighting, 4, "Fancy shader lighting");
				ui.selectable_value(&mut settings.lighting, 5, "Highest quality");
			});
	});

	ui.horizontal(|ui| {
		ui.label("Physically based sound:");
		ui.checkbox(&mut settings.sound, "Enable");
	});
}
