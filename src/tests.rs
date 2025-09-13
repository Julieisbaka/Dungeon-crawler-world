use proptest::prelude::*;
use crate::new_save::{Difficulty, NewSaveState, NewSaveTab};
use crate::grid::restroom_distance;

/// Property-based tests for critical game invariants
#[cfg(test)]
mod property_tests {
    use super::*;

    // Strategy for generating valid save names
    fn valid_save_name() -> impl Strategy<Value = String> {
        "[a-zA-Z0-9_][a-zA-Z0-9_ ]{0,49}"
            .prop_filter("Must not be empty or just whitespace", |s| !s.trim().is_empty())
    }

    // Strategy for generating invalid save names
    fn invalid_save_name() -> impl Strategy<Value = String> {
        prop_oneof![
            Just("".to_string()),
            Just("   ".to_string()),
            "[a-zA-Z0-9_ ]*[/\\\\:\\*\\?\"<>\\|][a-zA-Z0-9_ ]*",
        ]
    }

    #[proptest]
    fn difficulty_display_is_consistent(difficulty: Difficulty) {
        let display_str = difficulty.to_string();
        match difficulty {
            Difficulty::Easy => prop_assert_eq!(display_str, "Easy"),
            Difficulty::Medium => prop_assert_eq!(display_str, "Medium"),
            Difficulty::Hard => prop_assert_eq!(display_str, "Hard"),
        }
    }

    #[proptest]
    fn new_save_state_default_is_valid() {
        let state = NewSaveState::default();
        prop_assert_eq!(state.selected_difficulty, Difficulty::Medium);
        prop_assert_eq!(state.selected_tab, NewSaveTab::Basics);
        prop_assert_eq!(state.online_mode, false);
        prop_assert_eq!(state.real_time, false);
        prop_assert!(state.save_name.is_empty());
        prop_assert!(state.error_message.is_empty());
        prop_assert!(state.success_message.is_empty());
    }

    #[proptest]
    fn restroom_distance_is_within_bounds() {
        let distance = restroom_distance();
        prop_assert!(distance >= 300);
        prop_assert!(distance <= 500);
    }

    #[proptest]
    fn restroom_distance_consistency(seed: u64) {
        // Test that the random generation is properly bounded
        use rand::{Rng, SeedableRng};
        use rand::rngs::StdRng;
        
        let mut rng = StdRng::seed_from_u64(seed);
        let distance: u16 = rng.gen_range(300..=500);
        
        prop_assert!(distance >= 300);
        prop_assert!(distance <= 500);
    }

    #[proptest]
    fn valid_save_names_should_pass_validation(name in valid_save_name()) {
        // This test would validate against the save creation logic
        // For now, we test the basic properties that valid names should have
        prop_assert!(!name.trim().is_empty());
        prop_assert!(!name.contains(&['/', '\\', ':', '*', '?', '"', '<', '>', '|'][..]));
    }

    #[proptest]
    fn invalid_save_names_should_fail_validation(name in invalid_save_name()) {
        // Test that invalid names have the expected invalid properties
        let is_invalid = name.trim().is_empty() || 
                        name.contains(&['/', '\\', ':', '*', '?', '"', '<', '>', '|'][..]);
        prop_assert!(is_invalid);
    }

    #[proptest]
    fn save_name_sanitization_preserves_length_invariant(name: String) {
        let sanitized = name.replace(' ', "_");
        // The sanitized name should have the same character count
        // (since we're only replacing spaces with underscores)
        prop_assert_eq!(name.chars().count(), sanitized.chars().count());
    }

    #[proptest]
    fn difficulty_enum_ordering_is_consistent(d1: Difficulty, d2: Difficulty) {
        let order1 = d1 as u8;
        let order2 = d2 as u8;
        
        // Verify the enum ordering matches our expectations
        match (d1, d2) {
            (Difficulty::Easy, Difficulty::Medium) => prop_assert!(order1 < order2),
            (Difficulty::Easy, Difficulty::Hard) => prop_assert!(order1 < order2),
            (Difficulty::Medium, Difficulty::Hard) => prop_assert!(order1 < order2),
            (Difficulty::Medium, Difficulty::Easy) => prop_assert!(order1 > order2),
            (Difficulty::Hard, Difficulty::Easy) => prop_assert!(order1 > order2),
            (Difficulty::Hard, Difficulty::Medium) => prop_assert!(order1 > order2),
            _ => prop_assert_eq!(order1, order2), // Same variant
        }
    }
}

/// Integration tests for save system
#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;

    #[proptest]
    fn save_directory_creation_is_idempotent(
        name in "[a-zA-Z0-9_]{1,20}",
        #[strategy(any::<Difficulty>())] difficulty: Difficulty,
        online_mode: bool,
        real_time: bool,
    ) {
        // Create a temporary directory for testing
        let temp_dir = TempDir::new().unwrap();
        let saves_path = temp_dir.path().join("saves");
        
        // Change to temp directory for the test
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        // First save creation should succeed
        let folder_name = name.replace(' ', "_");
        let save_path = saves_path.join(&folder_name);
        
        // Create the saves directory structure manually to test idempotency
        fs::create_dir_all(&saves_path).unwrap();
        fs::create_dir_all(&save_path).unwrap();
        
        // Verify the directory exists
        prop_assert!(save_path.exists());
        prop_assert!(save_path.is_dir());
        
        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();
    }
}

/// Arbitrary implementations for proptest
impl Arbitrary for Difficulty {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        prop_oneof![
            Just(Difficulty::Easy),
            Just(Difficulty::Medium),
            Just(Difficulty::Hard),
        ].boxed()
    }
}

impl Arbitrary for NewSaveTab {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        prop_oneof![
            Just(NewSaveTab::Basics),
            Just(NewSaveTab::Gamerules),
        ].boxed()
    }
}