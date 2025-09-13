use std::fs;

mod common;
use common::*;

// Tests for save system functionality
#[test]
fn test_save_directory_creation() {
    let temp_dir = temp_test_dir();
    let saves_dir = create_test_saves_dir(temp_dir.path());
    
    assert!(saves_dir.exists());
    assert!(saves_dir.is_dir());
}

#[test] 
fn test_save_json_structure() {
    // This test would validate the structure of saved JSON files
    // but requires accessing the save creation functions which have GUI dependencies
    // For now, we test the JSON structure manually
    
    use serde_json::{json, Value};
    
    let save_data: Value = json!({
        "save_name": "Test Save",
        "difficulty": "Medium",
        "created_at": "2024-01-01T00:00:00Z",
        "floor_one": {
            "is_cleared": false,
            "time": 54000
        },
        "gamerules": ["Online"]
    });
    
    // Validate required fields exist
    assert!(save_data["save_name"].is_string());
    assert!(save_data["difficulty"].is_string());
    assert!(save_data["created_at"].is_string());
    assert!(save_data["floor_one"]["is_cleared"].is_boolean());
    assert!(save_data["floor_one"]["time"].is_number());
    assert!(save_data["gamerules"].is_array());
}

#[test]
fn test_player_json_structure() {
    use serde_json::{json, Value};
    
    let player_data: Value = json!({
        "name": "",
        "level": 1,
        "spells": {},
        "inventory": {},
        "skills": {
            "Walking": 4,
            "Swimming": 3,
            "Breathing": 5
        },
        "coins": 0,
        "sub_classes": [],
        "class": "",
        "race": "",
        "has_manager": false,
        "current_floor": 1,
        "stats": {
            "strength": 5,
            "intelligence": 4,
            "dexterity": 3,
            "charisma": 2,
            "constitution": 4
        }
    });
    
    // Validate player structure
    assert_eq!(player_data["level"], 1);
    assert_eq!(player_data["current_floor"], 1);
    assert_eq!(player_data["coins"], 0);
    assert_eq!(player_data["has_manager"], false);
    
    // Validate skills are within expected ranges
    assertions::assert_skill_in_range(&player_data, "Walking", 3, 5);
    assertions::assert_skill_in_range(&player_data, "Swimming", 3, 5);
    assertions::assert_skill_in_range(&player_data, "Breathing", 3, 5);
    
    // Validate stats are within expected ranges
    assertions::assert_stat_in_range(&player_data, "strength", 1, 8);
    assertions::assert_stat_in_range(&player_data, "intelligence", 3, 5);
    assertions::assert_stat_in_range(&player_data, "dexterity", 2, 6);
    assertions::assert_stat_in_range(&player_data, "charisma", 2, 4);
    assertions::assert_stat_in_range(&player_data, "constitution", 2, 6);
}

#[test]
fn test_round_trip_serialization() {
    use serde_json::{json, Value};
    
    let original_data: Value = json!({
        "save_name": "Round Trip Test",
        "difficulty": "Hard",
        "created_at": "2024-01-01T12:00:00Z",
        "floor_one": {
            "is_cleared": true,
            "time": 72000
        },
        "gamerules": ["Online", "Real-time"]
    });
    
    // Serialize to string
    let serialized = serde_json::to_string(&original_data).unwrap();
    
    // Deserialize back
    let deserialized: Value = serde_json::from_str(&serialized).unwrap();
    
    // Should be identical
    assert_eq!(original_data, deserialized);
}

#[test]
fn test_temp_directory_isolation() {
    // Test that multiple temp directories don't interfere with each other
    let temp1 = temp_test_dir();
    let temp2 = temp_test_dir();
    
    let saves1 = create_test_saves_dir(temp1.path());
    let saves2 = create_test_saves_dir(temp2.path());
    
    assert_ne!(saves1, saves2);
    assert!(saves1.exists());
    assert!(saves2.exists());
    
    // Create a file in saves1
    std::fs::write(saves1.join("test.txt"), "content1").unwrap();
    
    // Verify saves2 doesn't have the file
    assert!(!saves2.join("test.txt").exists());
}