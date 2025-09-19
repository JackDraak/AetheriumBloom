// Photosensitive Epilepsy Safety Engine
// Implements Phase 1: Immediate Safety Fixes from EPILEPSY.md
// Critical safety measures for AetheriumBloom's psychedelic visual effects

use std::collections::VecDeque;
use glam::Vec3;

/// Core safety configuration constants based on international standards
pub mod safety_constants {
    /// Maximum flashes per second (universal safety standard)
    pub const MAX_FLASH_RATE_HZ: f32 = 3.0;
    /// Minimum time between major visual changes (1/3 Hz = 0.33 seconds)
    pub const MIN_FLASH_INTERVAL: f32 = 1.0 / MAX_FLASH_RATE_HZ;
    /// Maximum luminance change per flash (10% WCAG standard)
    pub const MAX_LUMINANCE_CHANGE: f32 = 0.1;
    /// Maximum area that can flash simultaneously (25% of screen)
    pub const MAX_FLASH_AREA_RATIO: f32 = 0.25;
    /// Red hue range to restrict (most dangerous for seizures)
    pub const RED_HUE_MIN: f32 = 345.0;
    pub const RED_HUE_MAX: f32 = 15.0;
    /// Safety cooldown period after major change
    pub const SAFETY_COOLDOWN_DURATION: f32 = 0.5;
}

/// Represents a visual change with safety analysis
#[derive(Debug, Clone)]
pub struct VisualUpdate {
    pub timestamp: f64,
    pub color: Vec3,
    pub previous_color: Vec3,
    pub position: Vec3,
    pub size: f32,
    pub intensity: f32,
    pub change_magnitude: f32,
    pub entity_id: usize,
}

impl VisualUpdate {
    pub fn new(entity_id: usize, position: Vec3, color: Vec3, previous_color: Vec3, size: f32, intensity: f32, timestamp: f64) -> Self {
        let change_magnitude = Self::calculate_change_magnitude(&color, &previous_color);
        Self {
            timestamp,
            color,
            previous_color,
            position,
            size,
            intensity,
            change_magnitude,
            entity_id,
        }
    }

    fn calculate_change_magnitude(color: &Vec3, previous_color: &Vec3) -> f32 {
        // Calculate luminance change using standard formula
        let current_luminance = Self::calculate_luminance(color);
        let previous_luminance = Self::calculate_luminance(previous_color);

        (current_luminance - previous_luminance).abs()
    }

    fn calculate_luminance(color: &Vec3) -> f32 {
        // ITU-R BT.709 luma coefficients for perceived brightness
        0.2126 * color.x + 0.7152 * color.y + 0.0722 * color.z
    }

    pub fn get_luminance_change(&self) -> f32 {
        self.change_magnitude
    }

    pub fn is_red_flash(&self) -> bool {
        let hsv = rgb_to_hsv(&self.color);
        let prev_hsv = rgb_to_hsv(&self.previous_color);

        // Check if either color is in dangerous red range
        (hsv.x >= safety_constants::RED_HUE_MIN || hsv.x <= safety_constants::RED_HUE_MAX) &&
        (prev_hsv.x >= safety_constants::RED_HUE_MIN || prev_hsv.x <= safety_constants::RED_HUE_MAX) &&
        self.change_magnitude > safety_constants::MAX_LUMINANCE_CHANGE * 0.5
    }
}

/// Tracks flash timing to enforce 3 Hz rate limiting
#[derive(Debug)]
pub struct FlashTracker {
    major_changes: VecDeque<f64>,
    last_major_change: f64,
    flash_count_window: f32,
}

impl FlashTracker {
    pub fn new() -> Self {
        Self {
            major_changes: VecDeque::new(),
            last_major_change: 0.0,
            flash_count_window: 1.0, // 1 second window
        }
    }

    pub fn can_allow_flash(&mut self, current_time: f64, change_magnitude: f32) -> bool {
        // Clean up old entries
        self.cleanup_old_changes(current_time);

        // Check if this would be a major change
        if change_magnitude < safety_constants::MAX_LUMINANCE_CHANGE * 0.3 {
            return true; // Minor changes are always allowed
        }

        // Check minimum interval since last major change
        let time_since_last = current_time - self.last_major_change;
        if time_since_last < safety_constants::MIN_FLASH_INTERVAL as f64 {
            return false; // Too soon since last major change
        }

        // Check flash rate in current window
        let current_flash_count = self.major_changes.len();
        let max_flashes_in_window = (safety_constants::MAX_FLASH_RATE_HZ * self.flash_count_window) as usize;

        current_flash_count < max_flashes_in_window
    }

