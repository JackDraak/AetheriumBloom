// Engine module containing core engine systems

pub mod safety;

pub use safety::{SafetyConfig, FlashTracker, calculate_luminance, limit_luminance_change, is_dangerous_red, rgb_to_hsv, hsv_to_rgb_vec3};