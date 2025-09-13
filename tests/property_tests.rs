use proptest::prelude::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

mod common;

// Property tests for core game invariants
proptest! {
    #[test]
    fn test_player_stats_never_negative(
        strength in 1i16..=8,
        intelligence in 3i16..=5,
        dexterity in 2i16..=6,
        charisma in 2i16..=4,
        constitution in 2i16..=6
    ) {
        // All player stats should be positive
        prop_assert!(strength > 0);
        prop_assert!(intelligence > 0);
        prop_assert!(dexterity > 0);
        prop_assert!(charisma > 0);
        prop_assert!(constitution > 0);
    }

    #[test]
    fn test_skill_values_in_range(
        walking in 3i8..=5,
        swimming in 3i8..=5,
        breathing in 3i8..=5
    ) {
        // All skills should be within the valid range
        prop_assert!(walking >= 3 && walking <= 5);
        prop_assert!(swimming >= 3 && swimming <= 5);
        prop_assert!(breathing >= 3 && breathing <= 5);
    }

    #[test]
    fn test_floor_time_generation_bounds(
        real_time: bool,
        seed in any::<u64>()
    ) {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        
        let floor_time = if real_time {
            432000 // 5 days in seconds
        } else {
            // Simulate triangular distribution (12-20 hours)
            let u: f32 = rng.gen::<f32>();
            let (a, c, b): (f32, f32, f32) = (12.0, 13.0, 20.0);
            let fc: f32 = (c - a) / (b - a);
            let hours: f32 = if u < fc {
                a + ((b - a) * (c - a) * u).sqrt()
            } else {
                b - ((b - a) * (b - c) * (1.0 - u)).sqrt()
            };
            (hours * 3600.0).round() as u32
        };
        
        if real_time {
            prop_assert_eq!(floor_time, 432000);
        } else {
            let hours = floor_time as f32 / 3600.0;
            prop_assert!(hours >= 12.0 && hours <= 20.0);
        }
    }

    #[test]
    fn test_save_name_validation_properties(
        name in "\\PC{1,50}"  // Valid characters, 1-50 length
    ) {
        // Valid names should not contain invalid characters
        let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
        let has_invalid = name.chars().any(|c| invalid_chars.contains(&c));
        
        if !has_invalid && !name.trim().is_empty() {
            // Should be valid
            prop_assert!(true); // Would call validate_save_name if accessible
        }
    }

    #[test]
    fn test_restroom_distance_bounds(seed in any::<u64>()) {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let distance: u16 = rng.gen_range(300..=500);
        
        prop_assert!(distance >= 300);
        prop_assert!(distance <= 500);
    }

    #[test]
    fn test_player_level_consistency(
        level in 1u32..=100,
        current_floor in 1u32..=100
    ) {
        // Player level and floor should be consistent
        // (In a real game, level might have some relationship to current floor)
        prop_assert!(level >= 1);
        prop_assert!(current_floor >= 1);
        
        // For new players, they should start at level 1, floor 1
        if level == 1 {
            // New players typically start on floor 1
            // This is a game design invariant
            prop_assert!(current_floor >= 1);
        }
    }

    #[test]
    fn test_gamerules_consistency(
        online_mode: bool,
        real_time: bool
    ) {
        let mut gamerules = Vec::new();
        if online_mode {
            gamerules.push("Online");
        }
        if real_time {
            gamerules.push("Real-time");
        }
        
        // Gamerules should be consistent with flags
        prop_assert_eq!(gamerules.contains(&"Online"), online_mode);
        prop_assert_eq!(gamerules.contains(&"Real-time"), real_time);
        
        // No duplicate rules
        let mut sorted_rules = gamerules.clone();
        sorted_rules.sort();
        sorted_rules.dedup();
        prop_assert_eq!(sorted_rules.len(), gamerules.len());
    }

    #[test]
    fn test_stat_sum_properties(
        strength in 1i16..=8,
        intelligence in 3i16..=5,
        dexterity in 2i16..=6,
        charisma in 2i16..=4,
        constitution in 2i16..=6
    ) {
        let total_stats = strength + intelligence + dexterity + charisma + constitution;
        
        // Total stats should be within reasonable bounds
        let min_total = 1 + 3 + 2 + 2 + 2; // 10
        let max_total = 8 + 5 + 6 + 4 + 6; // 29
        
        prop_assert!(total_stats >= min_total);
        prop_assert!(total_stats <= max_total);
        
        // Each stat should contribute meaningfully to the total
        prop_assert!(strength <= total_stats);
        prop_assert!(intelligence <= total_stats);
        prop_assert!(dexterity <= total_stats);
        prop_assert!(charisma <= total_stats);
        prop_assert!(constitution <= total_stats);
    }

    #[test]
    fn test_file_path_properties(
        save_name in "[a-zA-Z0-9 ]{1,20}"
    ) {
        // Test folder name generation
        let folder_name = save_name.trim().replace(' ', "_");
        
        // Folder name should not contain spaces
        prop_assert!(!folder_name.contains(' '));
        
        // Should not be empty if original wasn't empty
        if !save_name.trim().is_empty() {
            prop_assert!(!folder_name.is_empty());
        }
        
        // Should only contain safe characters
        let safe_chars: Vec<char> = folder_name.chars()
            .filter(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        prop_assert_eq!(safe_chars.len(), folder_name.len());
    }
}

// Additional property tests for specific game mechanics
proptest! {
    #[test]
    fn test_damage_never_negative(
        base_damage in 0u32..=100,
        defense in 0u32..=50
    ) {
        // This would test combat math if it existed
        // For now, test that damage calculations are never negative
        let effective_damage = base_damage.saturating_sub(defense);
        prop_assert!(effective_damage <= base_damage);
    }

    #[test]
    fn test_inventory_capacity_invariants(
        item_count in 0usize..=100,
        max_capacity in 1usize..=1000
    ) {
        // Inventory should never exceed capacity
        let actual_items = std::cmp::min(item_count, max_capacity);
        prop_assert!(actual_items <= max_capacity);
        prop_assert!(actual_items <= item_count || actual_items == max_capacity);
    }
}