// === PSYCHEDELIC SYNTHESIZER ===
// Procedural audio generation with species-specific sonic signatures
// Transforms mathematical consciousness into raw audio chaos

use std::collections::HashMap;
use glam::Vec2;
use primes::{PrimeSet, Sieve};

use super::CompatLlamaSpecies;
use crate::mathematics::BeatState;
use super::AudioEnvironment;

/// Core waveform types for psychedelic synthesis
#[derive(Debug, Clone)]
pub enum AudioWaveform {
    Sine,
    Sawtooth,
    Square,
    Triangle,
    Noise,
    ConsciousnessFractal,  // Special fractal waveform based on consciousness patterns
}

/// Multi-oscillator bank for complex timbres
pub struct OscillatorBank {
    oscillators: Vec<Oscillator>,
    mix_levels: Vec<f32>,
    detune_amounts: Vec<f32>,
}

#[derive(Debug, Clone)]
struct Oscillator {
    frequency: f32,
    phase: f32,
    waveform: AudioWaveform,
    amplitude: f32,
    consciousness_modulation: f32,
}

/// The heart of psychedelic audio generation
pub struct PsychedelicSynthesizer {
    sample_rate: f32,
    prime_sieve: Sieve,

    // Species-specific oscillator banks
    disco_llama_bank: OscillatorBank,
    quantum_sheep_bank: OscillatorBank,
    bassdrop_vicuna_bank: OscillatorBank,

    // Environment-specific synthesis parameters
    environment_configs: HashMap<AudioEnvironment, EnvironmentSynthConfig>,

    // Real-time audio state
    master_phase: f64,
    bass_accumulator: f32,
    treble_accumulator: f32,
    consciousness_frequency_base: f32,

    // Mathematical consciousness modulation
    golden_ratio_modulator: f32,
    prime_harmonic_index: usize,
    fibonacci_sequence_position: usize,

    // Chaos injection system
    chaos_intensity: f32,
    reality_break_trigger: bool,
    hive_mind_emergence_trigger: bool,
    edm_mode_active: bool,
}

#[derive(Debug, Clone)]
struct EnvironmentSynthConfig {
    base_frequency: f32,
    harmonic_count: usize,
    noise_level: f32,
    distortion_amount: f32,
    reverb_amount: f32,
    bass_boost: f32,
    treble_boost: f32,
    chaos_factor: f32,
}

impl PsychedelicSynthesizer {
    pub fn new(sample_rate: f32) -> Self {
        let mut synthesizer = Self {
            sample_rate,
            prime_sieve: Sieve::new(),
            disco_llama_bank: OscillatorBank::new_disco(),
            quantum_sheep_bank: OscillatorBank::new_quantum(),
            bassdrop_vicuna_bank: OscillatorBank::new_bassdrop(),
            environment_configs: HashMap::new(),
            master_phase: 0.0,
            bass_accumulator: 0.0,
            treble_accumulator: 0.0,
            consciousness_frequency_base: 440.0, // A4
            golden_ratio_modulator: 0.0,
            prime_harmonic_index: 0,
            fibonacci_sequence_position: 1,
            chaos_intensity: 0.0,
            reality_break_trigger: false,
            hive_mind_emergence_trigger: false,
            edm_mode_active: false,
        };

        synthesizer.initialize_environment_configs();
        synthesizer
    }

