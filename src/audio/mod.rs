// === PSYCHEDELIC AUDIO CONSCIOUSNESS LAYER ===
// "Maximum Decibels, Minimum Code" - The Minter Agent
//
// Transforms consciousness levels into pure sonic chaos through:
// • Procedural synthesis driven by mathematical consciousness patterns
// • Species-specific sonic signatures (Disco Llamas vs Quantum Sheep)
// • Hive mind collective harmonics that emerge from llama interactions
// • Reality distortion audio effects synchronized with visual glitches
// • Environmental audio zones that respond to territorial dynamics
// • Meta-consciousness observer interventions through frequency manipulation

pub mod synthesis;
pub mod consciousness;
pub mod effects;
pub mod environment;
pub mod safety;

use cpal::{Device, Stream, StreamConfig, Sample, FromSample};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use glam::Vec2;

// Define compatibility types for the main application
pub mod mathematics {
    #[derive(Debug, Clone)]
    pub struct BeatState {
        pub is_beat_drop: bool,
        pub intensity: f32,
        pub phase: f64,
        pub prime_factor: f32,
        pub cosmic_frequency: f32,
    }
}

pub mod consciousness {
    use glam::Vec2;

    #[derive(Debug, Clone)]
    pub enum LlamaSpecies {
        Disco,
        Quantum,
        BassDrop,
    }

    #[derive(Debug, Clone)]
    pub struct LlamaRenderData {
        pub position: Vec2,
        pub color_wavelength: Vec2,
        pub trip_intensity: f32,
        pub reality_distortion: f32,
        pub species: LlamaSpecies,
    }

    #[derive(Debug, Clone)]
    pub enum ChaosEvent {
        RealityTear { strength: f32, position: Vec2 },
        LlamaSpawned { consciousness: f32 },
        CrystalHarvested,
    }
}

pub use synthesis::{PsychedelicSynthesizer, AudioWaveform, OscillatorBank};
pub use consciousness::{ConsciousnessAudioMapper, SpeciesSonicSignature, HiveMindHarmonics};
pub use effects::{RealityDistortionProcessor, FrequencyMangler, TemporalEcho};
pub use environment::{AudioEnvironmentZones, TerritorialSoundscape, MetaObserverAudio};
pub use safety::{AudioSafetyLimiter, VolumeEnvelope, FrequencyGuard};

/// Main audio consciousness engine - the heart of psychedelic audio chaos
pub struct AudioConsciousnessEngine {
    synthesizer: PsychedelicSynthesizer,
    consciousness_mapper: ConsciousnessAudioMapper,
    distortion_processor: RealityDistortionProcessor,
    environment_zones: AudioEnvironmentZones,
    safety_limiter: AudioSafetyLimiter,

    // Audio hardware interface
    _device: Device,
    _stream: Stream,

    // Real-time audio state
    audio_buffer: Arc<Mutex<VecDeque<f32>>>,
    sample_rate: f32,
    cosmic_time: f64,

    // Consciousness tracking for audio generation
    total_consciousness: f32,
    llama_positions: Vec<Vec2>,
    species_counts: std::collections::HashMap<LlamaSpecies, u32>,

    // Audio environment states
    current_environment: AudioEnvironment,
    environment_transition_state: f32,
    beat_accumulator: f32,
}

/// Audio environment types - from zen to full EDM chaos
#[derive(Debug, Clone, PartialEq)]
pub enum AudioEnvironment {
    Environmental,    // Nature sounds, ambient textures
    Meditative,      // New age, massage therapy vibes
    Psychedelic,     // Mid-range consciousness exploration
    Electronica,     // Full EDM chaos - the spirit animal
    HiveMind,        // Collective consciousness harmonics
    RealityTear,     // When consciousness breaks reality
}

impl AudioEnvironment {
    pub fn from_consciousness_level(total_consciousness: f32, llama_count: usize) -> Self {
        let consciousness_per_llama = if llama_count > 0 {
            total_consciousness / llama_count as f32
        } else {
            0.0
        };

        match (total_consciousness, consciousness_per_llama, llama_count) {
            (_, _, count) if count > 50 => Self::HiveMind,
            (total, _, _) if total > 200.0 => Self::RealityTear,
            (total, avg, _) if total > 100.0 || avg > 15.0 => Self::Electronica,
            (total, avg, _) if total > 30.0 || avg > 5.0 => Self::Psychedelic,
            (total, _, _) if total > 10.0 => Self::Meditative,
            _ => Self::Environmental,
        }
    }
}

