// Color consciousness system for psychedelic visual effects

pub struct ColorConsciousness {
    base_palette: Vec<glam::Vec3>,
    consciousness_shift: f32,
    reality_tint: glam::Vec3,
}

impl ColorConsciousness {
    pub fn new() -> Self {
        // Define base psychedelic color palette
        let base_palette = vec![
            glam::Vec3::new(1.0, 0.0, 1.0),    // Magenta
            glam::Vec3::new(0.0, 1.0, 1.0),    // Cyan
            glam::Vec3::new(1.0, 1.0, 0.0),    // Yellow
            glam::Vec3::new(1.0, 0.5, 0.0),    // Orange
            glam::Vec3::new(0.5, 0.0, 1.0),    // Purple
            glam::Vec3::new(0.0, 1.0, 0.5),    // Green-cyan
            glam::Vec3::new(1.0, 0.0, 0.5),    // Pink
        ];

        Self {
            base_palette,
            consciousness_shift: 0.0,
            reality_tint: glam::Vec3::ONE,
        }
    }

    pub fn update(&mut self, consciousness_level: f32, reality_distortion: f32, cosmic_time: f64) {
        // Update consciousness shift for color cycling
        self.consciousness_shift = (cosmic_time as f32 * 0.1 + consciousness_level) % 1.0;

        // Calculate reality tint based on distortion
        let distortion_hue = reality_distortion * 360.0;
        self.reality_tint = crate::reality::hsv_to_rgb(distortion_hue, 0.3, 1.0);
    }

    pub fn get_consciousness_color(&self, base_hue: f32, intensity: f32) -> glam::Vec3 {
        // Shift hue based on consciousness level
        let shifted_hue = (base_hue + self.consciousness_shift * 360.0) % 360.0;

        // Calculate color with psychedelic saturation and brightness
        let saturation = 0.8 + intensity * 0.2;
        let brightness = 0.7 + intensity * 0.3;

        crate::reality::hsv_to_rgb(shifted_hue, saturation, brightness) * self.reality_tint
    }

    pub fn get_palette_color(&self, index: usize, mix_factor: f32) -> glam::Vec3 {
        if self.base_palette.is_empty() {
            return glam::Vec3::ONE;
        }

        let base_color = self.base_palette[index % self.base_palette.len()];
        let next_color = self.base_palette[(index + 1) % self.base_palette.len()];

        // Mix between colors for smooth transitions
        base_color.lerp(next_color, mix_factor) * self.reality_tint
    }

    pub fn get_screen_tint(&self, consciousness_level: f32, beat_intensity: f32) -> glam::Vec3 {
        // Calculate screen-wide color tint
        let base_tint = glam::Vec3::new(1.0, 1.0, 1.0);

        // Consciousness creates color bias
        let consciousness_bias = glam::Vec3::new(
            1.0 + consciousness_level * 0.1,
            1.0,
            1.0 + consciousness_level * 0.05,
        );

        // Beat intensity adds pulsing
        let beat_pulse = 1.0 + beat_intensity * 0.1;

        base_tint * consciousness_bias * beat_pulse * self.reality_tint
    }

    pub fn apply_chaos_transformation(&self, color: glam::Vec3, chaos_level: f32) -> glam::Vec3 {
        if chaos_level < 0.1 {
            return color;
        }

        // Apply chaotic color transformations
        let chaos_hue_shift = (chaos_level * 180.0) % 360.0;

        // Convert to HSV, apply chaos, convert back
        let rgb_to_hsv = |rgb: glam::Vec3| -> glam::Vec3 {
            let max = rgb.x.max(rgb.y.max(rgb.z));
            let min = rgb.x.min(rgb.y.min(rgb.z));
            let delta = max - min;

            let hue = if delta == 0.0 {
                0.0
            } else if max == rgb.x {
                60.0 * (((rgb.y - rgb.z) / delta) % 6.0)
            } else if max == rgb.y {
                60.0 * ((rgb.z - rgb.x) / delta + 2.0)
            } else {
                60.0 * ((rgb.x - rgb.y) / delta + 4.0)
            };

            let saturation = if max == 0.0 { 0.0 } else { delta / max };
            let value = max;

            glam::Vec3::new(hue, saturation, value)
        };

        let mut hsv = rgb_to_hsv(color);
        hsv.x = (hsv.x + chaos_hue_shift) % 360.0;
        hsv.y = (hsv.y + chaos_level * 0.3).min(1.0);

        crate::reality::hsv_to_rgb(hsv.x, hsv.y, hsv.z)
    }
}