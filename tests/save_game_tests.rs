use dungeon_crawler_world::new_save::{create_new_save_in, Difficulty};
use dungeon_crawler_world::player::{Player, PlayerPosition};
use dungeon_crawler_world::save_game::{load_save, save_player_position};
use dungeon_crawler_world::terrain3d::FLOOR_ONE_WIDTH_CELLS;
use std::fs;

mod common;
use common::unique_temp_dir;

#[test]
fn test_load_save_requires_and_returns_3d_world() {
    let saves_root = unique_temp_dir("load_save_world");
    create_new_save_in(&saves_root, "Loaded World", &Difficulty::Easy, false, false).unwrap();

    let loaded = load_save(&saves_root, "Loaded_World").unwrap();

    assert_eq!(loaded.folder_name, "Loaded_World");
    assert_eq!(loaded.world.width_cells, FLOOR_ONE_WIDTH_CELLS);
    assert_eq!(loaded.player.current_floor, 1);
}

#[test]
fn test_load_save_fails_without_world_file() {
    let saves_root = unique_temp_dir("load_save_missing_world");
    create_new_save_in(&saves_root, "Missing World", &Difficulty::Hard, false, false).unwrap();
    fs::remove_file(saves_root.join("Missing_World").join("world.json")).unwrap();

    let error = load_save(&saves_root, "Missing_World").unwrap_err();

    assert!(error.contains("world.json"));
}

#[test]
fn test_save_player_position_persists_to_player_json() {
    let saves_root = unique_temp_dir("save_player_position");
    create_new_save_in(&saves_root, "Moved Player", &Difficulty::Medium, false, false).unwrap();
    let position = PlayerPosition::new(10.0, 0.0, 25.0);

    save_player_position(&saves_root, "Moved_Player", position).unwrap();

    let player: Player = serde_json::from_str(
        &fs::read_to_string(saves_root.join("Moved_Player").join("player.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(player.position, position);
}
