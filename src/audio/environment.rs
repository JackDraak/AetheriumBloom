// === AUDIO ENVIRONMENT ZONES ===
// Territorial soundscapes and meta-consciousness observer interventions
// Audio responds to spatial dynamics and territory control

use std::collections::HashMap;
use glam::Vec2;
use super::AudioEnvironment;

/// Manages territorial audio zones and environmental soundscapes
pub struct AudioEnvironmentZones {
    // Territorial zone mapping
    territory_zones: Vec<TerritorialZone>,
    zone_audio_generators: HashMap<usize, ZoneAudioGenerator>,

    // Meta-consciousness observer system
    meta_observer: MetaObserverAudio,

    // Environmental audio layers
    ambient_layer: AmbientSoundscapeGenerator,
    spatial_audio_processor: SpatialAudioProcessor,

    // Dynamic zone evolution
    zone_evolution_rate: f32,
    consciousness_influence_radius: f32,
}

/// Territorial zones that generate unique soundscapes
#[derive(Debug, Clone)]
struct TerritorialZone {
    center: Vec2,
    radius: f32,
    dominant_species: Option<crate::core::events::LlamaSpecies>,
    consciousness_density: f32,
    territory_age: f32,
    audio_signature: TerritoryAudioSignature,
}

/// Audio signature for each territory type
#[derive(Debug, Clone)]
struct TerritoryAudioSignature {
    base_frequency: f32,
    harmonic_complexity: f32,
    spatial_reverb: f32,
    dominance_factor: f32,
    evolution_speed: f32,
}

/// Generates audio for specific zones
struct ZoneAudioGenerator {
    oscillators: Vec<TerritorialOscillator>,
    phase_accumulator: f64,
    territory_id: usize,
    influence_strength: f32,
}

/// Meta-consciousness observer - intervenes when consciousness reaches critical levels
pub struct MetaObserverAudio {
    observer_active: bool,
    intervention_threshold: f32,
    intervention_intensity: f32,

    // Observer audio characteristics
    observer_frequency: f32,
    observer_harmonics: Vec<f32>,
    intervention_phase: f64,

    // Critical consciousness warnings
    warning_system_active: bool,
    warning_frequency: f32,
    warning_modulation: f32,
}

/// Ambient soundscape generator for environmental audio
struct AmbientSoundscapeGenerator {
    // Nature-inspired base layers
    wind_generator: WindNoiseGenerator,
    crystal_resonance: CrystalResonanceGenerator,
    cosmic_background: CosmicBackgroundGenerator,

    // Dynamic mixing based on consciousness levels
    mix_levels: [f32; 3],
    consciousness_influence: f32,
}

/// Spatial audio processing for 3D consciousness mapping with cursor responsiveness
struct SpatialAudioProcessor {
    // Consciousness field mapping
    consciousness_field_map: Vec<Vec<f32>>, // 2D grid of consciousness density
    field_resolution: usize,

    // Cursor-based spatial audio mechanics
    cursor_position: Vec2,
    previous_cursor_position: Vec2,
    cursor_velocity: Vec2,
    cursor_acceleration: Vec2,
    movement_chaos_accumulator: f32,
    stillness_duration: f32,

    // Chaotic audio mechanics from cursor behavior
    cursor_audio_influence: f32,
    movement_frequency_modulation: f32,
    stillness_tension_buildup: f32,
    chaos_burst_threshold: f32,

    // Audio zones created by cursor movement
    cursor_trail: Vec<CursorTrailPoint>,
    max_trail_length: usize,

    // Spatial audio parameters
    listener_position: Vec2,
    field_dampening: f32,
    spatial_reverb_zones: Vec<SpatialReverbZone>,
}

#[derive(Debug, Clone)]
struct TerritorialOscillator {
    frequency: f32,
    amplitude: f32,
    phase: f64,
    territorial_modulation: f32,
}

#[derive(Debug, Clone)]
struct SpatialReverbZone {
    center: Vec2,
    radius: f32,
    reverb_time: f32,
    damping: f32,
}

