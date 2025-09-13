use egui::{TextBuffer, TextEdit, Ui};
use rand::Rng;
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

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

/// Validates a save name for correctness
pub fn validate_save_name(save_name: &str) -> Result<(), String> {
    if save_name.trim().is_empty() {
        return Err("Save name cannot be empty".to_string());
    }
    if save_name.contains(&(&['/', '\\', ':', '*', '?', '"', '<', '>', '|'])[..]) {
        return Err("Save name contains invalid characters".to_string());
    }
    Ok(())
}

/// Generates player stats with deterministic RNG for testing
pub fn generate_player_stats<R: Rng>(rng: &mut R) -> (i8, i8, i8, i16, i16, i16, i16, i16) {
    let skill_min: i8 = 3;
    let skill_max: i8 = 5;
    let walking = rng.gen_range(skill_min..=skill_max);
    let swimming = rng.gen_range(skill_min..=skill_max);
    let breathing = rng.gen_range(skill_min..=skill_max);
    let strength = rng.gen_range(1..=8);
    let intelligence = rng.gen_range(3..=5);
    let dexterity = rng.gen_range(2..=6);
    let charisma = rng.gen_range(2..4);
    let constitution = rng.gen_range(2..6);
    
    (walking, swimming, breathing, strength, intelligence, dexterity, charisma, constitution)
}

/// Generates floor one time with deterministic RNG
pub fn generate_floor_one_time<R: Rng>(rng: &mut R, real_time: bool) -> u32 {
    if real_time {
        432000 // 5 days in seconds
    } else {
        // Generate a time between 12h and 20h with a mean of 15h, then convert to seconds.
        // Use a triangular distribution with (min=12, mode=13, max=20) hours
        let u: f32 = rng.gen::<f32>();
        let (a, c, b): (f32, f32, f32) = (12.0, 13.0, 20.0);
        let fc: f32 = (c - a) / (b - a); // CDF at the mode
        let hours: f32 = if u < fc {
            a + ((b - a) * (c - a) * u).sqrt()
        } else {
            b - ((b - a) * (b - c) * (1.0 - u)).sqrt()
        };
        (hours * 3600.0).round() as u32
    }
}

/// Creates a new save with the given RNG for deterministic testing
pub fn create_new_save_with_rng<R: Rng>(
    save_name: &str,
    difficulty: &Difficulty,
    online_mode: bool,
    real_time: bool,
    rng: &mut R,
    saves_dir: &Path,
) -> Result<(), String> {
    validate_save_name(save_name)?;
    
    // Store folder name with underscores instead of spaces
    let folder_name: String = save_name.trim().replace(' ', "_");
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
    
    let floor_one_time = generate_floor_one_time(rng, real_time);
    
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
    
    // Generate player stats
    let (walking, swimming, breathing, strength, intelligence, dexterity, charisma, constitution) =
        generate_player_stats(rng);

    let player_data: Value = json!({
        "name": "",
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
        "current_floor": 1,
        "stats": {
            "strength": strength,
            "intelligence": intelligence,
            "dexterity": dexterity,
            "charisma": charisma,
            "constitution": constitution
        }
    });
    
    let player_file_path: std::path::PathBuf = (&*save_path).join("player.json");
    fs::write(
        &player_file_path,
        serde_json::to_string_pretty(&player_data).unwrap(),
    )
    .map_err(|e: std::io::Error| -> String { format!("Failed to create player file: {}", e) })?;
    
    Ok(())
}

