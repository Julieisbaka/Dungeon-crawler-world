use std::collections::{HashSet, VecDeque};

use dungeon_crawler_world::terrain3d::{
    FloorOneTerrain, TerrainCell, TerrainNodeId, TerrainRoomType, CELL_SIZE_METERS,
    FLOOR_ONE_HEIGHT_CELLS, FLOOR_ONE_WIDTH_CELLS, NEIGHBORHOOD_SIZE_METERS,
    RESTROOM_MIN_DISTANCE_METERS, ROOM_GRID_SPACING_METERS, TEMPORARY_CHARACTER_SPEED,
};

#[test]
fn floor_one_terrain_is_separate_from_2d_grid_defaults() {
    let terrain = FloorOneTerrain::generate();

    assert_eq!(terrain.width_cells, FLOOR_ONE_WIDTH_CELLS);
    assert_eq!(terrain.height_cells, FLOOR_ONE_HEIGHT_CELLS);
    assert_eq!(terrain.cell_size_meters, CELL_SIZE_METERS);
    assert_eq!(terrain.cells.len(), FLOOR_ONE_HEIGHT_CELLS);
    assert_eq!(terrain.cells[0].len(), FLOOR_ONE_WIDTH_CELLS);
}

#[test]
fn each_cell_is_square_and_has_four_square_neighborhoods() {
    let terrain = FloorOneTerrain::new(2, 2);

    for row in &terrain.cells {
        for cell in row {
            assert_eq!(cell.size_meters, NEIGHBORHOOD_SIZE_METERS * 2.0);
            assert_eq!(cell.neighborhoods.len(), 4);

            for neighborhood in &cell.neighborhoods {
                assert_eq!(neighborhood.size_meters, NEIGHBORHOOD_SIZE_METERS);
            }
        }
    }
}

#[test]
fn neighborhood_interiors_use_weighted_kruskal_paths() {
    let terrain = FloorOneTerrain::new(1, 1);
    let cell = &terrain.cells[0][0];

    for neighborhood in &cell.neighborhoods {
        let expected_minimum_rooms =
            (NEIGHBORHOOD_SIZE_METERS / ROOM_GRID_SPACING_METERS).ceil() as usize;
        assert!(neighborhood.nodes.len() >= expected_minimum_rooms.pow(2));
        assert_eq!(neighborhood.pathways.len(), neighborhood.nodes.len() - 1);

        let node_ids: HashSet<TerrainNodeId> = neighborhood.nodes.iter().map(|node| node.id).collect();
        for pathway in &neighborhood.pathways {
            assert!(node_ids.contains(&pathway.from));
            assert!(node_ids.contains(&pathway.to));
            assert!(pathway.weight > 0.0);
        }
    }
}

#[test]
fn restrooms_are_spaced_at_least_three_hundred_meters_apart() {
    let terrain = FloorOneTerrain::new(2, 2);

    for row in &terrain.cells {
        for cell in row {
            for neighborhood in &cell.neighborhoods {
                let restrooms: Vec<_> = neighborhood
                    .nodes
                    .iter()
                    .filter(|node| node.room_type == TerrainRoomType::Restroom)
                    .collect();

                assert!(!restrooms.is_empty());
                for (left_index, left) in restrooms.iter().enumerate() {
                    for right in restrooms.iter().skip(left_index + 1) {
                        assert!(
                            left.position.distance_xz(right.position) >= RESTROOM_MIN_DISTANCE_METERS,
                            "restrooms were closer than {RESTROOM_MIN_DISTANCE_METERS} meters"
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn every_neighborhood_in_a_cell_can_reach_every_other_neighborhood() {
    let terrain = FloorOneTerrain::new(2, 2);

    for row in &terrain.cells {
        for cell in row {
            assert_cell_neighborhoods_are_connected(cell);
        }
    }
}

#[test]
fn terrain_has_a_ground_mesh_and_temporary_character_speed_one() {
    let terrain = FloorOneTerrain::generate();

    assert_eq!(terrain.terrain_mesh.vertices.len(), 4);
    assert_eq!(terrain.terrain_mesh.indices, vec![[0, 1, 2], [0, 2, 3]]);
    assert_eq!(terrain.temporary_character.speed, TEMPORARY_CHARACTER_SPEED);
    assert_eq!(terrain.temporary_character.speed, 1.0);
}

#[test]
fn terrain_generation_persists_real_3d_room_and_corridor_volumes() {
    let terrain = FloorOneTerrain::new(1, 1);
    let room_count: usize = terrain.cells[0][0]
        .neighborhoods
        .iter()
        .map(|neighborhood| neighborhood.nodes.len())
        .sum();
    let corridor_count: usize = terrain.cells[0][0]
        .neighborhoods
        .iter()
        .map(|neighborhood| neighborhood.pathways.len())
        .sum::<usize>()
        + terrain.cells[0][0].neighborhood_pathways.len();

    assert_eq!(terrain.rooms.len(), room_count);
    assert_eq!(terrain.corridors.len(), corridor_count);
    assert!(terrain.rooms.iter().all(|room| room.half_extents.y > 0.0));
    assert!(terrain.corridors.iter().all(|corridor| corridor.height > 0.0));
    assert!(terrain.corridors.iter().all(|corridor| corridor.half_width > 0.0));
}

fn assert_cell_neighborhoods_are_connected(cell: &TerrainCell) {
    let mut graph = vec![Vec::new(); cell.neighborhoods.len()];
    for pathway in &cell.neighborhood_pathways {
        graph[pathway.from.neighborhood].push(pathway.to.neighborhood);
        graph[pathway.to.neighborhood].push(pathway.from.neighborhood);
    }

    let mut visited = HashSet::new();
    let mut queue = VecDeque::from([0]);

    while let Some(neighborhood) = queue.pop_front() {
        if visited.insert(neighborhood) {
            for next in &graph[neighborhood] {
                queue.push_back(*next);
            }
        }
    }

    assert_eq!(visited.len(), cell.neighborhoods.len());
}