/// Tracks cursor movement for audio trail effects
#[derive(Debug, Clone)]
struct CursorTrailPoint {
    position: Vec2,
    timestamp: f64,
    movement_intensity: f32,
    audio_influence_radius: f32,
}

/// Wind noise for environmental ambience
struct WindNoiseGenerator {
    noise_buffer: Vec<f32>,
    buffer_index: usize,
    wind_intensity: f32,
    wind_direction_phase: f64,
}

/// Crystal resonance for magical ambience
struct CrystalResonanceGenerator {
    resonant_frequencies: Vec<f32>,
    resonance_amplitudes: Vec<f32>,
    phase_accumulators: Vec<f64>,
    crystal_density_influence: f32,
}

/// Cosmic background radiation as audio
struct CosmicBackgroundGenerator {
    cosmic_noise_seed: u64,
    cosmic_frequency_drift: f32,
    background_amplitude: f32,
    universe_phase: f64,
}

impl AudioEnvironmentZones {
    pub fn new() -> Self {
        Self {
            territory_zones: Vec::new(),
            zone_audio_generators: HashMap::new(),
            meta_observer: MetaObserverAudio::new(),
            ambient_layer: AmbientSoundscapeGenerator::new(),
            spatial_audio_processor: SpatialAudioProcessor::new(),
            zone_evolution_rate: 0.01,
            consciousness_influence_radius: 150.0,
        }
    }

    /// Update territorial zones based on llama positions and consciousness
    pub fn update(&mut self, cosmic_time: f64, llama_positions: &[Vec2]) {
        // Update existing zones and create new ones
        self.update_territorial_zones(llama_positions);

        // Update zone audio generators
        for (zone_id, generator) in self.zone_audio_generators.iter_mut() {
            if let Some(zone) = self.territory_zones.get(*zone_id) {
                generator.update(cosmic_time, zone);
            }
        }

        // Update meta-observer system
        let total_consciousness = self.calculate_total_consciousness();
        self.meta_observer.update(cosmic_time, total_consciousness);

        // Update ambient soundscape
        self.ambient_layer.update(cosmic_time, llama_positions.len(), total_consciousness);

        // Update spatial audio processing
        self.spatial_audio_processor.update(llama_positions);
    }

    /// Update cursor position for spatial audio responsiveness
    pub fn update_cursor_position(&mut self, cursor_position: Vec2) {
        self.spatial_audio_processor.update_cursor_position(cursor_position);
    }

    fn update_territorial_zones(&mut self, llama_positions: &[Vec2]) {
        // Clear old zones
        self.territory_zones.clear();

        if llama_positions.is_empty() {
            return;
        }

        // Cluster llamas into territorial zones using simple spatial clustering
        let mut zone_centers = Vec::new();
        let mut unassigned_positions = llama_positions.to_vec();

        while !unassigned_positions.is_empty() {
            let center = unassigned_positions[0];
            zone_centers.push(center);

            // Remove positions within influence radius of this center
            unassigned_positions.retain(|&pos| {
                (pos - center).length() > self.consciousness_influence_radius
            });
        }

        // Create territorial zones
        for (i, center) in zone_centers.iter().enumerate() {
            // Count llamas in this zone
            let llamas_in_zone: Vec<Vec2> = llama_positions.iter()
                .filter(|&&pos| (pos - *center).length() <= self.consciousness_influence_radius)
                .copied()
                .collect();

            let consciousness_density = llamas_in_zone.len() as f32 /
                (self.consciousness_influence_radius * self.consciousness_influence_radius);

            let zone = TerritorialZone {
                center: *center,
                radius: self.consciousness_influence_radius,
                dominant_species: None, // Would be determined by species analysis
                consciousness_density,
                territory_age: 0.0,
                audio_signature: TerritoryAudioSignature {
                    base_frequency: 220.0 + (i as f32 * 110.0) % 880.0,
                    harmonic_complexity: consciousness_density * 0.5,
                    spatial_reverb: 0.3 + consciousness_density * 0.2,
                    dominance_factor: consciousness_density,
                    evolution_speed: 0.1,
                },
            };

            self.territory_zones.push(zone);

            // Create audio generator for this zone if it doesn't exist
            if !self.zone_audio_generators.contains_key(&i) {
                let generator = ZoneAudioGenerator::new(i, &zone);
                self.zone_audio_generators.insert(i, generator);
            }
        }
    }

