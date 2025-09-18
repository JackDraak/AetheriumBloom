// === CONSCIOUSNESS AUDIO MAPPER ===
// Translates consciousness states and mathematical beat engines into collective audio harmonics
// The bridge between llama consciousness and sonic reality

use std::collections::HashMap;
use glam::Vec2;
use primes::{PrimeSet, Sieve};

// Internal conversion functions for compatibility
impl From<&super::mathematics::BeatState> for InternalBeatState {
    fn from(beat_state: &super::mathematics::BeatState) -> Self {
        Self {
            is_beat_drop: beat_state.is_beat_drop,
            intensity: beat_state.intensity,
            phase: beat_state.phase as f32,
            prime_factor: beat_state.prime_factor,
            cosmic_frequency: beat_state.cosmic_frequency,
        }
    }
}

impl From<&super::consciousness::LlamaRenderData> for InternalLlamaRenderData {
    fn from(llama_data: &super::consciousness::LlamaRenderData) -> Self {
        Self {
            position: llama_data.position,
            color_wavelength: llama_data.color_wavelength,
            trip_intensity: llama_data.trip_intensity,
            reality_distortion: llama_data.reality_distortion,
            species: match llama_data.species {
                super::consciousness::LlamaSpecies::Disco => InternalLlamaSpecies::Disco,
                super::consciousness::LlamaSpecies::Quantum => InternalLlamaSpecies::Quantum,
                super::consciousness::LlamaSpecies::BassDrop => InternalLlamaSpecies::BassDrop,
            },
        }
    }
}

// Internal types for audio processing
#[derive(Debug, Clone)]
struct InternalBeatState {
    is_beat_drop: bool,
    intensity: f32,
    phase: f32,
    prime_factor: f32,
    cosmic_frequency: f32,
}

#[derive(Debug, Clone)]
struct InternalLlamaRenderData {
    position: glam::Vec2,
    color_wavelength: glam::Vec2,
    trip_intensity: f32,
    reality_distortion: f32,
    species: InternalLlamaSpecies,
}

#[derive(Debug, Clone)]
enum InternalLlamaSpecies {
    Disco,
    Quantum,
    BassDrop,
}

/// Maps consciousness levels to fundamental frequencies and harmonics
pub struct ConsciousnessAudioMapper {
    prime_sieve: Sieve,

    // Species-specific sonic signatures
    species_signatures: HashMap<LlamaSpecies, SpeciesSonicSignature>,

    // Hive mind harmonics system
    hive_harmonics: HiveMindHarmonics,

    // Mathematical frequency relationships
    fundamental_frequency: f32,
    consciousness_scale_ratios: Vec<f32>,

    // Real-time consciousness tracking
    last_consciousness_state: f32,
    consciousness_velocity: f32,
    harmonic_coherence: f32,
}

/// Sonic signature for each llama species
#[derive(Debug, Clone)]
pub struct SpeciesSonicSignature {
    pub base_frequency_multiplier: f32,
    pub harmonic_series: Vec<f32>,
    pub modulation_depth: f32,
    pub chaos_factor: f32,
    pub spatial_spread: f32,
    pub consciousness_sensitivity: f32,
}

/// Collective hive mind harmonics system
pub struct HiveMindHarmonics {
    // Collective oscillator that emerges from llama interactions
    collective_phase: f64,
    coherence_level: f32,
    harmonic_lock_strength: f32,

    // Inter-llama frequency relationships
    spatial_frequency_map: Vec<Vec2>, // Positions mapped to frequencies
    frequency_interference_patterns: Vec<f32>,

    // Emergent harmonics that arise from collective behavior
    emergent_fundamentals: Vec<f32>,
    harmonic_convergence_points: Vec<f32>,

    // Consciousness field interactions
    consciousness_field_density: f32,
    field_resonance_frequencies: Vec<f32>,
}

impl ConsciousnessAudioMapper {
    pub fn new() -> Self {
        let mut mapper = Self {
            prime_sieve: Sieve::new(),
            species_signatures: HashMap::new(),
            hive_harmonics: HiveMindHarmonics::new(),
            fundamental_frequency: 432.0, // Universal healing frequency
            consciousness_scale_ratios: Vec::new(),
            last_consciousness_state: 0.0,
            consciousness_velocity: 0.0,
            harmonic_coherence: 0.0,
        };

        mapper.initialize_species_signatures();
        mapper.initialize_consciousness_scale();
        mapper
    }

