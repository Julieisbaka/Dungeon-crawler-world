use dungeon_crawler_world::terrain::{
    Camera3d, CellCoord3d, ChunkCoord, Direction3d, Terrain3dConfig, Terrain3dGenerator,
    NEIGHBORHOODS_PER_CELL,
};

fn test_config() -> Terrain3dConfig {
    Terrain3dConfig {
        chunk_size: [4, 4, 2],
        cell_size: [32.0, 32.0, 8.0],
        room_chance: 0.45,
        generation_radius: 1,
        seed: 11,
    }
}

#[test]
fn terrain_generates_chunks_on_demand_around_the_player() {
    let mut generator = Terrain3dGenerator::new(test_config());

    assert_eq!(generator.generated_chunk_count(), 0);
    assert_eq!(generator.ensure_chunks_around_player([0.0, 0.0, 0.0]), 27);
    assert_eq!(generator.generated_chunk_count(), 27);
    assert_eq!(generator.ensure_chunks_around_player([0.0, 0.0, 0.0]), 0);

    let chunk_world_x = generator.config().chunk_size[0] as f32 * generator.config().cell_size[0];
    let generated_after_move = generator.ensure_chunks_around_player([chunk_world_x, 0.0, 0.0]);

    assert!(generated_after_move > 0);
    assert!(generator.generated_chunk_count() > 27);
}

#[test]
fn chunk_lookup_uses_flooring_so_negative_positions_can_generate() {
    let mut generator = Terrain3dGenerator::new(test_config());
    let chunk_world_x = generator.config().chunk_size[0] as f32 * generator.config().cell_size[0];

    let coord = generator.chunk_for_world_position([-1.0, 0.0, 0.0]);
    assert_eq!(coord, ChunkCoord::new(-1, 0, 0));

    generator.ensure_chunks_around_player([-chunk_world_x, 0.0, 0.0]);
    assert!(generator.chunk(ChunkCoord::new(-2, -1, -1)).is_some());
}

#[test]
fn every_generated_cell_has_four_equal_neighborhoods() {
    let mut generator = Terrain3dGenerator::new(test_config());
    generator.ensure_chunks_around_player([0.0, 0.0, 0.0]);

    for chunk in generator.chunks() {
        assert_eq!(chunk.cells.len(), 4 * 4 * 2);
        for cell in &chunk.cells {
            assert_eq!(cell.neighborhoods.len(), NEIGHBORHOODS_PER_CELL);

            let first_size = cell.neighborhoods[0].size;
            for neighborhood in &cell.neighborhoods {
                assert_eq!(neighborhood.size, first_size);
                assert_eq!(neighborhood.size[0], cell.size[0] / 2.0);
                assert_eq!(neighborhood.size[1], cell.size[1] / 2.0);
                assert_eq!(neighborhood.size[2], cell.size[2]);
            }
        }
    }
}

#[test]
fn generated_cells_have_bidirectional_passages_across_chunk_borders() {
    let mut generator = Terrain3dGenerator::new(test_config());
    generator.ensure_chunks_around_player([0.0, 0.0, 0.0]);

    let border_cell = generator.cell(CellCoord3d::new(3, 0, 0)).unwrap();
    let east_passage = border_cell
        .passages
        .iter()
        .find(|passage| passage.direction == Direction3d::East)
        .expect("expected an east passage over the chunk border");
    let neighbor = generator.cell(east_passage.to).unwrap();

    assert!(neighbor
        .passages
        .iter()
        .any(|passage| passage.to == border_cell.coord && passage.direction == Direction3d::West));
}

#[test]
fn procedural_output_is_stable_for_the_same_seed_and_chunk() {
    let mut first = Terrain3dGenerator::new(test_config());
    let mut second = Terrain3dGenerator::new(test_config());

    first.ensure_chunks_around_player([0.0, 0.0, 0.0]);
    second.ensure_chunks_around_player([0.0, 0.0, 0.0]);

    assert_eq!(
        first.chunk(ChunkCoord::new(0, 0, 0)),
        second.chunk(ChunkCoord::new(0, 0, 0))
    );
}

