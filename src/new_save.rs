use egui::{TextEdit, Ui};
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

#[derive(Clone, Debug, PartialEq)]
pub enum Difficulty {
    Easy = 0,
    Medium = 1,
    Hard = 2,
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Difficulty::Easy => write!(f, "Easy"),
            Difficulty::Medium => write!(f, "Medium"),
            Difficulty::Hard => write!(f, "Hard"),
        }
    }
}

pub struct NewSaveState {
    pub show_new_save: bool,
    pub save_name: String,
    pub selected_difficulty: Difficulty,
    pub error_message: String,
    pub success_message: String,
}

impl Default for NewSaveState {
    fn default() -> Self {
        Self {
            show_new_save: false,
            save_name: String::new(),
            selected_difficulty: Difficulty::Medium,
            error_message: String::new(),
            success_message: String::new(),
        }
    }
}

impl NewSaveState {
    pub fn reset(&mut self) {
        (*self).save_name.clear();
        (*self).selected_difficulty = Difficulty::Medium;
        (*self).error_message.clear();
        (*self).success_message.clear();
    }
}

pub fn show_new_save_ui(ui: &mut Ui, state: &mut NewSaveState) -> bool {
    let mut should_close = false;

    ui.vertical_centered(|ui| {
        ui.heading("Create New Save");
        ui.add_space(20.0);

        // Save name input
        ui.horizontal(|ui| {
            ui.label("Save Name:");
            ui.add(
                TextEdit::singleline(&mut (*state).save_name)
                    .hint_text("Enter a unique save name..."),
            );
        });

        ui.add_space(10.0);

        // Difficulty selection
        ui.horizontal(|ui| {
            ui.label("Difficulty:");
            ui.radio_value(&mut (*state).selected_difficulty, Difficulty::Easy, "Easy");
            ui.radio_value(
                &mut (*state).selected_difficulty,
                Difficulty::Medium,
                "Medium",
            );
            ui.radio_value(&mut (*state).selected_difficulty, Difficulty::Hard, "Hard");
        });

        ui.add_space(20.0);

        // Error message
        if !(*state).error_message.is_empty() {
            ui.colored_label(egui::Color32::RED, &(*state).error_message);
            ui.add_space(10.0);
        }

        // Success message
        if !(*state).success_message.is_empty() {
            ui.colored_label(egui::Color32::GREEN, &(*state).success_message);
            ui.add_space(10.0);
        }

        // Buttons
        ui.horizontal(|ui| {
            if ui.button("Create Save").clicked() {
                if let Err(error) =
                    create_new_save(&(*state).save_name, &(*state).selected_difficulty)
                {
                    (*state).error_message = error;
                    (*state).success_message.clear();
                } else {
                    (*state).success_message =
                        format!("Save '{}' created successfully!", state.save_name);
                    (*state).error_message.clear();
                    // Clear the save name after successful creation
                    (*state).save_name.clear();
                }
            }

            if ui.button("Cancel").clicked() {
                should_close = true;
            }
        });
    });

    should_close
}

/// Helper function to write JSON data to a file with proper error handling
fn write_json_file(path: &std::path::Path, data: &Value, file_type: &str) -> Result<(), String> {
    let json_string = serde_json::to_string_pretty(data)
        .map_err(|e| format!("Failed to serialize JSON for {}: {}", file_type, e))?;

    fs::write(path, json_string).map_err(|e| format!("Failed to write {}: {}", file_type, e))
}

fn create_new_save(save_name: &str, difficulty: &Difficulty) -> Result<(), String> {
    // Validate save name
    if save_name.trim().is_empty() {
        return Err("Save name cannot be empty".to_string());
    }

    // Check for invalid characters in save name
    if save_name.contains(&['/', '\\', ':', '*', '?', '"', '<', '>', '|'][..]) {
        return Err("Save name contains invalid characters".to_string());
    }

    // Store folder name with underscores instead of spaces
    let folder_name = save_name.trim().replace(' ', "_");
    let saves_dir = Path::new("saves");
    let save_path = saves_dir.join(&folder_name);

    // Check if save already exists
    if save_path.exists() {
        return Err("A save with this name already exists".to_string());
    }

    // Create saves directory if it doesn't exist
    if !saves_dir.exists() {
        fs::create_dir_all(saves_dir)
            .map_err(|e| format!("Failed to create saves directory: {}", e))?;
    }

    // Create save directory
    fs::create_dir_all(&save_path)
        .map_err(|e| format!("Failed to create save directory: {}", e))?;

    // Create save.json file (metadata only)
    let save_data = json!({
        "save_name": save_name.trim(),
        "difficulty": difficulty.to_string(),
        "created_at": chrono::Utc::now().to_rfc3339()
    });
    write_json_file(&save_path.join("save.json"), &save_data, "save file")?;

    // Create player.json file (player info)
    let player_data = json!({
        "name": "",
        "level": 1,
        "spells": {},
        "inventory": {},
        "skills": {},
        "coins": 0,
        "sub_classes": [],
        "class": "",
        "race": "",
        "has_manager": false
    });
    write_json_file(&save_path.join("player.json"), &player_data, "player file")?;

    Ok(())
}
