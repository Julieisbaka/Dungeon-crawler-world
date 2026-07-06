/// Number of recent frames used when computing the smoothed FPS average.
const FPS_SMOOTHING_FRAMES: usize = 10;

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

    /// Returns a smoothed current FPS value (average of last [`FPS_SMOOTHING_FRAMES`] frames).
    pub fn current_fps(&self) -> f32 {
        if self.times_ms.is_empty() {
            return 0.0;
        }
        let n = self.times_ms.len().min(FPS_SMOOTHING_FRAMES);
        let avg_ms: f32 = self.times_ms.iter().rev().take(n).sum::<f32>() / n as f32;
        if avg_ms > 0.0 {
            1000.0 / avg_ms
        } else {
            0.0
        }
    }
}
