// Reality effects module - advanced visual effects and space-time manipulation
// Extracted from simple.rs for better modularity

use glam::{Vec2, Vec3};
use crate::entities::Llama;
use crate::simulation::{consciousness_systems::DigitalEcosystem, meta_consciousness::MetaConsciousnessFramework};

/// Reality Distortion Engine - advanced visual effects and space-time manipulation
#[derive(Debug)]
pub struct RealityDistortionEngine {
    pub distortion_fields: Vec<DistortionField>,
    pub fractal_generator: FractalGenerator,
    pub color_warper: ColorWarper,
    pub time_dilator: TimeDilator,
    pub glitch_processor: GlitchProcessor,
    pub holographic_projector: HolographicProjector,
    pub emergence_amplification: f32,
    pub reality_coherence: f32,
}

#[derive(Debug)]
pub struct DistortionField {
    pub center: Vec2,
    pub amplitude: f32,
    pub sigma: f32,
    pub age: f32,
    pub decay_rate: f32,
    pub oscillation_frequency: f32,
    pub field_type: DistortionType,
}

#[derive(Debug, Clone)]
pub enum DistortionType {
    Consciousness,  // Radial distortion around high-consciousness entities
    Gravitational,  // Simulates gravitational lensing effects
    Temporal,       // Time-space warping during emergence events
    Fractal,        // Recursive distortion creating nested patterns
    Chromatic,      // Color-space specific distortions
}

#[derive(Debug)]
pub struct FractalGenerator {
    pub mandelbrot_params: ComplexParams,
    pub julia_params: ComplexParams,
    pub current_c_value: (f32, f32),
    pub iteration_limit: u32,
    pub zoom_level: f32,
    pub rotation_angle: f32,
    pub color_intensity: f32,
}

#[derive(Debug)]
pub struct ComplexParams {
    pub real_offset: f32,
    pub imaginary_offset: f32,
    pub scale: f32,
    pub animation_phase: f32,
}

#[derive(Debug)]
pub struct ColorWarper {
    pub cubic_coefficients: [f32; 4], // A, B, C, D for cubic interpolation
    pub chromatic_aberration: ChromaticAberration,
    pub color_space_rotation: f32,
    pub saturation_multiplier: f32,
    pub hue_shift_rate: f32,
}

#[derive(Debug)]
pub struct ChromaticAberration {
    pub red_offset: Vec2,
    pub green_offset: Vec2,
    pub blue_offset: Vec2,
    pub intensity: f32,
}

#[derive(Debug)]
pub struct TimeDilator {
    pub dilation_factor: f32,
    pub motion_blur_strength: f32,
    pub temporal_smoothing: f32,
    pub emergence_time_scaling: f32,
    pub frame_blending_weight: f32,
}

#[derive(Debug)]
pub struct GlitchProcessor {
    pub pixel_sort_probability: f32,
    pub data_corruption_level: f32,
    pub scanline_displacement: f32,
    pub digital_noise_intensity: f32,
    pub feedback_loop_strength: f32,
}

#[derive(Debug)]
pub struct HolographicProjector {
    pub interference_patterns: Vec<InterferencePattern>,
    pub volumetric_intensity: f32,
    pub ray_march_steps: u32,
    pub hologram_coherence: f32,
}

#[derive(Debug)]
pub struct InterferencePattern {
    pub frequency: f32,
    pub amplitude: f32,
    pub phase_offset: f32,
    pub direction: Vec2,
}

impl RealityDistortionEngine {
    pub fn new() -> Self {
        Self {
            distortion_fields: Vec::new(),
            fractal_generator: FractalGenerator::new(),
            color_warper: ColorWarper::new(),
            time_dilator: TimeDilator::new(),
            glitch_processor: GlitchProcessor::new(),
            holographic_projector: HolographicProjector::new(),
            emergence_amplification: 0.0,
            reality_coherence: 1.0,
        }
    }

