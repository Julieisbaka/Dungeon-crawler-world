use egui::{Color32, Painter, Pos2, Rect, Stroke, Ui, Vec2};
use crate::grid::{RoomType, Cell, Neighborhood};
use crate::logic::grid_logic::GridState;

/// Color scheme for different room types
const COLOR_NORMAL: Color32 = Color32::from_rgb(100, 100, 100);
const COLOR_BATHROOM: Color32 = Color32::from_rgb(100, 150, 255);
const COLOR_SAFE_ROOM: Color32 = Color32::from_rgb(100, 255, 100);
const COLOR_STAIRWELL: Color32 = Color32::from_rgb(255, 200, 50);
const COLOR_CELL_BORDER: Color32 = Color32::from_rgb(50, 50, 50);
const COLOR_NEIGHBORHOOD_BORDER: Color32 = Color32::from_rgb(80, 80, 80);
const COLOR_MST_EDGE: Color32 = Color32::from_rgb(200, 200, 200);

/// Render the grid visualization UI
pub fn grid_ui(ui: &mut Ui, state: &mut GridState) {
    ui.heading("Floor Grid Visualization");
    
    // Control panel
    ui.horizontal(|ui| {
        if ui.button("Regenerate").clicked() {
            state.regenerate();
        }
        if ui.button("Reset View").clicked() {
            state.reset_view();
        }
    });
    
    // Zoom controls
    ui.horizontal(|ui| {
        ui.label("Zoom:");
        if ui.button("-").clicked() {
            state.zoom = (state.zoom * 0.8).max(0.1);
        }
        ui.label(format!("{:.1}x", state.zoom));
        if ui.button("+").clicked() {
            state.zoom = (state.zoom * 1.25).min(5.0);
        }
    });
    
    ui.separator();
    
    // Legend
    ui.horizontal(|ui| {
        ui.label("Legend:");
        
        let size = Vec2::new(12.0, 12.0);
        
        let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
        ui.painter().circle_filled(rect.center(), 5.0, COLOR_NORMAL);
        ui.label("Normal");
        
        let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
        ui.painter().circle_filled(rect.center(), 5.0, COLOR_BATHROOM);
        ui.label("Bathroom");
        
        let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
        ui.painter().circle_filled(rect.center(), 5.0, COLOR_SAFE_ROOM);
        ui.label("Safe Room");
        
        let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
        ui.painter().circle_filled(rect.center(), 5.0, COLOR_STAIRWELL);
        ui.label("Stairwell");
    });
    
    ui.separator();
    
    // Main grid rendering area
    egui::ScrollArea::both()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            let grid = &state.grid;
            let total_width = grid.width as f32 * grid.cell_size * state.zoom;
            let total_height = grid.height as f32 * grid.cell_size * state.zoom;
            
            let (response, painter) = ui.allocate_painter(
                Vec2::new(total_width.max(600.0), total_height.max(400.0)),
                egui::Sense::hover(),
            );
            
            let to_screen = |x: f32, y: f32| -> Pos2 {
                response.rect.min + Vec2::new(
                    (x + state.pan_x) * state.zoom,
                    (y + state.pan_y) * state.zoom,
                )
            };
            
            // Render each cell
            for cell_row in &grid.cells {
                for cell in cell_row {
                    render_cell(&painter, cell, grid.cell_size, state.zoom, &to_screen);
                }
            }
        });
    
    ui.separator();
    ui.label(format!(
        "Grid: {}x{} cells | Cell size: {:.0} units",
        state.grid.width, state.grid.height, state.grid.cell_size
    ));
}

/// Render a single cell with its 4 neighborhoods
fn render_cell(
    painter: &Painter,
    cell: &Cell,
    cell_size: f32,
    zoom: f32,
    to_screen: &dyn Fn(f32, f32) -> Pos2,
) {
    let base_x = cell.x as f32 * cell_size;
    let base_y = cell.y as f32 * cell_size;
    
    // Draw cell border
    let cell_rect = Rect::from_min_max(
        to_screen(base_x, base_y),
        to_screen(base_x + cell_size, base_y + cell_size),
    );
    painter.rect_stroke(
        cell_rect,
        egui::CornerRadius::ZERO,
        Stroke::new(2.0, COLOR_CELL_BORDER),
        egui::epaint::StrokeKind::Outside,
    );
    
    // Draw neighborhood boundaries
    let half = cell_size / 2.0;
    
    // Vertical divider
    painter.line_segment(
        [
            to_screen(base_x + half, base_y),
            to_screen(base_x + half, base_y + cell_size),
        ],
        Stroke::new(1.0, COLOR_NEIGHBORHOOD_BORDER),
    );
    
    // Horizontal divider
    painter.line_segment(
        [
            to_screen(base_x, base_y + half),
            to_screen(base_x + cell_size, base_y + half),
        ],
        Stroke::new(1.0, COLOR_NEIGHBORHOOD_BORDER),
    );
    
    // Render each neighborhood
    for neighborhood in &cell.neighborhoods {
        render_neighborhood(painter, neighborhood, zoom, to_screen);
    }
}

/// Render a single neighborhood with its nodes and MST
fn render_neighborhood(
    painter: &Painter,
    neighborhood: &Neighborhood,
    zoom: f32,
    to_screen: &dyn Fn(f32, f32) -> Pos2,
) {
    // Draw MST edges first (so they appear behind nodes)
    for edge in &neighborhood.mst_edges {
        let from = &neighborhood.nodes[edge.from];
        let to = &neighborhood.nodes[edge.to];
        
        painter.line_segment(
            [
                to_screen(from.x, from.y),
                to_screen(to.x, to.y),
            ],
            Stroke::new((1.5 * zoom).max(0.5), COLOR_MST_EDGE),
        );
    }
    
    // Draw nodes
    for node in &neighborhood.nodes {
        let color = match node.room_type {
            RoomType::Normal => COLOR_NORMAL,
            RoomType::Bathroom => COLOR_BATHROOM,
            RoomType::SafeRoom => COLOR_SAFE_ROOM,
            RoomType::Stairwell => COLOR_STAIRWELL,
        };
        
        let pos = to_screen(node.x, node.y);
        let radius = (4.0 * zoom).max(2.0).min(8.0);
        
        painter.circle_filled(pos, radius, color);
        
        // Draw outline for special rooms
        if node.room_type != RoomType::Normal {
            painter.circle_stroke(
                pos,
                radius + 1.0,
                Stroke::new(1.0, Color32::WHITE),
            );
        }
    }
}
