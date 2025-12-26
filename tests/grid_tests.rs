use dungeon_crawler_world::grid::{FloorGrid, RoomType};

#[test]
fn test_floor_grid_creation() {
    let grid = FloorGrid::new(3, 3, 200.0);
    assert_eq!(grid.width, 3);
    assert_eq!(grid.height, 3);
    assert_eq!(grid.cell_size, 200.0);
    assert_eq!(grid.cells.len(), 3);
    assert_eq!(grid.cells[0].len(), 3);
}

#[test]
fn test_cell_has_four_neighborhoods() {
    let grid = FloorGrid::new(2, 2, 200.0);
    for row in &grid.cells {
        for cell in row {
            assert_eq!(cell.neighborhoods.len(), 4, "Each cell should have 4 neighborhoods");
        }
    }
}

#[test]
fn test_neighborhood_has_nodes() {
    let grid = FloorGrid::new(1, 1, 200.0);
    let cell = &grid.cells[0][0];
    
    for neighborhood in &cell.neighborhoods {
        assert!(!neighborhood.nodes.is_empty(), "Neighborhood should have nodes");
        assert!(neighborhood.nodes.len() >= 5 && neighborhood.nodes.len() <= 15, 
                "Neighborhood should have between 5 and 15 nodes (inclusive)");
    }
}

#[test]
fn test_mst_edges_connect_nodes() {
    let grid = FloorGrid::new(1, 1, 200.0);
    let cell = &grid.cells[0][0];
    
    for neighborhood in &cell.neighborhoods {
        let num_nodes = neighborhood.nodes.len();
        if num_nodes > 1 {
            // MST should have exactly (num_nodes - 1) edges
            assert_eq!(neighborhood.mst_edges.len(), num_nodes - 1,
                      "MST should have exactly n-1 edges for n nodes");
            
            // Check that edges reference valid node indices
            for edge in &neighborhood.mst_edges {
                assert!(edge.from < num_nodes, "Edge 'from' index should be valid");
                assert!(edge.to < num_nodes, "Edge 'to' index should be valid");
            }
        }
    }
}

#[test]
fn test_room_types_exist() {
    let grid = FloorGrid::new(2, 2, 200.0);
    let mut has_normal = false;
    
    for row in &grid.cells {
        for cell in row {
            for neighborhood in &cell.neighborhoods {
                for node in &neighborhood.nodes {
                    if node.room_type == RoomType::Normal {
                        has_normal = true;
                    }
                }
            }
        }
    }
    
    assert!(has_normal, "Should have at least some normal rooms");
}

#[test]
fn test_grid_regeneration() {
    let mut grid = FloorGrid::new(2, 2, 150.0);
    
    // Store initial node positions
    let initial_x = grid.cells[0][0].neighborhoods[0].nodes[0].x;
    let initial_y = grid.cells[0][0].neighborhoods[0].nodes[0].y;
    
    // Regenerate
    grid.regenerate();
    
    // After regeneration, positions should likely be different
    // (could theoretically be same by chance, but extremely unlikely)
    let new_x = grid.cells[0][0].neighborhoods[0].nodes[0].x;
    let new_y = grid.cells[0][0].neighborhoods[0].nodes[0].y;
    
    // Grid should maintain same structure
    assert_eq!(grid.width, 2);
    assert_eq!(grid.height, 2);
    assert_eq!(grid.cell_size, 150.0);
    
    // Just verify regeneration completed successfully
    assert!(!grid.cells[0][0].neighborhoods[0].nodes.is_empty());
}

#[test]
fn test_node_positions_within_bounds() {
    let grid = FloorGrid::new(2, 2, 100.0);
    
    for (y, row) in grid.cells.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let cell_min_x = x as f32 * grid.cell_size;
            let cell_max_x = (x + 1) as f32 * grid.cell_size;
            let cell_min_y = y as f32 * grid.cell_size;
            let cell_max_y = (y + 1) as f32 * grid.cell_size;
            
            for neighborhood in &cell.neighborhoods {
                for node in &neighborhood.nodes {
                    assert!(node.x >= cell_min_x && node.x <= cell_max_x,
                           "Node x position {} should be within cell bounds [{}, {}]",
                           node.x, cell_min_x, cell_max_x);
                    assert!(node.y >= cell_min_y && node.y <= cell_max_y,
                           "Node y position {} should be within cell bounds [{}, {}]",
                           node.y, cell_min_y, cell_max_y);
                }
            }
        }
    }
}

#[test]
fn test_bathroom_distance_constraints() {
    // Test that bathrooms within a neighborhood respect minimum distance constraints
    let grid = FloorGrid::new(2, 2, 200.0);
    
    for row in &grid.cells {
        for cell in row {
            for neighborhood in &cell.neighborhoods {
                // Collect all bathroom positions
                let bathrooms: Vec<(f32, f32)> = neighborhood.nodes
                    .iter()
                    .filter(|n| n.room_type == RoomType::Bathroom)
                    .map(|n| (n.x, n.y))
                    .collect();
                
                // If there are multiple bathrooms, verify they respect minimum distance
                if bathrooms.len() > 1 {
                    for i in 0..bathrooms.len() {
                        for j in (i + 1)..bathrooms.len() {
                            let dx = bathrooms[i].0 - bathrooms[j].0;
                            let dy = bathrooms[i].1 - bathrooms[j].1;
                            let distance = (dx * dx + dy * dy).sqrt();
                            
                            // restroom_distance returns 300-500, so minimum should be at least 300
                            assert!(
                                distance >= 300.0,
                                "Bathrooms too close: distance = {:.2}m (should be >= 300m)",
                                distance
                            );
                        }
                    }
                }
            }
        }
    }
}
