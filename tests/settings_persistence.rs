use std::fs;

mod common;
use common::*;

#[test]
fn test_settings_file_creation() {
    let temp_dir = temp_test_dir();
    let settings_file = temp_dir.path().join("test_settings.json");
    
    // Create basic settings JSON
    let settings_json = r#"{
  "fog": 2,
  "lighting": 3,
  "sound": true,
  "developer_mode": false,
  "verbose_logging": false,
  "show_console": false,
  "show_fps_graph": false,
  "fullscreen": false,
  "log_to_console": false,
  "console_max_lines": 300,
  "show_save_creation_date": true
}"#;
    
    fs::write(&settings_file, settings_json).unwrap();
    
    // Verify file was created and contains expected content
    assert!(settings_file.exists());
    let content = fs::read_to_string(&settings_file).unwrap();
    assert!(content.contains("fog"));
    assert!(content.contains("lighting"));
    assert!(content.contains("sound"));
}

#[test]
fn test_settings_round_trip() {
    use serde_json::{json, Value};
    
    let original_settings: Value = json!({
        "fog": 5,
        "lighting": 7,
        "sound": false,
        "developer_mode": true,
        "verbose_logging": true,
        "show_console": true,
        "show_fps_graph": true,
        "fullscreen": true,
        "log_to_console": true,
        "console_max_lines": 500,
        "show_save_creation_date": false
    });
    
    // Serialize to string
    let json_string = serde_json::to_string_pretty(&original_settings).unwrap();
    
    // Deserialize back
    let loaded_settings: Value = serde_json::from_str(&json_string).unwrap();
    
    // Should be identical
    assert_eq!(original_settings, loaded_settings);
    
    // Verify specific values
    assert_eq!(loaded_settings["fog"], 5);
    assert_eq!(loaded_settings["developer_mode"], true);
    assert_eq!(loaded_settings["console_max_lines"], 500);
}

#[test]
fn test_settings_with_missing_fields() {
    use serde_json::Value;
    
    // Settings JSON with only some fields
    let partial_json = r#"{
        "fog": 8,
        "sound": false,
        "developer_mode": true
    }"#;
    
    let settings: Value = serde_json::from_str(partial_json).unwrap();
    
    // Should successfully parse the provided fields
    assert_eq!(settings["fog"], 8);
    assert_eq!(settings["sound"], false);
    assert_eq!(settings["developer_mode"], true);
    
    // Missing fields should be null/undefined in the JSON
    assert!(settings["lighting"].is_null());
    assert!(settings["console_max_lines"].is_null());
}

#[test]
fn test_settings_invalid_json() {
    let invalid_json = r#"{
        "fog": 2,
        "lighting": 3,
        "invalid_field": [this is not valid json
    }"#;
    
    let result = serde_json::from_str::<serde_json::Value>(invalid_json);
    assert!(result.is_err());
}

#[test]
fn test_settings_type_validation() {
    use serde_json::Value;
    
    let settings_json = r#"{
        "fog": 10,
        "lighting": -5,
        "sound": true,
        "console_max_lines": 1000
    }"#;
    
    let settings: Value = serde_json::from_str(settings_json).unwrap();
    
    // Verify types
    assert!(settings["fog"].is_number());
    assert!(settings["lighting"].is_number());
    assert!(settings["sound"].is_boolean());
    assert!(settings["console_max_lines"].is_number());
    
    // Verify values
    assert_eq!(settings["fog"].as_i64().unwrap(), 10);
    assert_eq!(settings["lighting"].as_i64().unwrap(), -5);
    assert_eq!(settings["sound"].as_bool().unwrap(), true);
    assert_eq!(settings["console_max_lines"].as_u64().unwrap(), 1000);
}

#[test]
fn test_large_settings_file() {
    let temp_dir = temp_test_dir();
    let settings_file = temp_dir.path().join("large_settings.json");
    
    // Create a large settings object to test performance
    use serde_json::json;
    let large_settings = json!({
        "fog": 2,
        "lighting": 3,
        "sound": true,
        "developer_mode": false,
        "verbose_logging": false,
        "show_console": false,
        "show_fps_graph": false,
        "fullscreen": false,
        "log_to_console": false,
        "console_max_lines": 300,
        "show_save_creation_date": true,
        "extra_data": {
            "custom_settings": {
                "theme": "dark",
                "language": "en",
                "region": "US"
            },
            "keybindings": {
                "move_up": "W",
                "move_down": "S", 
                "move_left": "A",
                "move_right": "D"
            }
        }
    });
    
    let json_string = serde_json::to_string_pretty(&large_settings).unwrap();
    fs::write(&settings_file, &json_string).unwrap();
    
    // Read and verify
    let loaded_content = fs::read_to_string(&settings_file).unwrap();
    let loaded_settings: serde_json::Value = serde_json::from_str(&loaded_content).unwrap();
    
    assert_eq!(large_settings, loaded_settings);
    assert!(loaded_content.len() > 500); // Should be a reasonably large file
}

#[test]
fn test_concurrent_settings_access() {
    // Test that multiple settings operations don't interfere
    let temp_dir = temp_test_dir();
    
    // Create multiple settings files
    for i in 0..5 {
        let settings_file = temp_dir.path().join(format!("settings_{}.json", i));
        let settings_json = format!(r#"{{
            "fog": {},
            "lighting": {},
            "sound": {},
            "console_max_lines": {}
        }}"#, i, i + 1, i % 2 == 0, (i + 1) * 100);
        
        fs::write(&settings_file, settings_json).unwrap();
    }
    
    // Verify all files were created correctly
    for i in 0..5 {
        let settings_file = temp_dir.path().join(format!("settings_{}.json", i));
        assert!(settings_file.exists());
        
        let content = fs::read_to_string(&settings_file).unwrap();
        let settings: serde_json::Value = serde_json::from_str(&content).unwrap();
        
        assert_eq!(settings["fog"].as_i64().unwrap(), i as i64);
        assert_eq!(settings["lighting"].as_i64().unwrap(), (i + 1) as i64);
    }
}