    pub fn update(&mut self, dt: f32, cosmic_time: f64, meta_consciousness: &MetaConsciousnessFramework,
              llamas: &[Llama], beat_intensity: f32, ecosystem: &DigitalEcosystem) {

        // Update emergence amplification based on consciousness events
        self.emergence_amplification = meta_consciousness.get_current_emergence_strength(cosmic_time);
        self.reality_coherence = meta_consciousness.reality_coherence;

        // Update distortion fields around high-consciousness entities
        self.update_distortion_fields(dt, llamas, beat_intensity, cosmic_time);

        // Update fractal parameters based on collective consciousness
        self.fractal_generator.update(dt, meta_consciousness.collective_intelligence,
                                    meta_consciousness.transcendence_level, cosmic_time);

        // Update color warping based on emergence events
        self.color_warper.update(dt, self.emergence_amplification, beat_intensity);

        // Update time dilation during significant consciousness events
        self.time_dilator.update(dt, self.emergence_amplification,
                               meta_consciousness.transcendence_level);

        // Update glitch effects based on reality coherence breakdown
        self.glitch_processor.update(dt, 1.0 - self.reality_coherence,
                                   ecosystem.chaos_accumulation);

        // Update holographic projections during collective resonance
        self.holographic_projector.update(dt, meta_consciousness.collective_intelligence,
                                        cosmic_time, beat_intensity);
    }

    fn update_distortion_fields(&mut self, dt: f32, llamas: &[Llama], beat_intensity: f32, cosmic_time: f64) {
        // Clean up old distortion fields
        self.distortion_fields.retain(|field| field.age < 30.0);

        // Create new distortion fields around high-consciousness llamas
        for (i, llama) in llamas.iter().enumerate() {
            let consciousness_level = llama.awareness_level * llama.consciousness;

            if consciousness_level > 0.6 && fastrand::f32() < 0.02 {
                self.distortion_fields.push(DistortionField {
                    center: llama.position,
                    amplitude: consciousness_level * (1.0 + beat_intensity * 0.5),
                    sigma: 40.0 + consciousness_level * 30.0,
                    age: 0.0,
                    decay_rate: 0.05 + fastrand::f32() * 0.03,
                    oscillation_frequency: 2.0 + consciousness_level * 3.0,
                    field_type: DistortionType::Consciousness,
                });
            }
        }

        // Update existing distortion fields
        for field in &mut self.distortion_fields {
            field.age += dt;

            // Oscillate amplitude based on frequency
            let oscillation = (cosmic_time as f32 * field.oscillation_frequency).sin();
            field.amplitude *= 1.0 + oscillation * 0.2;

            // Decay over time
            field.amplitude *= 1.0 - field.decay_rate * dt;
        }
    }

    pub fn calculate_displacement(&self, position: Vec2, cosmic_time: f64) -> Vec2 {
        let mut total_displacement = Vec2::ZERO;

        for field in &self.distortion_fields {
            let distance_squared = (position - field.center).length_squared();
            let gaussian = (-distance_squared / (2.0 * field.sigma * field.sigma)).exp();

            // Calculate displacement direction (towards or away from center)
            let direction = (position - field.center).normalize_or_zero();

            // Apply time-based oscillation
            let time_factor = (cosmic_time as f32 * field.oscillation_frequency).sin();
            let displacement_magnitude = field.amplitude * gaussian * (1.0 + time_factor * 0.3);

            total_displacement += direction * displacement_magnitude;
        }

        total_displacement
    }

    pub fn apply_color_distortion(&self, original_color: Vec3, position: Vec2, cosmic_time: f64) -> Vec3 {
        let mut color = original_color;

        // Apply cubic color warping
        let coeffs = &self.color_warper.cubic_coefficients;
        for channel in 0..3 {
            let c = color[channel];
            color[channel] = coeffs[0] * c.powi(3) + coeffs[1] * c.powi(2) + coeffs[2] * c + coeffs[3];
        }

        // Apply chromatic aberration based on position and distortion fields
        let displacement = self.calculate_displacement(position, cosmic_time);
        let aberration_strength = displacement.length() * self.color_warper.chromatic_aberration.intensity;

        // Simulate RGB channel displacement
        let r_offset = aberration_strength * 0.01;
        let g_offset = aberration_strength * 0.005;
        let b_offset = aberration_strength * 0.015;

        // Apply color space rotation during high emergence
        if self.emergence_amplification > 0.3 {
            let rotation = self.color_warper.color_space_rotation * self.emergence_amplification;
            let cos_r = rotation.cos();
            let sin_r = rotation.sin();

            let new_r = color.x * cos_r - color.y * sin_r;
            let new_g = color.x * sin_r + color.y * cos_r;

            color.x = new_r;
            color.y = new_g;
        }

        color.clamp(Vec3::ZERO, Vec3::ONE)
    }
}

