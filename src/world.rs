use crate::player::Player;
use crate::terrain::{
    Camera3d, CellCoord3d, Direction3d, Terrain3dConfig, Terrain3dGenerator, TerrainMesh,
};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

pub const PLAYER_MOVEMENT_SPEED_UNITS_PER_SECOND: f32 = 18.0;
pub const PLAYER_JUMP_VELOCITY_UNITS_PER_SECOND: f32 = 10.0;
pub const PLAYER_GRAVITY_UNITS_PER_SECOND_SQUARED: f32 = 28.0;
pub const CAMERA_YAW_RADIANS_PER_POINTER_POINT: f32 = 0.008;
pub const CAMERA_PITCH_RADIANS_PER_POINTER_POINT: f32 = 0.006;
pub const PLAYER_COLLISION_RADIUS_FRACTION_OF_CELL: f32 = 0.22;

#[derive(Debug)]
pub struct WorldSession {
    pub save_name: String,
    pub save_root: PathBuf,
    pub player: Player,
    pub player_position: [f32; 3],
    pub player_vertical_velocity: f32,
    pub camera_yaw_radians: f32,
    pub camera_pitch_radians: f32,
    pub terrain: Terrain3dGenerator,
    pub paused: bool,
}

impl WorldSession {
    pub fn load(save_name: impl Into<String>) -> Result<Self, String> {
        Self::load_from_root(Path::new("saves"), save_name)
    }

