// Psychedelic shader uniforms and GPU data structures
// Extracted from simple.rs for better modularity

/// Uniform data structure matching the psychedelic.wgsl shader
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PsychedelicUniforms {
    pub time: f32,
    pub reality_distortion: f32,
    pub consciousness_level: f32,
    pub beat_intensity: f32,
    pub screen_resolution: [f32; 2],
    pub beat_frequency: f32,
    pub cosmic_phase: f32,
}

impl Default for PsychedelicUniforms {
    fn default() -> Self {
        Self {
            time: 0.0,
            reality_distortion: 0.0,
            consciousness_level: 1.0,
            beat_intensity: 0.0,
            screen_resolution: [1200.0, 800.0],
            beat_frequency: 1.0,
            cosmic_phase: 0.0,
        }
    }
}