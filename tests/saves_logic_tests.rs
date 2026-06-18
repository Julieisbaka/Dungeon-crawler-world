use dungeon_crawler_world::logic::saves_logic::SaveMenuState;

// ── SaveMenuState::default ─────────────────────────────────────────────────────

#[test]
fn test_save_menu_state_default_values() {
    let state = SaveMenuState::default();

    assert!(!state.in_new_save_menu);
    assert!(state.editing_save.is_none());
    assert_eq!(state.edit_save_name, "");
    assert!(state.rename_error.is_none());
    assert!(!state.confirm_delete);
    assert!(state.delete_target.is_none());
    assert!(!state.back_requested);
    assert!(state.save_cache.is_empty());
    assert!(!state.cache_loaded);
    assert!(state.loaded_save.is_none());
    assert!(!state.enter_loaded_save_requested);
    assert!(state.load_error.is_none());
}

// ── SaveMenuState::invalidate_cache ───────────────────────────────────────────

#[test]
fn test_invalidate_cache_clears_entries_and_loaded_flag() {
    use dungeon_crawler_world::logic::saves_logic::SaveEntryCache;

    let mut state = SaveMenuState::default();

    // Manually populate the cache as the UI would.
    state.save_cache.insert(
        "my_save".to_string(),
        SaveEntryCache {
            folder_name: "my_save".to_string(),
            save_name: "my save".to_string(),
            difficulty_text: "Medium".to_string(),
            created_at_text: "2024-01-01 00:00".to_string(),
            icon: None,
        },
    );
    state.cache_loaded = true;

    state.invalidate_cache();

    assert!(state.save_cache.is_empty());
    assert!(!state.cache_loaded);
}

#[test]
fn test_invalidate_cache_on_empty_state_is_idempotent() {
    let mut state = SaveMenuState::default();
    state.invalidate_cache(); // Should not panic.

    assert!(state.save_cache.is_empty());
    assert!(!state.cache_loaded);
}
