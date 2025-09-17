use instant::Instant;

pub struct TimeManager {
    start_time: Instant,
    last_frame: Instant,
    delta_time: f32,
    time_scale: f32,
}

impl TimeManager {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            last_frame: now,
            delta_time: 0.0,
            time_scale: 1.0,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta_time = now.duration_since(self.last_frame).as_secs_f32() * self.time_scale;
        self.last_frame = now;
    }

    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }

    pub fn total_time(&self) -> f32 {
        self.last_frame.duration_since(self.start_time).as_secs_f32()
    }

    pub fn set_time_scale(&mut self, scale: f32) {
        self.time_scale = scale.max(0.0);
    }

    pub fn time_scale(&self) -> f32 {
        self.time_scale
    }
}

// Cosmic time for mathematical calculations
pub fn cosmic_time() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}