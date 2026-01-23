use crate::logic::saves_logic::{SaveEntryCache, SaveMenuState};
use crate::logic::settings_logic::Settings;
use crate::new_save::show_new_save_ui;
use egui::Ui;

/// Loads save entries from disk and caches them in the state.
/// Only called when the cache is empty or invalidated.
fn load_save_cache(ui: &mut Ui, state: &mut SaveMenuState) {
    use egui::ColorImage;
    use image::GenericImageView;
    use image::ImageReader;
    use std::fs;
    use std::path::Path;

    state.save_cache.clear();
    let saves_dir = Path::new("saves");

    if let Ok(entries) = fs::read_dir(saves_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let Some(folder_name_osstr) = path.file_name() else {
                    continue;
                };
                let folder_name = folder_name_osstr.to_string_lossy().to_string();
                let save_name = folder_name.replace('_', " ");

                // Try to read save.json to get additional info
                let save_json_path = path.join("save.json");
                let mut difficulty_text = String::new();
                let mut created_at_text = String::new();
                if let Ok(save_content) = fs::read_to_string(&save_json_path) {
                    if let Ok(save_data) = serde_json::from_str::<serde_json::Value>(&save_content)
                    {
                        if let Some(difficulty) =
                            save_data.get("difficulty").and_then(|d| d.as_str())
                        {
                            difficulty_text = format!("Difficulty: {}", difficulty);
                        }
                        if let Some(created_at) =
                            save_data.get("created_at").and_then(|d| d.as_str())
                        {
                            if let Ok(datetime) = chrono::DateTime::parse_from_rfc3339(created_at) {
                                created_at_text =
                                    format!("Created: {}", datetime.format("%Y-%m-%d %H:%M"));
                            }
                        }
                    }
                }

                // Load and cache icon texture
                let icon_path = path.join("icon.png");
                let icon = if icon_path.exists() {
                    match ImageReader::open(&icon_path) {
                        Ok(reader) => match reader.decode() {
                            Ok(img) => {
                                let size = img.dimensions();
                                let rgba = img.to_rgba8();
                                let pixels = rgba.as_flat_samples();
                                let color_image = ColorImage::from_rgba_unmultiplied(
                                    [size.0 as usize, size.1 as usize],
                                    pixels.as_slice(),
                                );
                                Some(ui.ctx().load_texture(
                                    format!("{}_icon", folder_name),
                                    color_image,
                                    egui::TextureOptions::default(),
                                ))
                            }
                            Err(e) => {
                                log::warn!(
                                    "Failed to decode icon for save '{}': {}",
                                    folder_name,
                                    e
                                );
                                None
                            }
                        },
                        Err(e) => {
                            log::warn!("Failed to open icon for save '{}': {}", folder_name, e);
                            None
                        }
                    }
                } else {
                    None
                };

                state.save_cache.insert(
                    folder_name.clone(),
                    SaveEntryCache {
                        folder_name,
                        save_name,
                        difficulty_text,
                        created_at_text,
                        icon,
                    },
                );
            }
        }
    }
    state.cache_loaded = true;
}