    fn initialize_environment_configs(&mut self) {
        // Environmental: Nature sounds, subtle tones
        self.environment_configs.insert(AudioEnvironment::Environmental, EnvironmentSynthConfig {
            base_frequency: 60.0,
            harmonic_count: 3,
            noise_level: 0.3,
            distortion_amount: 0.1,
            reverb_amount: 0.8,
            bass_boost: 0.2,
            treble_boost: 0.1,
            chaos_factor: 0.05,
        });

        // Meditative: New age, massage therapy vibes
        self.environment_configs.insert(AudioEnvironment::Meditative, EnvironmentSynthConfig {
            base_frequency: 110.0,
            harmonic_count: 5,
            noise_level: 0.1,
            distortion_amount: 0.05,
            reverb_amount: 0.9,
            bass_boost: 0.3,
            treble_boost: 0.2,
            chaos_factor: 0.1,
        });

        // Psychedelic: Mid-range consciousness exploration
        self.environment_configs.insert(AudioEnvironment::Psychedelic, EnvironmentSynthConfig {
            base_frequency: 220.0,
            harmonic_count: 8,
            noise_level: 0.2,
            distortion_amount: 0.3,
            reverb_amount: 0.6,
            bass_boost: 0.5,
            treble_boost: 0.4,
            chaos_factor: 0.3,
        });

        // Electronica: FULL EDM CHAOS - THE SPIRIT ANIMAL
        self.environment_configs.insert(AudioEnvironment::Electronica, EnvironmentSynthConfig {
            base_frequency: 440.0,
            harmonic_count: 16,
            noise_level: 0.4,
            distortion_amount: 0.7,
            reverb_amount: 0.4,
            bass_boost: 1.0,
            treble_boost: 0.8,
            chaos_factor: 0.6,
        });

        // HiveMind: Collective consciousness harmonics
        self.environment_configs.insert(AudioEnvironment::HiveMind, EnvironmentSynthConfig {
            base_frequency: 528.0, // "Love frequency"
            harmonic_count: 32,
            noise_level: 0.1,
            distortion_amount: 0.4,
            reverb_amount: 1.0,
            bass_boost: 0.7,
            treble_boost: 0.6,
            chaos_factor: 0.2,
        });

        // RealityTear: When consciousness breaks reality
        self.environment_configs.insert(AudioEnvironment::RealityTear, EnvironmentSynthConfig {
            base_frequency: 666.0, // Maximum chaos
            harmonic_count: 64,
            noise_level: 0.8,
            distortion_amount: 1.0,
            reverb_amount: 0.2,
            bass_boost: 1.5,
            treble_boost: 1.2,
            chaos_factor: 1.0,
        });
    }

    /// Generate a single audio sample - the core of psychedelic chaos
    pub fn generate_sample(&mut self,
                          sample_time: f64,
                          beat_state: &BeatState,
                          environment: &AudioEnvironment,
                          total_consciousness: f32,
                          species_counts: &HashMap<CompatLlamaSpecies, u32>) -> f32 {

        self.master_phase += 1.0 / self.sample_rate as f64;

        // Get environment configuration
        let config = self.environment_configs.get(environment)
            .unwrap_or(&self.environment_configs[&AudioEnvironment::Environmental])
            .clone();
        let base_frequency = config.base_frequency;

        // Calculate consciousness-driven base frequency
        let consciousness_factor = (total_consciousness / 100.0).min(2.0);
        let base_freq = base_frequency * (1.0 + consciousness_factor * 0.5);

        // Apply mathematical modulation
        let modulated_freq = self.apply_mathematical_modulation(base_freq, sample_time, beat_state);

        // Generate species-specific contributions
        let mut sample = 0.0;

        // Disco Llamas: Main melodic content
        if let Some(&disco_count) = species_counts.get(&CompatLlamaSpecies::Disco) {
            if disco_count > 0 {
                let disco_contribution = self.disco_llama_bank.generate_sample(
                    modulated_freq * 1.0, // Fundamental frequency
                    sample_time,
                    disco_count as f32 / 10.0, // Normalize count
                );
                sample += disco_contribution * 0.4;
            }
        }

        // Quantum Sheep: High-frequency consciousness harmonics
        if let Some(&quantum_count) = species_counts.get(&CompatLlamaSpecies::Quantum) {
            if quantum_count > 0 {
                let quantum_contribution = self.quantum_sheep_bank.generate_sample(
                    modulated_freq * 2.0, // Higher harmonics
                    sample_time,
                    quantum_count as f32 / 5.0,
                );
                sample += quantum_contribution * 0.3;
            }
        }

        // BassDrop Vicunas: Massive low-frequency content
        if let Some(&bass_count) = species_counts.get(&CompatLlamaSpecies::BassDrop) {
            if bass_count > 0 {
                let bass_contribution = self.bassdrop_vicuna_bank.generate_sample(
                    modulated_freq * 0.25, // Sub-bass
                    sample_time,
                    bass_count as f32 / 3.0,
                );
                sample += bass_contribution * 0.6;
            }
        }

        // Apply environment-specific processing
        sample = self.apply_environment_processing(sample, &config, beat_state);

        // Apply chaos injection
        sample = self.apply_chaos_injection(sample, &config, beat_state);

        // Handle special triggers
        sample = self.handle_special_triggers(sample, sample_time);

        // Update accumulators for analysis
        self.update_audio_analysis(sample);

        // Final limiting
        sample.clamp(-1.0, 1.0)
    }

