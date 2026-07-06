use super::mesh::{add_cell_geometry, TerrainMesh};
use super::types::{
    CellCoord3d, ChunkCoord, Direction3d, LabyrinthCell3d, Neighborhood3d, Passage3d, Room3d,
    RoomKind, Terrain3dConfig, TerrainChunk3d, NEIGHBORHOODS_PER_CELL,
};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Terrain3dGenerator {
    config: Terrain3dConfig,
    chunks: HashMap<ChunkCoord, TerrainChunk3d>,
}

impl Terrain3dGenerator {
    pub fn new(config: Terrain3dConfig) -> Self {
        Self {
            config: config.validated(),
            chunks: HashMap::new(),
        }
    }

    pub fn config(&self) -> &Terrain3dConfig {
        &self.config
    }

    pub fn generated_chunk_count(&self) -> usize {
        self.chunks.len()
    }

    pub fn chunks(&self) -> impl Iterator<Item = &TerrainChunk3d> {
        self.chunks.values()
    }

    pub fn chunk(&self, coord: ChunkCoord) -> Option<&TerrainChunk3d> {
        self.chunks.get(&coord)
    }

    pub fn chunk_for_world_position(&self, position: [f32; 3]) -> ChunkCoord {
        let chunk_world_size = [
            self.config.chunk_size[0] as f32 * self.config.cell_size[0],
            self.config.chunk_size[1] as f32 * self.config.cell_size[1],
            self.config.chunk_size[2] as f32 * self.config.cell_size[2],
        ];

        ChunkCoord::new(
            (position[0] / chunk_world_size[0]).floor() as i32,
            (position[1] / chunk_world_size[1]).floor() as i32,
            (position[2] / chunk_world_size[2]).floor() as i32,
        )
    }

    pub fn ensure_chunks_around_player(&mut self, player_position: [f32; 3]) -> usize {
        let center = self.chunk_for_world_position(player_position);
        let radius = self.config.generation_radius;
        let mut generated = 0;

        for z in (center.z - radius)..=(center.z + radius) {
            for y in (center.y - radius)..=(center.y + radius) {
                for x in (center.x - radius)..=(center.x + radius) {
                    let coord = ChunkCoord::new(x, y, z);
                    if !self.chunks.contains_key(&coord) {
                        let chunk = self.generate_chunk(coord);
                        self.chunks.insert(coord, chunk);
                        generated += 1;
                    }
                }
            }
        }

        generated
    }

    pub fn cell(&self, coord: CellCoord3d) -> Option<&LabyrinthCell3d> {
        let chunk_coord = self.chunk_for_cell(coord);
        self.chunk(chunk_coord)
            .and_then(|chunk| chunk.cells.iter().find(|cell| cell.coord == coord))
    }

    pub fn mesh_around(&self, player_position: [f32; 3], radius: i32) -> TerrainMesh {
        let center = CellCoord3d::new(
            (player_position[0] / self.config.cell_size[0]).floor() as i32,
            (player_position[1] / self.config.cell_size[1]).floor() as i32,
            (player_position[2] / self.config.cell_size[2]).floor() as i32,
        );
        let mut mesh = TerrainMesh::default();
        let radius = radius.max(1);

        for z in (center.z - 1)..=(center.z + 1) {
            for y in (center.y - radius)..=(center.y + radius) {
                for x in (center.x - radius)..=(center.x + radius) {
                    if let Some(cell) = self.cell(CellCoord3d::new(x, y, z)) {
                        add_cell_geometry(&mut mesh, cell);
                    }
                }
            }
        }

        mesh
    }

    fn generate_chunk(&self, coord: ChunkCoord) -> TerrainChunk3d {
        let [chunk_width, chunk_depth, chunk_levels] = self.config.chunk_size;
        let mut cells = Vec::with_capacity(chunk_width * chunk_depth * chunk_levels);

        for z in 0..chunk_levels {
            for y in 0..chunk_depth {
                for x in 0..chunk_width {
                    let global = CellCoord3d::new(
                        coord.x * chunk_width as i32 + x as i32,
                        coord.y * chunk_depth as i32 + y as i32,
                        coord.z * chunk_levels as i32 + z as i32,
                    );
                    cells.push(self.generate_cell(coord, [x, y, z], global));
                }
            }
        }

        TerrainChunk3d { coord, cells }
    }

    fn generate_cell(
        &self,
        chunk: ChunkCoord,
        local: [usize; 3],
        coord: CellCoord3d,
    ) -> LabyrinthCell3d {
        let origin = [
            coord.x as f32 * self.config.cell_size[0],
            coord.y as f32 * self.config.cell_size[1],
            coord.z as f32 * self.config.cell_size[2],
        ];
        let neighborhoods = self.neighborhoods_for_cell(coord, origin);
        let passages = Direction3d::ALL
            .into_iter()
            .filter(|direction| self.edge_is_open(coord, *direction))
            .map(|direction| Passage3d {
                from: coord,
                to: coord.offset(direction),
                direction,
            })
            .collect();

        LabyrinthCell3d {
            coord,
            chunk,
            local,
            origin,
            size: self.config.cell_size,
            neighborhoods,
            passages,
        }
    }