    pub fn record_flash(&mut self, timestamp: f64) {
        self.major_changes.push_back(timestamp);
        self.last_major_change = timestamp;
    }

    fn cleanup_old_changes(&mut self, current_time: f64) {
        let cutoff_time = current_time - self.flash_count_window as f64;
        while let Some(&front_time) = self.major_changes.front() {
            if front_time < cutoff_time {
                self.major_changes.pop_front();
            } else {
                break;
            }
        }
    }

    pub fn get_current_flash_rate(&self, current_time: f64) -> f32 {
        let window_start = current_time - self.flash_count_window as f64;
        let recent_flashes = self.major_changes.iter()
            .filter(|&&time| time >= window_start)
            .count();

        recent_flashes as f32 / self.flash_count_window
    }
}

/// Limits luminance changes to safe levels
#[derive(Debug)]
pub struct LuminanceLimiter {
    max_change_per_frame: f32,
    smoothing_factor: f32,
}

impl LuminanceLimiter {
    pub fn new() -> Self {
        Self {
            max_change_per_frame: safety_constants::MAX_LUMINANCE_CHANGE,
            smoothing_factor: 0.1,
        }
    }

    pub fn limit_color_change(&self, current: Vec3, previous: Vec3) -> Vec3 {
        let luminance_change = VisualUpdate::calculate_change_magnitude(&current, &previous);

        if luminance_change <= self.max_change_per_frame {
            return current; // Change is within safe limits
        }

        // Interpolate to safe luminance change
        let safe_factor = self.max_change_per_frame / luminance_change;
        self.interpolate_safe_color(previous, current, safe_factor)
    }

    fn interpolate_safe_color(&self, from: Vec3, to: Vec3, factor: f32) -> Vec3 {
        // Smooth interpolation to maintain visual continuity while staying safe
        let clamped_factor = factor.clamp(0.0, 1.0);
        from.lerp(to, clamped_factor * self.smoothing_factor)
    }

    pub fn apply_red_flash_protection(&self, color: Vec3) -> Vec3 {
        let hsv = rgb_to_hsv(&color);

        // If in dangerous red range, shift hue slightly
        if hsv.x >= safety_constants::RED_HUE_MIN || hsv.x <= safety_constants::RED_HUE_MAX {
            let safe_hue = if hsv.x >= safety_constants::RED_HUE_MIN {
                safety_constants::RED_HUE_MIN - 20.0 // Shift to safe orange
            } else {
                safety_constants::RED_HUE_MAX + 20.0 // Shift to safe orange
            };

            let safe_hsv = Vec3::new(safe_hue, hsv.y * 0.8, hsv.z); // Reduce saturation too
            return hsv_to_rgb(&safe_hsv);
        }

        color
    }
}

/// Prevents multiple entities from flashing simultaneously
#[derive(Debug)]
pub struct ChaosDampener {
    max_simultaneous_flashing: usize,
    currently_flashing: Vec<usize>,
    entity_flash_cooldowns: std::collections::HashMap<usize, f64>,
    suppression_active: bool,
}

impl ChaosDampener {
    pub fn new() -> Self {
        Self {
            max_simultaneous_flashing: 3, // Conservative limit
            currently_flashing: Vec::new(),
            entity_flash_cooldowns: std::collections::HashMap::new(),
            suppression_active: false,
        }
    }

    pub fn can_entity_flash(&mut self, entity_id: usize, current_time: f64) -> bool {
        // Check cooldown
        if let Some(&cooldown_end) = self.entity_flash_cooldowns.get(&entity_id) {
            if current_time < cooldown_end {
                return false;
            }
        }

        // Check global flash limit
        self.cleanup_finished_flashes(current_time);

        if self.currently_flashing.len() >= self.max_simultaneous_flashing {
            return false;
        }

        // Emergency suppression mode
        if self.suppression_active {
            return false;
        }

        true
    }

    pub fn register_flash_start(&mut self, entity_id: usize, current_time: f64, duration: f32) {
        if !self.currently_flashing.contains(&entity_id) {
            self.currently_flashing.push(entity_id);
        }

        // Set cooldown for this entity
        let cooldown_end = current_time + duration + safety_constants::MIN_FLASH_INTERVAL as f64;
        self.entity_flash_cooldowns.insert(entity_id, cooldown_end);
    }

    pub fn end_flash(&mut self, entity_id: usize) {
        self.currently_flashing.retain(|&id| id != entity_id);
    }

