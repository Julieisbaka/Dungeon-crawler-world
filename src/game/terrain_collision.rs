use crate::player::PlayerPosition;
use crate::terrain3d::{
    FloorOneTerrain, TerrainCorridorVolume, TerrainPoint, TerrainRoomVolume,
};
use parry3d::math::{Isometry, Point, Vector};
use parry3d::query::PointQuery;
use parry3d::shape::Cuboid;

pub const PLAYER_RADIUS_METERS: f32 = 1.6;

#[derive(Debug, Clone)]
pub struct TerrainCollisionMap {
    rooms: Vec<RoomBounds>,
    pathways: Vec<PathwayBounds>,
}

impl TerrainCollisionMap {
    pub fn from_terrain(terrain: &FloorOneTerrain) -> Self {
        let rooms = terrain.rooms.iter().map(RoomBounds::from_volume).collect();
        let pathways = terrain
            .corridors
            .iter()
            .map(PathwayBounds::from_volume)
            .collect();

        Self { rooms, pathways }
    }

    pub fn contains_position(&self, position: PlayerPosition) -> bool {
        let point = Point::new(position.x, position.y, position.z);
        self.rooms.iter().any(|room| room.contains_point(&point))
            || self.pathways.iter().any(|pathway| pathway.contains_position(position))
    }

    pub fn constrain_movement(
        &self,
        current: PlayerPosition,
        requested: PlayerPosition,
    ) -> PlayerPosition {
        let dx = requested.x - current.x;
        let dz = requested.z - current.z;
        let distance = (dx * dx + dz * dz).sqrt();
        let steps = (distance / (PLAYER_RADIUS_METERS * 0.5)).ceil().max(1.0) as usize;
        let mut last_valid = current;

        for step in 1..=steps {
            let t = step as f32 / steps as f32;
            let candidate = PlayerPosition::new(
                current.x + dx * t,
                requested.y,
                current.z + dz * t,
            );
            if self.contains_position(candidate) {
                last_valid = candidate;
            } else {
                return last_valid;
            }
        }

        last_valid
    }

    pub fn constrain_movement_axes(
        &self,
        current: PlayerPosition,
        requested: PlayerPosition,
    ) -> PlayerPosition {
        if self.contains_position(requested) {
            return requested;
        }

        let x_only = PlayerPosition::new(requested.x, current.y, current.z);
        if self.contains_position(x_only) {
            return x_only;
        }

        let z_only = PlayerPosition::new(current.x, current.y, requested.z);
        if self.contains_position(z_only) {
            return z_only;
        }

        current
    }
}

#[derive(Debug, Clone)]
struct RoomBounds {
    center: TerrainPoint,
    shape: Cuboid,
}

impl RoomBounds {
    fn from_volume(volume: &TerrainRoomVolume) -> Self {
        Self {
            center: volume.center,
            shape: Cuboid::new(Vector::new(
                volume.half_extents.x + PLAYER_RADIUS_METERS,
                volume.half_extents.y + PLAYER_RADIUS_METERS,
                volume.half_extents.z + PLAYER_RADIUS_METERS,
            )),
        }
    }

    fn contains_point(&self, point: &Point<f32>) -> bool {
        let transform = Isometry::translation(self.center.x, self.center.y, self.center.z);
        self.shape.contains_point(&transform, point)
    }
}

#[derive(Debug, Clone)]
struct PathwayBounds {
    center: TerrainPoint,
    yaw: f32,
    shape: Cuboid,
}

impl PathwayBounds {
    fn from_volume(volume: &TerrainCorridorVolume) -> Self {
        let dx = volume.end.x - volume.start.x;
        let dz = volume.end.z - volume.start.z;
        let length = (dx * dx + dz * dz).sqrt();
        let center = TerrainPoint::new(
            (volume.start.x + volume.end.x) * 0.5,
            volume.start.y + volume.height * 0.5,
            (volume.start.z + volume.end.z) * 0.5,
        );

        Self {
            center,
            yaw: dx.atan2(dz),
            shape: Cuboid::new(Vector::new(
                volume.half_width + PLAYER_RADIUS_METERS,
                volume.height * 0.5 + PLAYER_RADIUS_METERS,
                length * 0.5 + PLAYER_RADIUS_METERS,
            )),
        }
    }

    fn contains_position(&self, position: PlayerPosition) -> bool {
        let point = Point::new(position.x, position.y, position.z);
        let transform = Isometry::new(
            Vector::new(self.center.x, self.center.y, self.center.z),
            Vector::y() * self.yaw,
        );
        self.shape.contains_point(&transform, &point)
    }
}
