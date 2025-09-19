// === PSYCHEDELIC SYNTHESIZER ===
// Procedural audio generation with species-specific sonic signatures
// Transforms mathematical consciousness into raw audio chaos

use std::collections::HashMap;
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

        // Electronica: Energizing EDM with musical structure
        self.environment_configs.insert(AudioEnvironment::Electronica, EnvironmentSynthConfig {
            base_frequency: 440.0,
            harmonic_count: 16,
            noise_level: 0.15, // Reduced harsh noise
            distortion_amount: 0.3, // Gentler distortion
            reverb_amount: 0.4,
            bass_boost: 1.0,
            treble_boost: 0.8,
            chaos_factor: 0.3, // Reduced chaos for musicality
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

        // RealityTear: Intense but musical consciousness expansion
        self.environment_configs.insert(AudioEnvironment::RealityTear, EnvironmentSynthConfig {
            base_frequency: 432.0, // Changed to healing frequency
            harmonic_count: 32, // Reduced from 64
            noise_level: 0.2, // Drastically reduced
            distortion_amount: 0.4, // Much gentler
            reverb_amount: 0.6, // More reverb for spaciousness
            bass_boost: 0.8, // Reduced
            treble_boost: 0.6, // Reduced
            chaos_factor: 0.4, // Much reduced chaos
        });
    }

    /// Generate a single audio sample using procedural mood music generation
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

        // Generate procedural mood music based on environment
        let mood_music = self.generate_mood_music(modulated_freq, sample_time, environment, total_consciousness, beat_state);

        // Generate species-specific contributions
        let mut sample = mood_music;

        // Species add subtle accents to the mood music (reduced levels)

        // Disco Llamas: Melodic accents
        if let Some(&disco_count) = species_counts.get(&CompatLlamaSpecies::Disco) {
            if disco_count > 0 {
                let disco_contribution = self.disco_llama_bank.generate_sample(
                    modulated_freq * 1.0, // Fundamental frequency
                    sample_time,
                    disco_count as f32 / 10.0, // Normalize count
                );
                sample += disco_contribution * 0.15; // Reduced from 0.4
            }
        }

        // Quantum Sheep: High-frequency harmonics
        if let Some(&quantum_count) = species_counts.get(&CompatLlamaSpecies::Quantum) {
            if quantum_count > 0 {
                let quantum_contribution = self.quantum_sheep_bank.generate_sample(
                    modulated_freq * 2.0, // Higher harmonics
                    sample_time,
                    quantum_count as f32 / 5.0,
                );
                sample += quantum_contribution * 0.1; // Reduced from 0.3
            }
        }

        // BassDrop Vicunas: Bass enhancement
        if let Some(&bass_count) = species_counts.get(&CompatLlamaSpecies::BassDrop) {
            if bass_count > 0 {
                let bass_contribution = self.bassdrop_vicuna_bank.generate_sample(
                    modulated_freq * 0.25, // Sub-bass
                    sample_time,
                    bass_count as f32 / 3.0,
                );
                sample += bass_contribution * 0.2; // Reduced from 0.6
            }
        }

        // Apply environment-specific processing
        sample = self.apply_environment_processing(sample, &config, beat_state);

        // Apply musical enhancement effects
        sample = self.apply_musical_effects(sample, &config, beat_state);

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

        // Replace random noise with structured harmonic texture
        if config.noise_level > 0.0 {
            let musical_texture = self.generate_musical_texture(config.base_frequency, config.noise_level);
            processed += musical_texture * 0.1;
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

    fn apply_musical_effects(&mut self, sample: f32, config: &EnvironmentSynthConfig, beat_state: &BeatState) -> f32 {
        if config.chaos_factor <= 0.0 {
            return sample;
        }

        let mut musical = sample;

        // Transform chaos intensity into musical expression intensity
        self.chaos_intensity = (self.chaos_intensity + beat_state.intensity * 0.01).min(1.0);
        self.chaos_intensity *= 0.999; // Slow decay

        // Apply musical effects based on intensity
        if self.chaos_intensity > 0.3 {
            // Musical vibrato instead of chaotic FM
            let vibrato_freq = 5.0; // 5Hz vibrato rate
            let vibrato_amount = self.chaos_intensity * config.chaos_factor * 0.02; // Gentle modulation
            let vibrato_modulation = (self.master_phase * vibrato_freq as f64 * std::f64::consts::TAU).sin() as f32;
            musical *= 1.0 + vibrato_modulation * vibrato_amount;
        }

        if self.chaos_intensity > 0.6 {
            // Gentle chorus effect instead of bit crushing
            let _chorus_delay = 0.02; // 20ms delay (unused in simplified implementation)
            let chorus_rate = 0.7; // Slow LFO
            let chorus_depth = 0.05 * config.chaos_factor;
            let chorus_lfo = (self.master_phase * chorus_rate * std::f64::consts::TAU).sin() as f32;
            let chorus_delayed = musical * (1.0 + chorus_lfo * chorus_depth);
            musical = musical * 0.7 + chorus_delayed * 0.3;
        }

        if self.chaos_intensity > 0.8 {
            // Harmonic enhancement instead of harsh ring modulation
            let harmonic_freq = beat_state.cosmic_frequency * 1.5; // Perfect fifth
            let harmonic_modulator = (self.master_phase * harmonic_freq as f64 * std::f64::consts::TAU).sin() as f32;
            musical += harmonic_modulator * config.chaos_factor * 0.2;
        }

        musical
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
            let sidechain_env = 1.0 - ((self.master_phase * sidechain_freq * std::f64::consts::TAU).sin() as f32).abs();
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

    fn generate_musical_texture(&self, base_frequency: f32, texture_intensity: f32) -> f32 {
        // Generate structured harmonic texture instead of random noise
        let mut texture = 0.0;

        // Add harmonic overtones for richness
        for i in 1..=5 {
            let harmonic_freq = base_frequency * i as f32;
            let harmonic_amplitude = texture_intensity / (i as f32 * i as f32); // Natural rolloff
            let harmonic_phase = self.master_phase * harmonic_freq as f64 * std::f64::consts::TAU;
            texture += (harmonic_phase.sin() as f32) * harmonic_amplitude;
        }

        // Add gentle amplitude modulation for organic feel
        let am_freq = 0.3; // Slow breathing rate
        let am_depth = texture_intensity * 0.3;
        let am_modulator = (self.master_phase * am_freq * std::f64::consts::TAU).sin() as f32;
        texture *= 1.0 + am_modulator * am_depth;

        texture * 0.1 // Keep it subtle
    }

    /// Generate ambient mood music using Brian Eno-inspired techniques
    fn generate_mood_music(&mut self, _base_freq: f32, sample_time: f64, environment: &AudioEnvironment, consciousness: f32, _beat_state: &BeatState) -> f32 {
        match environment {
            AudioEnvironment::Environmental | AudioEnvironment::Meditative => {
                self.generate_eno_ambient_mellow(sample_time, consciousness)
            },
            AudioEnvironment::Psychedelic | AudioEnvironment::Electronica => {
                self.generate_eno_ambient_active(sample_time, consciousness)
            },
            AudioEnvironment::HiveMind | AudioEnvironment::RealityTear => {
                self.generate_eno_ambient_chaotic(sample_time, consciousness)
            },
        }
    }

    /// Mellow Mood: Dorian mode, slow tempo (60-80 BPM), gentle harmonics
    fn generate_eno_ambient_mellow(&mut self, sample_time: f64, consciousness: f32) -> f32 {
        // C Major scale frequencies (pleasant, familiar, soothing)
        let c_major_freqs = [261.63, 293.66, 329.63, 349.23, 392.00, 440.00, 493.88]; // C4-B4

        // Brian Eno's incommensurable loop technique - different loop lengths that slowly drift
        let loop1_period = 23.5; // seconds
        let loop2_period = 25.875; // seconds
        let loop3_period = 29.916; // seconds

        // Select notes based on loop positions (creates slowly evolving combinations)
        let note1_index = ((sample_time / loop1_period) as usize) % c_major_freqs.len();
        let note2_index = ((sample_time / loop2_period) as usize) % c_major_freqs.len();
        let note3_index = ((sample_time / loop3_period) as usize) % c_major_freqs.len();

        // Generate simple sine wave tones
        let tone1 = (sample_time * c_major_freqs[note1_index] * std::f64::consts::TAU).sin() as f32;
        let tone2 = (sample_time * c_major_freqs[note2_index] * std::f64::consts::TAU).sin() as f32;
        let tone3 = (sample_time * c_major_freqs[note3_index] * std::f64::consts::TAU).sin() as f32;

        // Gentle volume balancing (Eno's technique)
        let mix = tone1 * 0.4 + tone2 * 0.3 + tone3 * 0.2;

        // Subtle consciousness influence on overall amplitude
        let consciousness_envelope = 0.3 + consciousness * 0.2; // Very gentle, stays mellow

        mix * consciousness_envelope * 0.3 // Keep volume low for ambient background
    }

    /// Active Mood: Major modes (Ionian/Mixolydian), dynamic tempo (120-140 BPM)
    fn generate_eno_ambient_active(&mut self, sample_time: f64, consciousness: f32) -> f32 {
        // A Minor scale frequencies (more active, slightly more complex)
        let a_minor_freqs = [440.00, 493.88, 523.25, 587.33, 659.25, 698.46, 783.99]; // A4-G5

        // Faster, more active loop periods
        let loop1_period = 18.75; // seconds
        let loop2_period = 21.333; // seconds
        let loop3_period = 24.125; // seconds
        let loop4_period = 27.666; // seconds - extra layer for more activity

        // Select notes based on loop positions
        let note1_index = ((sample_time / loop1_period) as usize) % a_minor_freqs.len();
        let note2_index = ((sample_time / loop2_period) as usize) % a_minor_freqs.len();
        let note3_index = ((sample_time / loop3_period) as usize) % a_minor_freqs.len();
        let note4_index = ((sample_time / loop4_period) as usize) % a_minor_freqs.len();

        // Generate simple sine wave tones
        let tone1 = (sample_time * a_minor_freqs[note1_index] * std::f64::consts::TAU).sin() as f32;
        let tone2 = (sample_time * a_minor_freqs[note2_index] * std::f64::consts::TAU).sin() as f32;
        let tone3 = (sample_time * a_minor_freqs[note3_index] * std::f64::consts::TAU).sin() as f32;
        let tone4 = (sample_time * a_minor_freqs[note4_index] * std::f64::consts::TAU).sin() as f32;

        // Active volume balancing with more layers
        let mix = tone1 * 0.35 + tone2 * 0.3 + tone3 * 0.25 + tone4 * 0.2;

        // More responsive to consciousness for active feel
        let consciousness_envelope = 0.4 + consciousness * 0.4;

        mix * consciousness_envelope * 0.4 // Slightly louder than mellow
    }

    /// Chaotic Mood: Controlled mathematical chaos, complex polyrhythms
    fn generate_eno_ambient_chaotic(&mut self, sample_time: f64, consciousness: f32) -> f32 {
        // Pentatonic scale frequencies (can't sound bad, but more unpredictable)
        let pentatonic_freqs = [261.63, 293.66, 329.63, 392.00, 440.00]; // C D E G A

        // Complex but still musical loop periods (prime numbers for maximum drift)
        let loop1_period = 17.0; // seconds
        let loop2_period = 19.0; // seconds
        let loop3_period = 23.0; // seconds
        let loop4_period = 29.0; // seconds
        let loop5_period = 31.0; // seconds - 5 layers for controlled chaos

        // Select notes based on loop positions
        let note1_index = ((sample_time / loop1_period) as usize) % pentatonic_freqs.len();
        let note2_index = ((sample_time / loop2_period) as usize) % pentatonic_freqs.len();
        let note3_index = ((sample_time / loop3_period) as usize) % pentatonic_freqs.len();
        let note4_index = ((sample_time / loop4_period) as usize) % pentatonic_freqs.len();
        let note5_index = ((sample_time / loop5_period) as usize) % pentatonic_freqs.len();

        // Generate simple sine wave tones
        let tone1 = (sample_time * pentatonic_freqs[note1_index] * std::f64::consts::TAU).sin() as f32;
        let tone2 = (sample_time * pentatonic_freqs[note2_index] * std::f64::consts::TAU).sin() as f32;
        let tone3 = (sample_time * pentatonic_freqs[note3_index] * std::f64::consts::TAU).sin() as f32;
        let tone4 = (sample_time * pentatonic_freqs[note4_index] * std::f64::consts::TAU).sin() as f32;
        let tone5 = (sample_time * pentatonic_freqs[note5_index] * std::f64::consts::TAU).sin() as f32;

        // Complex but balanced volume mixing
        let mix = tone1 * 0.3 + tone2 * 0.25 + tone3 * 0.2 + tone4 * 0.15 + tone5 * 0.1;

        // Highly responsive to consciousness for chaotic but controlled feel
        let consciousness_envelope = 0.5 + consciousness * 0.5;

        mix * consciousness_envelope * 0.5 // Loudest mode but still ambient
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
        cycle_phase as f32 * 2.0 - 1.0
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