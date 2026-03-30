use egui::{Align2, Color32, FontId, Pos2, Stroke, Ui, Vec2};

pub struct FpsGraph {
    times_ms: std::collections::VecDeque<f32>,
    capacity: usize,
}

impl Default for FpsGraph {
    fn default() -> Self {
        Self {
            times_ms: std::collections::VecDeque::with_capacity(240),
            capacity: 240,
        }
    }
}

impl FpsGraph {
    pub fn push_frame_time(&mut self, dt_ms: f32) {
        if self.times_ms.len() == self.capacity {
            self.times_ms.pop_front();
        }
        self.times_ms.push_back(dt_ms);
    }

    pub fn ui(&self, ui: &mut Ui) {
        let desired: Vec2 = Vec2::new(260.0, 80.0);
        let (rect, _resp) = ui.allocate_at_least(desired, egui::Sense::hover());
        let painter: egui::Painter = ui.painter_at(rect);
        painter.rect_stroke(
            rect,
            egui::CornerRadius::same(4),
            Stroke {
                width: 1.0,
                color: Color32::from_gray(120),
            },
            egui::StrokeKind::Outside,
        );

        if self.times_ms.is_empty() {
            return;
        }

        let max_ms: f32 = 50.0; // clamp to 20 FPS floor for scale
        let w: f32 = rect.width().max(1.0);
        let h: f32 = rect.height().max(1.0);
        // Compute FPS stats
        let mut sum_fps: f32 = 0.0;
        let mut min_fps: f32 = f32::INFINITY;
        let mut max_fps: f32 = 0.0;
        for (i, ms) in self.times_ms.iter().enumerate() {
            let x: f32 = rect.left() + (i as f32 / (self.capacity.max(1) as f32)) * w;
            let y: f32 = rect.bottom() - ((*ms).min(max_ms) / max_ms) * h;
            let p1: Pos2 = Pos2::new(x, rect.bottom());
            let p2: Pos2 = Pos2::new(x, y);
            painter.line_segment(
                [p1, p2],
                Stroke {
                    width: 1.0,
                    color: Color32::from_rgb(50, 200, 50),
                },
            );
            let fps: f32 = if *ms > 0.0 { 1000.0 / *ms } else { 0.0 };
            sum_fps += fps;
            if fps < min_fps {
                min_fps = fps;
            }
            if fps > max_fps {
                max_fps = fps;
            }
        }
        // Draw a 16.7ms (60 FPS) line
        let sixty_y: f32 = rect.bottom() - (16.7 / max_ms) * h;
        painter.hline(
            rect.x_range(),
            sixty_y,
            Stroke {
                width: 1.0,
                color: Color32::from_rgb(200, 200, 50),
            },
        );
        // Label the 60 FPS line
        painter.text(
            Pos2::new(rect.left() + 4.0, sixty_y - 2.0),
            Align2::LEFT_BOTTOM,
            "60 FPS",
            FontId::monospace(12.0),
            Color32::from_rgb(200, 200, 50),
        );

        // Stats labels (Avg / Min / Max)
        let n: f32 = self.times_ms.len().max(1) as f32;
        let avg_fps: f32 = (sum_fps / n).clamp(0.0, 9999.0);
        let label: String = format!(
            "FPS  avg:{:.1}  min:{:.1}  max:{:.1}",
            avg_fps, min_fps, max_fps
        );
        painter.text(
            Pos2::new(rect.left() + 6.0, rect.top() + 6.0),
            Align2::LEFT_TOP,
            label,
            FontId::monospace(12.0),
            Color32::WHITE,
        );
    }
}