    fn apply_mathematical_modulation(&mut self, base_freq: f32, sample_time: f64, beat_state: &BeatState) -> f32 {
        // Prime number modulation
        self.prime_harmonic_index = ((sample_time * 0.1) as usize) % 1000;
        let prime = self.prime_sieve.iter().nth(self.prime_harmonic_index).unwrap_or(2);
        let prime_factor = (prime % 12) as f32 / 12.0;

        // Golden ratio modulation
        let golden_ratio = 1.618033988749;
        self.golden_ratio_modulator = ((sample_time * golden_ratio * 0.05) % 1.0) as f32;

        // Fibonacci sequence modulation
        let fib_factor = self.get_fibonacci_factor(sample_time);

        // Combine mathematical modulations
        let math_modulation = 1.0 +
            prime_factor * 0.1 +
            self.golden_ratio_modulator * 0.05 +
            fib_factor * 0.08;

        // Apply beat state influence
        let beat_modulation = 1.0 + beat_state.prime_factor * 0.15;

        base_freq * math_modulation * beat_modulation
    }

    fn get_fibonacci_factor(&mut self, sample_time: f64) -> f32 {
        // Update Fibonacci position based on time
        let new_position = ((sample_time * 0.2) as usize) % 20; // Cycle through first 20 Fibonacci numbers
        if new_position != self.fibonacci_sequence_position {
            self.fibonacci_sequence_position = new_position;
        }

        // Generate Fibonacci number at current position
        let fib_value = self.fibonacci(self.fibonacci_sequence_position);
        (fib_value % 100) as f32 / 100.0
    }

