use dungeon_crawler_world::player::{Player, PlayerLook, PlayerPosition, PlayerStats};
use std::collections::HashMap;

#[test]
fn test_player_new_creates_valid_instance() {
    // Arrange
    let stats = PlayerStats {
        strength: 10,
        intelligence: 12,
        dexterity: 8,
        charisma: 9,
        constitution: 11,
    };

    let mut spells = HashMap::new();
    spells.insert("Fireball".to_string(), 3);
    spells.insert("Lightning Bolt".to_string(), 2);

    let mut inventory = HashMap::new();
    inventory.insert("Health Potion".to_string(), 5);
    inventory.insert("Sword".to_string(), 1);

    let mut skills = HashMap::new();
    skills.insert("Swordsmanship".to_string(), 7);
    skills.insert("Magic".to_string(), 5);

    let sub_classes = vec!["Warrior".to_string(), "Mage".to_string()];

    // Act
    let player = Player::new(
        "TestHero".to_string(),
        5,
        spells.clone(),
        inventory.clone(),
        skills.clone(),
        100,
        sub_classes.clone(),
        "Fighter".to_string(),
        "Human".to_string(),
        false,
        1,
        stats.clone(),
    );

    // Assert
    assert_eq!(player.name, "TestHero");
    assert_eq!(player.level, 5);
    assert_eq!(player.position, PlayerPosition::spawn());
    assert_eq!(player.look, PlayerLook::forward());
    assert_eq!(player.spells, spells);
    assert_eq!(player.inventory, inventory);
    assert_eq!(player.skills, skills);
    assert_eq!(player.coins, 100);
    assert_eq!(player.sub_classes, sub_classes);
    assert_eq!(player.class, "Fighter");
    assert_eq!(player.race, "Human");
    assert!(!player.has_manager);
    assert_eq!(player.current_floor, 1);
    assert_eq!(player.stats, stats);
}

#[test]
fn test_player_to_json_serialization() {
    // Arrange
    let stats = PlayerStats {
        strength: 15,
        intelligence: 10,
        dexterity: 12,
        charisma: 8,
        constitution: 14,
    };

    let mut spells = HashMap::new();
    spells.insert("Fireball".to_string(), 3);

    let mut inventory = HashMap::new();
    inventory.insert("Health Potion".to_string(), 5);

    let mut skills = HashMap::new();
    skills.insert("Swordsmanship".to_string(), 7);

    let player = Player::new(
        "TestHero".to_string(),
        5,
        spells,
        inventory,
        skills,
        100,
        vec!["Warrior".to_string()],
        "Fighter".to_string(),
        "Human".to_string(),
        true,
        2,
        stats,
    );

    // Act
    let json = player.to_json();

    // Assert
    assert!(json.is_object());
    assert_eq!(json["name"], "TestHero");
    assert_eq!(json["level"], 5);
    assert_eq!(json["position"]["x"], 0.0);
    assert_eq!(json["position"]["y"], 0.0);
    assert_eq!(json["position"]["z"], 0.0);
    assert_eq!(json["look"]["yaw"], 0.0);
    assert_eq!(json["look"]["pitch"], 0.0);
    assert_eq!(json["coins"], 100);
    assert_eq!(json["class"], "Fighter");
    assert_eq!(json["race"], "Human");
    assert!(json["has_manager"].as_bool().unwrap());
    assert_eq!(json["current_floor"], 2);
    assert_eq!(json["stats"]["strength"], 15);
    assert_eq!(json["stats"]["intelligence"], 10);
    assert_eq!(json["stats"]["dexterity"], 12);
    assert_eq!(json["stats"]["charisma"], 8);
    assert_eq!(json["stats"]["constitution"], 14);
}

