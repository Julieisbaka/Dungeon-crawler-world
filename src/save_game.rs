use std::fs;
use std::path::Path;

use serde_json::Value;

use crate::game::terrain_collision::TerrainCollisionMap;
use crate::player::{Player, PlayerPosition};
use crate::terrain3d::{FloorOneTerrain, TerrainPoint};

pub const SAVE_FILE_NAME: &str = "save.json";
pub const PLAYER_FILE_NAME: &str = "player.json";
pub const WORLD_FILE_NAME: &str = "world.json";

#[derive(Debug, Clone)]
pub struct SaveGame {
    pub folder_name: String,
    pub save_data: Value,
    pub player: Player,
    pub world: FloorOneTerrain,
    pub collision_map: TerrainCollisionMap,
}

impl SaveGame {
    pub fn new(
        folder_name: String,
        save_data: Value,
        mut player: Player,
        world: FloorOneTerrain,
    ) -> Self {
        player.position = player_position_from_terrain(world.temporary_character.position);
        let collision_map = TerrainCollisionMap::from_terrain(&world);
        Self {
            folder_name,
            save_data,
            player,
            world,
            collision_map,
        }
    }
}

pub fn load_save(root: &Path, folder_name: &str) -> Result<SaveGame, String> {
    let save_dir = root.join(folder_name);
    let save_data: Value = read_json(&save_dir.join(SAVE_FILE_NAME))?;
    let player: Player = read_json(&save_dir.join(PLAYER_FILE_NAME))?;
    let world: FloorOneTerrain = read_json(&save_dir.join(WORLD_FILE_NAME))?;

    Ok(SaveGame {
        folder_name: folder_name.to_string(),
        save_data,
        player,
        collision_map: TerrainCollisionMap::from_terrain(&world),
        world,
    })
}

pub fn write_save_game(save_dir: &Path, save_game: &SaveGame) -> Result<(), String> {
    write_json(&save_dir.join(SAVE_FILE_NAME), &save_game.save_data)?;
    write_json(&save_dir.join(PLAYER_FILE_NAME), &save_game.player)?;
    write_json(&save_dir.join(WORLD_FILE_NAME), &save_game.world)
}

pub fn write_world(save_dir: &Path, world: &FloorOneTerrain) -> Result<(), String> {
    write_json(&save_dir.join(WORLD_FILE_NAME), world)
}

pub fn save_player_position(
    root: &Path,
    folder_name: &str,
    position: PlayerPosition,
) -> Result<(), String> {
    let player_path = root.join(folder_name).join(PLAYER_FILE_NAME);
    let mut player: Player = read_json(&player_path)?;
    player.position = position;
    write_json(&player_path, &player)
}

pub fn save_player_state(root: &Path, folder_name: &str, player: &Player) -> Result<(), String> {
    write_json(&root.join(folder_name).join(PLAYER_FILE_NAME), player)
}

pub fn player_position_from_terrain(point: TerrainPoint) -> PlayerPosition {
    PlayerPosition::new(point.x, point.y, point.z)
}

fn read_json<T>(path: &Path) -> Result<T, String>
where
    T: serde::de::DeserializeOwned,
{
    let content = fs::read_to_string(path)
        .map_err(|error| format!("Failed to read '{}': {}", path.display(), error))?;
    serde_json::from_str(&content)
        .map_err(|error| format!("Failed to parse '{}': {}", path.display(), error))
}

fn write_json<T>(path: &Path, value: &T) -> Result<(), String>
where
    T: serde::Serialize,
{
    let content = serde_json::to_string_pretty(value)
        .map_err(|error| format!("Failed to serialize '{}': {}", path.display(), error))?;
    fs::write(path, content)
        .map_err(|error| format!("Failed to write '{}': {}", path.display(), error))
}