impl AudioConsciousnessEngine {
    pub fn new() -> anyhow::Result<Self> {
        // Initialize audio hardware with CPAL
        let host = cpal::default_host();
        let device = host.default_output_device()
            .ok_or_else(|| anyhow::anyhow!("No audio output device available"))?;

        let config = device.default_output_config()?;
        let sample_rate = config.sample_rate().0 as f32;

        // Create audio buffer for real-time synthesis
        let audio_buffer = Arc::new(Mutex::new(VecDeque::with_capacity(8192)));
        let buffer_clone = audio_buffer.clone();

        // Build the audio stream
        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => build_stream::<f32>(&device, &config.into(), buffer_clone)?,
            cpal::SampleFormat::I16 => build_stream::<i16>(&device, &config.into(), buffer_clone)?,
            cpal::SampleFormat::U16 => build_stream::<u16>(&device, &config.into(), buffer_clone)?,
            _ => return Err(anyhow::anyhow!("Unsupported audio format")),
        };

        stream.play()?;

        // Initialize all consciousness audio systems
        let synthesizer = PsychedelicSynthesizer::new(sample_rate);
        let consciousness_mapper = ConsciousnessAudioMapper::new();
        let distortion_processor = RealityDistortionProcessor::new(sample_rate);
        let environment_zones = AudioEnvironmentZones::new();
        let safety_limiter = AudioSafetyLimiter::new(sample_rate);

