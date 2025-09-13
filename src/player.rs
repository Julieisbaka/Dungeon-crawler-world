use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Player {
    pub name: String,
    pub level: u32,
    pub spells: HashMap<String, i32>,
    pub inventory: HashMap<String, i32>,
    pub skills: HashMap<String, i8>,
    pub coins: i32,
    pub sub_classes: Vec<String>,
    pub class: String,
    pub race: String,
    pub has_manager: bool,
    pub current_floor: u32,
    pub stats: PlayerStats,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PlayerStats {
    pub strength: i16,
    pub intelligence: i16,
    pub dexterity: i16,
    pub charisma: i16,
    pub constitution: i16,
}

impl Player {
    pub fn new(
        name: String,
        level: u32,
        spells: HashMap<String, i32>,
        inventory: HashMap<String, i32>,
        skills: HashMap<String, i8>,
        coins: i32,
        sub_classes: Vec<String>,
        class: String,
        race: String,
        has_manager: bool,
        current_floor: u32,
        stats: PlayerStats,
    ) -> Self {
        Self {
            name,
            level,
            spells,
            inventory,
            skills,
            coins,
            sub_classes,
            class,
            race,
            has_manager,
            current_floor,
            stats,
        }
    }

    pub fn from_json(json: &serde_json::Value) -> Option<Self> {
        serde_json::from_value(json.clone()).ok()
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}
