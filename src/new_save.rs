use crate::player::{Player, PlayerStats};
use egui::{TextBuffer, TextEdit, Ui};
use rand::Rng;
use serde_json::{json, Value};
use std::fs;
use std::path::{Component, Path};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NewSaveTab {
    Basics = 0,
    Gamerules = 1,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Difficulty {
    Easy = 0,
    Medium = 1,
    Hard = 2,
}

impl std::fmt::Display for Difficulty {
    /// Formats the `Difficulty` enum as a user-friendly string.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Difficulty::Easy => write!(f, "Easy"),
            &Difficulty::Medium => write!(f, "Medium"),
            &Difficulty::Hard => write!(f, "Hard"),
        }
    }
}

pub struct NewSaveState {
    #[allow(dead_code)]
    pub show_new_save: bool,
    pub save_name: String,
    pub selected_difficulty: Difficulty,
    pub selected_tab: NewSaveTab,
    // Gamerules
    pub online_mode: bool,
    pub real_time: bool,
    pub error_message: String,
    pub success_message: String,
}

impl Default for NewSaveState {
    /// Returns a new `NewSaveState` with default values for all fields.
    fn default() -> Self {
        Self {
            show_new_save: false,
            save_name: String::new(),
            selected_difficulty: Difficulty::Medium,
            selected_tab: NewSaveTab::Basics,
            online_mode: false,
            real_time: false,
            error_message: String::new(),
            success_message: String::new(),
        }
    }
}

impl NewSaveState {
    /// Resets the state to its default values, clearing all fields.
    pub fn reset(&mut self) {
        (&mut (*self).save_name).clear();
        (*self).selected_difficulty = Difficulty::Medium;
        (*self).selected_tab = NewSaveTab::Basics;
        (*self).online_mode = false;
        (*self).real_time = false;
        (&mut (*self).error_message).clear();
        (&mut (*self).success_message).clear();
    }
}

/// Renders the UI for creating a new save, including tabs and input fields.
///
/// # Arguments
/// * `ui` - The egui UI to render into.
/// * `state` - The mutable state for the new save dialog.
///
/// # Returns
/// * `bool` - Returns true if the dialog should be closed, false otherwise.
pub fn show_new_save_ui(ui: &mut Ui, state: &mut NewSaveState) -> bool {
    let mut should_close: bool = false;

    ui.vertical_centered(|ui: &mut Ui| {
        ui.heading("Create New Save");
        ui.add_space(20.0);
        // Tabs
        ui.horizontal(|ui: &mut Ui| {
            let basics_selected: bool = matches!((*state).selected_tab, NewSaveTab::Basics);
            if (&ui.selectable_label(basics_selected, "Basics")).clicked() {
                (*state).selected_tab = NewSaveTab::Basics;
            }
            let gamerules_selected: bool = matches!((*state).selected_tab, NewSaveTab::Gamerules);
            if (&ui.selectable_label(gamerules_selected, "Gamerules")).clicked() {
                (*state).selected_tab = NewSaveTab::Gamerules;
            }
        });

        ui.add_space(10.0);

        match (*state).selected_tab {
            NewSaveTab::Basics => {
                // Save name input
                ui.horizontal(|ui: &mut Ui| {
                    ui.label("Save Name:");
                    ui.add(
                        TextEdit::singleline(&mut (*state).save_name as &mut dyn TextBuffer)
                            .hint_text("Enter a unique save name..."),
                    );
                });

                ui.add_space(10.0);

                // Difficulty selection
                ui.horizontal(|ui: &mut Ui| {
                    ui.label("Difficulty:");
                    ui.radio_value(&mut (*state).selected_difficulty, Difficulty::Easy, "Easy");
                    ui.radio_value(
                        &mut (*state).selected_difficulty,
                        Difficulty::Medium,
                        "Medium",
                    );
                    ui.radio_value(&mut (*state).selected_difficulty, Difficulty::Hard, "Hard");
                });
            }
            NewSaveTab::Gamerules => {
                ui.checkbox(&mut (*state).online_mode, "Online mode");
                ui.checkbox(&mut (*state).real_time, "Real-time");
            }
        }

        ui.add_space(20.0);

        // Error message
        if !(&(*state).error_message).is_empty() {
            ui.colored_label(egui::Color32::RED, &(*state).error_message);
            ui.add_space(10.0);
        }

        // Success message
        if !(&(*state).success_message).is_empty() {
            ui.colored_label(egui::Color32::GREEN, &(*state).success_message);
            ui.add_space(10.0);
        }

        // Buttons
        ui.horizontal(|ui: &mut Ui| {
            if (&ui.button("Create Save")).clicked() {
                if let Err(error) = create_new_save(
                    &**&(*state).save_name,
                    &(*state).selected_difficulty,
                    (*state).online_mode,
                    (*state).real_time,
                ) {
                    (*state).error_message = error;
                    (&mut (*state).success_message).clear();
                } else {
                    (&mut (*state).error_message).clear();
                    // Close the dialog after successful creation
                    should_close = true;
                }
            }

            if (&ui.button("Cancel")).clicked() {
                should_close = true;
            }
        });
    });

    should_close
}

/// Generate the time for floor one based on real_time flag.
pub fn generate_floor_one_time<R: Rng + ?Sized>(real_time: bool, rng: &mut R) -> u32 {
    if real_time {
        432_000 // 5 days in seconds
    } else {
        // Triangular distribution: min=12, mode=13, max=20 hours
        let u: f32 = rng.gen::<f32>();
        let (a, c, b): (f32, f32, f32) = (12.0, 13.0, 20.0);
        let fc: f32 = (c - a) / (b - a);
        let hours: f32 = if u < fc {
            a + ((b - a) * (c - a) * u).sqrt()
        } else {
            b - ((b - a) * (b - c) * (1.0 - u)).sqrt()
        };
        (hours * 3600.0).round() as u32
    }
}

