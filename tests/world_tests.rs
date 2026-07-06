use dungeon_crawler_world::input::{GameCommand, KeyBindings};
use dungeon_crawler_world::player::{Player, PlayerStats};
use dungeon_crawler_world::world::WorldSession;
use serde_json::json;
use std::collections::HashMap;
use std::fs;

mod common;
use common::unique_temp_dir;

#[test]
fn loading_save_enters_generated_world_session() {
    let root = unique_temp_dir("world_load_save");
    let save = root.join("Crawler_One");
    fs::create_dir_all(&save).unwrap();
    write_save_json(&save, 99);
    write_player_json(&save);

    let world = WorldSession::load_from_root(&root, "Crawler_One").unwrap();

    assert_eq!(world.save_name, "Crawler_One");
    assert_eq!(world.player.level, 1);
    assert_eq!(world.current_cell().z, 0);
    assert!(world.terrain.generated_chunk_count() > 0);
    assert!(world.terrain.cell(world.current_cell()).is_some());

    let _ = fs::remove_dir_all(root);
}

#[test]
fn loaded_world_exposes_real_3d_terrain_mesh() {
    let root = unique_temp_dir("world_mesh");
    let save = root.join("Crawler_Mesh");
    fs::create_dir_all(&save).unwrap();
    write_save_json(&save, 777);
    write_player_json(&save);

    let world = WorldSession::load_from_root(&root, "Crawler_Mesh").unwrap();
    let mesh = world.visible_terrain_mesh();

    assert!(!mesh.is_empty());
    assert!(mesh.triangle_count() > 100);
    assert!(mesh.vertices.iter().any(|vertex| vertex.position[2] > 1.0));
    assert!(mesh
        .vertices
        .iter()
        .any(|vertex| vertex.color == [0.95, 0.78, 0.22]));

    let _ = fs::remove_dir_all(root);
}

#[test]
fn movement_generates_more_terrain_when_crossing_chunks() {
    let root = unique_temp_dir("world_move_chunks");
    let save = root.join("Crawler_Two");
    fs::create_dir_all(&save).unwrap();
    write_save_json(&save, 123);
    write_player_json(&save);

    let mut world = WorldSession::load_from_root(&root, "Crawler_Two").unwrap();
    let initial_chunks = world.terrain.generated_chunk_count();

    for _ in 0..20 {
        world.move_player_planar([1.0, 0.0], 1.0);
    }

    assert!(world.terrain.generated_chunk_count() > initial_chunks);

    let _ = fs::remove_dir_all(root);
}

#[test]
fn keybinds_default_to_skills_h_and_inventory_e() {
    let keybinds = KeyBindings::default();

    assert_eq!(keybinds.key_for(GameCommand::Skills), egui::Key::H);
    assert_eq!(keybinds.key_for(GameCommand::Inventory), egui::Key::E);
}

#[test]
fn movement_is_continuous_and_jump_returns_to_ground() {
    let root = unique_temp_dir("world_continuous_movement");
    let save = root.join("Crawler_Move");
    fs::create_dir_all(&save).unwrap();
    write_save_json(&save, 789);
    write_player_json(&save);

    let mut world = WorldSession::load_from_root(&root, "Crawler_Move").unwrap();
    let start = world.player_position;

    world.move_player_planar([1.0, 0.0], 0.25);
    assert!(world.player_position[0] > start[0]);
    assert_eq!(world.player_position[2], start[2]);

    world.jump();
    world.update_physics(0.1);
    assert!(world.player_position[2] > start[2]);

    for _ in 0..30 {
        world.update_physics(0.1);
    }
    assert_eq!(world.player_position[2], start[2]);

    let _ = fs::remove_dir_all(root);
}

#[test]
fn pause_changes_world_state() {
    let root = unique_temp_dir("world_pause_panels");
    let save = root.join("Crawler_Three");
    fs::create_dir_all(&save).unwrap();
    write_save_json(&save, 456);
    write_player_json(&save);

    let mut world = WorldSession::load_from_root(&root, "Crawler_Three").unwrap();
    world.toggle_pause();
    assert!(world.paused);

    world.resume();
    assert!(!world.paused);

    let _ = fs::remove_dir_all(root);
}

fn write_save_json(save: &std::path::Path, seed: u64) {
    fs::write(
        save.join("save.json"),
        serde_json::to_string_pretty(&json!({
            "save_name": "Crawler",
            "difficulty": "Medium",
            "created_at": "2026-07-06T00:00:00Z",
            "terrain_seed": seed,
            "floor_one": { "is_cleared": false, "time": 50000 },
            "gamerules": []
        }))
        .unwrap(),
    )
    .unwrap();
}

fn write_player_json(save: &std::path::Path) {
    let mut skills = HashMap::new();
    skills.insert("Walking".to_string(), 4);
    let player = Player::new(
        "Crawler".to_string(),
        1,
        HashMap::new(),
        HashMap::new(),
        skills,
        0,
        Vec::new(),
        String::new(),
        String::new(),
        false,
        1,
        PlayerStats {
            strength: 4,
            intelligence: 4,
            dexterity: 4,
            charisma: 4,
            constitution: 4,
        },
    );
    fs::write(
        save.join("player.json"),
        serde_json::to_string_pretty(&player).unwrap(),
    )
    .unwrap();
}