    fn calculate_total_consciousness(&self) -> f32 {
        self.territory_zones.iter()
            .map(|zone| zone.consciousness_density * zone.radius * zone.radius)
            .sum()
    }

    /// Process audio sample through environmental zones
    pub fn process_sample(&mut self,
                         input_sample: f32,
                         sample_time: f64,
                         environment: &AudioEnvironment) -> f32 {

        let mut sample = input_sample;

        // Apply ambient soundscape
        sample = self.ambient_layer.process_sample(sample, sample_time, environment);

        // Apply territorial zone influences
        for (zone_id, generator) in self.zone_audio_generators.iter_mut() {
            if let Some(zone) = self.territory_zones.get(*zone_id) {
                let zone_influence = generator.generate_territorial_influence(sample_time);
                let influence_strength = zone.consciousness_density * 0.1;
                sample += zone_influence * influence_strength;
            }
        }

        // Apply spatial audio processing
        sample = self.spatial_audio_processor.process_sample(sample);

        // Apply meta-observer interventions
        sample = self.meta_observer.process_sample(sample, sample_time);

        sample
    }
}

impl ZoneAudioGenerator {
    fn new(territory_id: usize, zone: &TerritorialZone) -> Self {
        let oscillator_count = (zone.consciousness_density * 5.0) as usize + 1;
        let mut oscillators = Vec::with_capacity(oscillator_count);

        for i in 0..oscillator_count {
            oscillators.push(TerritorialOscillator {
                frequency: zone.audio_signature.base_frequency * (i + 1) as f32,
                amplitude: 1.0 / (i + 1) as f32,
                phase: fastrand::f64() * std::f64::consts::TAU,
                territorial_modulation: zone.audio_signature.dominance_factor,
            });
        }

        Self {
            oscillators,
            phase_accumulator: 0.0,
            territory_id,
            influence_strength: zone.consciousness_density,
        }
    }

    fn update(&mut self, cosmic_time: f64, zone: &TerritorialZone) {
        self.phase_accumulator = cosmic_time;
        self.influence_strength = zone.consciousness_density;

        // Update oscillator parameters based on zone evolution
        for oscillator in &mut self.oscillators {
            oscillator.territorial_modulation = zone.audio_signature.dominance_factor;
        }
    }

    fn generate_territorial_influence(&self, sample_time: f64) -> f32 {
        let mut territorial_audio = 0.0;

        for oscillator in &self.oscillators {
            let modulated_frequency = oscillator.frequency *
                (1.0 + oscillator.territorial_modulation *
                 (sample_time * 0.1).sin() as f32 * 0.1);

            let oscillator_sample = (sample_time * modulated_frequency as f64 * std::f64::consts::TAU).sin() as f32;
            territorial_audio += oscillator_sample * oscillator.amplitude;
        }

        territorial_audio * self.influence_strength * 0.2
    }
}

impl MetaObserverAudio {
    fn new() -> Self {
        Self {
            observer_active: false,
            intervention_threshold: 100.0, // High consciousness threshold
            intervention_intensity: 0.0,
            observer_frequency: 528.0, // Love frequency for benevolent intervention
            observer_harmonics: vec![528.0, 792.0, 1056.0, 1584.0], // Harmonic series
            intervention_phase: 0.0,
            warning_system_active: false,
            warning_frequency: 440.0,
            warning_modulation: 0.0,
        }
    }

