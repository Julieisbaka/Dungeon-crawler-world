use rand::Rng;
use serde::{Deserialize, Serialize};

pub const FLOOR_ONE_WIDTH_CELLS: usize = 3;
pub const FLOOR_ONE_HEIGHT_CELLS: usize = 3;
pub const NEIGHBORHOOD_SIZE_METERS: f32 = 1609.0;
pub const CELL_SIZE_METERS: f32 = NEIGHBORHOOD_SIZE_METERS * 2.0;
pub const RESTROOM_MIN_DISTANCE_METERS: f32 = 300.0;
pub const RESTROOM_MAX_DISTANCE_METERS: f32 = 500.0;
pub const ROOM_GRID_SPACING_METERS: f32 = 185.0;
pub const ROOM_HALF_EXTENTS_METERS: f32 = 52.0;
pub const RESTROOM_HALF_EXTENTS_METERS: f32 = 64.0;
pub const ROOM_HEIGHT_METERS: f32 = 85.0;
pub const PATHWAY_HALF_WIDTH_METERS: f32 = 30.0;
pub const NEIGHBORHOOD_CONNECTOR_HALF_WIDTH_METERS: f32 = 48.0;
pub const CORRIDOR_HEIGHT_METERS: f32 = 54.0;
pub const TEMPORARY_CHARACTER_SPEED: f32 = 1.0;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TerrainPoint {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl TerrainPoint {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn ground(x: f32, z: f32) -> Self {
        Self { x, y: 0.0, z }
    }

    pub fn distance_xz(self, other: Self) -> f32 {
        let dx = self.x - other.x;
        let dz = self.z - other.z;
        (dx * dx + dz * dz).sqrt()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TerrainRoomType {
    Room,
    Restroom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TerrainNodeId {
    pub cell_x: usize,
    pub cell_y: usize,
    pub neighborhood: usize,
    pub node: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainRoomNode {
    pub id: TerrainNodeId,
    pub position: TerrainPoint,
    pub room_type: TerrainRoomType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainPathway {
    pub from: TerrainNodeId,
    pub to: TerrainNodeId,
    pub weight: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainNeighborhood {
    pub id: usize,
    pub origin: TerrainPoint,
    pub size_meters: f32,
    pub nodes: Vec<TerrainRoomNode>,
    pub pathways: Vec<TerrainPathway>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainCell {
    pub x: usize,
    pub y: usize,
    pub origin: TerrainPoint,
    pub size_meters: f32,
    pub neighborhoods: Vec<TerrainNeighborhood>,
    pub neighborhood_pathways: Vec<TerrainPathway>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainMesh {
    pub vertices: Vec<TerrainPoint>,
    pub indices: Vec<[usize; 3]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainRoomVolume {
    pub node_id: TerrainNodeId,
    pub room_type: TerrainRoomType,
    pub center: TerrainPoint,
    pub half_extents: TerrainPoint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainCorridorVolume {
    pub from: TerrainNodeId,
    pub to: TerrainNodeId,
    pub start: TerrainPoint,
    pub end: TerrainPoint,
    pub half_width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporaryCharacterModel {
    pub position: TerrainPoint,
    pub speed: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloorOneTerrain {
    pub cells: Vec<Vec<TerrainCell>>,
    pub width_cells: usize,
    pub height_cells: usize,
    pub cell_size_meters: f32,
    pub terrain_mesh: TerrainMesh,
    pub rooms: Vec<TerrainRoomVolume>,
    pub corridors: Vec<TerrainCorridorVolume>,
    pub temporary_character: TemporaryCharacterModel,
}

impl FloorOneTerrain {
    pub fn generate() -> Self {
        Self::new(FLOOR_ONE_WIDTH_CELLS, FLOOR_ONE_HEIGHT_CELLS)
    }

    pub fn new(width_cells: usize, height_cells: usize) -> Self {
        let mut cells = Vec::new();
        for y in 0..height_cells {
            let mut row = Vec::new();
            for x in 0..width_cells {
                row.push(TerrainCell::new(x, y));
            }
            cells.push(row);
        }

        let terrain_mesh = TerrainMesh::for_rect(
            width_cells as f32 * CELL_SIZE_METERS,
            height_cells as f32 * CELL_SIZE_METERS,
        );
        let (rooms, corridors) = build_terrain_volumes(&cells);
        let spawn_position = cells
            .first()
            .and_then(|row| row.first())
            .and_then(|cell| cell.neighborhoods.first())
            .and_then(|neighborhood| neighborhood.nodes.first())
            .map(|node| node.position)
            .unwrap_or_else(|| TerrainPoint::ground(0.0, 0.0));

        Self {
            cells,
            width_cells,
            height_cells,
            cell_size_meters: CELL_SIZE_METERS,
            terrain_mesh,
            rooms,
            corridors,
            temporary_character: TemporaryCharacterModel {
                position: spawn_position,
                speed: TEMPORARY_CHARACTER_SPEED,
            },
        }
    }
}

impl TerrainCell {
    pub fn new(x: usize, y: usize) -> Self {
        let origin = TerrainPoint::ground(x as f32 * CELL_SIZE_METERS, y as f32 * CELL_SIZE_METERS);
        let neighborhoods = vec![
            TerrainNeighborhood::new(0, x, y, origin),
            TerrainNeighborhood::new(
                1,
                x,
                y,
                TerrainPoint::ground(origin.x + NEIGHBORHOOD_SIZE_METERS, origin.z),
            ),
            TerrainNeighborhood::new(
                2,
                x,
                y,
                TerrainPoint::ground(origin.x, origin.z + NEIGHBORHOOD_SIZE_METERS),
            ),
            TerrainNeighborhood::new(
                3,
                x,
                y,
                TerrainPoint::ground(
                    origin.x + NEIGHBORHOOD_SIZE_METERS,
                    origin.z + NEIGHBORHOOD_SIZE_METERS,
                ),
            ),
        ];
        let neighborhood_pathways = Self::connect_neighborhoods(&neighborhoods);

        Self {
            x,
            y,
            origin,
            size_meters: CELL_SIZE_METERS,
            neighborhoods,
            neighborhood_pathways,
        }
    }

    fn connect_neighborhoods(neighborhoods: &[TerrainNeighborhood]) -> Vec<TerrainPathway> {
        [(0, 1), (0, 2), (1, 3), (2, 3)]
            .iter()
            .filter_map(|&(from_neighborhood, to_neighborhood)| {
                let from = neighborhoods.get(from_neighborhood)?;
                let to = neighborhoods.get(to_neighborhood)?;
                closest_pathway_between(from, to)
            })
            .collect()
    }
}

impl TerrainNeighborhood {
    pub fn new(id: usize, cell_x: usize, cell_y: usize, origin: TerrainPoint) -> Self {
        let mut rng = rand::thread_rng();
        let target_restroom_spacing = rng.gen_range(
            RESTROOM_MIN_DISTANCE_METERS..=RESTROOM_MAX_DISTANCE_METERS,
        );
        let rooms_per_side = (NEIGHBORHOOD_SIZE_METERS / ROOM_GRID_SPACING_METERS).ceil() as usize;
        let node_count = rooms_per_side.max(2).pow(2);
        let mut nodes = Vec::with_capacity(node_count);

        for node in 0..node_count {
            let row = node / rooms_per_side;
            let column = node % rooms_per_side;
            let x = origin.x
                + ((column as f32 + rng.gen_range(0.2..0.8)) / rooms_per_side as f32)
                    * NEIGHBORHOOD_SIZE_METERS;
            let z = origin.z
                + ((row as f32 + rng.gen_range(0.2..0.8)) / rooms_per_side as f32)
                    * NEIGHBORHOOD_SIZE_METERS;

            nodes.push(TerrainRoomNode {
                id: TerrainNodeId {
                    cell_x,
                    cell_y,
                    neighborhood: id,
                    node,
                },
                position: TerrainPoint::ground(x, z),
                room_type: TerrainRoomType::Room,
            });
        }

        mark_restrooms(&mut nodes, target_restroom_spacing);
        let pathways = generate_weighted_kruskal_paths(&nodes);

        Self {
            id,
            origin,
            size_meters: NEIGHBORHOOD_SIZE_METERS,
            nodes,
            pathways,
        }
    }
}

impl TerrainMesh {
    pub fn for_rect(width: f32, depth: f32) -> Self {
        Self {
            vertices: vec![
                TerrainPoint::ground(0.0, 0.0),
                TerrainPoint::ground(width, 0.0),
                TerrainPoint::ground(width, depth),
                TerrainPoint::ground(0.0, depth),
            ],
            indices: vec![[0, 1, 2], [0, 2, 3]],
        }
    }
}

fn mark_restrooms(nodes: &mut [TerrainRoomNode], target_spacing: f32) {
    let minimum_distance = target_spacing.clamp(
        RESTROOM_MIN_DISTANCE_METERS,
        RESTROOM_MAX_DISTANCE_METERS,
    );
    let mut restrooms: Vec<TerrainPoint> = Vec::new();

    for node in nodes.iter_mut() {
        let far_enough = restrooms
            .iter()
            .all(|restroom| node.position.distance_xz(*restroom) >= minimum_distance);
        if far_enough {
            node.room_type = TerrainRoomType::Restroom;
            restrooms.push(node.position);
        }
    }
}

fn generate_weighted_kruskal_paths(nodes: &[TerrainRoomNode]) -> Vec<TerrainPathway> {
    if nodes.len() < 2 {
        return Vec::new();
    }

    let mut candidates = Vec::new();
    for from in 0..nodes.len() {
        for to in (from + 1)..nodes.len() {
            candidates.push((
                from,
                to,
                nodes[from].position.distance_xz(nodes[to].position),
            ));
        }
    }
    candidates.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));

    let mut parent: Vec<usize> = (0..nodes.len()).collect();
    let mut pathways = Vec::with_capacity(nodes.len() - 1);

    for (from, to, weight) in candidates {
        if find(&mut parent, from) != find(&mut parent, to) {
            union(&mut parent, from, to);
            pathways.push(TerrainPathway {
                from: nodes[from].id,
                to: nodes[to].id,
                weight,
            });
            if pathways.len() == nodes.len() - 1 {
                break;
            }
        }
    }

    pathways
}

fn closest_pathway_between(
    from: &TerrainNeighborhood,
    to: &TerrainNeighborhood,
) -> Option<TerrainPathway> {
    let mut closest: Option<(&TerrainRoomNode, &TerrainRoomNode, f32)> = None;

    for from_node in &from.nodes {
        for to_node in &to.nodes {
            let distance = from_node.position.distance_xz(to_node.position);
            match closest {
                Some((_, _, current_distance)) if current_distance <= distance => {}
                _ => closest = Some((from_node, to_node, distance)),
            }
        }
    }

    closest.map(|(from_node, to_node, weight)| TerrainPathway {
        from: from_node.id,
        to: to_node.id,
        weight,
    })
}

fn find(parent: &mut [usize], index: usize) -> usize {
    if parent[index] != index {
        parent[index] = find(parent, parent[index]);
    }
    parent[index]
}

fn union(parent: &mut [usize], left: usize, right: usize) {
    let left_root = find(parent, left);
    let right_root = find(parent, right);
    if left_root != right_root {
        parent[left_root] = right_root;
    }
}

fn build_terrain_volumes(
    cells: &[Vec<TerrainCell>],
) -> (Vec<TerrainRoomVolume>, Vec<TerrainCorridorVolume>) {
    let mut rooms = Vec::new();
    let mut corridors = Vec::new();

    for row in cells {
        for cell in row {
            for neighborhood in &cell.neighborhoods {
                for node in &neighborhood.nodes {
                    rooms.push(TerrainRoomVolume::from_node(node));
                }

                for pathway in &neighborhood.pathways {
                    let Some(from) = neighborhood.nodes.iter().find(|node| node.id == pathway.from) else {
                        continue;
                    };
                    let Some(to) = neighborhood.nodes.iter().find(|node| node.id == pathway.to) else {
                        continue;
                    };
                    corridors.push(TerrainCorridorVolume::new(
                        from.id,
                        to.id,
                        from.position,
                        to.position,
                        PATHWAY_HALF_WIDTH_METERS,
                    ));
                }
            }

            for pathway in &cell.neighborhood_pathways {
                let Some(from) = find_node(cells, pathway.from) else {
                    continue;
                };
                let Some(to) = find_node(cells, pathway.to) else {
                    continue;
                };
                corridors.push(TerrainCorridorVolume::new(
                    pathway.from,
                    pathway.to,
                    from,
                    to,
                    NEIGHBORHOOD_CONNECTOR_HALF_WIDTH_METERS,
                ));
            }
        }
    }

    (rooms, corridors)
}

impl TerrainRoomVolume {
    fn from_node(node: &TerrainRoomNode) -> Self {
        let half_extent = match node.room_type {
            TerrainRoomType::Room => ROOM_HALF_EXTENTS_METERS,
            TerrainRoomType::Restroom => RESTROOM_HALF_EXTENTS_METERS,
        };

        Self {
            node_id: node.id,
            room_type: node.room_type,
            center: TerrainPoint::new(node.position.x, ROOM_HEIGHT_METERS * 0.5, node.position.z),
            half_extents: TerrainPoint::new(
                half_extent,
                ROOM_HEIGHT_METERS * 0.5,
                half_extent,
            ),
        }
    }
}

impl TerrainCorridorVolume {
    fn new(
        from: TerrainNodeId,
        to: TerrainNodeId,
        start: TerrainPoint,
        end: TerrainPoint,
        half_width: f32,
    ) -> Self {
        Self {
            from,
            to,
            start,
            end,
            half_width,
            height: CORRIDOR_HEIGHT_METERS,
        }
    }
}

fn find_node(cells: &[Vec<TerrainCell>], id: TerrainNodeId) -> Option<TerrainPoint> {
    cells
        .get(id.cell_y)?
        .get(id.cell_x)?
        .neighborhoods
        .get(id.neighborhood)?
        .nodes
        .iter()
        .find(|node| node.id.node == id.node)
        .map(|node| node.position)
}