#[test]
fn rooms_stay_inside_their_neighborhoods() {
    let mut config = test_config();
    config.room_chance = 1.0;
    let mut generator = Terrain3dGenerator::new(config);
    generator.ensure_chunks_around_player([0.0, 0.0, 0.0]);

    for chunk in generator.chunks() {
        for cell in &chunk.cells {
            for neighborhood in &cell.neighborhoods {
                let room = neighborhood
                    .room
                    .as_ref()
                    .expect("room should be generated");
                assert!(room.center[0] >= neighborhood.origin[0]);
                assert!(room.center[0] <= neighborhood.origin[0] + neighborhood.size[0]);
                assert!(room.center[1] >= neighborhood.origin[1]);
                assert!(room.center[1] <= neighborhood.origin[1] + neighborhood.size[1]);
                assert!(room.center[2] >= neighborhood.origin[2]);
                assert!(room.center[2] <= neighborhood.origin[2] + neighborhood.size[2]);
            }
        }
    }
}

#[test]
fn generated_terrain_builds_real_3d_mesh_geometry() {
    let mut config = test_config();
    config.room_chance = 1.0;
    let mut generator = Terrain3dGenerator::new(config);
    generator.ensure_chunks_around_player([0.0, 0.0, 0.0]);

    let mesh = generator.mesh_around([0.0, 0.0, 0.0], 3);

    assert!(!mesh.is_empty());
    assert!(mesh.triangle_count() > 100);
    assert_eq!(mesh.indices.len() % 3, 0);
    assert!(mesh
        .indices
        .iter()
        .all(|index| (*index as usize) < mesh.vertices.len()));

    let min_z = mesh
        .vertices
        .iter()
        .map(|vertex| vertex.position[2])
        .fold(f32::INFINITY, f32::min);
    let max_z = mesh
        .vertices
        .iter()
        .map(|vertex| vertex.position[2])
        .fold(f32::NEG_INFINITY, f32::max);

    assert!(max_z - min_z > generator.config().cell_size[2] * 0.5);
}

#[test]
fn player_marker_is_part_of_terrain_mesh_space() {
    let mut generator = Terrain3dGenerator::new(test_config());
    let player_position = [16.0, 16.0, 8.0];
    generator.ensure_chunks_around_player(player_position);

    let mut mesh = generator.mesh_around(player_position, 2);
    let before_vertices = mesh.vertices.len();
    mesh.add_player_marker(player_position, generator.config().cell_size);

    assert!(mesh.vertices.len() > before_vertices);
    assert!(mesh.vertices.iter().any(|vertex| {
        vertex.color == [0.95, 0.78, 0.22] && vertex.position[2] >= player_position[2]
    }));
}

#[test]
fn camera_builds_perspective_view_projection_matrix() {
    let camera = Camera3d::follow_player([30.0, 30.0, 0.0], [30.0, 30.0, 10.0]);
    let matrix = camera.view_projection_matrix(16.0 / 9.0);

    assert_eq!(matrix.len(), 4);
    assert!(matrix.iter().flatten().all(|value| value.is_finite()));
    assert_ne!(matrix[3][3], 1.0);
    assert!(camera.eye[2] > camera.target[2]);
}

#[test]
fn config_validation_clamps_degenerate_values_to_playable_generation() {
    let generator = Terrain3dGenerator::new(Terrain3dConfig {
        chunk_size: [0, 0, 0],
        cell_size: [0.0, -10.0, 0.0],
        room_chance: 2.0,
        generation_radius: -4,
        seed: 3,
    });

    assert_eq!(generator.config().chunk_size, [1, 1, 1]);
    assert_eq!(generator.config().cell_size, [1.0, 1.0, 1.0]);
    assert_eq!(generator.config().room_chance, 1.0);
    assert_eq!(generator.config().generation_radius, 0);
}