    fn fibonacci(&self, n: usize) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => {
                let mut a = 0;
                let mut b = 1;
                for _ in 2..=n {
                    let temp = a + b;
                    a = b;
                    b = temp;
                }
                b
            }
        }
    }

    fn apply_environment_processing(&mut self, sample: f32, config: &EnvironmentSynthConfig, beat_state: &BeatState) -> f32 {
        let mut processed = sample;

        // Apply harmonic enhancement
        if beat_state.is_beat_drop {
            processed += self.generate_harmonic_content(config.base_frequency, config.harmonic_count) * 0.3;
        }

        // Add environment-specific noise
        if config.noise_level > 0.0 {
            let noise = (fastrand::f32() - 0.5) * 2.0;
            processed += noise * config.noise_level * 0.1;
        }

        // Apply bass and treble boosts
        if config.bass_boost > 0.0 {
            let bass_freq = config.base_frequency * 0.5;
            let bass_component = self.generate_sine_wave(bass_freq, self.master_phase);
            processed += bass_component * config.bass_boost * 0.2;
        }

        if config.treble_boost > 0.0 {
            let treble_freq = config.base_frequency * 4.0;
            let treble_component = self.generate_sine_wave(treble_freq, self.master_phase);
            processed += treble_component * config.treble_boost * 0.15;
        }

        processed
    }

    fn apply_chaos_injection(&mut self, sample: f32, config: &EnvironmentSynthConfig, beat_state: &BeatState) -> f32 {
        if config.chaos_factor <= 0.0 {
            return sample;
        }

        let mut chaotic = sample;

        // Chaos intensity builds with beat intensity
        self.chaos_intensity = (self.chaos_intensity + beat_state.intensity * 0.01).min(1.0);
        self.chaos_intensity *= 0.999; // Slow decay

        // Apply various chaos effects
        if self.chaos_intensity > 0.3 {
            // Frequency modulation chaos
            let fm_freq = beat_state.cosmic_frequency * 0.1;
            let fm_amount = self.chaos_intensity * config.chaos_factor;
            let fm_modulation = (self.master_phase * fm_freq as f64 * std::f64::consts::TAU).sin() as f32;
            chaotic *= 1.0 + fm_modulation * fm_amount * 0.3;
        }

        if self.chaos_intensity > 0.6 {
            // Bit crushing for digital chaos
            let bit_reduction = (self.chaos_intensity * 8.0) as i32;
            if bit_reduction > 0 {
                let quantization = 2.0_f32.powi(bit_reduction);
                chaotic = (chaotic * quantization).round() / quantization;
            }
        }

        if self.chaos_intensity > 0.8 {
            // Ring modulation for extreme chaos
            let ring_freq = beat_state.cosmic_frequency * 2.0;
            let ring_modulator = (self.master_phase * ring_freq as f64 * std::f64::consts::TAU).sin() as f32;
            chaotic *= ring_modulator.abs();
        }

        chaotic
    }

    fn handle_special_triggers(&mut self, sample: f32, sample_time: f64) -> f32 {
        let mut result = sample;

        // Reality break trigger
        if self.reality_break_trigger {
            let break_intensity = ((sample_time * 20.0).sin() * 0.5 + 0.5) as f32;
            result *= 1.0 + break_intensity * 2.0;
            result += (fastrand::f32() - 0.5) * break_intensity * 0.5;

            // Trigger fades over time
            self.reality_break_trigger = false; // Single-shot effect
        }

        // Hive mind emergence
        if self.hive_mind_emergence_trigger {
            let emergence_freq = 40.0; // Sub-bass for emergence
            let emergence_wave = (self.master_phase * emergence_freq * std::f64::consts::TAU).sin() as f32;
            result += emergence_wave * 0.8;
            self.hive_mind_emergence_trigger = false;
        }

        // EDM mode
        if self.edm_mode_active {
            // Add aggressive sidechain compression simulation
            let sidechain_freq = 4.0; // 4/4 beat
            let sidechain_env = (1.0 - ((self.master_phase * sidechain_freq * std::f64::consts::TAU).sin() as f32).abs());
            result *= 0.3 + sidechain_env * 0.7;

            // Add EDM-style supersaw
            result += self.generate_supersaw(440.0, self.master_phase) * 0.3;
        }

        result
    }

    fn generate_harmonic_content(&self, fundamental: f32, harmonic_count: usize) -> f32 {
        let mut harmonic_sum = 0.0;

        for i in 1..=harmonic_count {
            let harmonic_freq = fundamental * i as f32;
            let harmonic_amplitude = 1.0 / i as f32; // Natural harmonic rolloff
            let harmonic_wave = self.generate_sine_wave(harmonic_freq, self.master_phase);
            harmonic_sum += harmonic_wave * harmonic_amplitude;
        }

        harmonic_sum / harmonic_count as f32
    }

    fn generate_sine_wave(&self, frequency: f32, phase: f64) -> f32 {
        (phase * frequency as f64 * std::f64::consts::TAU).sin() as f32
    }

    fn generate_supersaw(&self, frequency: f32, phase: f64) -> f32 {
        let mut supersaw = 0.0;
        let detune_amounts = [-0.11, -0.06, 0.0, 0.06, 0.11, 0.17];

        for &detune in &detune_amounts {
            let detuned_freq = frequency * (1.0 + detune);
            let saw_wave = self.generate_sawtooth_wave(detuned_freq, phase);
            supersaw += saw_wave;
        }

        supersaw / detune_amounts.len() as f32
    }

    fn generate_sawtooth_wave(&self, frequency: f32, phase: f64) -> f32 {
        let cycle_phase = (phase * frequency as f64) % 1.0;
        (cycle_phase as f32 * 2.0 - 1.0)
    }

    fn update_audio_analysis(&mut self, sample: f32) {
        // Update bass and treble accumulators for analysis
        let sample_abs = sample.abs();

        // Simple frequency analysis - this could be much more sophisticated
        if sample_abs > self.bass_accumulator {
            self.bass_accumulator = sample_abs;
        } else {
            self.bass_accumulator *= 0.99;
        }

        if sample_abs > 0.1 {
            self.treble_accumulator = (self.treble_accumulator + sample_abs * 0.1).min(1.0);
        } else {
            self.treble_accumulator *= 0.995;
        }
    }

    // Public trigger methods for events
    pub fn trigger_spawn_sound(&mut self, consciousness: f32) {
        // Trigger a brief harmonic burst based on consciousness level
        let burst_intensity = consciousness.min(1.0);
        self.chaos_intensity = (self.chaos_intensity + burst_intensity * 0.2).min(1.0);
    }

    pub fn trigger_crystal_chime(&mut self) {
        // Add a brief crystalline chime sound
        // This would typically involve adding a decaying sine wave at a higher frequency
        self.treble_accumulator = (self.treble_accumulator + 0.5).min(1.0);
    }

    pub fn trigger_hive_mind_emergence(&mut self) {
        self.hive_mind_emergence_trigger = true;
    }

    pub fn trigger_reality_break(&mut self) {
        self.reality_break_trigger = true;
    }

    pub fn activate_edm_mode(&mut self) {
        self.edm_mode_active = true;
    }

    pub fn deactivate_edm_mode(&mut self) {
        self.edm_mode_active = false;
    }

    // Analysis getters
    pub fn get_bass_level(&self) -> f32 {
        self.bass_accumulator
    }

    pub fn get_treble_level(&self) -> f32 {
        self.treble_accumulator
    }
}

