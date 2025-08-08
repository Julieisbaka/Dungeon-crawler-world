use egui::{Ui};
use crate::new_save::{show_new_save_ui, NewSaveState};

pub struct SaveMenuState {
    pub show_menu: bool,
    pub new_save_state: NewSaveState,
}

impl Default for SaveMenuState {
    fn default() -> Self {
        Self { 
            show_menu: false,
            new_save_state: NewSaveState::default(),
        }
    }
}

pub fn show_save_ui(ui: &mut Ui, state: &mut SaveMenuState) {
    use std::fs;
    use std::path::Path;
    use egui::TextureHandle;
    use egui::ColorImage;
    use egui::Vec2;
    use image::ImageReader as ImageReader;
    use image::GenericImageView;

    if (*state).new_save_state.show_new_save {
        if show_new_save_ui(ui, &mut (*state).new_save_state) {
            state.new_save_state.show_new_save = false;
            state.new_save_state.reset();
        }
        return;
    }

    if ui.button("Create New Save").clicked() {
        state.new_save_state.show_new_save = true;
        (*state).new_save_state.reset();
    }

    ui.add_space(20.0);

    // List all save folders in the 'saves' directory
    let saves_dir: &Path = Path::new("saves");
    if let Ok(entries) = fs::read_dir(saves_dir) {
        for entry in entries.flatten() {
            let path: std::path::PathBuf = entry.path();
            if path.is_dir() {
                let save_name: std::borrow::Cow<'_, str> = path.file_name().unwrap().to_string_lossy();
                
                // Try to read save.json to get additional info
                let save_json_path: std::path::PathBuf = path.join("save.json");
                let mut difficulty_text: String = String::new();
                let mut created_at_text: String = String::new();
                
                if let Ok(save_content) = fs::read_to_string(&save_json_path) {
                    if let Ok(save_data) = serde_json::from_str::<serde_json::Value>(&save_content) {
                        if let Some(difficulty) = save_data.get("difficulty").and_then(|d: &serde_json::Value| -> Option<&str> { d.as_str() }) {
                            difficulty_text = format!("Difficulty: {}", difficulty);
                        }
                        if let Some(created_at) = save_data.get("created_at").and_then(|d: &serde_json::Value| -> Option<&str> { d.as_str() }) {
                            if let Ok(datetime) = chrono::DateTime::parse_from_rfc3339(created_at) {
                                created_at_text = format!("Created: {}", datetime.format("%Y-%m-%d %H:%M"));
                            }
                        }
                    }
                }
                
                let icon_path: std::path::PathBuf = path.join("icon.png");
                let mut icon_texture: Option<TextureHandle> = None;
                if icon_path.exists() {
                    if let Ok(img) = ImageReader::open(&icon_path).and_then(|r: image::ImageReader<std::io::BufReader<fs::File>>| -> Result<image::DynamicImage, std::io::Error> { r.decode().map_err(|e: image::ImageError| -> std::io::Error { std::io::Error::new(std::io::ErrorKind::Other, e) }) }) {
                        let size = img.dimensions();
                        let rgba: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = img.to_rgba8();
                        let pixels: image::FlatSamples<&[u8]> = rgba.as_flat_samples();
                        let color_image: ColorImage = ColorImage::from_rgba_unmultiplied(
                            [size.0 as usize, size.1 as usize],
                            pixels.as_slice()
                        );
                        icon_texture = Some(ui.ctx().load_texture(
                            format!("{}_icon", save_name),
                            color_image,
                            egui::TextureOptions::default()
                        ));
                    }
                }
                ui.horizontal(|ui: &mut Ui| {
                    if let Some(texture) = &icon_texture {
                        ui.add(egui::Image::new(texture).fit_to_exact_size(Vec2::splat(64.0)));
                    } else {
                        ui.label("[No Icon]");
                    }
                    ui.vertical(|ui: &mut Ui| {
                        ui.label(format!("Save: {}", save_name));
                        if !difficulty_text.is_empty() {
                            ui.label(difficulty_text);
                        }
                        if !created_at_text.is_empty() {
                            ui.label(created_at_text);
                        }
                        if ui.button("Load Save").clicked() {
                            // TODO: Implement save loading logic
                        }
                    });
                });
                ui.add_space(16.0); // Space between saves
            }
        }
    } else {
        ui.label("No saves found or saves directory doesn't exist yet.");
    }
}