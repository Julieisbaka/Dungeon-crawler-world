use super::types::{Direction3d, LabyrinthCell3d};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TerrainVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 3],
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TerrainMesh {
    pub vertices: Vec<TerrainVertex>,
    pub indices: Vec<u32>,
}

impl TerrainMesh {
    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty() || self.indices.is_empty()
    }

    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }

    pub fn add_player_marker(&mut self, position: [f32; 3], cell_size: [f32; 3]) {
        let radius = cell_size[0].min(cell_size[1]) * 0.14;
        let height = cell_size[2] * 0.58;
        self.add_box(
            [position[0] - radius, position[1] - radius, position[2]],
            [
                position[0] + radius,
                position[1] + radius,
                position[2] + height,
            ],
            [0.95, 0.78, 0.22],
        );
    }

    pub(crate) fn add_box(&mut self, min: [f32; 3], max: [f32; 3], color: [f32; 3]) {
        let faces = [
            (
                [0.0, 0.0, 1.0],
                [
                    [min[0], min[1], max[2]],
                    [max[0], min[1], max[2]],
                    [max[0], max[1], max[2]],
                    [min[0], max[1], max[2]],
                ],
            ),
            (
                [0.0, 0.0, -1.0],
                [
                    [max[0], min[1], min[2]],
                    [min[0], min[1], min[2]],
                    [min[0], max[1], min[2]],
                    [max[0], max[1], min[2]],
                ],
            ),
            (
                [0.0, -1.0, 0.0],
                [
                    [min[0], min[1], min[2]],
                    [max[0], min[1], min[2]],
                    [max[0], min[1], max[2]],
                    [min[0], min[1], max[2]],
                ],
            ),
            (
                [0.0, 1.0, 0.0],
                [
                    [max[0], max[1], min[2]],
                    [min[0], max[1], min[2]],
                    [min[0], max[1], max[2]],
                    [max[0], max[1], max[2]],
                ],
            ),
            (
                [1.0, 0.0, 0.0],
                [
                    [max[0], min[1], min[2]],
                    [max[0], max[1], min[2]],
                    [max[0], max[1], max[2]],
                    [max[0], min[1], max[2]],
                ],
            ),
            (
                [-1.0, 0.0, 0.0],
                [
                    [min[0], max[1], min[2]],
                    [min[0], min[1], min[2]],
                    [min[0], min[1], max[2]],
                    [min[0], max[1], max[2]],
                ],
            ),
        ];

        for (normal, positions) in faces {
            let start = self.vertices.len() as u32;
            self.vertices
                .extend(positions.map(|position| TerrainVertex {
                    position,
                    normal,
                    color,
                }));
            self.indices.extend_from_slice(&[
                start,
                start + 1,
                start + 2,
                start,
                start + 2,
                start + 3,
            ]);
        }
    }
}

pub(crate) fn add_cell_geometry(mesh: &mut TerrainMesh, cell: &LabyrinthCell3d) {
    let [x, y, z] = cell.origin;
    let [width, depth, height] = cell.size;
    let floor_thickness = height * 0.08;
    let ceiling_thickness = height * 0.08;
    let wall_thickness = width.min(depth) * 0.08;
    let wall_height = height * 0.78;
    let floor_color = [0.22, 0.25, 0.24];
    let ceiling_color = [0.13, 0.15, 0.16];
    let wall_color = [0.28, 0.34, 0.33];
    let room_color = [0.18, 0.42, 0.37];
    let shaft_color = [0.35, 0.48, 0.58];

    mesh.add_box(
        [x, y, z - floor_thickness],
        [x + width, y + depth, z],
        floor_color,
    );
    mesh.add_box(
        [x, y, z + wall_height],
        [x + width, y + depth, z + wall_height + ceiling_thickness],
        ceiling_color,
    );

    if !has_passage(cell, Direction3d::North) {
        mesh.add_box(
            [x, y, z],
            [x + width, y + wall_thickness, z + wall_height],
            wall_color,
        );
    }
    if !has_passage(cell, Direction3d::South) {
        mesh.add_box(
            [x, y + depth - wall_thickness, z],
            [x + width, y + depth, z + wall_height],
            wall_color,
        );
    }
    if !has_passage(cell, Direction3d::West) {
        mesh.add_box(
            [x, y, z],
            [x + wall_thickness, y + depth, z + wall_height],
            wall_color,
        );
    }
    if !has_passage(cell, Direction3d::East) {
        mesh.add_box(
            [x + width - wall_thickness, y, z],
            [x + width, y + depth, z + wall_height],
            wall_color,
        );
    }

    for neighborhood in &cell.neighborhoods {
        if let Some(room) = &neighborhood.room {
            let min = [
                room.center[0] - room.size[0] / 2.0,
                room.center[1] - room.size[1] / 2.0,
                z,
            ];
            let max = [
                room.center[0] + room.size[0] / 2.0,
                room.center[1] + room.size[1] / 2.0,
                z + (height * 0.18).max(0.75),
            ];
            mesh.add_box(min, max, room_color);
        }
    }

    if has_passage(cell, Direction3d::Up) || has_passage(cell, Direction3d::Down) {
        let shaft_width = width.min(depth) * 0.22;
        let cx = x + width / 2.0;
        let cy = y + depth / 2.0;
        mesh.add_box(
            [cx - shaft_width / 2.0, cy - shaft_width / 2.0, z],
            [
                cx + shaft_width / 2.0,
                cy + shaft_width / 2.0,
                z + wall_height,
            ],
            shaft_color,
        );
    }
}

fn has_passage(cell: &LabyrinthCell3d, direction: Direction3d) -> bool {
    cell.passages
        .iter()
        .any(|passage| passage.direction == direction)
}