    fn cleanup_finished_flashes(&mut self, current_time: f64) {
        let mut to_remove = Vec::new();

        for &entity_id in &self.currently_flashing {
            if let Some(&cooldown_end) = self.entity_flash_cooldowns.get(&entity_id) {
                if current_time >= cooldown_end {
                    to_remove.push(entity_id);
                }
            }
        }

        for entity_id in to_remove {
            self.end_flash(entity_id);
        }
    }

    pub fn activate_emergency_suppression(&mut self) {
        self.suppression_active = true;
        self.currently_flashing.clear();
        println!("⚠️ EMERGENCY SAFETY SUPPRESSION ACTIVATED");
    }

    pub fn deactivate_emergency_suppression(&mut self) {
        self.suppression_active = false;
        println!("✅ Emergency safety suppression deactivated");
    }

    pub fn get_flash_status(&self) -> (usize, usize) {
        (self.currently_flashing.len(), self.max_simultaneous_flashing)
    }
}

/// User configurable safety settings
#[derive(Debug, Clone)]
pub struct SafetySettings {
    pub enabled: bool,
    pub max_flash_rate: f32,
    pub max_luminance_change: f32,
    pub red_flash_protection: bool,
    pub chaos_dampening: bool,
    pub visual_intensity_limit: f32, // 0.0 to 1.0
    pub emergency_stop_enabled: bool,
}

impl Default for SafetySettings {
    fn default() -> Self {
        Self {
            enabled: true,
            max_flash_rate: safety_constants::MAX_FLASH_RATE_HZ,
            max_luminance_change: safety_constants::MAX_LUMINANCE_CHANGE,
            red_flash_protection: true,
            chaos_dampening: true,
            visual_intensity_limit: 1.0,
            emergency_stop_enabled: true,
        }
    }
}

/// Result of safety filtering
#[derive(Debug, Clone)]
pub enum SafetyResult {
    Allowed,
    Blocked(String),
    Modified(Vec3), // Color was modified for safety
}

/// Main safety engine that coordinates all safety systems
#[derive(Debug)]
pub struct SafetyEngine {
    flash_tracker: FlashTracker,
    luminance_limiter: LuminanceLimiter,
    chaos_dampener: ChaosDampener,
    settings: SafetySettings,
    emergency_stop_active: bool,
    last_warning_time: f64,
    warning_interval: f64,
}

impl SafetyEngine {
    pub fn new() -> Self {
        Self {
            flash_tracker: FlashTracker::new(),
            luminance_limiter: LuminanceLimiter::new(),
            chaos_dampener: ChaosDampener::new(),
            settings: SafetySettings::default(),
            emergency_stop_active: false,
            last_warning_time: 0.0,
            warning_interval: 10.0, // Warn every 10 seconds if needed
        }
    }

    pub fn new_with_settings(settings: SafetySettings) -> Self {
        let mut engine = Self::new();
        engine.settings = settings;
        engine
    }

    /// Main safety filtering function - processes all visual updates
    pub fn filter_visual_update(&mut self, update: &mut VisualUpdate, current_time: f64) -> SafetyResult {
        if !self.settings.enabled {
            return SafetyResult::Allowed;
        }

        if self.emergency_stop_active {
            return SafetyResult::Blocked("Emergency stop active".to_string());
        }

        // Check flash rate limiting
        if !self.flash_tracker.can_allow_flash(current_time, update.change_magnitude) {
            self.log_safety_warning(current_time, "Flash rate limit exceeded");
            return SafetyResult::Blocked("Flash rate limit exceeded".to_string());
        }

        // Check chaos dampening
        if self.settings.chaos_dampening {
            if !self.chaos_dampener.can_entity_flash(update.entity_id, current_time) {
                return SafetyResult::Blocked("Chaos dampener active".to_string());
            }
        }

        // Apply luminance limiting
        let original_color = update.color;
        update.color = self.luminance_limiter.limit_color_change(update.color, update.previous_color);

        // Apply red flash protection
        if self.settings.red_flash_protection {
            if update.is_red_flash() {
                self.log_safety_warning(current_time, "Red flash detected and blocked");
                return SafetyResult::Blocked("Red flash blocked".to_string());
            }
            update.color = self.luminance_limiter.apply_red_flash_protection(update.color);
        }

        // Apply visual intensity limiting
        if self.settings.visual_intensity_limit < 1.0 {
            let intensity_factor = self.settings.visual_intensity_limit;
            update.color = update.previous_color.lerp(update.color, intensity_factor);
            update.intensity *= intensity_factor;
        }

        // Record flash if significant change
        if update.change_magnitude > safety_constants::MAX_LUMINANCE_CHANGE * 0.3 {
            self.flash_tracker.record_flash(current_time);

            if self.settings.chaos_dampening {
                self.chaos_dampener.register_flash_start(
                    update.entity_id,
                    current_time,
                    safety_constants::MIN_FLASH_INTERVAL
                );
            }
        }

        // Return result
        if original_color != update.color {
            SafetyResult::Modified(update.color)
        } else {
            SafetyResult::Allowed
        }
    }

