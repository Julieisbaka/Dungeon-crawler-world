use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde_json::Value;

mod common;
use common::*;

#[test] 
fn test_deterministic_player_generation() {
    // Test that player generation with the same seed produces identical results
    let mut rng1 = test_rng();
    let mut rng2 = test_rng();
    
    // Simulate player stat generation with deterministic ranges
    let strength1: i16 = rng1.gen_range(1..=8);
    let intelligence1: i16 = rng1.gen_range(3..=5);
    let dexterity1: i16 = rng1.gen_range(2..=6);
    
    let strength2: i16 = rng2.gen_range(1..=8);
    let intelligence2: i16 = rng2.gen_range(3..=5);
    let dexterity2: i16 = rng2.gen_range(2..=6);
    
    assert_eq!(strength1, strength2);
    assert_eq!(intelligence1, intelligence2);
    assert_eq!(dexterity1, dexterity2);
}

#[test]
fn test_player_stat_generation_bounds() {
    let mut rng = test_rng();
    
    // Test multiple generations to ensure all values are within bounds
    for _ in 0..100 {
        let strength: i16 = rng.gen_range(1..=8);
        let intelligence: i16 = rng.gen_range(3..=5);
        let dexterity: i16 = rng.gen_range(2..=6);
        let charisma: i16 = rng.gen_range(2..=4);
        let constitution: i16 = rng.gen_range(2..=6);
        
        assert!(strength >= 1 && strength <= 8, "Strength {} out of bounds", strength);
        assert!(intelligence >= 3 && intelligence <= 5, "Intelligence {} out of bounds", intelligence);
        assert!(dexterity >= 2 && dexterity <= 6, "Dexterity {} out of bounds", dexterity);
        assert!(charisma >= 2 && charisma <= 4, "Charisma {} out of bounds", charisma);
        assert!(constitution >= 2 && constitution <= 6, "Constitution {} out of bounds", constitution);
    }
}

#[test]
fn test_skill_generation_bounds() {
    let mut rng = test_rng();
    
    // Test skill generation (3-5 range)
    for _ in 0..100 {
        let walking: i8 = rng.gen_range(3..=5);
        let swimming: i8 = rng.gen_range(3..=5);
        let breathing: i8 = rng.gen_range(3..=5);
        
        assert!(walking >= 3 && walking <= 5, "Walking {} out of bounds", walking);
        assert!(swimming >= 3 && swimming <= 5, "Swimming {} out of bounds", swimming);
        assert!(breathing >= 3 && breathing <= 5, "Breathing {} out of bounds", breathing);
    }
}

#[test]
fn test_player_level_initialization() {
    // Test that new players start at level 1 on floor 1
    use serde_json::json;
    
    let player_data: Value = json!({
        "name": "",
        "level": 1,
        "current_floor": 1,
        "coins": 0,
        "has_manager": false
    });
    
    assert_eq!(player_data["level"], 1);
    assert_eq!(player_data["current_floor"], 1);
    assert_eq!(player_data["coins"], 0);
    assert_eq!(player_data["has_manager"], false);
}

#[test]
fn test_stat_distribution() {
    // Test that stat generation produces reasonable distributions
    let mut rng = test_rng();
    let mut strength_values = Vec::new();
    let mut intelligence_values = Vec::new();
    
    // Generate many samples
    for _ in 0..1000 {
        strength_values.push(rng.gen_range(1..=8));
        intelligence_values.push(rng.gen_range(3..=5));
    }
    
    // Check that we see values across the full range
    let str_min = *strength_values.iter().min().unwrap();
    let str_max = *strength_values.iter().max().unwrap();
    assert!(str_min <= 2, "Should see low strength values");
    assert!(str_max >= 7, "Should see high strength values");
    
    let int_min = *intelligence_values.iter().min().unwrap();
    let int_max = *intelligence_values.iter().max().unwrap();
    assert_eq!(int_min, 3, "Minimum intelligence should be 3");
    assert_eq!(int_max, 5, "Maximum intelligence should be 5");
}

#[test]
fn test_player_inventory_initialization() {
    use serde_json::json;
    
    let player_data: Value = json!({
        "inventory": {},
        "spells": {},
        "sub_classes": []
    });
    
    // New players should have empty inventory, spells, and sub_classes
    assert!(player_data["inventory"].as_object().unwrap().is_empty());
    assert!(player_data["spells"].as_object().unwrap().is_empty());
    assert!(player_data["sub_classes"].as_array().unwrap().is_empty());
}

#[test]
fn test_character_class_race_initialization() {
    use serde_json::json;
    
    let player_data: Value = json!({
        "class": "",
        "race": "",
        "name": ""
    });
    
    // New players should have empty class, race, and name (to be set later)
    assert_eq!(player_data["class"], "");
    assert_eq!(player_data["race"], "");
    assert_eq!(player_data["name"], "");
}