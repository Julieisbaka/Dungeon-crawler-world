use egui::{Ui, TextEdit};
use std::fs;
use std::path::Path;
use serde_json::{json, Value};
use rand::Rng;

#[derive(Clone, Debug, PartialEq, Eq)]
enum NewSaveTab {
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
    pub selected_tab: NewSaveTab,
    // Gamerules
    pub online_mode: bool,
    pub real_time: bool,
    pub error_message: String,
    pub success_message: String,
}

impl Default for NewSaveState {
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
    pub fn reset(&mut self) {
        (*self).save_name.clear();
        (*self).selected_difficulty = Difficulty::Medium;
    (*self).selected_tab = NewSaveTab::Basics;
    (*self).online_mode = false;
    (*self).real_time = false;
        (*self).error_message.clear();
        (*self).success_message.clear();
    }
}

pub fn show_new_save_ui(ui: &mut Ui, state: &mut NewSaveState) -> bool {
    let mut should_close = false;
    
    ui.vertical_centered(|ui: &mut Ui| {
        ui.heading("Create New Save");
        ui.add_space(20.0);
        // Tabs
        ui.horizontal(|ui: &mut Ui| {
            let basics_selected: bool = matches!((*state).selected_tab, NewSaveTab::Basics);
            if ui.selectable_label(basics_selected, "Basics").clicked() {
                (*state).selected_tab = NewSaveTab::Basics;
            }
            let gamerules_selected: bool = matches!((*state).selected_tab, NewSaveTab::Gamerules);
            if ui.selectable_label(gamerules_selected, "Gamerules").clicked() {
                (*state).selected_tab = NewSaveTab::Gamerules;
            }
        });

        ui.add_space(10.0);

        match (*state).selected_tab {
            NewSaveTab::Basics => {
                // Save name input
                ui.horizontal(|ui: &mut Ui| {
                    ui.label("Save Name:");
                    ui.add(TextEdit::singleline(&mut (*state).save_name)
                        .hint_text("Enter a unique save name..."));
                });

                ui.add_space(10.0);

                // Difficulty selection
                ui.horizontal(|ui: &mut Ui| {
                    ui.label("Difficulty:");
                    ui.radio_value(&mut (*state).selected_difficulty, Difficulty::Easy, "Easy");
                    ui.radio_value(&mut (*state).selected_difficulty, Difficulty::Medium, "Medium");
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
        ui.horizontal(|ui: &mut Ui| {
            if ui.button("Create Save").clicked() {
                if let Err(error) = create_new_save(
                    &(*state).save_name,
                    &(*state).selected_difficulty,
                    (*state).online_mode,
                    (*state).real_time,
                ) {
                    (*state).error_message = error;
                    (*state).success_message.clear();
                } else {
                    (*state).success_message = format!("Save '{}' created successfully!", state.save_name);
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

fn create_new_save(save_name: &str, difficulty: &Difficulty, online_mode: bool, real_time: bool) -> Result<(), String> {
    // Validate save name
    if save_name.trim().is_empty() {
        return Err("Save name cannot be empty".to_string());
    }
    // Check for invalid characters in save name
    if save_name.contains(&['/', '\\', ':', '*', '?', '"', '<', '>', '|'][..]) {
        return Err("Save name contains invalid characters".to_string());
    }
    // Store folder name with underscores instead of spaces
    let folder_name: String = save_name.trim().replace(' ', "_");
    let saves_dir: &Path = Path::new("saves");
    let save_path: std::path::PathBuf = saves_dir.join(&folder_name);
    // Check if save already exists
    if save_path.exists() {
        return Err("A save with this name already exists".to_string());
    }
    // Create saves directory if it doesn't exist
    if !saves_dir.exists() {
        fs::create_dir_all(saves_dir)
            .map_err(|e: std::io::Error| -> String { format!("Failed to create saves directory: {}", e) })?;
    }
    // Create save directory
    fs::create_dir_all(&save_path)
        .map_err(|e: std::io::Error| -> String { format!("Failed to create save directory: {}", e) })?;
    // Generate a time between 12h and 20h with a mean of 15h, then convert to seconds.
    // Use a triangular distribution with (min=12, mode=13, max=20) hours so that
    // the mean (a + b + c) / 3 = (12 + 20 + 13) / 3 = 15.
    // Implemented via inverse transform sampling from two uniforms.
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let u: f32 = rng.gen::<f32>();
    let (a, c, b): (f32, f32, f32) = (12.0, 13.0, 20.0);
    let fc: f32 = (c - a) / (b - a); // CDF at the mode
    let hours: f32 = if u < fc {
        a + ((b - a) * (c - a) * u).sqrt()
    } else {
        b - ((b - a) * (b - c) * (1.0 - u)).sqrt()
    };
    let floor_one_time_seconds: u32 = (hours * 3600.0).round() as u32;

    // Create save.json file including floor_one section
    let mut gamerules: Vec<&str> = Vec::new();
    if online_mode { gamerules.push("Online"); }
    if real_time { gamerules.push("Real-time"); }

    let save_data: Value = json!({
        "save_name": save_name.trim(),
        "difficulty": difficulty.to_string(),
        "created_at": chrono::Utc::now().to_rfc3339(),
        "current_floor": 1,
        "floor_one": {
            "is_cleared": false,
            "time": floor_one_time_seconds
        },
        "gamerules": gamerules
    });
    let save_file_path: std::path::PathBuf = save_path.join("save.json");
    fs::write(&save_file_path, serde_json::to_string_pretty(&save_data).unwrap())
        .map_err(|e: std::io::Error| -> String { format!("Failed to create save file: {}", e) })?;
    // Create player.json file (player info)
    // Initialize core skill values randomly in [3,5]
    let skill_min: i8 = 3;
    let skill_max: i8 = 5;
    let walking: i8 = rng.gen_range(skill_min..=skill_max);
    let swimming: i8 = rng.gen_range(skill_min..=skill_max);
    let breathing: i8 = rng.gen_range(skill_min..=skill_max);
    let strength: i16 = rng.gen_range(1..=8);
    let intelligence: i16 = rng.gen_range(3..=5);
    let dexterity: i16 = rng.gen_range(2..=6);
    let charisma: i16 = rng.gen_range(2..4);
    let constitution: i16 = rng.gen_range(2..6);

    let player_data: Value = json!({
        "name": "", // Unimplemented: Player name will be set later
        "level": 1,
        "spells": {},
        "inventory": {},
        "skills": {
            "Walking": walking,
            "Swimming": swimming,
            "Breathing": breathing
        },
        "coins": 0,
        "sub_classes": [],
        "class": "",
        "race": "",
        "has_manager": false,
        "stats": {
            "strength": strength,
            "intelligence": intelligence,
            "dexterity": dexterity,
            "charisma": charisma,
            "constitution": constitution
        }

    });
    let player_file_path: std::path::PathBuf = save_path.join("player.json");
    fs::write(&player_file_path, serde_json::to_string_pretty(&player_data).unwrap())
        .map_err(|e: std::io::Error| -> String { format!("Failed to create player file: {}", e) })?;
    Ok(())
}