    /// Emergency stop all visual effects
    pub fn activate_emergency_stop(&mut self) {
        self.emergency_stop_active = true;
        self.chaos_dampener.activate_emergency_suppression();
        println!("🚨 EMERGENCY SAFETY STOP ACTIVATED - ALL VISUAL EFFECTS SUPPRESSED");
    }

    pub fn deactivate_emergency_stop(&mut self) {
        self.emergency_stop_active = false;
        self.chaos_dampener.deactivate_emergency_suppression();
        println!("✅ Emergency safety stop deactivated");
    }

    /// Check if system is operating safely
    pub fn get_safety_status(&self, current_time: f64) -> SafetyStatus {
        let flash_rate = self.flash_tracker.get_current_flash_rate(current_time);
        let (current_flashing, max_flashing) = self.chaos_dampener.get_flash_status();

        SafetyStatus {
            emergency_stop_active: self.emergency_stop_active,
            current_flash_rate: flash_rate,
            max_flash_rate: self.settings.max_flash_rate,
            entities_flashing: current_flashing,
            max_entities_flashing: max_flashing,
            safety_enabled: self.settings.enabled,
            visual_intensity_limit: self.settings.visual_intensity_limit,
        }
    }

    fn log_safety_warning(&mut self, current_time: f64, message: &str) {
        if current_time - self.last_warning_time > self.warning_interval {
            println!("⚠️ SAFETY WARNING: {}", message);
            self.last_warning_time = current_time;
        }
    }

    /// Update settings during runtime
    pub fn update_settings(&mut self, settings: SafetySettings) {
        self.settings = settings;
    }

    pub fn get_settings(&self) -> &SafetySettings {
        &self.settings
    }
}

#[derive(Debug, Clone)]
pub struct SafetyStatus {
    pub emergency_stop_active: bool,
    pub current_flash_rate: f32,
    pub max_flash_rate: f32,
    pub entities_flashing: usize,
    pub max_entities_flashing: usize,
    pub safety_enabled: bool,
    pub visual_intensity_limit: f32,
}

/// Helper function to convert RGB to HSV
fn rgb_to_hsv(rgb: &Vec3) -> Vec3 {
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

    Vec3::new(hue.abs(), saturation, value)
}

/// Helper function to convert HSV to RGB
fn hsv_to_rgb(hsv: &Vec3) -> Vec3 {
    let h = hsv.x;
    let s = hsv.y;
    let v = hsv.z;

    let c = v * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());
    let m = v - c;

    let (r, g, b) = match h_prime as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        5 => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    };

    Vec3::new(r + m, g + m, b + m)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flash_rate_limiting() {
        let mut tracker = FlashTracker::new();
        let current_time = 0.0;

        // Should allow first flash
        assert!(tracker.can_allow_flash(current_time, 0.5));
        tracker.record_flash(current_time);

        // Should block immediate second flash
        assert!(!tracker.can_allow_flash(current_time + 0.1, 0.5));

        // Should allow flash after safe interval
        assert!(tracker.can_allow_flash(current_time + safety_constants::MIN_FLASH_INTERVAL + 0.1, 0.5));
    }

    #[test]
    fn test_luminance_limiting() {
        let limiter = LuminanceLimiter::new();
        let bright = Vec3::new(1.0, 1.0, 1.0);
        let dark = Vec3::new(0.0, 0.0, 0.0);

        // Large change should be limited
        let result = limiter.limit_color_change(bright, dark);
        assert!(result != bright);
        assert!(VisualUpdate::calculate_change_magnitude(&result, &dark) <= safety_constants::MAX_LUMINANCE_CHANGE);
    }

    #[test]
    fn test_red_flash_detection() {
        let red = Vec3::new(1.0, 0.0, 0.0);
        let blue = Vec3::new(0.0, 0.0, 1.0);

        let update = VisualUpdate::new(0, Vec3::ZERO, red, blue, 10.0, 1.0, 0.0);
        assert!(update.is_red_flash());
    }
}