use dungeon_crawler_world::new_save::{create_new_save_in, Difficulty};
use dungeon_crawler_world::player::Player;
use dungeon_crawler_world::save_game::{load_save, player_position_from_terrain};
use dungeon_crawler_world::terrain3d::{FloorOneTerrain, FLOOR_ONE_HEIGHT_CELLS, FLOOR_ONE_WIDTH_CELLS};
use dungeon_crawler_world::{generate_floor_one_time, generate_stats};
use rand::thread_rng;
use std::fs;

mod common;
use common::unique_temp_dir;

#[test]
fn test_real_time_generation() {
    let mut rng: rand::prelude::ThreadRng = thread_rng();
    let time: u32 = generate_floor_one_time(true, &mut rng);
    assert_eq!(time, 432_000, "Real time should be exactly 432000 seconds (5 days)");
}

#[test]
fn test_normal_time_generation_range() {
    let mut rng: rand::prelude::ThreadRng = thread_rng();
    // Run multiple times to check the range
    for _ in 0..100 {
        let time: u32 = generate_floor_one_time(false, &mut rng);
        // 12h = 43200, 20h = 72000
        assert!((43_200..=72_000).contains(&time), "Normal time should be between 12h and 20h in seconds, got {}", time);
    }
}

#[test]
fn test_stat_generation_ranges() {
    let mut rng: rand::prelude::ThreadRng = thread_rng();
    for _ in 0..100 {
        let (walking, swimming, breathing, strength, intelligence, dexterity, charisma, constitution) = generate_stats(&mut rng);
        assert!((3..=5).contains(&walking), "Walking out of range: {}", walking);
        assert!((3..=5).contains(&swimming), "Swimming out of range: {}", swimming);
        assert!((3..=5).contains(&breathing), "Breathing out of range: {}", breathing);
        assert!((1..=8).contains(&strength), "Strength out of range: {}", strength);
        assert!((3..=5).contains(&intelligence), "Intelligence out of range: {}", intelligence);
        assert!((2..=6).contains(&dexterity), "Dexterity out of range: {}", dexterity);
        assert!((2..=4).contains(&charisma), "Charisma out of range: {}", charisma);
        assert!((2..=6).contains(&constitution), "Constitution out of range: {}", constitution);
    }
}

#[test]
fn test_create_new_save_writes_3d_world_and_player_position() {
    let saves_root = unique_temp_dir("new_save_world");

    create_new_save_in(
        &saves_root,
        "Floor One",
        &Difficulty::Medium,
        false,
        false,
    )
    .unwrap();

    let save_dir = saves_root.join("Floor_One");
    let save_json: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(save_dir.join("save.json")).unwrap(),
    )
    .unwrap();
    let player: Player = serde_json::from_str(
        &fs::read_to_string(save_dir.join("player.json")).unwrap(),
    )
    .unwrap();
    let world: FloorOneTerrain = serde_json::from_str(
        &fs::read_to_string(save_dir.join("world.json")).unwrap(),
    )
    .unwrap();

    assert_eq!(save_json["world_file"], "world.json");
    assert_eq!(world.width_cells, FLOOR_ONE_WIDTH_CELLS);
    assert_eq!(world.height_cells, FLOOR_ONE_HEIGHT_CELLS);
    assert_eq!(
        player.position,
        player_position_from_terrain(world.temporary_character.position)
    );

    let loaded = load_save(&saves_root, "Floor_One").unwrap();
    assert_eq!(loaded.folder_name, "Floor_One");
    assert_eq!(loaded.player.position, player.position);
    assert_eq!(loaded.world.width_cells, FLOOR_ONE_WIDTH_CELLS);
}