    fn update(&mut self, cosmic_time: f64, total_consciousness: f32) {
        self.intervention_phase = cosmic_time;

        // Activate observer when consciousness exceeds threshold
        if total_consciousness > self.intervention_threshold {
            self.observer_active = true;
            self.intervention_intensity = ((total_consciousness - self.intervention_threshold) /
                                         self.intervention_threshold).min(1.0);
        } else {
            self.observer_active = false;
            self.intervention_intensity *= 0.95; // Fade out
        }

        // Warning system for dangerous consciousness levels
        if total_consciousness > self.intervention_threshold * 1.5 {
            self.warning_system_active = true;
            self.warning_modulation = (cosmic_time * 4.0).sin() as f32;
        } else {
            self.warning_system_active = false;
        }
    }

    fn process_sample(&self, input_sample: f32, sample_time: f64) -> f32 {
        let mut sample = input_sample;

        if self.observer_active {
            // Observer intervention - calming harmonic overlay
            let mut observer_signal = 0.0;

            for &harmonic_freq in &self.observer_harmonics {
                let harmonic_wave = (sample_time * harmonic_freq as f64 * std::f64::consts::TAU).sin() as f32;
                observer_signal += harmonic_wave * 0.25;
            }

            // Mix observer signal with decreasing intensity as it takes effect
            sample += observer_signal * self.intervention_intensity * 0.3;
        }

        if self.warning_system_active {
            // Warning system - pulsing tone
            let warning_wave = (sample_time * self.warning_frequency as f64 * std::f64::consts::TAU).sin() as f32;
            let warning_envelope = (self.warning_modulation * 0.5 + 0.5).max(0.0);
            sample += warning_wave * warning_envelope * 0.2;
        }

        sample
    }
}

impl AmbientSoundscapeGenerator {
    fn new() -> Self {
        Self {
            wind_generator: WindNoiseGenerator::new(),
            crystal_resonance: CrystalResonanceGenerator::new(),
            cosmic_background: CosmicBackgroundGenerator::new(),
            mix_levels: [0.3, 0.4, 0.2], // Wind, Crystal, Cosmic
            consciousness_influence: 0.0,
        }
    }

    fn update(&mut self, cosmic_time: f64, llama_count: usize, total_consciousness: f32) {
        self.consciousness_influence = total_consciousness / 100.0;

        // Adjust mix levels based on consciousness
        self.mix_levels = match total_consciousness {
            level if level < 10.0 => [0.5, 0.3, 0.2],  // More wind (nature)
            level if level < 50.0 => [0.3, 0.5, 0.2],  // More crystal (magical)
            _ => [0.2, 0.3, 0.5],                       // More cosmic (transcendent)
        };

        self.wind_generator.update(cosmic_time, self.consciousness_influence);
        self.crystal_resonance.update(cosmic_time, llama_count as f32);
        self.cosmic_background.update(cosmic_time, total_consciousness);
    }

    fn process_sample(&mut self, input_sample: f32, sample_time: f64, environment: &AudioEnvironment) -> f32 {
        let mut sample = input_sample;

        // Generate ambient layers
        let wind_sample = self.wind_generator.generate_sample(sample_time) * self.mix_levels[0];
        let crystal_sample = self.crystal_resonance.generate_sample(sample_time) * self.mix_levels[1];
        let cosmic_sample = self.cosmic_background.generate_sample(sample_time) * self.mix_levels[2];

        // Mix ambient layers based on environment
        let ambient_intensity = match environment {
            AudioEnvironment::Environmental => 0.6,
            AudioEnvironment::Meditative => 0.4,
            AudioEnvironment::Psychedelic => 0.3,
            AudioEnvironment::Electronica => 0.1,
            AudioEnvironment::HiveMind => 0.2,
            AudioEnvironment::RealityTear => 0.05,
        };

        sample += (wind_sample + crystal_sample + cosmic_sample) * ambient_intensity;
        sample
    }
}

