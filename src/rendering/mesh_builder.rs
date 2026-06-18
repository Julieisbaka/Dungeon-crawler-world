use super::vertex::TerrainVertex;
use dungeon_crawler_world::save_game::SaveGame;
use dungeon_crawler_world::terrain3d::{
    TerrainCorridorVolume, TerrainRoomType, TerrainRoomVolume,
};
use glam::Vec3;

const DOORWAY_HALF_WIDTH: f32 = 34.0;

pub struct TerrainMeshBuilder<'a> {
    save: &'a SaveGame,
}

impl<'a> TerrainMeshBuilder<'a> {
    pub fn new(save: &'a SaveGame) -> Self {
        Self { save }
    }

    pub fn build(&self) -> Vec<TerrainVertex> {
        let mut vertices = Vec::new();

        for room in &self.save.world.rooms {
            let connected_corridors: Vec<&TerrainCorridorVolume> = self
                .save
                .world
                .corridors
                .iter()
                .filter(|corridor| corridor.from == room.node_id || corridor.to == room.node_id)
                .collect();
            add_room_volume(&mut vertices, room, &connected_corridors);
        }

        for corridor in &self.save.world.corridors {
            add_corridor_volume(&mut vertices, corridor);
        }

        vertices
    }
}

fn add_room_volume(
    vertices: &mut Vec<TerrainVertex>,
    room: &TerrainRoomVolume,
    connected_corridors: &[&TerrainCorridorVolume],
) {
    let color = match room.room_type {
        TerrainRoomType::Room => [0.36, 0.39, 0.37, 1.0],
        TerrainRoomType::Restroom => [0.12, 0.36, 0.58, 1.0],
    };
    let floor_color = shade(color, 1.12);
    let wall_color = shade(color, 0.72);
    let ceiling_color = shade(color, 0.46);

    let min_x = room.center.x - room.half_extents.x;
    let max_x = room.center.x + room.half_extents.x;
    let min_z = room.center.z - room.half_extents.z;
    let max_z = room.center.z + room.half_extents.z;
    let floor_y = room.center.y - room.half_extents.y;
    let top_y = room.center.y + room.half_extents.y;

    add_horizontal_quad(vertices, min_x, min_z, max_x, max_z, floor_y, floor_color);
    add_horizontal_quad(vertices, min_x, min_z, max_x, max_z, top_y, ceiling_color);
    let doorways = room_doorways(room, connected_corridors);
    add_wall_with_openings(
        vertices,
        WallSide::North,
        min_x,
        max_x,
        min_z,
        floor_y,
        top_y,
        wall_color,
        &doorways,
    );
    add_wall_with_openings(
        vertices,
        WallSide::East,
        min_z,
        max_z,
        max_x,
        floor_y,
        top_y,
        shade(wall_color, 0.9),
        &doorways,
    );
    add_wall_with_openings(
        vertices,
        WallSide::South,
        min_x,
        max_x,
        max_z,
        floor_y,
        top_y,
        shade(wall_color, 0.82),
        &doorways,
    );
    add_wall_with_openings(
        vertices,
        WallSide::West,
        min_z,
        max_z,
        min_x,
        floor_y,
        top_y,
        shade(wall_color, 0.78),
        &doorways,
    );
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WallSide {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, Debug)]
struct Doorway {
    side: WallSide,
    center: f32,
    half_width: f32,
}

fn room_doorways(
    room: &TerrainRoomVolume,
    connected_corridors: &[&TerrainCorridorVolume],
) -> Vec<Doorway> {
    connected_corridors
        .iter()
        .map(|corridor| {
            let other = if corridor.from == room.node_id {
                corridor.end
            } else {
                corridor.start
            };
            let dx = other.x - room.center.x;
            let dz = other.z - room.center.z;
            let half_width = corridor.half_width.max(DOORWAY_HALF_WIDTH);
            if dx.abs() > dz.abs() {
                Doorway {
                    side: if dx > 0.0 { WallSide::East } else { WallSide::West },
                    center: other.z.clamp(
                        room.center.z - room.half_extents.z + half_width,
                        room.center.z + room.half_extents.z - half_width,
                    ),
                    half_width,
                }
            } else {
                Doorway {
                    side: if dz > 0.0 { WallSide::South } else { WallSide::North },
                    center: other.x.clamp(
                        room.center.x - room.half_extents.x + half_width,
                        room.center.x + room.half_extents.x - half_width,
                    ),
                    half_width,
                }
            }
        })
        .collect()
}

