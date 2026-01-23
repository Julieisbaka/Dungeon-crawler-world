use crate::grid::FloorGrid;

/// State for the grid visualization UI
#[derive(Debug, Clone)]
pub struct GridState {
    pub grid: FloorGrid,
    pub zoom: f32,
    pub pan_x: f32,
    pub pan_y: f32,
}

impl Default for GridState {
    fn default() -> Self {
        Self {
            grid: FloorGrid::new(3, 3, 200.0), // 3x3 grid of cells, each 200 units
            zoom: 1.0,
            pan_x: 0.0,
            pan_y: 0.0,
        }
    }
}

impl GridState {
    /// Regenerate the grid with new random data
    pub fn regenerate(&mut self) {
        self.grid.regenerate();
    }

    /// Reset zoom and pan to default
    pub fn reset_view(&mut self) {
        self.zoom = 1.0;
        self.pan_x = 0.0;
        self.pan_y = 0.0;
    }
}
