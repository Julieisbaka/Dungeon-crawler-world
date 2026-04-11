use dungeon_crawler_world::logic::skills_logic::{read_player_skills_for_save, SkillsState};
use dungeon_crawler_world::player::{Player, PlayerStats};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

mod common;
use common::unique_temp_dir;

// ── SkillsState default ────────────────────────────────────────────────────────

#[test]
fn test_skills_state_default_values() {
    let state = SkillsState::default();

    assert!(state.catalog.is_empty());
    assert!(state.selected.is_none());
    assert!(!state.loaded);
    assert!(!state.show_all);
    assert!(!state.dev_controls);
    assert!(!state.only_owned);
    assert_eq!(state.search, "");
    assert_eq!(state.sort_mode, 0);
    assert_eq!(state.page, 0);
}

// ── SkillsState::enable_preview ───────────────────────────────────────────────

#[test]
fn test_enable_preview_sets_show_all() {
    let mut state = SkillsState::default();
    assert!(!state.show_all);

    state.enable_preview();

    assert!(state.show_all);
}

// ── SkillsState::enable_dev_controls ──────────────────────────────────────────

#[test]
fn test_enable_dev_controls_sets_flag() {
    let mut state = SkillsState::default();
    assert!(!state.dev_controls);

    state.enable_dev_controls();

    assert!(state.dev_controls);
}

// ── read_player_skills_for_save ────────────────────────────────────────────────

fn write_player_json(dir: &Path, skills: HashMap<String, i8>) {
    let player = Player::new(
        "Hero".to_string(),
        1,
        HashMap::new(),
        HashMap::new(),
        skills,
        0,
        vec![],
        "Fighter".to_string(),
        "Human".to_string(),
        false,
        1,
        PlayerStats {
            strength: 5,
            intelligence: 5,
            dexterity: 5,
            charisma: 5,
            constitution: 5,
        },
    );
    let json = serde_json::to_string_pretty(&player).unwrap();
    fs::write(dir.join("player.json"), json).unwrap();
}

#[test]
fn test_read_player_skills_returns_correct_map() {
    let root = unique_temp_dir("skills_logic_valid");
    let save_dir = root.join("TestSave");
    fs::create_dir_all(&save_dir).unwrap();

    let mut skills = HashMap::new();
    skills.insert("Walking".to_string(), 4i8);
    skills.insert("Swimming".to_string(), 3i8);
    write_player_json(&save_dir, skills.clone());

    let result = read_player_skills_for_save(&root, "TestSave");

    assert_eq!(result.get("Walking"), Some(&4));
    assert_eq!(result.get("Swimming"), Some(&3));
    let _ = fs::remove_dir_all(root);
}

#[test]
fn test_read_player_skills_returns_empty_for_missing_save() {
    let root = unique_temp_dir("skills_logic_missing");

    let result = read_player_skills_for_save(&root, "DoesNotExist");

    assert!(result.is_empty());
    let _ = fs::remove_dir_all(root);
}

#[test]
fn test_read_player_skills_returns_empty_for_malformed_json() {
    let root = unique_temp_dir("skills_logic_bad_json");
    let save_dir = root.join("BadSave");
    fs::create_dir_all(&save_dir).unwrap();
    fs::write(save_dir.join("player.json"), "{ bad json").unwrap();

    let result = read_player_skills_for_save(&root, "BadSave");

    assert!(result.is_empty());
    let _ = fs::remove_dir_all(root);
}

#[test]
fn test_read_player_skills_returns_empty_for_player_with_no_skills() {
    let root = unique_temp_dir("skills_logic_empty_skills");
    let save_dir = root.join("EmptySave");
    fs::create_dir_all(&save_dir).unwrap();
    write_player_json(&save_dir, HashMap::new());

    let result = read_player_skills_for_save(&root, "EmptySave");

    assert!(result.is_empty());
    let _ = fs::remove_dir_all(root);
}