#[test]
fn test_player_from_json_deserialization() {
    // Arrange
    let json_str = r#"{
        "name": "TestHero",
        "level": 5,
        "position": {
            "x": 12.5,
            "y": 0.0,
            "z": 40.25
        },
        "look": {
            "yaw": 1.25,
            "pitch": -0.2
        },
        "spells": {
            "Fireball": 3
        },
        "inventory": {
            "Health Potion": 5
        },
        "skills": {
            "Swordsmanship": 7
        },
        "coins": 100,
        "sub_classes": ["Warrior"],
        "class": "Fighter",
        "race": "Human",
        "has_manager": true,
        "current_floor": 2,
        "stats": {
            "strength": 15,
            "intelligence": 10,
            "dexterity": 12,
            "charisma": 8,
            "constitution": 14
        }
    }"#;

    let json: serde_json::Value = serde_json::from_str(json_str).unwrap();

    // Act
    let player = Player::from_json(&json);

    // Assert
    assert!(player.is_some());
    let player = player.unwrap();
    assert_eq!(player.name, "TestHero");
    assert_eq!(player.level, 5);
    assert_eq!(player.position, PlayerPosition::new(12.5, 0.0, 40.25));
    assert_eq!(player.look, PlayerLook::new(1.25, -0.2));
    assert_eq!(player.spells.get("Fireball"), Some(&3));
    assert_eq!(player.inventory.get("Health Potion"), Some(&5));
    assert_eq!(player.skills.get("Swordsmanship"), Some(&7));
    assert_eq!(player.coins, 100);
    assert_eq!(player.sub_classes, vec!["Warrior"]);
    assert_eq!(player.class, "Fighter");
    assert_eq!(player.race, "Human");
    assert!(player.has_manager);
    assert_eq!(player.current_floor, 2);
    assert_eq!(player.stats.strength, 15);
    assert_eq!(player.stats.intelligence, 10);
    assert_eq!(player.stats.dexterity, 12);
    assert_eq!(player.stats.charisma, 8);
    assert_eq!(player.stats.constitution, 14);
}

#[test]
fn test_player_round_trip_serialization() {
    // Arrange
    let stats = PlayerStats {
        strength: 18,
        intelligence: 14,
        dexterity: 16,
        charisma: 12,
        constitution: 15,
    };

    let mut spells = HashMap::new();
    spells.insert("Fireball".to_string(), 3);
    spells.insert("Ice Storm".to_string(), 2);
    spells.insert("Lightning Bolt".to_string(), 4);

    let mut inventory = HashMap::new();
    inventory.insert("Health Potion".to_string(), 5);
    inventory.insert("Mana Potion".to_string(), 3);
    inventory.insert("Sword".to_string(), 1);

    let mut skills = HashMap::new();
    skills.insert("Swordsmanship".to_string(), 7);
    skills.insert("Magic".to_string(), 5);
    skills.insert("Stealth".to_string(), 3);

    let sub_classes = vec![
        "Warrior".to_string(),
        "Mage".to_string(),
        "Rogue".to_string(),
    ];

    let original_player = Player::new(
        "TestHero".to_string(),
        10,
        spells,
        inventory,
        skills,
        500,
        sub_classes,
        "Adventurer".to_string(),
        "Elf".to_string(),
        true,
        5,
        stats,
    );

    // Act - Serialize and then deserialize
    let json = original_player.to_json();
    let deserialized_player = Player::from_json(&json);

    // Assert
    assert!(deserialized_player.is_some());
    let deserialized_player = deserialized_player.unwrap();
    assert_eq!(deserialized_player, original_player);
}

#[test]
fn test_player_with_empty_strings() {
    // Arrange
    let stats = PlayerStats {
        strength: 10,
        intelligence: 10,
        dexterity: 10,
        charisma: 10,
        constitution: 10,
    };

    let spells = HashMap::new();
    let inventory = HashMap::new();
    let skills = HashMap::new();

    // Act
    let player = Player::new(
        "".to_string(),
        1,
        spells,
        inventory,
        skills,
        0,
        vec![],
        "".to_string(),
        "".to_string(),
        false,
        0,
        stats.clone(),
    );

    // Assert
    assert_eq!(player.name, "");
    assert_eq!(player.class, "");
    assert_eq!(player.race, "");
    assert_eq!(player.sub_classes.len(), 0);
}

#[test]
fn test_player_with_zero_values() {
    // Arrange
    let stats = PlayerStats {
        strength: 0,
        intelligence: 0,
        dexterity: 0,
        charisma: 0,
        constitution: 0,
    };

    let spells = HashMap::new();
    let inventory = HashMap::new();
    let skills = HashMap::new();

    // Act
    let player = Player::new(
        "ZeroHero".to_string(),
        0,
        spells,
        inventory,
        skills,
        0,
        vec![],
        "Fighter".to_string(),
        "Human".to_string(),
        false,
        0,
        stats.clone(),
    );

    // Assert
    assert_eq!(player.level, 0);
    assert_eq!(player.coins, 0);
    assert_eq!(player.current_floor, 0);
    assert_eq!(player.stats.strength, 0);
    assert_eq!(player.stats.intelligence, 0);
    assert_eq!(player.stats.dexterity, 0);
    assert_eq!(player.stats.charisma, 0);
    assert_eq!(player.stats.constitution, 0);
}