impl SpatialAudioProcessor {
    fn new() -> Self {
        Self {
            consciousness_field_map: vec![vec![0.0; 64]; 64], // 64x64 grid
            field_resolution: 64,

            // Initialize cursor tracking
            cursor_position: Vec2::new(600.0, 400.0), // Start at center
            previous_cursor_position: Vec2::new(600.0, 400.0),
            cursor_velocity: Vec2::ZERO,
            cursor_acceleration: Vec2::ZERO,
            movement_chaos_accumulator: 0.0,
            stillness_duration: 0.0,

            // Chaotic audio mechanics
            cursor_audio_influence: 0.0,
            movement_frequency_modulation: 1.0,
            stillness_tension_buildup: 0.0,
            chaos_burst_threshold: 100.0,

            // Audio trail
            cursor_trail: Vec::new(),
            max_trail_length: 50,

            listener_position: Vec2::new(600.0, 400.0), // Center of screen
            field_dampening: 0.001,
            spatial_reverb_zones: Vec::new(),
        }
    }

    fn update(&mut self, llama_positions: &[Vec2]) {
        self.update_cursor_mechanics();
        self.update_consciousness_field(llama_positions);
    }

    /// Update cursor-based chaotic audio mechanics
    fn update_cursor_mechanics(&mut self) {
        // Calculate cursor velocity and acceleration
        let new_velocity = self.cursor_position - self.previous_cursor_position;
        let new_acceleration = new_velocity - self.cursor_velocity;

        self.previous_cursor_position = self.cursor_position;
        self.cursor_velocity = new_velocity;
        self.cursor_acceleration = new_acceleration;

        // Movement analysis
        let movement_magnitude = self.cursor_velocity.length();
        let acceleration_magnitude = self.cursor_acceleration.length();

        if movement_magnitude < 0.5 {
            // User is still - build tension
            self.stillness_duration += 0.016; // ~60fps frame time
            self.stillness_tension_buildup += 0.02;

            // Stillness creates atmospheric tension that builds up
            if self.stillness_duration > 2.0 {
                self.stillness_tension_buildup = (self.stillness_tension_buildup + 0.01).min(1.0);
            }
        } else {
            // User is moving - create chaos
            self.stillness_duration = 0.0;
            self.stillness_tension_buildup *= 0.95; // Fade tension

            // Movement creates immediate audio chaos
            let movement_chaos = movement_magnitude * 0.1 + acceleration_magnitude * 0.05;
            self.movement_chaos_accumulator += movement_chaos;

            // Rapid movements create frequency modulation
            self.movement_frequency_modulation = 1.0 + (movement_magnitude * 0.02).min(0.5);

            // Add to cursor trail for spatial audio effects
            self.add_cursor_trail_point(movement_magnitude);
        }

        // Calculate overall cursor audio influence
        self.cursor_audio_influence =
            (self.movement_chaos_accumulator * 0.5 + self.stillness_tension_buildup * 0.3).min(1.0);

        // Chaos burst when threshold is exceeded
        if self.movement_chaos_accumulator > self.chaos_burst_threshold {
            self.trigger_chaos_burst();
        }

        // Decay chaos accumulator over time
        self.movement_chaos_accumulator *= 0.98;
    }

    fn add_cursor_trail_point(&mut self, movement_intensity: f32) {
        let trail_point = CursorTrailPoint {
            position: self.cursor_position,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
            movement_intensity,
            audio_influence_radius: 50.0 + movement_intensity * 30.0,
        };

        self.cursor_trail.push(trail_point);

        // Limit trail length
        if self.cursor_trail.len() > self.max_trail_length {
            self.cursor_trail.remove(0);
        }
    }

    fn trigger_chaos_burst(&mut self) {
        // Reset chaos accumulator after burst
        self.movement_chaos_accumulator = 0.0;

        // Create massive spatial reverb zone at cursor position
        let chaos_reverb = SpatialReverbZone {
            center: self.cursor_position,
            radius: 200.0,
            reverb_time: 2.0,
            damping: 0.3,
        };

        self.spatial_reverb_zones.push(chaos_reverb);

        // Limit number of reverb zones
        if self.spatial_reverb_zones.len() > 5 {
            self.spatial_reverb_zones.remove(0);
        }
    }

