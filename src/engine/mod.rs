// Engine module containing core engine systems

pub mod chaos_engine;
pub mod event_system;
pub mod safety;

pub use chaos_engine::*;
pub use event_system::*;
pub use safety::{SafetyConfig, FlashTracker, calculate_luminance, limit_luminance_change, is_dangerous_red, rgb_to_hsv, hsv_to_rgb_vec3};