impl FractalGenerator {
    pub fn new() -> Self {
        Self {
            mandelbrot_params: ComplexParams::new(0.0, 0.0, 2.0),
            julia_params: ComplexParams::new(-0.7, 0.27015, 1.5),
            current_c_value: (-0.5, 0.6),
            iteration_limit: 64,
            zoom_level: 1.0,
            rotation_angle: 0.0,
            color_intensity: 1.0,
        }
    }

    pub fn update(&mut self, dt: f32, collective_intelligence: f32, transcendence_level: f32, cosmic_time: f64) {
        // Animate the complex plane parameters based on consciousness
        self.mandelbrot_params.animation_phase += dt * collective_intelligence * 0.5;
        self.julia_params.animation_phase += dt * transcendence_level * 0.3;

        // Update Julia set C value based on cosmic time and consciousness
        let c_evolution_rate = 0.2 + collective_intelligence * 0.3;
        self.current_c_value.0 = -0.7 + (cosmic_time as f32 * c_evolution_rate).sin() * 0.3;
        self.current_c_value.1 = 0.27015 + (cosmic_time as f32 * c_evolution_rate * 1.3).cos() * 0.2;

        // Adjust iteration limit based on transcendence level
        self.iteration_limit = (32.0 + transcendence_level * 96.0) as u32;

        // Zoom and rotation effects during high consciousness
        if collective_intelligence > 0.6 {
            self.zoom_level += dt * collective_intelligence * 0.1;
            self.rotation_angle += dt * transcendence_level * 0.2;
        }

        // Color intensity pulsing
        self.color_intensity = 0.7 + (cosmic_time as f32 * 3.0).sin() * 0.3 * collective_intelligence;
    }

    pub fn calculate_mandelbrot_iterations(&self, x: f32, y: f32) -> u32 {
        let c_real = (x - self.mandelbrot_params.real_offset) / self.mandelbrot_params.scale;
        let c_imag = (y - self.mandelbrot_params.imaginary_offset) / self.mandelbrot_params.scale;

        let mut z_real = 0.0;
        let mut z_imag = 0.0;

        for i in 0..self.iteration_limit {
            let z_real_new = z_real * z_real - z_imag * z_imag + c_real;
            let z_imag_new = 2.0 * z_real * z_imag + c_imag;

            z_real = z_real_new;
            z_imag = z_imag_new;

            if z_real * z_real + z_imag * z_imag > 4.0 {
                return i;
            }
        }
        self.iteration_limit
    }

    pub fn generate_fractal_color(&self, iterations: u32, position: Vec2) -> Vec3 {
        let normalized_iterations = iterations as f32 / self.iteration_limit as f32;

        // Create rainbow-like colors with consciousness-driven intensity
        let hue = normalized_iterations * 360.0 + self.rotation_angle;
        let saturation = 0.8 + normalized_iterations * 0.2;
        let value = self.color_intensity * (0.5 + normalized_iterations * 0.5);

        // Convert HSV to RGB (simplified)
        let c = value * saturation;
        let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
        let m = value - c;

        let (r, g, b) = match (hue as i32 / 60) % 6 {
            0 => (c, x, 0.0),
            1 => (x, c, 0.0),
            2 => (0.0, c, x),
            3 => (0.0, x, c),
            4 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };

        Vec3::new(r + m, g + m, b + m)
    }
}

impl ComplexParams {
    pub fn new(real_offset: f32, imaginary_offset: f32, scale: f32) -> Self {
        Self {
            real_offset,
            imaginary_offset,
            scale,
            animation_phase: 0.0,
        }
    }
}