    fn update_consciousness_field(&mut self, llama_positions: &[Vec2]) {
        // Reset consciousness field
        for row in &mut self.consciousness_field_map {
            for cell in row {
                *cell *= 0.9; // Decay
            }
        }

        // Add consciousness influence from each llama
        for &position in llama_positions {
            let grid_x = ((position.x / 1200.0) * self.field_resolution as f32) as usize;
            let grid_y = ((position.y / 800.0) * self.field_resolution as f32) as usize;

            if grid_x < self.field_resolution && grid_y < self.field_resolution {
                self.consciousness_field_map[grid_y][grid_x] += 0.1;

                // Spread influence to neighboring cells
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        let nx = grid_x as i32 + dx;
                        let ny = grid_y as i32 + dy;

                        if nx >= 0 && ny >= 0 &&
                           nx < self.field_resolution as i32 &&
                           ny < self.field_resolution as i32 {
                            self.consciousness_field_map[ny as usize][nx as usize] += 0.05;
                        }
                    }
                }
            }
        }

        // Add cursor influence to consciousness field
        let cursor_grid_x = ((self.cursor_position.x / 1200.0) * self.field_resolution as f32) as usize;
        let cursor_grid_y = ((self.cursor_position.y / 800.0) * self.field_resolution as f32) as usize;

        if cursor_grid_x < self.field_resolution && cursor_grid_y < self.field_resolution {
            // Cursor adds consciousness based on movement chaos and stillness tension
            let cursor_influence = self.cursor_audio_influence * 0.2;
            self.consciousness_field_map[cursor_grid_y][cursor_grid_x] += cursor_influence;
        }
    }

    fn process_sample(&self, input_sample: f32) -> f32 {
        let mut sample = input_sample;

        // Apply cursor-based frequency modulation
        sample *= self.movement_frequency_modulation;

        // Add cursor trail spatial effects
        let cursor_trail_influence = self.calculate_cursor_trail_influence();
        sample += cursor_trail_influence * 0.1;

        // Apply stillness tension (creates eerie buildup)
        if self.stillness_tension_buildup > 0.1 {
            let tension_frequency = 60.0 + self.stillness_tension_buildup * 40.0; // Low frequency tension
            let tension_wave = (std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64() * tension_frequency as f64 * std::f64::consts::TAU).sin() as f32;
            sample += tension_wave * self.stillness_tension_buildup * 0.15;
        }

        // Apply spatial reverb from chaos bursts
        for reverb_zone in &self.spatial_reverb_zones {
            let distance_to_cursor = (self.cursor_position - reverb_zone.center).length();
            if distance_to_cursor < reverb_zone.radius {
                let reverb_intensity = 1.0 - (distance_to_cursor / reverb_zone.radius);
                sample += sample * reverb_intensity * 0.3; // Echo effect
            }
        }

        // Get consciousness density at cursor position (listener follows cursor)
        let cursor_grid_x = ((self.cursor_position.x / 1200.0) * self.field_resolution as f32) as usize;
        let cursor_grid_y = ((self.cursor_position.y / 800.0) * self.field_resolution as f32) as usize;

        let consciousness_density = if cursor_grid_x < self.field_resolution &&
                                   cursor_grid_y < self.field_resolution {
            self.consciousness_field_map[cursor_grid_y][cursor_grid_x]
        } else {
            0.0
        };

        // Apply consciousness-based spatial effects at cursor location
        sample *= 1.0 + consciousness_density * 0.2;

        // Overall cursor audio influence multiplier
        sample *= 1.0 + self.cursor_audio_influence * 0.3;

        sample
    }

    fn calculate_cursor_trail_influence(&self) -> f32 {
        let mut trail_influence = 0.0;

        for trail_point in &self.cursor_trail {
            let distance_to_cursor = (self.cursor_position - trail_point.position).length();
            if distance_to_cursor < trail_point.audio_influence_radius {
                let influence_strength = 1.0 - (distance_to_cursor / trail_point.audio_influence_radius);
                trail_influence += influence_strength * trail_point.movement_intensity * 0.1;
            }
        }

        trail_influence.min(0.5) // Cap maximum trail influence
    }

    /// Update cursor position (called from main loop)
    pub fn update_cursor_position(&mut self, new_cursor_position: Vec2) {
        self.cursor_position = new_cursor_position;
    }
}

