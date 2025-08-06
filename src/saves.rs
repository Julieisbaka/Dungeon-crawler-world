use egui::{Ui};

pub struct SaveMenuState {
    pub show_menu: bool,
}

impl Default for SaveMenuState {
    fn default() -> Self {
        Self { show_menu: false }
    }
}

pub fn show_save_ui(ui: &mut Ui, _state: &mut SaveMenuState) {
    use std::fs;
    use std::path::Path;
    use egui::TextureHandle;
    use egui::ColorImage;
    use egui::Vec2;
    use image::ImageReader as ImageReader;
    use image::GenericImageView;

    if ui.button("Create New Save").clicked() {
        //TODO: Creation logic
    }

    // List all save folders in the 'saves' directory
    let saves_dir: &Path = Path::new("saves");
    if let Ok(entries) = fs::read_dir(saves_dir) {
        for entry in entries.flatten() {
            let path: std::path::PathBuf = entry.path();
            if path.is_dir() {
                let save_name: std::borrow::Cow<'_, str> = path.file_name().unwrap().to_string_lossy();
                let icon_path: std::path::PathBuf = path.join("icon.png");
                let mut icon_texture: Option<TextureHandle> = None;
                if icon_path.exists() {
                    if let Ok(img) = ImageReader::open(&icon_path).and_then(|r: image::ImageReader<std::io::BufReader<fs::File>>| r.decode().map_err(|e: image::ImageError| std::io::Error::new(std::io::ErrorKind::Other, e))) {
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
                        // Extra space for future info
                        ui.add_space(32.0);
                    });
                });
                ui.add_space(16.0); // Space between saves
            }
        }
    } else {
        ui.label("No saves found.");
    }
}