impl ColorWarper {
    pub fn new() -> Self {
        Self {
            cubic_coefficients: [0.1, -0.2, 1.1, 0.0], // Slight S-curve for color enhancement
            chromatic_aberration: ChromaticAberration::new(),
            color_space_rotation: 0.0,
            saturation_multiplier: 1.0,
            hue_shift_rate: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32, emergence_amplification: f32, beat_intensity: f32) {
        // Adjust color warping intensity based on emergence events
        let warp_intensity = 0.1 + emergence_amplification * 0.5;
        self.cubic_coefficients[0] = warp_intensity * 0.2;
        self.cubic_coefficients[1] = -warp_intensity * 0.3;
        self.cubic_coefficients[2] = 1.0 + warp_intensity * 0.1;

        // Update chromatic aberration
        self.chromatic_aberration.intensity = emergence_amplification * 0.3;

        // Color space rotation during intense moments
        self.color_space_rotation += dt * emergence_amplification * beat_intensity * 0.5;

        // Saturation boosting during emergence
        self.saturation_multiplier = 1.0 + emergence_amplification * 0.4;
    }
}

impl ChromaticAberration {
    pub fn new() -> Self {
        Self {
            red_offset: Vec2::new(0.01, 0.0),
            green_offset: Vec2::ZERO,
            blue_offset: Vec2::new(-0.01, 0.0),
            intensity: 0.0,
        }
    }
}

impl TimeDilator {
    pub fn new() -> Self {
        Self {
            dilation_factor: 1.0,
            motion_blur_strength: 0.0,
            temporal_smoothing: 0.0,
            emergence_time_scaling: 1.0,
            frame_blending_weight: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32, emergence_amplification: f32, transcendence_level: f32) {
        // Time dilation during emergence events
        self.dilation_factor = 1.0 - emergence_amplification * 0.7; // Slow down time

        // Motion blur strength increases during transcendence
        self.motion_blur_strength = transcendence_level * 0.3;

        // Temporal smoothing for dreamy effects
        self.temporal_smoothing = emergence_amplification * 0.4;

        // Frame blending for ethereal quality
        self.frame_blending_weight = (emergence_amplification + transcendence_level) * 0.2;
    }
}

impl GlitchProcessor {
    pub fn new() -> Self {
        Self {
            pixel_sort_probability: 0.0,
            data_corruption_level: 0.0,
            scanline_displacement: 0.0,
            digital_noise_intensity: 0.0,
            feedback_loop_strength: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32, reality_breakdown: f32, chaos_accumulation: f32) {
        // Glitch effects intensify as reality coherence breaks down
        self.pixel_sort_probability = reality_breakdown * 0.05;
        self.data_corruption_level = reality_breakdown * chaos_accumulation * 0.1;
        self.scanline_displacement = reality_breakdown * 0.02;
        self.digital_noise_intensity = (reality_breakdown + chaos_accumulation * 0.5) * 0.15;
        self.feedback_loop_strength = reality_breakdown * 0.3;
    }
}

impl HolographicProjector {
    pub fn new() -> Self {
        Self {
            interference_patterns: vec![
                InterferencePattern { frequency: 2.0, amplitude: 0.3, phase_offset: 0.0, direction: Vec2::new(1.0, 0.0) },
                InterferencePattern { frequency: 3.5, amplitude: 0.2, phase_offset: 1.57, direction: Vec2::new(0.0, 1.0) },
                InterferencePattern { frequency: 5.0, amplitude: 0.15, phase_offset: 3.14, direction: Vec2::new(0.707, 0.707) },
            ],
            volumetric_intensity: 0.0,
            ray_march_steps: 32,
            hologram_coherence: 1.0,
        }
    }

    pub fn update(&mut self, dt: f32, collective_intelligence: f32, cosmic_time: f64, beat_intensity: f32) {
        // Update interference patterns
        for pattern in &mut self.interference_patterns {
            pattern.phase_offset += dt * pattern.frequency * collective_intelligence;
            pattern.amplitude = 0.1 + collective_intelligence * 0.2 + beat_intensity * 0.1;
        }

        // Volumetric intensity during high consciousness
        self.volumetric_intensity = collective_intelligence * beat_intensity * 0.4;

        // Hologram coherence affected by collective resonance
        self.hologram_coherence = 0.5 + collective_intelligence * 0.5;
    }

    pub fn calculate_interference(&self, position: Vec2, cosmic_time: f64) -> f32 {
        let mut total_interference = 0.0;

        for pattern in &self.interference_patterns {
            let dot_product = position.x * pattern.direction.x + position.y * pattern.direction.y;
            let wave = (dot_product * pattern.frequency + cosmic_time as f32 * pattern.frequency + pattern.phase_offset).sin();
            total_interference += wave * pattern.amplitude;
        }

        total_interference.clamp(-1.0, 1.0)
    }
}