    fn initialize_species_signatures(&mut self) {
        // Disco Llamas: Melodic, harmonic, party vibes
        self.species_signatures.insert(LlamaSpecies::Disco, SpeciesSonicSignature {
            base_frequency_multiplier: 1.0,
            harmonic_series: vec![1.0, 1.5, 2.0, 3.0, 4.0], // Perfect harmonics
            modulation_depth: 0.3,
            chaos_factor: 0.2,
            spatial_spread: 1.0,
            consciousness_sensitivity: 0.8,
        });

        // Quantum Sheep: High-frequency, uncertain, phase-shifting
        self.species_signatures.insert(LlamaSpecies::Quantum, SpeciesSonicSignature {
            base_frequency_multiplier: 2.618, // Golden ratio for quantum uncertainty
            harmonic_series: vec![1.0, 1.618, 2.618, 4.236, 6.854], // Fibonacci ratios
            modulation_depth: 0.7,
            chaos_factor: 0.8,
            spatial_spread: 3.0, // Quantum superposition spreads frequency
            consciousness_sensitivity: 1.5,
        });

        // BassDrop Vicunas: Massive sub-bass, earth-shaking power
        self.species_signatures.insert(LlamaSpecies::BassDrop, SpeciesSonicSignature {
            base_frequency_multiplier: 0.25, // Sub-bass range
            harmonic_series: vec![1.0, 2.0, 4.0, 8.0], // Power-of-2 harmonics for maximum impact
            modulation_depth: 0.1, // Less modulation for solid bass
            chaos_factor: 0.05,
            spatial_spread: 0.5, // Bass is omnidirectional
            consciousness_sensitivity: 0.3,
        });
    }

    fn initialize_consciousness_scale(&mut self) {
        // Create consciousness-to-frequency scale based on mathematical constants
        self.consciousness_scale_ratios = vec![
            1.0,           // Base consciousness
            1.059463094,   // Semitone (12th root of 2)
            1.122462048,   // Whole tone
            1.189207115,   // Minor third
            1.259921050,   // Major third
            1.334839854,   // Perfect fourth
            1.414213562,   // Tritone (sqrt(2))
            1.498307077,   // Perfect fifth
            1.587401052,   // Minor sixth
            1.681792831,   // Major sixth
            1.781797436,   // Minor seventh
            1.887748625,   // Major seventh
            2.0,           // Octave
            2.618033989,   // Golden ratio
            3.141592654,   // Pi
            2.718281828,   // e (Euler's number)
        ];
    }

    /// Main update - processes consciousness state and generates audio mappings
    pub fn update(&mut self,
                  cosmic_time: f64,
                  beat_state: &BeatState,
                  llama_data: &[LlamaRenderData]) {

        // Update hive mind harmonics
        self.hive_harmonics.update(cosmic_time, llama_data);

        // Calculate consciousness velocity (rate of change)
        let total_consciousness: f32 = llama_data.iter()
            .map(|llama| llama.trip_intensity)
            .sum();

        self.consciousness_velocity = total_consciousness - self.last_consciousness_state;
        self.last_consciousness_state = total_consciousness;

        // Update fundamental frequency based on mathematical beat engine
        self.update_fundamental_frequency(beat_state, total_consciousness);

        // Calculate harmonic coherence from llama interactions
        self.calculate_harmonic_coherence(llama_data);
    }

    fn update_fundamental_frequency(&mut self, beat_state: &BeatState, total_consciousness: f32) {
        // Start with base frequency (432 Hz - universal healing frequency)
        let mut freq = 432.0;

        // Apply mathematical beat engine modulation
        freq *= 1.0 + beat_state.prime_factor * 0.2;

        // Apply cosmic frequency from beat engine
        let cosmic_influence = beat_state.cosmic_frequency / 440.0; // Normalize to A4
        freq *= cosmic_influence.clamp(0.5, 2.0);

        // Consciousness level influences overall pitch
        let consciousness_influence = (total_consciousness / 100.0).min(2.0);
        let scale_index = (consciousness_influence * self.consciousness_scale_ratios.len() as f32) as usize;
        let scale_ratio = self.consciousness_scale_ratios.get(scale_index)
            .unwrap_or(&1.0);

        freq *= scale_ratio;

        // Apply prime number harmonic relationships
        let prime_index = ((beat_state.phase * 100.0) as usize) % 100;
        let prime = self.prime_sieve.nth(prime_index);
        let prime_harmonic = (prime % 16) as f32 / 16.0;
        freq *= 1.0 + prime_harmonic * 0.1;

        // Smooth frequency changes to avoid audio artifacts
        let smoothing_factor = 0.95;
        self.fundamental_frequency = self.fundamental_frequency * smoothing_factor + freq * (1.0 - smoothing_factor);
    }