fn create_new_save(
    save_name: &str,
    difficulty: &Difficulty,
    online_mode: bool,
    real_time: bool,
) -> Result<(), String> {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    create_new_save_with_rng(save_name, difficulty, online_mode, real_time, &mut rng, Path::new("saves"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use tempfile::TempDir;

    #[test]
    fn test_validate_save_name_valid() {
        assert!(validate_save_name("My Game Save").is_ok());
        assert!(validate_save_name("Test123").is_ok());
        assert!(validate_save_name("  Valid Name  ").is_ok());
    }

    #[test]
    fn test_validate_save_name_empty() {
        assert!(validate_save_name("").is_err());
        assert!(validate_save_name("   ").is_err());
    }

    #[test]
    fn test_validate_save_name_invalid_chars() {
        let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
        for ch in invalid_chars {
            let name = format!("Test{}Save", ch);
            assert!(validate_save_name(&name).is_err(), "Character '{}' should be invalid", ch);
        }
    }

    #[test]
    fn test_generate_player_stats_deterministic() {
        let mut rng1 = ChaCha8Rng::seed_from_u64(42);
        let stats1 = generate_player_stats(&mut rng1);

        let mut rng2 = ChaCha8Rng::seed_from_u64(42);
        let stats2 = generate_player_stats(&mut rng2);

        assert_eq!(stats1, stats2, "Same seed should produce identical stats");
    }

    #[test]
    fn test_generate_player_stats_ranges() {
        let mut rng = ChaCha8Rng::seed_from_u64(12345);
        
        for _ in 0..100 {
            let (walking, swimming, breathing, strength, intelligence, dexterity, charisma, constitution) = 
                generate_player_stats(&mut rng);
            
            // Test skill ranges (3-5)
            assert!(walking >= 3 && walking <= 5, "Walking skill out of range: {}", walking);
            assert!(swimming >= 3 && swimming <= 5, "Swimming skill out of range: {}", swimming);
            assert!(breathing >= 3 && breathing <= 5, "Breathing skill out of range: {}", breathing);
            
            // Test stat ranges
            assert!(strength >= 1 && strength <= 8, "Strength out of range: {}", strength);
            assert!(intelligence >= 3 && intelligence <= 5, "Intelligence out of range: {}", intelligence);
            assert!(dexterity >= 2 && dexterity <= 6, "Dexterity out of range: {}", dexterity);
            assert!(charisma >= 2 && charisma <= 4, "Charisma out of range: {}", charisma);
            assert!(constitution >= 2 && constitution <= 6, "Constitution out of range: {}", constitution);
        }
    }

    #[test]
    fn test_generate_floor_one_time_real_time() {
        let mut rng = ChaCha8Rng::seed_from_u64(54321);
        let time = generate_floor_one_time(&mut rng, true);
        assert_eq!(time, 432000, "Real-time mode should always return 5 days in seconds");
    }

    #[test]
    fn test_generate_floor_one_time_game_time() {
        let mut rng = ChaCha8Rng::seed_from_u64(54321);
        
        for _ in 0..100 {
            let time = generate_floor_one_time(&mut rng, false);
            let hours = time as f32 / 3600.0;
            assert!(hours >= 12.0 && hours <= 20.0, "Game time hours out of range: {}", hours);
        }
    }

    #[test]
    fn test_create_new_save_with_rng_success() {
        let temp_dir = TempDir::new().unwrap();
        let mut rng = ChaCha8Rng::seed_from_u64(99999);
        
        let result = create_new_save_with_rng(
            "Test Save",
            &Difficulty::Medium,
            true,
            false,
            &mut rng,
            temp_dir.path(),
        );
        
        assert!(result.is_ok());
        
        // Verify save directory was created
        let save_dir = temp_dir.path().join("Test_Save");
        assert!(save_dir.exists());
        
        // Verify save.json exists and has correct content
        let save_file = save_dir.join("save.json");
        assert!(save_file.exists());
        
        let save_content = std::fs::read_to_string(save_file).unwrap();
        let save_data: serde_json::Value = serde_json::from_str(&save_content).unwrap();
        
        assert_eq!(save_data["save_name"], "Test Save");
        assert_eq!(save_data["difficulty"], "Medium");
        assert_eq!(save_data["gamerules"].as_array().unwrap(), &vec!["Online"]);
        assert!(save_data["floor_one"]["time"].as_u64().is_some());
        
        // Verify player.json exists and has correct content
        let player_file = save_dir.join("player.json");
        assert!(player_file.exists());
        
        let player_content = std::fs::read_to_string(player_file).unwrap();
        let player_data: serde_json::Value = serde_json::from_str(&player_content).unwrap();
        
        assert_eq!(player_data["level"], 1);
        assert_eq!(player_data["current_floor"], 1);
        assert!(player_data["skills"]["Walking"].as_i64().is_some());
        assert!(player_data["stats"]["strength"].as_i64().is_some());
    }

    #[test]
    fn test_create_new_save_duplicate_error() {
        let temp_dir = TempDir::new().unwrap();
        let mut rng = ChaCha8Rng::seed_from_u64(11111);
        
        // Create first save
        let result1 = create_new_save_with_rng(
            "Duplicate",
            &Difficulty::Easy,
            false,
            false,
            &mut rng,
            temp_dir.path(),
        );
        assert!(result1.is_ok());
        
        // Try to create another with same name
        let result2 = create_new_save_with_rng(
            "Duplicate",
            &Difficulty::Hard,
            false,
            false,
            &mut rng,
            temp_dir.path(),
        );
        assert!(result2.is_err());
        assert!(result2.unwrap_err().contains("already exists"));
    }

    #[test]
    fn test_difficulty_display() {
        assert_eq!(Difficulty::Easy.to_string(), "Easy");
        assert_eq!(Difficulty::Medium.to_string(), "Medium");
        assert_eq!(Difficulty::Hard.to_string(), "Hard");
    }
}
