// === CRITICAL SAFETY SYSTEMS FOR EPILEPSY PROTECTION ===
// Extracted from simple.rs for better modularity

use glam::Vec3;

/// Simple safety configuration
#[derive(Debug, Clone)]
pub struct SafetyConfig {
    pub visual_intensity_limit: f32,  // 0.0 to 1.0
    pub max_flash_rate: f32,          // Hz (3.0 is international standard)
    pub max_luminance_change: f32,    // 0.0 to 1.0 (0.1 is 10% standard)
    pub red_flash_protection: bool,
}

impl Default for SafetyConfig {
    fn default() -> Self {
        Self {
            visual_intensity_limit: 1.0,     // Full intensity by default
            max_flash_rate: 3.0,             // International safety standard
            max_luminance_change: 0.1,       // 10% WCAG standard
            red_flash_protection: true,
        }
    }
}

impl SafetyConfig {
    pub fn safe_mode() -> Self {
        Self {
            visual_intensity_limit: 0.5,     // 50% intensity
            max_flash_rate: 2.0,             // Even more conservative
            max_luminance_change: 0.05,      // 5% change
            red_flash_protection: true,
        }
    }
}

/// Simple flash tracker to enforce rate limiting
#[derive(Debug)]
pub struct FlashTracker {
    last_major_change: f64,
    change_count_in_window: u32,
    window_start_time: f64,
    window_duration: f64,
}

impl FlashTracker {
    pub fn new() -> Self {
        Self {
            last_major_change: 0.0,
            change_count_in_window: 0,
            window_start_time: 0.0,
            window_duration: 1.0, // 1 second window
        }
    }

    pub fn can_allow_flash(&mut self, current_time: f64, max_rate: f32) -> bool {
        // Reset window if needed
        if current_time - self.window_start_time > self.window_duration {
            self.window_start_time = current_time;
            self.change_count_in_window = 0;
        }

        // Check if we're under the rate limit
        let flashes_per_second = self.change_count_in_window as f32 / self.window_duration as f32;
        if flashes_per_second >= max_rate {
            return false;
        }

        // Check minimum interval (1/3 Hz = 0.33 seconds for 3 Hz)
        let min_interval = 1.0 / max_rate as f64;
        if current_time - self.last_major_change < min_interval {
            return false;
        }

        true
    }

    pub fn record_flash(&mut self, current_time: f64) {
        self.last_major_change = current_time;
        self.change_count_in_window += 1;
    }
}

// Safety utility functions

pub fn calculate_luminance(color: &Vec3) -> f32 {
    // Standard luminance calculation (ITU-R BT.709)
    0.2126 * color.x + 0.7152 * color.y + 0.0722 * color.z
}

pub fn limit_luminance_change(new_color: Vec3, old_color: Vec3, max_change: f32) -> Vec3 {
    let old_luminance = calculate_luminance(&old_color);
    let new_luminance = calculate_luminance(&new_color);
    let luminance_change = (new_luminance - old_luminance).abs();

    if luminance_change > max_change {
        // Gradually transition to new color to stay within safe limits
        let blend_factor = max_change / luminance_change;
        old_color.lerp(new_color, blend_factor)
    } else {
        new_color
    }
}

pub fn is_dangerous_red(color: Vec3) -> bool {
    // Check for dangerous red flashes (high red content with low blue/green)
    color.x > 0.8 && color.y < 0.3 && color.z < 0.3
}

pub fn rgb_to_hsv(rgb: Vec3) -> Vec3 {
    let r = rgb.x;
    let g = rgb.y;
    let b = rgb.z;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    let s = if max == 0.0 { 0.0 } else { delta / max };
    let v = max;

    Vec3::new(h, s, v)
}

pub fn hsv_to_rgb_vec3(hsv: Vec3) -> Vec3 {
    let h = hsv.x;
    let s = hsv.y;
    let v = hsv.z;

    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Vec3::new(r + m, g + m, b + m)
}