    fn calculate_harmonic_coherence(&mut self, llama_data: &[LlamaRenderData]) {
        if llama_data.len() < 2 {
            self.harmonic_coherence = 0.0;
            return;
        }

        // Calculate spatial distribution entropy
        let mut position_variance = 0.0;
        let center = llama_data.iter()
            .map(|llama| llama.position)
            .fold(Vec2::ZERO, |acc, pos| acc + pos) / llama_data.len() as f32;

        for llama in llama_data {
            let distance = (llama.position - center).length();
            position_variance += distance * distance;
        }
        position_variance /= llama_data.len() as f32;

        // Calculate consciousness synchronization
        let consciousness_values: Vec<f32> = llama_data.iter()
            .map(|llama| llama.trip_intensity)
            .collect();

        let mean_consciousness = consciousness_values.iter().sum::<f32>() / consciousness_values.len() as f32;
        let consciousness_variance = consciousness_values.iter()
            .map(|&x| (x - mean_consciousness).powi(2))
            .sum::<f32>() / consciousness_values.len() as f32;

        // Coherence is high when llamas are close together AND have similar consciousness levels
        let spatial_coherence = 1.0 / (1.0 + position_variance * 0.001);
        let consciousness_coherence = 1.0 / (1.0 + consciousness_variance);

        self.harmonic_coherence = (spatial_coherence * consciousness_coherence).min(1.0);
    }

    /// Apply species-specific modulation to an audio sample
    pub fn apply_species_modulation(&self,
                                  base_sample: f32,
                                  sample_time: f64,
                                  llama_positions: &[Vec2],
                                  species_counts: &HashMap<LlamaSpecies, u32>) -> f32 {

        let mut modulated_sample = base_sample;

        // Apply each species' sonic influence
        for (species, &count) in species_counts {
            if count == 0 { continue; }

            let signature = self.species_signatures.get(species).unwrap();
            let species_influence = count as f32 / 20.0; // Normalize influence

            // Apply frequency modulation based on species characteristics
            let fm_frequency = self.fundamental_frequency * signature.base_frequency_multiplier * 0.1;
            let fm_modulation = (sample_time * fm_frequency as f64 * std::f64::consts::TAU).sin() as f32;

            let modulation_amount = signature.modulation_depth * species_influence;
            modulated_sample *= 1.0 + fm_modulation * modulation_amount;

            // Add harmonic content specific to this species
            for (i, &harmonic_ratio) in signature.harmonic_series.iter().enumerate() {
                let harmonic_freq = self.fundamental_frequency * harmonic_ratio;
                let harmonic_wave = (sample_time * harmonic_freq as f64 * std::f64::consts::TAU).sin() as f32;
                let harmonic_amplitude = species_influence / (i + 1) as f32 * 0.1;
                modulated_sample += harmonic_wave * harmonic_amplitude;
            }

            // Apply species-specific chaos
            if signature.chaos_factor > 0.0 {
                let chaos_sample = (fastrand::f32() - 0.5) * 2.0;
                modulated_sample += chaos_sample * signature.chaos_factor * species_influence * 0.05;
            }
        }

        // Apply hive mind collective harmonics
        modulated_sample = self.hive_harmonics.apply_collective_harmonics(
            modulated_sample,
            sample_time,
            self.harmonic_coherence
        );

        modulated_sample
    }

    /// Get current fundamental frequency for other systems
    pub fn get_fundamental_frequency(&self) -> f32 {
        self.fundamental_frequency
    }

    /// Get hive mind coherence level (0.0 to 1.0)
    pub fn get_hive_coherence(&self) -> f32 {
        self.harmonic_coherence
    }

    /// Get consciousness velocity for dynamic effects
    pub fn get_consciousness_velocity(&self) -> f32 {
        self.consciousness_velocity
    }
}

impl HiveMindHarmonics {
    fn new() -> Self {
        Self {
            collective_phase: 0.0,
            coherence_level: 0.0,
            harmonic_lock_strength: 0.0,
            spatial_frequency_map: Vec::new(),
            frequency_interference_patterns: Vec::new(),
            emergent_fundamentals: Vec::new(),
            harmonic_convergence_points: Vec::new(),
            consciousness_field_density: 0.0,
            field_resonance_frequencies: Vec::new(),
        }
    }

    fn update(&mut self, cosmic_time: f64, llama_data: &[LlamaRenderData]) {
        self.collective_phase += cosmic_time * 0.01;

        // Update spatial frequency mapping
        self.update_spatial_frequency_map(llama_data);

        // Calculate interference patterns from llama interactions
        self.calculate_interference_patterns(llama_data);

        // Find emergent harmonics that arise from collective behavior
        self.find_emergent_harmonics(llama_data);

        // Update consciousness field density
        self.update_consciousness_field(llama_data);
    }

    fn update_spatial_frequency_map(&mut self, llama_data: &[LlamaRenderData]) {
        self.spatial_frequency_map.clear();

        // Map each llama position to a frequency based on spatial relationships
        for (i, llama) in llama_data.iter().enumerate() {
            // Use position to generate frequency (like a spatial synthesizer)
            let freq_x = (llama.position.x / 1200.0) * 880.0 + 220.0; // Map X to frequency range
            let freq_y = (llama.position.y / 800.0) * 440.0 + 110.0;  // Map Y to frequency range

            // Combine X and Y influences using consciousness as weight
            let spatial_freq = freq_x * (1.0 - llama.trip_intensity * 0.5) +
                              freq_y * (llama.trip_intensity * 0.5);

            self.spatial_frequency_map.push(Vec2::new(spatial_freq, llama.trip_intensity));
        }
    }