impl WindNoiseGenerator {
    fn new() -> Self {
        let mut noise_buffer = Vec::with_capacity(1024);
        for _ in 0..1024 {
            noise_buffer.push((fastrand::f32() - 0.5) * 2.0);
        }

        Self {
            noise_buffer,
            buffer_index: 0,
            wind_intensity: 0.3,
            wind_direction_phase: 0.0,
        }
    }

    fn update(&mut self, cosmic_time: f64, consciousness_influence: f32) {
        self.wind_direction_phase = cosmic_time * 0.1;
        self.wind_intensity = 0.2 + consciousness_influence * 0.3;
    }

    fn generate_sample(&mut self, sample_time: f64) -> f32 {
        // Get noise sample
        let noise = self.noise_buffer[self.buffer_index % self.noise_buffer.len()];
        self.buffer_index += 1;

        // Apply wind characteristics
        let wind_modulation = (self.wind_direction_phase + sample_time * 0.05).sin() as f32;
        noise * self.wind_intensity * (0.5 + wind_modulation * 0.5)
    }
}

impl CrystalResonanceGenerator {
    fn new() -> Self {
        Self {
            resonant_frequencies: vec![261.63, 293.66, 329.63, 349.23, 392.0, 440.0, 493.88], // C major scale
            resonance_amplitudes: vec![0.1; 7],
            phase_accumulators: vec![0.0; 7],
            crystal_density_influence: 0.0,
        }
    }

    fn update(&mut self, cosmic_time: f64, llama_count: f32) {
        self.crystal_density_influence = llama_count / 20.0;

        // Update phase accumulators
        for accumulator in &mut self.phase_accumulators {
            *accumulator = cosmic_time;
        }
    }

    fn generate_sample(&self, sample_time: f64) -> f32 {
        let mut crystal_sample = 0.0;

        for (i, &frequency) in self.resonant_frequencies.iter().enumerate() {
            let amplitude = self.resonance_amplitudes[i] * self.crystal_density_influence;
            let crystal_wave = (sample_time * frequency as f64 * std::f64::consts::TAU).sin() as f32;
            crystal_sample += crystal_wave * amplitude;
        }

        crystal_sample
    }
}

impl CosmicBackgroundGenerator {
    fn new() -> Self {
        Self {
            cosmic_noise_seed: 42,
            cosmic_frequency_drift: 0.0,
            background_amplitude: 0.1,
            universe_phase: 0.0,
        }
    }

    fn update(&mut self, cosmic_time: f64, total_consciousness: f32) {
        self.universe_phase = cosmic_time * 0.01;
        self.background_amplitude = 0.05 + (total_consciousness / 200.0).min(0.15);
        self.cosmic_frequency_drift = (cosmic_time * 0.001).sin() as f32 * 0.1;
    }

    fn generate_sample(&self, sample_time: f64) -> f32 {
        // Generate cosmic background based on universe phase
        let cosmic_frequency = 40.0 + self.cosmic_frequency_drift;
        let cosmic_wave = (self.universe_phase + sample_time * cosmic_frequency as f64 * std::f64::consts::TAU).sin() as f32;

        // Add some cosmic noise
        let cosmic_noise = ((sample_time * 1000.0) as u64 ^ self.cosmic_noise_seed) as f32 / u64::MAX as f32;

        (cosmic_wave + cosmic_noise * 0.3) * self.background_amplitude
    }
}