    fn neighborhoods_for_cell(
        &self,
        coord: CellCoord3d,
        origin: [f32; 3],
    ) -> [Neighborhood3d; NEIGHBORHOODS_PER_CELL] {
        let neighborhood_size = [
            self.config.cell_size[0] / 2.0,
            self.config.cell_size[1] / 2.0,
            self.config.cell_size[2],
        ];

        [
            self.neighborhood(coord, 0, origin, neighborhood_size),
            self.neighborhood(
                coord,
                1,
                [origin[0] + neighborhood_size[0], origin[1], origin[2]],
                neighborhood_size,
            ),
            self.neighborhood(
                coord,
                2,
                [origin[0], origin[1] + neighborhood_size[1], origin[2]],
                neighborhood_size,
            ),
            self.neighborhood(
                coord,
                3,
                [
                    origin[0] + neighborhood_size[0],
                    origin[1] + neighborhood_size[1],
                    origin[2],
                ],
                neighborhood_size,
            ),
        ]
    }

    fn neighborhood(
        &self,
        coord: CellCoord3d,
        id: usize,
        origin: [f32; 3],
        size: [f32; 3],
    ) -> Neighborhood3d {
        let room_seed = self.hash_neighborhood(coord, id);
        let mut rng = StdRng::seed_from_u64(room_seed);
        let room = if rng.gen_bool(self.config.room_chance as f64) {
            Some(Room3d {
                id: room_seed,
                center: [
                    origin[0] + size[0] / 2.0,
                    origin[1] + size[1] / 2.0,
                    origin[2] + size[2] / 2.0,
                ],
                size: [
                    size[0] * rng.gen_range(0.35..=0.75),
                    size[1] * rng.gen_range(0.35..=0.75),
                    size[2] * rng.gen_range(0.55..=0.9),
                ],
                kind: match rng.gen_range(0..5) {
                    0 => RoomKind::Lab,
                    1 => RoomKind::Storage,
                    2 => RoomKind::Maintenance,
                    3 => RoomKind::Security,
                    _ => RoomKind::ElevatorCore,
                },
            })
        } else {
            None
        };

        Neighborhood3d {
            id,
            origin,
            size,
            room,
        }
    }

    fn chunk_for_cell(&self, coord: CellCoord3d) -> ChunkCoord {
        ChunkCoord::new(
            div_floor(coord.x, self.config.chunk_size[0] as i32),
            div_floor(coord.y, self.config.chunk_size[1] as i32),
            div_floor(coord.z, self.config.chunk_size[2] as i32),
        )
    }

    fn edge_is_open(&self, from: CellCoord3d, direction: Direction3d) -> bool {
        let (cell, canonical_direction) = canonical_edge(from, direction);

        match canonical_direction {
            Direction3d::East => {
                cell.y.rem_euclid(4) == 0 || self.hash_edge(cell, canonical_direction) % 100 < 32
            }
            Direction3d::South => {
                cell.x.rem_euclid(3) == 0 || self.hash_edge(cell, canonical_direction) % 100 < 28
            }
            Direction3d::Up => {
                (cell.x.rem_euclid(4) == 0 && cell.y.rem_euclid(4) == 0)
                    || self.hash_edge(cell, canonical_direction) % 100 < 14
            }
            Direction3d::West | Direction3d::North | Direction3d::Down => unreachable!(),
        }
    }

    fn hash_edge(&self, cell: CellCoord3d, direction: Direction3d) -> u64 {
        let direction_value = match direction {
            Direction3d::East => 11,
            Direction3d::South => 17,
            Direction3d::Up => 23,
            _ => 0,
        };
        hash_values(
            self.config.seed,
            cell.x as i64,
            cell.y as i64,
            cell.z as i64,
            direction_value,
        )
    }

    fn hash_neighborhood(&self, coord: CellCoord3d, neighborhood: usize) -> u64 {
        hash_values(
            self.config.seed ^ 0x9e37_79b9_7f4a_7c15,
            coord.x as i64,
            coord.y as i64,
            coord.z as i64,
            neighborhood as i64,
        )
    }
}

fn canonical_edge(from: CellCoord3d, direction: Direction3d) -> (CellCoord3d, Direction3d) {
    match direction {
        Direction3d::East => (from, Direction3d::East),
        Direction3d::West => (from.offset(Direction3d::West), Direction3d::East),
        Direction3d::South => (from, Direction3d::South),
        Direction3d::North => (from.offset(Direction3d::North), Direction3d::South),
        Direction3d::Up => (from, Direction3d::Up),
        Direction3d::Down => (from.offset(Direction3d::Down), Direction3d::Up),
    }
}

fn div_floor(value: i32, divisor: i32) -> i32 {
    let quotient = value / divisor;
    let remainder = value % divisor;
    if remainder != 0 && (remainder < 0) != (divisor < 0) {
        quotient - 1
    } else {
        quotient
    }
}

fn hash_values(seed: u64, x: i64, y: i64, z: i64, extra: i64) -> u64 {
    let mut value = seed ^ 0xa076_1d64_78bd_642f;
    for part in [x, y, z, extra] {
        value ^= part as u64;
        value = value.wrapping_mul(0xe703_7ed1_a0b4_28db);
        value ^= value >> 32;
    }
    value
}