    pub fn load_from_root(saves_root: &Path, save_name: impl Into<String>) -> Result<Self, String> {
        let save_name = save_name.into();
        let save_root = saves_root.join(&save_name);
        let player = load_player(&save_root)?;
        let terrain_seed = load_terrain_seed(&save_root).unwrap_or_else(|| stable_seed(&save_name));
        let mut terrain = Terrain3dGenerator::new(Terrain3dConfig {
            chunk_size: [8, 8, 3],
            cell_size: [30.0, 30.0, 10.0],
            room_chance: 0.34,
            generation_radius: 1,
            seed: terrain_seed,
        });
        let player_position = [15.0, 15.0, 0.0];
        terrain.ensure_chunks_around_player(player_position);

        Ok(Self {
            save_name,
            save_root,
            player,
            player_position,
            player_vertical_velocity: 0.0,
            camera_yaw_radians: 0.0,
            camera_pitch_radians: 0.0,
            terrain,
            paused: false,
        })
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    pub fn resume(&mut self) {
        self.paused = false;
    }

    pub fn move_player(&mut self, direction: Direction3d) {
        let movement = match direction {
            Direction3d::North => [0.0, -1.0],
            Direction3d::South => [0.0, 1.0],
            Direction3d::East => [1.0, 0.0],
            Direction3d::West => [-1.0, 0.0],
            Direction3d::Up | Direction3d::Down => return,
        };
        self.move_player_planar(movement, 1.0);
    }

    pub fn move_player_planar(&mut self, movement: [f32; 2], dt_seconds: f32) {
        let length = (movement[0] * movement[0] + movement[1] * movement[1]).sqrt();
        if length <= f32::EPSILON || dt_seconds <= 0.0 {
            return;
        }

        // TODO: derive this from player stats once movement-affecting stats are finalized.
        let distance = PLAYER_MOVEMENT_SPEED_UNITS_PER_SECOND * dt_seconds;
        let step = [
            movement[0] / length * distance,
            movement[1] / length * distance,
        ];

        self.try_move_axis(0, step[0]);
        self.try_move_axis(1, step[1]);
        self.terrain
            .ensure_chunks_around_player(self.player_position);
    }

    pub fn move_player_relative(&mut self, movement: [f32; 2], dt_seconds: f32) {
        let forward = [self.camera_yaw_radians.sin(), self.camera_yaw_radians.cos()];
        let right = [forward[1], -forward[0]];
        let world_movement = [
            right[0] * movement[0] + forward[0] * movement[1],
            right[1] * movement[0] + forward[1] * movement[1],
        ];
        self.move_player_planar(world_movement, dt_seconds);
    }

    pub fn turn_camera(&mut self, pointer_delta_x: f32, pointer_delta_y: f32) {
        self.camera_yaw_radians += pointer_delta_x * CAMERA_YAW_RADIANS_PER_POINTER_POINT;
        self.camera_pitch_radians = (self.camera_pitch_radians
            - pointer_delta_y * CAMERA_PITCH_RADIANS_PER_POINTER_POINT)
            .clamp(-1.2, 1.2);
    }

    pub fn jump(&mut self) {
        if self.is_on_ground() {
            self.player_vertical_velocity = PLAYER_JUMP_VELOCITY_UNITS_PER_SECOND;
        }
    }

    pub fn update_physics(&mut self, dt_seconds: f32) {
        if dt_seconds <= 0.0 {
            return;
        }

        self.player_vertical_velocity -= PLAYER_GRAVITY_UNITS_PER_SECOND_SQUARED * dt_seconds;
        self.player_position[2] += self.player_vertical_velocity * dt_seconds;

        let ground_z = self.ground_z();
        if self.player_position[2] <= ground_z {
            self.player_position[2] = ground_z;
            self.player_vertical_velocity = 0.0;
        }
    }

    pub fn can_move(&self, direction: Direction3d) -> bool {
        self.terrain
            .cell(self.current_cell())
            .map(|cell| {
                cell.passages
                    .iter()
                    .any(|passage| passage.direction == direction)
            })
            .unwrap_or(false)
    }

    pub fn current_cell(&self) -> CellCoord3d {
        CellCoord3d::new(
            (self.player_position[0] / self.terrain.config().cell_size[0]).floor() as i32,
            (self.player_position[1] / self.terrain.config().cell_size[1]).floor() as i32,
            (self.ground_z() / self.terrain.config().cell_size[2]).floor() as i32,
        )
    }

    pub fn visible_terrain_mesh(&self) -> TerrainMesh {
        let mut mesh = self.terrain.mesh_around(self.player_position, 8);
        mesh.add_player_marker(self.player_position, self.terrain.config().cell_size);
        mesh
    }

    pub fn camera(&self) -> Camera3d {
        Camera3d::follow_player(
            self.player_position,
            self.terrain.config().cell_size,
            self.camera_yaw_radians,
            self.camera_pitch_radians,
        )
    }

    fn try_move_axis(&mut self, axis: usize, amount: f32) {
        if amount.abs() <= f32::EPSILON {
            return;
        }

        let mut next_position = self.player_position;
        next_position[axis] += amount;
        if self.can_occupy_position(next_position) {
            self.player_position = next_position;
        }
    }

    fn can_occupy_position(&self, position: [f32; 3]) -> bool {
        let radius = self.player_collision_radius();
        let sample_offsets = [
            [0.0, 0.0],
            [radius, 0.0],
            [-radius, 0.0],
            [0.0, radius],
            [0.0, -radius],
        ];
        sample_offsets.into_iter().all(|offset| {
            self.can_occupy_point([
                position[0] + offset[0],
                position[1] + offset[1],
                position[2],
            ])
        })
    }

    fn can_occupy_point(&self, position: [f32; 3]) -> bool {
        let current = self.current_cell();
        let target = CellCoord3d::new(
            (position[0] / self.terrain.config().cell_size[0]).floor() as i32,
            (position[1] / self.terrain.config().cell_size[1]).floor() as i32,
            current.z,
        );

        if target == current {
            return true;
        }

        let dx = target.x - current.x;
        let dy = target.y - current.y;
        if dx.abs() + dy.abs() != 1 {
            return false;
        }

        let direction = match (dx, dy) {
            (1, 0) => Direction3d::East,
            (-1, 0) => Direction3d::West,
            (0, 1) => Direction3d::South,
            (0, -1) => Direction3d::North,
            _ => return false,
        };
        self.can_move(direction)
    }

    fn player_collision_radius(&self) -> f32 {
        self.terrain.config().cell_size[0].min(self.terrain.config().cell_size[1])
            * PLAYER_COLLISION_RADIUS_FRACTION_OF_CELL
    }

    fn ground_z(&self) -> f32 {
        0.0
    }

    fn is_on_ground(&self) -> bool {
        (self.player_position[2] - self.ground_z()).abs() <= 0.01
            && self.player_vertical_velocity.abs() <= 0.01
    }
}

fn load_player(save_root: &Path) -> Result<Player, String> {
    let player_path = save_root.join("player.json");
    let content = fs::read_to_string(&player_path)
        .map_err(|error| format!("Failed to read player data: {error}"))?;
    serde_json::from_str::<Player>(&content)
        .map_err(|error| format!("Failed to parse player data: {error}"))
}

fn load_terrain_seed(save_root: &Path) -> Option<u64> {
    let content = fs::read_to_string(save_root.join("save.json")).ok()?;
    let value = serde_json::from_str::<Value>(&content).ok()?;
    value.get("terrain_seed")?.as_u64()
}

fn stable_seed(value: &str) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in value.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}
