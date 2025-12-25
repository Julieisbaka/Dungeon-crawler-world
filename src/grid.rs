use rand::Rng;

// Every square consists of 4 neighborhoods. The interior of these neighborhoods will not be a grid.
pub const NEIGHBORHOOD_SIZE: u16 = 1609;

/// Restroom distance is approximately 400m (300-500m)
/// Returns a random value between 300 and 500 (inclusive).
pub fn restroom_distance() -> u16 {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    (&mut rng).gen_range(300..=500)
}

/// Types of special rooms in the dungeon
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoomType {
    Normal,
    Bathroom,
    SafeRoom,
    Stairwell,
}

/// A node in a neighborhood representing a room or intersection
#[derive(Debug, Clone)]
pub struct Node {
    pub id: usize,
    pub x: f32,
    pub y: f32,
    pub room_type: RoomType,
}

/// An edge in the MST connecting two nodes
#[derive(Debug, Clone)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub weight: f32,
}

/// Represents one of 4 neighborhoods within a cell
#[derive(Debug, Clone)]
pub struct Neighborhood {
    pub id: usize,
    pub nodes: Vec<Node>,
    pub mst_edges: Vec<Edge>,
}

impl Neighborhood {
    /// Create a new neighborhood with random nodes and MST
    pub fn new(id: usize, offset_x: f32, offset_y: f32, size: f32) -> Self {
        let mut rng = rand::thread_rng();
        let num_nodes = rng.gen_range(5..=15);
        
        // Generate random nodes within this neighborhood's bounds
        let mut nodes = Vec::new();
        for i in 0..num_nodes {
            let x = offset_x + rng.gen::<f32>() * size;
            let y = offset_y + rng.gen::<f32>() * size;
            
            // Randomly assign special room types - each node has independent probability
            let room_type = if rng.gen_bool(0.3) {
                RoomType::Bathroom
            } else if rng.gen_bool(0.1) {
                RoomType::SafeRoom
            } else if rng.gen_bool(0.05) {
                RoomType::Stairwell
            } else {
                RoomType::Normal
            };
            
            nodes.push(Node {
                id: i,
                x,
                y,
                room_type,
            });
        }
        
        // Generate MST using Kruskal's algorithm
        let mst_edges = Self::generate_mst(&nodes);
        
        Self {
            id,
            nodes,
            mst_edges,
        }
    }
    
    /// Generate a Minimum Spanning Tree using Kruskal's algorithm
    fn generate_mst(nodes: &[Node]) -> Vec<Edge> {
        if nodes.len() < 2 {
            return Vec::new();
        }
        
        // Generate all possible edges with weights (distances)
        let mut edges = Vec::new();
        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                let dx = nodes[i].x - nodes[j].x;
                let dy = nodes[i].y - nodes[j].y;
                let weight = (dx * dx + dy * dy).sqrt();
                edges.push(Edge {
                    from: i,
                    to: j,
                    weight,
                });
            }
        }
        
        // Sort edges by weight
        edges.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap_or(std::cmp::Ordering::Equal));
        
        // Union-Find data structure for cycle detection
        let mut parent: Vec<usize> = (0..nodes.len()).collect();
        
        fn find(parent: &mut Vec<usize>, x: usize) -> usize {
            if parent[x] != x {
                parent[x] = find(parent, parent[x]);
            }
            parent[x]
        }
        
        fn union(parent: &mut Vec<usize>, x: usize, y: usize) {
            let root_x = find(parent, x);
            let root_y = find(parent, y);
            if root_x != root_y {
                parent[root_x] = root_y;
            }
        }
        
        // Kruskal's algorithm: add edges that don't form cycles
        let mut mst = Vec::new();
        for edge in edges {
            if find(&mut parent, edge.from) != find(&mut parent, edge.to) {
                union(&mut parent, edge.from, edge.to);
                mst.push(edge);
                if mst.len() == nodes.len() - 1 {
                    break;
                }
            }
        }
        
        mst
    }
}

/// Represents a grid cell containing 4 neighborhoods
#[derive(Debug, Clone)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub neighborhoods: Vec<Neighborhood>,
}

impl Cell {
    /// Create a new cell with 4 neighborhoods
    pub fn new(x: usize, y: usize, cell_size: f32) -> Self {
        let half_size = cell_size / 2.0;
        let base_x = x as f32 * cell_size;
        let base_y = y as f32 * cell_size;
        
        // Create 4 neighborhoods (top-left, top-right, bottom-left, bottom-right)
        let neighborhoods = vec![
            Neighborhood::new(0, base_x, base_y, half_size),
            Neighborhood::new(1, base_x + half_size, base_y, half_size),
            Neighborhood::new(2, base_x, base_y + half_size, half_size),
            Neighborhood::new(3, base_x + half_size, base_y + half_size, half_size),
        ];
        
        Self {
            x,
            y,
            neighborhoods,
        }
    }
}

/// Represents the dungeon floor grid
#[derive(Debug, Clone)]
pub struct FloorGrid {
    pub cells: Vec<Vec<Cell>>,
    pub width: usize,
    pub height: usize,
    pub cell_size: f32,
}

impl FloorGrid {
    /// Create a new floor grid with the specified dimensions
    pub fn new(width: usize, height: usize, cell_size: f32) -> Self {
        let mut cells = Vec::new();
        for y in 0..height {
            let mut row = Vec::new();
            for x in 0..width {
                row.push(Cell::new(x, y, cell_size));
            }
            cells.push(row);
        }
        
        Self {
            cells,
            width,
            height,
            cell_size,
        }
    }
    
    /// Regenerate the grid with new random data
    pub fn regenerate(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.cells[y][x] = Cell::new(x, y, self.cell_size);
            }
        }
    }
}
