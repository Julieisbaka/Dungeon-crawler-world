use dungeon_crawler_world::game::terrain_collision::TerrainCollisionMap;
use dungeon_crawler_world::player::PlayerPosition;
use dungeon_crawler_world::terrain3d::{FloorOneTerrain, TerrainPoint};

#[test]
fn generated_room_centers_are_walkable() {
    let terrain = FloorOneTerrain::new(1, 1);
    let collision = TerrainCollisionMap::from_terrain(&terrain);

    for cell_row in &terrain.cells {
        for cell in cell_row {
            for neighborhood in &cell.neighborhoods {
                for node in &neighborhood.nodes {
                    assert!(
                        collision.contains_position(player_position(node.position)),
                        "room center should be walkable"
                    );
                }
            }
        }
    }
}

#[test]
fn generated_pathway_midpoints_are_walkable() {
    let terrain = FloorOneTerrain::new(1, 1);
    let collision = TerrainCollisionMap::from_terrain(&terrain);
    let cell = &terrain.cells[0][0];

    for neighborhood in &cell.neighborhoods {
        for pathway in &neighborhood.pathways {
            let from = neighborhood
                .nodes
                .iter()
                .find(|node| node.id == pathway.from)
                .unwrap();
            let to = neighborhood
                .nodes
                .iter()
                .find(|node| node.id == pathway.to)
                .unwrap();
            assert!(collision.contains_position(midpoint(from.position, to.position)));
        }
    }
}

#[test]
fn movement_outside_walkable_terrain_is_blocked() {
    let terrain = FloorOneTerrain::new(1, 1);
    let collision = TerrainCollisionMap::from_terrain(&terrain);
    let spawn = player_position(terrain.temporary_character.position);
    let outside = PlayerPosition::new(-100.0, 0.0, -100.0);

    assert!(collision.contains_position(spawn));
    assert!(!collision.contains_position(outside));
    let constrained = collision.constrain_movement(spawn, outside);
    assert!(collision.contains_position(constrained));
    assert_ne!(constrained, outside);
}

#[test]
fn movement_sweeps_and_stops_before_leaving_walkable_terrain() {
    let terrain = FloorOneTerrain::new(1, 1);
    let collision = TerrainCollisionMap::from_terrain(&terrain);
    let spawn = player_position(terrain.temporary_character.position);
    let requested = PlayerPosition::new(spawn.x - 500.0, spawn.y, spawn.z - 500.0);

    let constrained = collision.constrain_movement(spawn, requested);

    assert!(collision.contains_position(constrained));
    assert_ne!(constrained, requested);
}

fn player_position(point: TerrainPoint) -> PlayerPosition {
    PlayerPosition::new(point.x, point.y, point.z)
}

fn midpoint(from: TerrainPoint, to: TerrainPoint) -> PlayerPosition {
    PlayerPosition::new((from.x + to.x) * 0.5, 0.0, (from.z + to.z) * 0.5)
}
