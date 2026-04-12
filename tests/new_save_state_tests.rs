use dungeon_crawler_world::new_save::{
    has_invalid_save_characters, is_safe_folder_name, Difficulty, NewSaveState, NewSaveTab,
};

// ── Difficulty Display ─────────────────────────────────────────────────────────

#[test]
fn test_difficulty_display_easy() {
    assert_eq!(format!("{}", Difficulty::Easy), "Easy");
}

#[test]
fn test_difficulty_display_medium() {
    assert_eq!(format!("{}", Difficulty::Medium), "Medium");
}

#[test]
fn test_difficulty_display_hard() {
    assert_eq!(format!("{}", Difficulty::Hard), "Hard");
}

#[test]
fn test_difficulty_variants_are_distinct() {
    assert_ne!(Difficulty::Easy, Difficulty::Medium);
    assert_ne!(Difficulty::Medium, Difficulty::Hard);
    assert_ne!(Difficulty::Easy, Difficulty::Hard);
}

// ── NewSaveTab equality ────────────────────────────────────────────────────────

#[test]
fn test_new_save_tab_equality() {
    assert_eq!(NewSaveTab::Basics, NewSaveTab::Basics);
    assert_eq!(NewSaveTab::Gamerules, NewSaveTab::Gamerules);
    assert_ne!(NewSaveTab::Basics, NewSaveTab::Gamerules);
}

// ── NewSaveState default ───────────────────────────────────────────────────────

#[test]
fn test_new_save_state_default_values() {
    let state = NewSaveState::default();

    assert_eq!(state.save_name, "");
    assert_eq!(state.selected_difficulty, Difficulty::Medium);
    assert_eq!(state.selected_tab, NewSaveTab::Basics);
    assert!(!state.online_mode);
    assert!(!state.real_time);
    assert_eq!(state.error_message, "");
    assert_eq!(state.success_message, "");
}

// ── NewSaveState::reset ────────────────────────────────────────────────────────

#[test]
fn test_new_save_state_reset_clears_all_fields() {
    let mut state = NewSaveState::default();

    // Mutate every field away from its default.
    state.save_name = "MyHero".to_string();
    state.selected_difficulty = Difficulty::Hard;
    state.selected_tab = NewSaveTab::Gamerules;
    state.online_mode = true;
    state.real_time = true;
    state.error_message = "oops".to_string();
    state.success_message = "done".to_string();

    state.reset();

    assert_eq!(state.save_name, "");
    assert_eq!(state.selected_difficulty, Difficulty::Medium);
    assert_eq!(state.selected_tab, NewSaveTab::Basics);
    assert!(!state.online_mode);
    assert!(!state.real_time);
    assert_eq!(state.error_message, "");
    assert_eq!(state.success_message, "");
}

#[test]
fn test_reset_on_already_default_state_is_idempotent() {
    let mut state = NewSaveState::default();
    state.reset();

    assert_eq!(state.save_name, "");
    assert_eq!(state.selected_difficulty, Difficulty::Medium);
    assert_eq!(state.selected_tab, NewSaveTab::Basics);
    assert!(!state.online_mode);
    assert!(!state.real_time);
}

// ── has_invalid_save_characters – full character set ──────────────────────────

#[test]
fn test_has_invalid_characters_backslash() {
    assert!(has_invalid_save_characters("bad\\name"));
}

#[test]
fn test_has_invalid_characters_question_mark() {
    assert!(has_invalid_save_characters("bad?name"));
}

#[test]
fn test_has_invalid_characters_double_quote() {
    assert!(has_invalid_save_characters("bad\"name"));
}

#[test]
fn test_has_invalid_characters_angle_brackets() {
    assert!(has_invalid_save_characters("bad<name"));
    assert!(has_invalid_save_characters("bad>name"));
}

#[test]
fn test_has_invalid_characters_pipe() {
    assert!(has_invalid_save_characters("bad|name"));
}

#[test]
fn test_has_invalid_characters_asterisk() {
    assert!(has_invalid_save_characters("bad*name"));
}

#[test]
fn test_has_invalid_characters_colon() {
    assert!(has_invalid_save_characters("bad:name"));
}

#[test]
fn test_valid_name_with_hyphen_and_digits() {
    assert!(!has_invalid_save_characters("save-01"));
}

#[test]
fn test_valid_name_with_spaces_does_not_trigger_invalid_char_check() {
    // Spaces are not in the banned list (they are handled separately).
    assert!(!has_invalid_save_characters("save with spaces"));
}

// ── is_safe_folder_name – additional edge cases ────────────────────────────────

#[test]
fn test_is_safe_folder_name_rejects_empty_string() {
    assert!(!is_safe_folder_name(""));
}

#[test]
fn test_is_safe_folder_name_rejects_dot() {
    assert!(!is_safe_folder_name("."));
}

#[test]
fn test_is_safe_folder_name_rejects_double_dot() {
    assert!(!is_safe_folder_name(".."));
}

#[test]
fn test_is_safe_folder_name_rejects_path_with_separator() {
    assert!(!is_safe_folder_name("foo/bar"));
}

#[test]
fn test_is_safe_folder_name_accepts_alphanumeric() {
    assert!(is_safe_folder_name("MySave123"));
}

#[test]
fn test_is_safe_folder_name_accepts_underscore() {
    assert!(is_safe_folder_name("my_save"));
}

#[test]
fn test_is_safe_folder_name_accepts_hyphen() {
    assert!(is_safe_folder_name("my-save"));
}