fn add_wall_with_openings(
    vertices: &mut Vec<TerrainVertex>,
    side: WallSide,
    span_min: f32,
    span_max: f32,
    fixed_axis: f32,
    floor_y: f32,
    top_y: f32,
    color: [f32; 4],
    doorways: &[Doorway],
) {
    let mut openings: Vec<(f32, f32)> = doorways
        .iter()
        .filter(|doorway| doorway.side == side)
        .map(|doorway| {
            (
                (doorway.center - doorway.half_width).clamp(span_min, span_max),
                (doorway.center + doorway.half_width).clamp(span_min, span_max),
            )
        })
        .filter(|(start, end)| end > start)
        .collect();
    openings.sort_by(|left, right| left.0.partial_cmp(&right.0).unwrap_or(std::cmp::Ordering::Equal));

    let mut cursor = span_min;
    for (opening_start, opening_end) in openings {
        if opening_start > cursor {
            add_wall_segment(vertices, side, cursor, opening_start, fixed_axis, floor_y, top_y, color);
        }
        cursor = cursor.max(opening_end);
    }
    if cursor < span_max {
        add_wall_segment(vertices, side, cursor, span_max, fixed_axis, floor_y, top_y, color);
    }
}

fn add_wall_segment(
    vertices: &mut Vec<TerrainVertex>,
    side: WallSide,
    start: f32,
    end: f32,
    fixed_axis: f32,
    floor_y: f32,
    top_y: f32,
    color: [f32; 4],
) {
    if end <= start {
        return;
    }

    let (from, to) = match side {
        WallSide::North | WallSide::South => (
            Vec3::new(start, floor_y, fixed_axis),
            Vec3::new(end, floor_y, fixed_axis),
        ),
        WallSide::East | WallSide::West => (
            Vec3::new(fixed_axis, floor_y, start),
            Vec3::new(fixed_axis, floor_y, end),
        ),
    };
    add_wall(vertices, from, to, top_y, color);
}

fn add_corridor_volume(vertices: &mut Vec<TerrainVertex>, corridor: &TerrainCorridorVolume) {
    let direction = Vec3::new(
        corridor.end.x - corridor.start.x,
        0.0,
        corridor.end.z - corridor.start.z,
    );
    if direction.length_squared() <= f32::EPSILON {
        return;
    }

    let normal = Vec3::new(-direction.z, 0.0, direction.x).normalize() * corridor.half_width;
    let floor_y = corridor.start.y + 0.035;
    let top_y = floor_y + corridor.height;
    let start = Vec3::new(corridor.start.x, floor_y, corridor.start.z);
    let end = Vec3::new(corridor.end.x, floor_y, corridor.end.z);
    let a = start + normal;
    let b = start - normal;
    let c = end - normal;
    let d = end + normal;

    let floor_color = [0.30, 0.31, 0.30, 1.0];
    let wall_color = [0.18, 0.20, 0.19, 1.0];
    let ceiling_color = [0.11, 0.12, 0.12, 1.0];
    add_face(vertices, a, d, c, b, floor_color);
    add_face(
        vertices,
        Vec3::new(a.x, top_y, a.z),
        Vec3::new(b.x, top_y, b.z),
        Vec3::new(c.x, top_y, c.z),
        Vec3::new(d.x, top_y, d.z),
        ceiling_color,
    );
    add_wall(vertices, a, d, top_y, wall_color);
    add_wall(vertices, c, b, top_y, shade(wall_color, 0.85));
}

fn add_horizontal_quad(
    vertices: &mut Vec<TerrainVertex>,
    min_x: f32,
    min_z: f32,
    max_x: f32,
    max_z: f32,
    y: f32,
    color: [f32; 4],
) {
    let a = Vec3::new(min_x, y, min_z);
    let b = Vec3::new(max_x, y, min_z);
    let c = Vec3::new(max_x, y, max_z);
    let d = Vec3::new(min_x, y, max_z);
    add_face(vertices, a, d, c, b, color);
}

fn add_wall(
    vertices: &mut Vec<TerrainVertex>,
    start: Vec3,
    end: Vec3,
    top_y: f32,
    color: [f32; 4],
) {
    let a = start;
    let b = end;
    let c = Vec3::new(end.x, top_y, end.z);
    let d = Vec3::new(start.x, top_y, start.z);
    add_face(vertices, a, b, c, d, color);
}

fn add_face(vertices: &mut Vec<TerrainVertex>, a: Vec3, b: Vec3, c: Vec3, d: Vec3, color: [f32; 4]) {
    add_triangle(vertices, a, b, c, color);
    add_triangle(vertices, a, c, d, color);
}

fn add_triangle(vertices: &mut Vec<TerrainVertex>, a: Vec3, b: Vec3, c: Vec3, color: [f32; 4]) {
    vertices.push(TerrainVertex::new(a, color));
    vertices.push(TerrainVertex::new(b, color));
    vertices.push(TerrainVertex::new(c, color));
}

fn shade(mut color: [f32; 4], factor: f32) -> [f32; 4] {
    color[0] *= factor;
    color[1] *= factor;
    color[2] *= factor;
    color
}
