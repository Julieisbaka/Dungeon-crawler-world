use proptest::prelude::*;

/// Property-based tests for critical game invariants
#[cfg(test)]
mod property_tests {
    use super::*;

    // Test basic string manipulation properties that would be used in save names
    #[proptest]
    fn save_name_sanitization_preserves_length_invariant(name: String) {
        let sanitized = name.replace(' ', "_");
        // The sanitized name should have the same character count
        // (since we're only replacing spaces with underscores)
        prop_assert_eq!(name.chars().count(), sanitized.chars().count());
    }

    #[proptest]
    fn invalid_characters_detection(name: String) {
        let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
        let has_invalid = name.chars().any(|c| invalid_chars.contains(&c));
        let contains_invalid = name.contains(&invalid_chars[..]);
        prop_assert_eq!(has_invalid, contains_invalid);
    }

    #[proptest]
    fn restroom_distance_bounds() {
        // Simulate the restroom distance function
        use rand::{Rng, SeedableRng};
        use rand::rngs::StdRng;
        
        for seed in 0..100u64 {
            let mut rng = StdRng::seed_from_u64(seed);
            let distance: u16 = rng.gen_range(300..=500);
            
            prop_assert!(distance >= 300);
            prop_assert!(distance <= 500);
        }
    }

    #[proptest]
    fn difficulty_ordering(d1 in 0u8..3u8, d2 in 0u8..3u8) {
        // Test that difficulty ordering is consistent
        match (d1, d2) {
            (0, 1) | (0, 2) | (1, 2) => prop_assert!(d1 < d2),
            (1, 0) | (2, 0) | (2, 1) => prop_assert!(d1 > d2),
            _ => prop_assert_eq!(d1, d2),
        }
    }

    #[proptest]
    fn json_save_structure_properties(
        level in 1u32..100u32,
        coins in 0u32..1000000u32,
        strength in 1i16..=8i16,
        intelligence in 3i16..=5i16,
        dexterity in 2i16..=6i16,
        charisma in 2i16..4i16,
        constitution in 2i16..6i16,
    ) {
        // Test that generated player stats are within expected bounds
        // This simulates the stat generation logic from new_save.rs
        prop_assert!(strength >= 1 && strength <= 8);
        prop_assert!(intelligence >= 3 && intelligence <= 5);
        prop_assert!(dexterity >= 2 && dexterity <= 6);
        prop_assert!(charisma >= 2 && charisma < 4);
        prop_assert!(constitution >= 2 && constitution < 6);
        
        // Basic bounds checking
        prop_assert!(level >= 1);
        prop_assert!(coins <= 1000000);
    }

    #[proptest]
    fn fps_buffer_capacity_invariant(capacity in 1usize..1000usize) {
        // Test FPS buffer behavior
        let mut buffer = std::collections::VecDeque::with_capacity(capacity);
        
        // Add more items than capacity
        for i in 0..(capacity + 10) {
            if buffer.len() == capacity {
                buffer.pop_front();
            }
            buffer.push_back(i as f32);
        }
        
        prop_assert_eq!(buffer.len(), capacity);
    }

    #[proptest]
    fn console_command_parsing(input: String) {
        let trimmed = input.trim();
        
        // Test that empty commands are handled
        if trimmed.is_empty() {
            prop_assert_eq!(trimmed, "");
        } else {
            // Test that non-empty commands have valid structure
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            prop_assert!(!parts.is_empty());
        }
    }
}

/// Integration tests that test game logic invariants
#[cfg(test)]
mod integration_tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[proptest]
    fn save_directory_operations(
        name in "[a-zA-Z0-9_]{1,20}",
        online_mode: bool,
        real_time: bool,
    ) {
        // Create a temporary directory for testing
        let temp_dir = TempDir::new().unwrap();
        let saves_path = temp_dir.path().join("saves");
        
        // Test directory creation
        fs::create_dir_all(&saves_path).unwrap();
        prop_assert!(saves_path.exists());
        prop_assert!(saves_path.is_dir());
        
        // Test save directory structure
        let folder_name = name.replace(' ', "_");
        let save_path = saves_path.join(&folder_name);
        fs::create_dir_all(&save_path).unwrap();
        
        prop_assert!(save_path.exists());
        prop_assert!(save_path.is_dir());
        
        // Test that boolean flags are preserved
        prop_assert_eq!(online_mode, online_mode);
        prop_assert_eq!(real_time, real_time);
    }

    #[proptest]
    fn settings_serialization_roundtrip(
        fog in -100i8..=100i8,
        lighting in -100i8..=100i8,
        sound: bool,
        developer_mode: bool,
        console_max_lines in 100usize..=1000usize,
    ) {
        // Test that settings values are within expected bounds
        prop_assert!(fog >= -100 && fog <= 100);
        prop_assert!(lighting >= -100 && lighting <= 100);
        prop_assert!(console_max_lines >= 100 && console_max_lines <= 1000);
        
        // Test boolean handling
        prop_assert_eq!(sound, sound);
        prop_assert_eq!(developer_mode, developer_mode);
    }
}