/// Generate random skills and stats for a new player.
pub fn generate_stats<R: Rng + ?Sized>(rng: &mut R) -> (i8, i8, i8, i16, i16, i16, i16, i16) {
    let skill_min: i8 = 3;
    let skill_max: i8 = 5;
    let walking: i8 = rng.gen_range(skill_min..=skill_max);
    let swimming: i8 = rng.gen_range(skill_min..=skill_max);
    let breathing: i8 = rng.gen_range(skill_min..=skill_max);
    let strength: i16 = rng.gen_range(1..=8);
    let intelligence: i16 = rng.gen_range(3..=5);
    let dexterity: i16 = rng.gen_range(2..=6);
    let charisma: i16 = rng.gen_range(2..=4);
    let constitution: i16 = rng.gen_range(2..=6);
    (
        walking,
        swimming,
        breathing,
        strength,
        intelligence,
        dexterity,
        charisma,
        constitution,
    )
}

fn create_new_save(
    save_name: &str,
    difficulty: &Difficulty,
    online_mode: bool,
    real_time: bool,
) -> Result<(), String> {
    // Validate save name
    if save_name.trim().is_empty() {
        return Err("Save name cannot be empty".to_string());
    }
    // Store folder name with underscores instead of spaces
    let folder_name: String = save_name.trim().replace(' ', "_");
    // Check for invalid characters in save name
    if has_invalid_save_characters(&folder_name) {
        return Err("Save name contains invalid characters".to_string());
    }
    if !is_safe_folder_name(&folder_name) {
        return Err("Save name must not reference relative paths".to_string());
    }
    let saves_dir: &Path = Path::new("saves");
    let save_path: std::path::PathBuf = saves_dir.join(&folder_name);
    // Check if save already exists
    if (&*save_path).exists() {
        return Err("A save with this name already exists".to_string());
    }
    // Create saves directory if it doesn't exist
    if !saves_dir.exists() {
        fs::create_dir_all(saves_dir).map_err(|e: std::io::Error| -> String {
            format!("Failed to create saves directory: {}", e)
        })?;
    }
    // Create save directory
    fs::create_dir_all(&save_path).map_err(|e: std::io::Error| -> String {
        format!("Failed to create save directory: {}", e)
    })?;
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let floor_one_time: u32 = generate_floor_one_time(real_time, &mut rng);

    // Create save.json file including floor_one section
    let mut gamerules: Vec<&str> = Vec::new();
    if online_mode {
        (&mut gamerules).push("Online");
    }
    if real_time {
        (&mut gamerules).push("Real-time");
    }

    let save_data: Value = json!({
        "save_name": save_name.trim(),
        "difficulty": difficulty.to_string(),
        "created_at": chrono::Utc::now().to_rfc3339(),
        "floor_one": {
            "is_cleared": false,
            "time": floor_one_time
        },
        "gamerules": gamerules
    });
    let save_file_path: std::path::PathBuf = (&*save_path).join("save.json");
    fs::write(
        &save_file_path,
        serde_json::to_string_pretty(&save_data).unwrap(),
    )
    .map_err(|e: std::io::Error| -> String { format!("Failed to create save file: {}", e) })?;
    // Create player.json file (player info)
    let (walking, swimming, breathing, strength, intelligence, dexterity, charisma, constitution) =
        generate_stats(&mut rng);
    use std::collections::HashMap;
    let mut skills: HashMap<String, i8> = HashMap::new();
    (&mut skills).insert("Walking".to_string(), walking);
    (&mut skills).insert("Swimming".to_string(), swimming);
    (&mut skills).insert("Breathing".to_string(), breathing);
    let stats: PlayerStats = PlayerStats {
        strength,
        intelligence,
        dexterity,
        charisma,
        constitution,
    };
    let player: Player = Player {
        name: "".to_string(),
        level: 1,
        spells: HashMap::new(),
        inventory: HashMap::new(),
        skills,
        coins: 0,
        sub_classes: Vec::new(),
        class: "".to_string(),
        race: "".to_string(),
        has_manager: false,
        current_floor: 1,
        stats,
    };
    let player_file_path: std::path::PathBuf = (&*save_path).join("player.json");
    fs::write(
        &player_file_path,
        serde_json::to_string_pretty(&player).unwrap(),
    )
    .map_err(|e: std::io::Error| -> String { format!("Failed to create player file: {}", e) })?;
    Ok(())
}

pub(crate) fn is_safe_folder_name(folder_name: &str) -> bool {
    if folder_name.is_empty() {
        return false;
    }
    Path::new(folder_name)
        .components()
        .all(|component| matches!(component, Component::Normal(_)))
}

pub(crate) fn has_invalid_save_characters(folder_name: &str) -> bool {
    folder_name
        .chars()
        .any(|c| ['/', '\\', ':', '*', '?', '"', '<', '>', '|'].contains(&c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_relative_paths_in_save_name() {
        assert!(!is_safe_folder_name("."));
        assert!(!is_safe_folder_name(".."));
        assert!(!is_safe_folder_name("../foo"));
        assert!(!is_safe_folder_name("foo/../bar"));
    }

    #[test]
    fn accepts_simple_folder_names() {
        assert!(is_safe_folder_name("Test_Save"));
        assert!(is_safe_folder_name("save1"));
    }

    #[test]
    fn rejects_invalid_characters() {
        assert!(has_invalid_save_characters("bad/name"));
        assert!(has_invalid_save_characters("bad:name"));
        assert!(has_invalid_save_characters("bad*name"));
        assert!(!has_invalid_save_characters("Good_Name"));
    }
}
