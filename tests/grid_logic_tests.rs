use dungeon_crawler_world::logic::grid_logic::GridState;

// ── GridState::default ─────────────────────────────────────────────────────────

#[test]
fn test_grid_state_default_values() {
    let state = GridState::default();

    // The default grid is 3×3 with cell_size 200.
    assert_eq!(state.grid.width, 3);
    assert_eq!(state.grid.height, 3);
    assert_eq!(state.grid.cell_size, 200.0);

    // Default zoom and pan.
    assert_eq!(state.zoom, 1.0);
    assert_eq!(state.pan_x, 0.0);
    assert_eq!(state.pan_y, 0.0);
}

// ── GridState::reset_view ──────────────────────────────────────────────────────

#[test]
fn test_reset_view_restores_zoom_and_pan() {
    let mut state = GridState::default();
    state.zoom = 2.5;
    state.pan_x = 100.0;
    state.pan_y = -50.0;

    state.reset_view();

    assert_eq!(state.zoom, 1.0);
    assert_eq!(state.pan_x, 0.0);
    assert_eq!(state.pan_y, 0.0);
}

// ── GridState::regenerate ──────────────────────────────────────────────────────

#[test]
fn test_regenerate_preserves_grid_dimensions() {
    let mut state = GridState::default();

    state.regenerate();

    assert_eq!(state.grid.width, 3);
    assert_eq!(state.grid.height, 3);
    assert_eq!(state.grid.cell_size, 200.0);
}

#[test]
fn test_regenerate_does_not_change_zoom_or_pan() {
    let mut state = GridState::default();
    state.zoom = 1.5;
    state.pan_x = 10.0;
    state.pan_y = 20.0;

    state.regenerate();

    // regenerate() should only affect the grid, not the view state.
    assert_eq!(state.zoom, 1.5);
    assert_eq!(state.pan_x, 10.0);
    assert_eq!(state.pan_y, 20.0);
}

#[test]
fn test_regenerate_maintains_non_empty_cells() {
    let mut state = GridState::default();
    state.regenerate();

    for row in &state.grid.cells {
        for cell in row {
            assert_eq!(cell.neighborhoods.len(), 4);
            for neighborhood in &cell.neighborhoods {
                assert!(!neighborhood.nodes.is_empty());
            }
        }
    }
}