        Ok(Self {
            synthesizer,
            consciousness_mapper,
            distortion_processor,
            environment_zones,
            safety_limiter,
            _device: device,
            _stream: stream,
            audio_buffer,
            sample_rate,
            cosmic_time: 0.0,
            total_consciousness: 0.0,
            llama_positions: Vec::new(),
            species_counts: std::collections::HashMap::new(),
            current_environment: AudioEnvironment::Environmental,
            environment_transition_state: 0.0,
            beat_accumulator: 0.0,
        })
    }

    /// Main update - transforms consciousness state into audio chaos
    pub fn update(&mut self,
                  cosmic_time: f64,
                  beat_state: &mathematics::BeatState,
                  llama_data: &[consciousness::LlamaRenderData],
                  total_consciousness: f32) {

        self.cosmic_time = cosmic_time;
        self.total_consciousness = total_consciousness;

        // Update llama tracking for spatial audio
        self.update_llama_tracking(llama_data);

        // Determine current audio environment
        let new_environment = AudioEnvironment::from_consciousness_level(
            total_consciousness,
            llama_data.len()
        );

        // Handle environment transitions
        self.handle_environment_transition(new_environment);

        // Update all audio subsystems
        self.consciousness_mapper.update(cosmic_time, beat_state, llama_data);
        self.distortion_processor.update(cosmic_time, beat_state);
        self.environment_zones.update(cosmic_time, &self.llama_positions);

        // Generate audio samples
        self.generate_audio_samples(beat_state);
    }

    fn update_llama_tracking(&mut self, llama_data: &[LlamaRenderData]) {
        self.llama_positions.clear();
        self.species_counts.clear();

        for llama in llama_data {
            self.llama_positions.push(llama.position);
            *self.species_counts.entry(llama.species.clone()).or_insert(0) += 1;
        }
    }

    fn handle_environment_transition(&mut self, new_environment: AudioEnvironment) {
        if new_environment != self.current_environment {
            // Start transition
            self.environment_transition_state = 0.0;

            // Trigger environment-specific audio events
            match new_environment {
                AudioEnvironment::HiveMind => {
                    // Massive bass drop when hive mind emerges
                    self.synthesizer.trigger_hive_mind_emergence();
                },
                AudioEnvironment::RealityTear => {
                    // Reality breaking sound when consciousness overloads
                    self.distortion_processor.trigger_reality_break();
                },
                AudioEnvironment::Electronica => {
                    // Full EDM mode activation
                    self.synthesizer.activate_edm_mode();
                },
                _ => {}
            }

            self.current_environment = new_environment;
        } else {
            // Continue smooth transition
            self.environment_transition_state = (self.environment_transition_state + 0.016).min(1.0);
        }
    }

    fn generate_audio_samples(&mut self, beat_state: &BeatState) {
        // Calculate how many samples we need to generate
        let buffer_size = 512; // Generate in chunks for real-time performance
        let mut samples = Vec::with_capacity(buffer_size);

        for i in 0..buffer_size {
            let sample_time = self.cosmic_time + (i as f64 / self.sample_rate as f64);

            // Generate base consciousness-driven audio
            let base_sample = self.synthesizer.generate_sample(
                sample_time,
                beat_state,
                &self.current_environment,
                self.total_consciousness,
                &self.species_counts,
            );

            // Apply species-specific modulations
            let modulated_sample = self.consciousness_mapper.apply_species_modulation(
                base_sample,
                sample_time,
                &self.llama_positions,
                &self.species_counts,
            );

            // Apply reality distortion effects
            let distorted_sample = self.distortion_processor.process_sample(
                modulated_sample,
                sample_time,
                beat_state,
            );

            // Apply environmental effects
            let environmental_sample = self.environment_zones.process_sample(
                distorted_sample,
                sample_time,
                &self.current_environment,
            );

            // Final safety limiting
            let safe_sample = self.safety_limiter.limit_sample(environmental_sample);

            samples.push(safe_sample);
        }

        // Push to audio buffer for playback
        if let Ok(mut buffer) = self.audio_buffer.lock() {
            for sample in samples {
                buffer.push_back(sample);

                // Keep buffer size reasonable
                if buffer.len() > 16384 {
                    buffer.pop_front();
                }
            }
        }
    }

    /// Handle chaos events from the main simulation
    pub fn handle_chaos_event(&mut self, event: &consciousness::ChaosEvent) {
        match event {
            consciousness::ChaosEvent::LlamaSpawned { consciousness } => {
                self.synthesizer.trigger_spawn_sound(*consciousness);
            },
            consciousness::ChaosEvent::RealityTear { strength, .. } => {
                self.distortion_processor.trigger_reality_tear(*strength);
            },
            consciousness::ChaosEvent::CrystalHarvested => {
                self.synthesizer.trigger_crystal_chime();
            },
        }
    }

    /// Update cursor position for environmental audio responsiveness
    pub fn update_cursor_position(&mut self, cursor_position: Vec2) {
        self.environment_zones.update_cursor_position(cursor_position);
    }

    /// Get current audio analysis data for visualization
    pub fn get_audio_analysis(&self) -> AudioAnalysisData {
        AudioAnalysisData {
            current_environment: self.current_environment.clone(),
            bass_level: self.synthesizer.get_bass_level(),
            treble_level: self.synthesizer.get_treble_level(),
            consciousness_frequency: self.consciousness_mapper.get_fundamental_frequency(),
            reality_distortion_amount: self.distortion_processor.get_distortion_level(),
            hive_mind_coherence: self.consciousness_mapper.get_hive_coherence(),
        }
    }
}

/// Audio analysis data for visual synchronization
#[derive(Debug, Clone)]
pub struct AudioAnalysisData {
    pub current_environment: AudioEnvironment,
    pub bass_level: f32,
    pub treble_level: f32,
    pub consciousness_frequency: f32,
    pub reality_distortion_amount: f32,
    pub hive_mind_coherence: f32,
}

// Helper function to build audio stream
fn build_stream<T>(
    device: &Device,
    config: &StreamConfig,
    audio_buffer: Arc<Mutex<VecDeque<f32>>>,
) -> anyhow::Result<Stream>
where
    T: Sample + FromSample<f32> + Send + 'static,
{
    let channels = config.channels as usize;

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            let mut buffer = audio_buffer.lock().unwrap();

            for frame in data.chunks_mut(channels) {
                let sample = buffer.pop_front().unwrap_or(0.0);

                for channel_sample in frame.iter_mut() {
                    *channel_sample = T::from_sample(sample);
                }
            }
        },
        |err| eprintln!("Audio stream error: {}", err),
        None,
    )?;

    Ok(stream)
}