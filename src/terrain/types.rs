pub const NEIGHBORHOODS_PER_CELL: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkCoord {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl ChunkCoord {
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CellCoord3d {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl CellCoord3d {
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn offset(self, direction: Direction3d) -> Self {
        let [dx, dy, dz] = direction.offset();
        Self {
            x: self.x + dx,
            y: self.y + dy,
            z: self.z + dz,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction3d {
    North,
    South,
    East,
    West,
    Up,
    Down,
}

impl Direction3d {
    pub const ALL: [Self; 6] = [
        Self::North,
        Self::South,
        Self::East,
        Self::West,
        Self::Up,
        Self::Down,
    ];

    pub const fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }

    pub(crate) const fn offset(self) -> [i32; 3] {
        match self {
            Self::North => [0, -1, 0],
            Self::South => [0, 1, 0],
            Self::East => [1, 0, 0],
            Self::West => [-1, 0, 0],
            Self::Up => [0, 0, 1],
            Self::Down => [0, 0, -1],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoomKind {
    Lab,
    Storage,
    Maintenance,
    Security,
    ElevatorCore,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Room3d {
    pub id: u64,
    pub center: [f32; 3],
    pub size: [f32; 3],
    pub kind: RoomKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Passage3d {
    pub from: CellCoord3d,
    pub to: CellCoord3d,
    pub direction: Direction3d,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Neighborhood3d {
    pub id: usize,
    pub origin: [f32; 3],
    pub size: [f32; 3],
    pub room: Option<Room3d>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LabyrinthCell3d {
    pub coord: CellCoord3d,
    pub chunk: ChunkCoord,
    pub local: [usize; 3],
    pub origin: [f32; 3],
    pub size: [f32; 3],
    pub neighborhoods: [Neighborhood3d; NEIGHBORHOODS_PER_CELL],
    pub passages: Vec<Passage3d>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TerrainChunk3d {
    pub coord: ChunkCoord,
    pub cells: Vec<LabyrinthCell3d>,
}

#[derive(Debug, Clone)]
pub struct Terrain3dConfig {
    pub chunk_size: [usize; 3],
    pub cell_size: [f32; 3],
    pub room_chance: f32,
    pub generation_radius: i32,
    pub seed: u64,
}

impl Default for Terrain3dConfig {
    fn default() -> Self {
        Self {
            chunk_size: [6, 6, 3],
            cell_size: [32.0, 32.0, 8.0],
            room_chance: 0.22,
            generation_radius: 1,
            seed: 1,
        }
    }
}

impl Terrain3dConfig {
    pub fn validated(mut self) -> Self {
        self.chunk_size = [
            self.chunk_size[0].max(1),
            self.chunk_size[1].max(1),
            self.chunk_size[2].max(1),
        ];
        self.cell_size = [
            self.cell_size[0].max(1.0),
            self.cell_size[1].max(1.0),
            self.cell_size[2].max(1.0),
        ];
        self.room_chance = self.room_chance.clamp(0.0, 1.0);
        self.generation_radius = self.generation_radius.max(0);
        self
    }
}
