use dungeon_crawler_world::console::{
    read_player_skills_from_path, read_save_list, ConsoleState,
};
use dungeon_crawler_world::player::{Player, PlayerStats};
use std::collections::HashMap;
use std::fs;

mod common;
use common::unique_temp_dir;

fn make_player(skills: HashMap<String, i8>) -> Player {
    Player::new(
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
    )
}

// ── ConsoleState ───────────────────────────────────────────────────────────────

#[test]
fn test_console_state_default_is_not_dirty() {
    let state = ConsoleState::default();
    assert!(!state.is_dirty());
}

#[test]
fn test_log_line_marks_dirty() {
    let mut state = ConsoleState::default();
    state.log_line("hello");
    assert!(state.is_dirty());
}

#[test]
fn test_clear_dirty_resets_flag() {
    let mut state = ConsoleState::default();
    state.log_line("hi");
    assert!(state.is_dirty());

    state.clear_dirty();
    assert!(!state.is_dirty());
}

#[test]
fn test_log_lines_adds_multiple_lines_and_marks_dirty() {
    let mut state = ConsoleState::default();
    state.log_lines(vec!["line one", "line two", "line three"]);
    assert!(state.is_dirty());
}

#[test]
fn test_clear_marks_dirty() {
    let mut state = ConsoleState::default();
    // Add a line first to have something to clear.
    state.log_line("something");
    state.clear_dirty();

    state.clear();
    assert!(state.is_dirty());
}

#[test]
fn test_take_pending_returns_empty_vec_when_nothing_pending() {
    let mut state = ConsoleState::default();
    let pending = state.take_pending();
    assert!(pending.is_empty());
}

#[test]
fn test_set_input_marks_dirty_when_value_changes() {
    let mut state = ConsoleState::default();
    state.set_input("hello".to_string());
    assert!(state.is_dirty());
}

#[test]
fn test_set_input_does_not_mark_dirty_when_same_value() {
    let mut state = ConsoleState::default();
    state.set_input("same".to_string());
    state.clear_dirty();

    // Setting the same value again must not flip dirty.
    state.set_input("same".to_string());
    assert!(!state.is_dirty());
}

// ── read_save_list ─────────────────────────────────────────────────────────────

#[test]
fn test_read_save_list_returns_empty_for_missing_root() {
    let root = std::env::temp_dir().join("__nonexistent_saves_root__");
    let _ = fs::remove_dir_all(&root);

    let result = read_save_list(&root).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_read_save_list_returns_entry_for_each_save_dir() {
    let root = unique_temp_dir("console_extra_read_save_list");
    fs::create_dir_all(root.join("save_alpha")).unwrap();
    fs::create_dir_all(root.join("save_beta")).unwrap();

    let result = read_save_list(&root).unwrap();

    assert_eq!(result.len(), 2);
    let names: Vec<&str> = result.iter().map(|e| e.folder_name.as_str()).collect();
    assert!(names.contains(&"save_alpha"));
    assert!(names.contains(&"save_beta"));

    let _ = fs::remove_dir_all(root);
}

#[test]
fn test_read_save_list_parses_difficulty_from_save_json() {
    let root = unique_temp_dir("console_extra_read_save_list_diff");
    let save_dir = root.join("hard_run");
    fs::create_dir_all(&save_dir).unwrap();
    fs::write(
        save_dir.join("save.json"),
        r#"{"difficulty":"Hard","save_name":"hard run"}"#,
    )
    .unwrap();

    let result = read_save_list(&root).unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].difficulty.as_deref(), Some("Hard"));

    let _ = fs::remove_dir_all(root);
}

#[test]
fn test_read_save_list_tolerates_missing_save_json() {
    let root = unique_temp_dir("console_extra_read_save_list_no_json");
    fs::create_dir_all(root.join("bare_save")).unwrap();

    let result = read_save_list(&root).unwrap();

    assert_eq!(result.len(), 1);
    assert!(result[0].difficulty.is_none());
    assert!(result[0].created_at.is_none());

    let _ = fs::remove_dir_all(root);
}

#[test]
fn test_read_save_list_ignores_files_not_dirs() {
    let root = unique_temp_dir("console_extra_read_save_list_files");
    fs::write(root.join("not_a_dir.json"), "{}").unwrap();
    fs::create_dir_all(root.join("real_save")).unwrap();

    let result = read_save_list(&root).unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].folder_name, "real_save");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn test_read_save_list_results_are_sorted_by_folder_name() {
    let root = unique_temp_dir("console_extra_read_save_list_sort");
    fs::create_dir_all(root.join("zzz_save")).unwrap();
    fs::create_dir_all(root.join("aaa_save")).unwrap();
    fs::create_dir_all(root.join("mmm_save")).unwrap();

    let result = read_save_list(&root).unwrap();

    let names: Vec<&str> = result.iter().map(|e| e.folder_name.as_str()).collect();
    assert_eq!(names, vec!["aaa_save", "mmm_save", "zzz_save"]);

    let _ = fs::remove_dir_all(root);
}

// ── read_player_skills_from_path ───────────────────────────────────────────────

#[test]
fn test_read_player_skills_from_path_returns_err_when_no_save_selected() {
    let root = unique_temp_dir("console_extra_player_skills_no_save");

    let result = read_player_skills_from_path(&root, None);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("No current save selected"));

    let _ = fs::remove_dir_all(root);
}

#[test]
fn test_read_player_skills_from_path_returns_err_for_missing_player_file() {
    let root = unique_temp_dir("console_extra_player_skills_missing");
    fs::create_dir_all(root.join("MySave")).unwrap();

    let result = read_player_skills_from_path(&root, Some("MySave".to_string()));
    assert!(result.is_err());

    let _ = fs::remove_dir_all(root);
}

#[test]
fn test_read_player_skills_from_path_returns_sorted_skills() {
    let root = unique_temp_dir("console_extra_player_skills_valid");
    let save_dir = root.join("MySave");
    fs::create_dir_all(&save_dir).unwrap();

    let mut skills = HashMap::new();
    skills.insert("Walking".to_string(), 5i8);
    skills.insert("Breathing".to_string(), 4i8);
    skills.insert("Swimming".to_string(), 3i8);

    let player = make_player(skills);
    fs::write(
        save_dir.join("player.json"),
        serde_json::to_string_pretty(&player).unwrap(),
    )
    .unwrap();

    let result = read_player_skills_from_path(&root, Some("MySave".to_string())).unwrap();

    // Result should be sorted alphabetically.
    let names: Vec<&str> = result.iter().map(|(n, _)| n.as_str()).collect();
    assert_eq!(names, vec!["Breathing", "Swimming", "Walking"]);

    let _ = fs::remove_dir_all(root);
}
