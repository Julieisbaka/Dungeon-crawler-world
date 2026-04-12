use dungeon_crawler_world::grid::{restroom_distance, FloorGrid, RoomType};

// ── restroom_distance ──────────────────────────────────────────────────────────

#[test]
fn test_restroom_distance_within_range() {
    // Run many iterations to exercise the random range.
    for _ in 0..200 {
        let dist = restroom_distance();
        assert!(
            (300..=500).contains(&dist),
            "restroom_distance out of range: {}",
            dist
        );
    }
}

// ── Single-cell grid ───────────────────────────────────────────────────────────

#[test]
fn test_single_cell_grid_has_four_neighborhoods() {
    let grid = FloorGrid::new(1, 1, 100.0);
    assert_eq!(grid.cells.len(), 1);
    assert_eq!(grid.cells[0].len(), 1);
    assert_eq!(grid.cells[0][0].neighborhoods.len(), 4);
}

// ── RoomType equality ──────────────────────────────────────────────────────────

#[test]
fn test_room_type_variants_are_distinct() {
    assert_ne!(RoomType::Normal, RoomType::Bathroom);
    assert_ne!(RoomType::Normal, RoomType::SafeRoom);
    assert_ne!(RoomType::Normal, RoomType::Stairwell);
    assert_ne!(RoomType::Bathroom, RoomType::SafeRoom);
    assert_ne!(RoomType::Bathroom, RoomType::Stairwell);
    assert_ne!(RoomType::SafeRoom, RoomType::Stairwell);
}

// ── Grid cell coordinates ──────────────────────────────────────────────────────

#[test]
fn test_cell_coordinates_are_correct() {
    let grid = FloorGrid::new(3, 2, 100.0);

    for (y, row) in grid.cells.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            assert_eq!(cell.x, x, "cell.x mismatch at ({}, {})", x, y);
            assert_eq!(cell.y, y, "cell.y mismatch at ({}, {})", x, y);
        }
    }
}

// ── Grid zero-dimension edge case ──────────────────────────────────────────────

#[test]
fn test_zero_height_grid_has_no_cells() {
    let grid = FloorGrid::new(3, 0, 100.0);
    assert_eq!(grid.cells.len(), 0);
    assert_eq!(grid.width, 3);
    assert_eq!(grid.height, 0);
}

#[test]
fn test_zero_width_grid_has_empty_rows() {
    let grid = FloorGrid::new(0, 3, 100.0);
    // There are 3 rows, each with 0 columns.
    assert_eq!(grid.cells.len(), 3);
    for row in &grid.cells {
        assert!(row.is_empty());
    }
}