impl OscillatorBank {
    fn new_disco() -> Self {
        Self {
            oscillators: vec![
                Oscillator {
                    frequency: 440.0,
                    phase: 0.0,
                    waveform: AudioWaveform::Sine,
                    amplitude: 0.7,
                    consciousness_modulation: 0.2,
                },
                Oscillator {
                    frequency: 440.0 * 1.5, // Perfect fifth
                    phase: 0.0,
                    waveform: AudioWaveform::Triangle,
                    amplitude: 0.5,
                    consciousness_modulation: 0.15,
                },
                Oscillator {
                    frequency: 440.0 * 2.0, // Octave
                    phase: 0.0,
                    waveform: AudioWaveform::Sawtooth,
                    amplitude: 0.3,
                    consciousness_modulation: 0.1,
                },
            ],
            mix_levels: vec![1.0, 0.7, 0.4],
            detune_amounts: vec![0.0, 0.02, -0.015],
        }
    }

    fn new_quantum() -> Self {
        Self {
            oscillators: vec![
                Oscillator {
                    frequency: 880.0, // Higher base frequency
                    phase: 0.0,
                    waveform: AudioWaveform::ConsciousnessFractal,
                    amplitude: 0.6,
                    consciousness_modulation: 0.5,
                },
                Oscillator {
                    frequency: 1320.0,
                    phase: 0.0,
                    waveform: AudioWaveform::Square,
                    amplitude: 0.3,
                    consciousness_modulation: 0.3,
                },
            ],
            mix_levels: vec![1.0, 0.6],
            detune_amounts: vec![0.0, 0.05],
        }
    }

    fn new_bassdrop() -> Self {
        Self {
            oscillators: vec![
                Oscillator {
                    frequency: 55.0, // Low A
                    phase: 0.0,
                    waveform: AudioWaveform::Sawtooth,
                    amplitude: 1.0,
                    consciousness_modulation: 0.1,
                },
                Oscillator {
                    frequency: 110.0, // Octave up
                    phase: 0.0,
                    waveform: AudioWaveform::Square,
                    amplitude: 0.8,
                    consciousness_modulation: 0.05,
                },
                Oscillator {
                    frequency: 27.5, // Sub-bass
                    phase: 0.0,
                    waveform: AudioWaveform::Sine,
                    amplitude: 1.2,
                    consciousness_modulation: 0.02,
                },
            ],
            mix_levels: vec![1.0, 0.8, 1.5],
            detune_amounts: vec![0.0, 0.01, 0.0],
        }
    }

    fn generate_sample(&mut self, base_frequency: f32, sample_time: f64, intensity: f32) -> f32 {
        let mut sample = 0.0;

        // Copy data needed during mutable iteration
        let detune_amounts = self.detune_amounts.clone();
        let mix_levels = self.mix_levels.clone();
        let oscillator_count = self.oscillators.len();

        for (i, oscillator) in self.oscillators.iter_mut().enumerate() {
            let detuned_freq = base_frequency * (1.0 + detune_amounts[i]);
            let mix_level = mix_levels[i] * intensity;

            let osc_sample = Self::generate_oscillator_sample(oscillator, detuned_freq, sample_time);
            sample += osc_sample * mix_level;
        }

        sample / oscillator_count as f32
    }

    fn generate_oscillator_sample(oscillator: &mut Oscillator, frequency: f32, sample_time: f64) -> f32 {
        let phase = sample_time * frequency as f64 * std::f64::consts::TAU;

        (match oscillator.waveform {
            AudioWaveform::Sine => (phase).sin() as f32,
            AudioWaveform::Sawtooth => {
                let cycle_phase = phase % std::f64::consts::TAU;
                ((cycle_phase / std::f64::consts::PI) - 1.0) as f32
            },
            AudioWaveform::Square => {
                (if (phase % std::f64::consts::TAU) < std::f64::consts::PI { 1.0 } else { -1.0 }) as f32
            },
            AudioWaveform::Triangle => {
                let cycle_phase = (phase % std::f64::consts::TAU) / std::f64::consts::TAU;
                if cycle_phase < 0.5 {
                    (cycle_phase * 4.0 - 1.0) as f32
                } else {
                    (3.0 - cycle_phase * 4.0) as f32
                }
            },
            AudioWaveform::Noise => (fastrand::f32() - 0.5) * 2.0,
            AudioWaveform::ConsciousnessFractal => {
                // Special fractal waveform that changes based on consciousness
                let fractal_phase = phase * (1.0 + oscillator.consciousness_modulation as f64);
                let base_wave = (fractal_phase).sin();
                let harmonic_wave = (fractal_phase * 3.0).sin() * 0.3;
                let chaos_wave = (fractal_phase * 7.0).sin() * 0.1;
                (base_wave + harmonic_wave + chaos_wave) as f32 * 0.6
            },
        }) * oscillator.amplitude
    }
}