/// Renders the save menu UI, including save slots and actions.
///
/// # Arguments
/// * `ui` - The egui UI to render into.
/// * `state` - The mutable state for the save menu.
/// * `settings` - The application settings (passed from parent to avoid disk I/O).
pub fn show_save_ui(ui: &mut Ui, state: &mut SaveMenuState, settings: &Settings) {
    use std::fs;
    use std::path::Path;

    // Handle delete confirmation dialog
    if state.confirm_delete {
        egui::Window::new("Confirm Delete")
            .collapsible(false)
            .resizable(false)
            .show(ui.ctx(), |ui: &mut Ui| {
                ui.label("Are you sure you want to delete this save? This cannot be undone.");
                ui.horizontal(|ui: &mut Ui| {
                    if ui.button("Delete").clicked() {
                        if let Some(ref folder) = state.delete_target {
                            let _ = fs::remove_dir_all(Path::new("saves").join(folder));
                        }
                        state.confirm_delete = false;
                        state.delete_target = None;
                        state.editing_save = None;
                        state.invalidate_cache(); // Invalidate cache after delete
                    }
                    if ui.button("Cancel").clicked() {
                        state.confirm_delete = false;
                        state.delete_target = None;
                    }
                });
            });
    }

    if state.in_new_save_menu {
        // No back button in new save menu; only allow closing via Cancel or successful creation
        if show_new_save_ui(ui, &mut state.new_save_state) {
            state.in_new_save_menu = false;
            state.new_save_state.reset();
            state.invalidate_cache(); // Invalidate cache after creating new save
        }
        return;
    }

    // Edit save UI
    if let Some(ref folder_name) = state.editing_save {
        let folder_name: String = folder_name.clone(); // Clone to avoid borrowing issues
        let display_name: String = folder_name.replace('_', " ");
        egui::Window::new("Edit Save")
            .collapsible(false)
            .resizable(false)
            .show(ui.ctx(), |ui: &mut Ui| {
                ui.label(format!("Editing save: {}", display_name));
                ui.horizontal(|ui: &mut Ui| {
                    ui.label("New name:");
                    ui.text_edit_singleline(&mut state.edit_save_name);
                });
                ui.add_space(10.0);
                ui.horizontal(|ui: &mut Ui| {
                    if ui.button("Rename").clicked() {
                        let new_folder: String = state.edit_save_name.trim().replace(' ', "_");
                        if !new_folder.is_empty() && new_folder != folder_name {
                            let old_path = Path::new("saves").join(&folder_name);
                            let new_path = Path::new("saves").join(&new_folder);
                            if !new_path.exists() && fs::rename(&old_path, &new_path).is_ok() {
                                state.editing_save = Some(new_folder);
                                state.invalidate_cache(); // Invalidate cache after rename
                            }
                        }
                    }
                    if ui.button("Delete").clicked() {
                        state.confirm_delete = true;
                        state.delete_target = Some(folder_name);
                    }
                    if ui.button("Done").clicked() {
                        state.editing_save = None;
                    }
                });
            });
        return;
    }

    if ui.button("Create New Save").clicked() {
        state.in_new_save_menu = true;
        state.new_save_state.reset();
        // Note: cache is invalidated after save creation in show_new_save_ui callback, not here
    }

    ui.add_space(20.0);

    // Load save cache if not already loaded
    if !state.cache_loaded {
        load_save_cache(ui, state);
    }

    // Render cached save entries
    if state.save_cache.is_empty() {
        ui.label("No saves found or saves directory doesn't exist yet.");
    } else {
        // Collect edit request to apply after iteration
        // (mutating state.editing_save would conflict with the borrow of state.save_cache)
        let mut edit_request: Option<(String, String)> = None;

        // Iterate directly over cache values to avoid cloning
        for entry in state.save_cache.values() {
            ui.horizontal(|ui: &mut Ui| {
                if let Some(ref texture) = entry.icon {
                    ui.add(egui::Image::new(texture).fit_to_exact_size(egui::Vec2::splat(64.0)));
                } else {
                    ui.label("[No Icon]");
                }
                ui.vertical(|ui: &mut Ui| {
                    ui.label(format!("Save: {}", entry.save_name));
                    if !entry.difficulty_text.is_empty() {
                        ui.label(&entry.difficulty_text);
                    }
                    if !entry.created_at_text.is_empty() && settings.show_save_creation_date {
                        ui.label(&entry.created_at_text);
                    }
                    ui.horizontal(|ui: &mut Ui| {
                        if ui.button("Load Save").clicked() {
                            // Update the library's current save tracker
                            if let Ok(mut g) = crate::CURRENT_SAVE.lock() {
                                *g = Some(entry.folder_name.clone());
                                log::info!("Current save set to: {}", entry.folder_name);
                            }
                        }
                        if ui.button("Edit").clicked() {
                            edit_request =
                                Some((entry.folder_name.clone(), entry.save_name.clone()));
                        }
                    });
                });
            });
            ui.add_space(16.0); // Space between saves
        }

        // Apply edit request after iteration to avoid borrow conflicts
        if let Some((folder_name, save_name)) = edit_request {
            state.editing_save = Some(folder_name);
            state.edit_save_name = save_name;
        }
    }

    // Bottom Back button: below the list of saves
    ui.add_space(16.0);
    if ui
        .add_sized([120.0, 30.0], egui::Button::new("Back"))
        .clicked()
    {
        state.in_new_save_menu = false;
        state.editing_save = None;
        state.confirm_delete = false;
        state.delete_target = None;
        state.back_requested = true;
    }
}