    fn calculate_interference_patterns(&mut self, llama_data: &[LlamaRenderData]) {
        self.frequency_interference_patterns.clear();

        // Calculate beat frequencies between nearby llamas
        for i in 0..llama_data.len() {
            for j in (i + 1)..llama_data.len() {
                let llama1 = &llama_data[i];
                let llama2 = &llama_data[j];

                let distance = (llama1.position - llama2.position).length();

                // Llamas close together create beat frequencies
                if distance < 100.0 {
                    let freq1 = self.spatial_frequency_map[i].x;
                    let freq2 = self.spatial_frequency_map[j].x;

                    let beat_frequency = (freq1 - freq2).abs();
                    let beat_strength = 1.0 / (1.0 + distance * 0.01);

                    self.frequency_interference_patterns.push(beat_frequency * beat_strength);
                }
            }
        }
    }

    fn find_emergent_harmonics(&mut self, llama_data: &[LlamaRenderData]) {
        self.emergent_fundamentals.clear();

        if llama_data.len() < 3 {
            return;
        }

        // Look for harmonic relationships that emerge from the collective
        let frequencies: Vec<f32> = self.spatial_frequency_map.iter()
            .map(|freq_data| freq_data.x)
            .collect();

        // Find common harmonic series
        for base_freq in &frequencies {
            let mut harmonic_matches = 0;

            for freq in &frequencies {
                // Check if this frequency is a harmonic of the base frequency
                let ratio = freq / base_freq;
                let nearest_integer = ratio.round();

                if (ratio - nearest_integer).abs() < 0.1 && nearest_integer >= 1.0 {
                    harmonic_matches += 1;
                }
            }

            // If many frequencies are harmonically related, this is an emergent fundamental
            if harmonic_matches >= 3 {
                self.emergent_fundamentals.push(*base_freq);
            }
        }

        // Remove duplicates and sort
        self.emergent_fundamentals.sort_by(|a, b| a.partial_cmp(b).unwrap());
        self.emergent_fundamentals.dedup_by(|a, b| (a - b).abs() < 1.0);
    }

    fn update_consciousness_field(&mut self, llama_data: &[LlamaRenderData]) {
        // Calculate consciousness field density
        let total_consciousness: f32 = llama_data.iter()
            .map(|llama| llama.trip_intensity)
            .sum();

        let field_area = 1200.0 * 800.0; // Screen area
        self.consciousness_field_density = total_consciousness / field_area * 1000000.0;

        // Generate field resonance frequencies based on consciousness density
        self.field_resonance_frequencies.clear();

        if self.consciousness_field_density > 0.001 {
            // Higher consciousness density creates more complex resonance patterns
            let resonance_count = (self.consciousness_field_density * 10.0) as usize;

            for i in 0..resonance_count.min(16) {
                let resonance_freq = 110.0 * (i + 1) as f32 *
                    (1.0 + self.consciousness_field_density * 0.1);
                self.field_resonance_frequencies.push(resonance_freq);
            }
        }
    }

    fn apply_collective_harmonics(&self,
                                 base_sample: f32,
                                 sample_time: f64,
                                 coherence_level: f32) -> f32 {
        let mut enhanced_sample = base_sample;

        // Apply emergent harmonics
        for &fundamental in &self.emergent_fundamentals {
            let harmonic_wave = (sample_time * fundamental as f64 * std::f64::consts::TAU).sin() as f32;
            enhanced_sample += harmonic_wave * coherence_level * 0.1;
        }

        // Apply interference patterns as amplitude modulation
        for &beat_freq in &self.frequency_interference_patterns {
            if beat_freq > 0.1 && beat_freq < 20.0 { // Audible beat frequency range
                let beat_modulation = (sample_time * beat_freq as f64 * std::f64::consts::TAU).sin() as f32;
                enhanced_sample *= 1.0 + beat_modulation * coherence_level * 0.2;
            }
        }

        // Apply consciousness field resonances
        for &resonance_freq in &self.field_resonance_frequencies {
            let resonance_wave = (sample_time * resonance_freq as f64 * std::f64::consts::TAU).sin() as f32;
            enhanced_sample += resonance_wave * self.consciousness_field_density * 0.05;
        }

        // Apply collective phase modulation
        let collective_modulation = (self.collective_phase * 4.0).sin() as f32;
        enhanced_sample *= 1.0 + collective_modulation * coherence_level * 0.1;

        enhanced_sample
    }
}