#[test]
fn test_player_with_negative_values() {
    // Arrange
    let stats = PlayerStats {
        strength: -5,
        intelligence: -3,
        dexterity: -7,
        charisma: -2,
        constitution: -10,
    };

    let spells = HashMap::new();
    let inventory = HashMap::new();
    let skills = HashMap::new();

    // Act
    let player = Player::new(
        "NegativeHero".to_string(),
        1,
        spells,
        inventory,
        skills,
        -100,
        vec![],
        "Cursed".to_string(),
        "Undead".to_string(),
        false,
        1,
        stats.clone(),
    );

    // Assert
    assert_eq!(player.coins, -100);
    assert_eq!(player.stats.strength, -5);
    assert_eq!(player.stats.intelligence, -3);
    assert_eq!(player.stats.dexterity, -7);
    assert_eq!(player.stats.charisma, -2);
    assert_eq!(player.stats.constitution, -10);
}

#[test]
fn test_player_with_empty_collections() {
    // Arrange
    let stats = PlayerStats {
        strength: 10,
        intelligence: 10,
        dexterity: 10,
        charisma: 10,
        constitution: 10,
    };

    let spells = HashMap::new();
    let inventory = HashMap::new();
    let skills = HashMap::new();

    // Act
    let player = Player::new(
        "EmptyHero".to_string(),
        5,
        spells.clone(),
        inventory.clone(),
        skills.clone(),
        100,
        vec![],
        "Fighter".to_string(),
        "Human".to_string(),
        false,
        1,
        stats,
    );

    // Assert
    assert_eq!(player.spells.len(), 0);
    assert_eq!(player.inventory.len(), 0);
    assert_eq!(player.skills.len(), 0);
    assert_eq!(player.sub_classes.len(), 0);
}

#[test]
fn test_player_round_trip_with_empty_collections() {
    // Arrange
    let stats = PlayerStats {
        strength: 10,
        intelligence: 10,
        dexterity: 10,
        charisma: 10,
        constitution: 10,
    };

    let original_player = Player::new(
        "EmptyHero".to_string(),
        5,
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        100,
        vec![],
        "Fighter".to_string(),
        "Human".to_string(),
        false,
        1,
        stats,
    );

    // Act
    let json = original_player.to_json();
    let deserialized_player = Player::from_json(&json);

    // Assert
    assert!(deserialized_player.is_some());
    let deserialized_player = deserialized_player.unwrap();
    assert_eq!(deserialized_player, original_player);
}

#[test]
fn test_player_from_json_invalid_data_returns_none() {
    // Arrange - Missing required field "name"
    let json_str = r#"{
        "level": 5,
        "spells": {},
        "inventory": {},
        "skills": {},
        "coins": 100,
        "sub_classes": [],
        "class": "Fighter",
        "race": "Human",
        "has_manager": false,
        "current_floor": 1,
        "stats": {
            "strength": 10,
            "intelligence": 10,
            "dexterity": 10,
            "charisma": 10,
            "constitution": 10
        }
    }"#;

    let json: serde_json::Value = serde_json::from_str(json_str).unwrap();

    // Act
    let player = Player::from_json(&json);

    // Assert
    assert!(player.is_none());
}

#[test]
fn test_player_from_json_wrong_types_returns_none() {
    // Arrange - Wrong type for "level" (string instead of number)
    let json_str = r#"{
        "name": "TestHero",
        "level": "five",
        "spells": {},
        "inventory": {},
        "skills": {},
        "coins": 100,
        "sub_classes": [],
        "class": "Fighter",
        "race": "Human",
        "has_manager": false,
        "current_floor": 1,
        "stats": {
            "strength": 10,
            "intelligence": 10,
            "dexterity": 10,
            "charisma": 10,
            "constitution": 10
        }
    }"#;

    let json: serde_json::Value = serde_json::from_str(json_str).unwrap();

    // Act
    let player = Player::from_json(&json);

    // Assert
    assert!(player.is_none());
}

#[test]
fn test_player_stats_equality() {
    // Arrange
    let stats1 = PlayerStats {
        strength: 10,
        intelligence: 12,
        dexterity: 8,
        charisma: 9,
        constitution: 11,
    };

    let stats2 = PlayerStats {
        strength: 10,
        intelligence: 12,
        dexterity: 8,
        charisma: 9,
        constitution: 11,
    };

    let stats3 = PlayerStats {
        strength: 15,
        intelligence: 12,
        dexterity: 8,
        charisma: 9,
        constitution: 11,
    };

    // Assert
    assert_eq!(stats1, stats2);
    assert_ne!(stats1, stats3);
}
