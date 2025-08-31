use crate::new_save::{show_new_save_ui, NewSaveState};
use egui::Ui;

pub struct SaveMenuState {
    #[allow(dead_code)]
    pub show_menu: bool,
    pub new_save_state: NewSaveState,
    pub in_new_save_menu: bool,
    pub editing_save: Option<String>,
    pub edit_save_name: String,
    pub confirm_delete: bool,
    pub delete_target: Option<String>,
    // Set to true when the top-level Back is clicked; caller can observe and react
    pub back_requested: bool,
}

impl Default for SaveMenuState {
    /// Returns a new `SaveMenuState` with default values for all fields.
    fn default() -> Self {
        Self {
            show_menu: false,
            new_save_state: NewSaveState::default(),
            in_new_save_menu: false,
            editing_save: None,
            edit_save_name: String::new(),
            confirm_delete: false,
            delete_target: None,
            back_requested: false,
        }
    }
}

/// Renders the save menu UI, including save slots and actions.
///
/// # Arguments
/// * `ui` - The egui UI to render into.
/// * `state` - The mutable state for the save menu.
pub fn show_save_ui(ui: &mut Ui, state: &mut SaveMenuState) {
    use egui::ColorImage;
    use egui::TextureHandle;
    use egui::Vec2;
    use image::GenericImageView;
    use image::ImageReader;
    use std::fs;
    use std::path::Path;

    // Handle delete confirmation dialog
    if (*state).confirm_delete {
        egui::Window::new("Confirm Delete")
            .collapsible(false)
            .resizable(false)
            .show(ui.ctx(), |ui: &mut Ui| {
                ui.label("Are you sure you want to delete this save? This cannot be undone.");
                ui.horizontal(|ui: &mut Ui| {
                    if (&ui.button("Delete")).clicked() {
                        if let Some(ref folder) = (*state).delete_target {
                            let _ = fs::remove_dir_all(Path::new("saves").join(folder));
                        }
                        (*state).confirm_delete = false;
                        (*state).delete_target = None;
                        (*state).editing_save = None;
                    }
                    if (&ui.button("Cancel")).clicked() {
                        (*state).confirm_delete = false;
                        (*state).delete_target = None;
                    }
                });
            });
    }

    if (*state).in_new_save_menu {
        // No back button in new save menu; only allow closing via Cancel or successful creation
        if show_new_save_ui(ui, &mut (*state).new_save_state) {
            (*state).in_new_save_menu = false;
            (&mut (*state).new_save_state).reset();
        }
        return;
    }

    // No top Back button; we'll place it at the bottom of the list instead

    // Edit save UI
    if let Some(ref folder_name) = (*state).editing_save {
        let folder_name: String = folder_name.clone(); // Clone to avoid borrowing issues
        let display_name: String = (&*folder_name).replace('_', " ");
        egui::Window::new("Edit Save")
            .collapsible(false)
            .resizable(false)
            .show(ui.ctx(), |ui: &mut Ui| {
                ui.label(format!("Editing save: {}", display_name));
                ui.horizontal(|ui: &mut Ui| {
                    ui.label("New name:");
                    ui.text_edit_singleline(&mut (*state).edit_save_name);
                });
                ui.add_space(10.0);
                ui.horizontal(|ui: &mut Ui| {
                    if (&ui.button("Rename")).clicked() {
                        let new_folder: String = (&*(*state).edit_save_name).trim().replace(' ', "_");
                        if !(&new_folder).is_empty() && new_folder != *folder_name {
                            let old_path: std::path::PathBuf =
                                Path::new("saves").join((&folder_name).clone());
                            let new_path: std::path::PathBuf = Path::new("saves").join(&new_folder);
                            if !(&*new_path).exists() {
                                let _ = fs::rename(&old_path, &new_path);
                                (*state).editing_save = Some(new_folder);
                            }
                        }
                    }
                    if (&ui.button("Delete")).clicked() {
                        (*state).confirm_delete = true;
                        (*state).delete_target = Some(folder_name);
                    }
                    if (&ui.button("Done")).clicked() {
                        (*state).editing_save = None;
                    }
                });
            });
        return;
    }

    if (&ui.button("Create New Save")).clicked() {
        (*state).in_new_save_menu = true;
        (&mut (*state).new_save_state).reset();
    }

    ui.add_space(20.0);

    // List all save folders in the 'saves' directory
    let saves_dir: &Path = Path::new("saves");
    if let Ok(entries) = fs::read_dir(saves_dir) {
        for entry in entries.flatten() {
            let path: std::path::PathBuf = (&entry).path();
            if (&*path).is_dir() {
                let folder_name: std::borrow::Cow<'_, str> =
                    (&*path).file_name().unwrap().to_string_lossy();
                let save_name: String = (&*folder_name).replace('_', " ");

                // Try to read save.json to get additional info
                let save_json_path: std::path::PathBuf = (&*path).join("save.json");
                let mut difficulty_text: String = String::new();
                let mut created_at_text: String = String::new();

                if let Ok(save_content) = fs::read_to_string(&save_json_path) {
                    if let Ok(save_data) = serde_json::from_str::<serde_json::Value>(&save_content)
                    {
                        if let Some(difficulty) = (&save_data)
                            .get("difficulty")
                            .and_then(|d: &serde_json::Value| -> Option<&str> { d.as_str() })
                        {
                            difficulty_text = format!("Difficulty: {}", difficulty);
                        }
                        if let Some(created_at) = (&save_data)
                            .get("created_at")
                            .and_then(|d: &serde_json::Value| -> Option<&str> { d.as_str() })
                        {
                            if let Ok(datetime) = chrono::DateTime::parse_from_rfc3339(created_at) {
                                created_at_text =
                                    format!("Created: {}", datetime.format("%Y-%m-%d %H:%M"));
                            }
                        }
                    }
                }

                let icon_path: std::path::PathBuf = (&*path).join("icon.png");
                let mut icon_texture: Option<TextureHandle> = None;
                if (&*icon_path).exists() {
                    if let Ok(img) = ImageReader::open(&icon_path).and_then(|r: image::ImageReader<std::io::BufReader<fs::File>>| -> Result<image::DynamicImage, std::io::Error> { r.decode().map_err(|e: image::ImageError| -> std::io::Error { std::io::Error::new(std::io::ErrorKind::Other, e) }) }) {
                        let size: (u32, u32) = (&img).dimensions();
                        let rgba: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = (&img).to_rgba8();
                        let pixels: image::FlatSamples<&[u8]> = (&rgba).as_flat_samples();
                        let color_image: ColorImage = ColorImage::from_rgba_unmultiplied(
                            [size.0 as usize, size.1 as usize],
                            (&pixels).as_slice()
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
                        if !(&difficulty_text).is_empty() {
                            ui.label(difficulty_text);
                        }
                        if !(&created_at_text).is_empty() {
                            ui.label(created_at_text);
                        }
                        ui.horizontal(|ui: &mut Ui| {
                            if (&ui.button("Load Save")).clicked() {
                                crate::set_current_save(&**&folder_name);
                            }
                            if (&ui.button("Edit")).clicked() {
                                (*state).editing_save = Some((&folder_name).to_string());
                                (*state).edit_save_name = (&save_name).clone();
                            }
                        });
                    });
                });
                ui.add_space(16.0); // Space between saves
            }
        }
    } else {
        ui.label("No saves found or saves directory doesn't exist yet.");
    }

    // Bottom Back button: below the list of saves
    ui.add_space(16.0);
    if (&ui
        .add_sized([120.0, 30.0], egui::Button::new("Back")))
        .clicked()
    {
        (*state).in_new_save_menu = false;
        (*state).editing_save = None;
        (*state).confirm_delete = false;
        (*state).delete_target = None;
        (*state).back_requested = true;
    }
}
