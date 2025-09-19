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
// pub mod consciousness; // Temporarily disabled due to type conflicts
pub mod effects;
pub mod environment;
pub mod safety;

use cpal::{Device, Stream, StreamConfig, Sample, FromSample};
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use glam::Vec2;

// Use local BeatState for audio processing
use crate::mathematics::beat_engine::BeatState;

// Define compatibility types locally

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum CompatLlamaSpecies {
    Disco,
    Quantum,
    BassDrop,
}

#[derive(Debug, Clone)]
pub struct CompatLlamaRenderData {
    pub position: Vec2,
    pub color_wavelength: Vec2,
    pub trip_intensity: f32,
    pub reality_distortion: f32,
    pub species: CompatLlamaSpecies,
}

#[derive(Debug, Clone)]
pub enum CompatChaosEvent {
    RealityTear { strength: f32, position: Vec2 },
    LlamaSpawned { consciousness: f32 },
    CrystalHarvested,
}

pub use synthesis::{PsychedelicSynthesizer, AudioWaveform, OscillatorBank};
pub use effects::{RealityDistortionProcessor, FrequencyMangler, TemporalEcho};
pub use environment::{AudioEnvironmentZones, MetaObserverAudio};
pub use safety::{AudioSafetyLimiter, VolumeEnvelope, FrequencyGuard};

// Temporarily disable complex audio modules due to type conflicts
// pub use consciousness::{ConsciousnessAudioMapper, SpeciesSonicSignature, HiveMindHarmonics};

// Export new user control types - they will be defined below

// Temporary stub for consciousness mapper
pub struct ConsciousnessAudioMapper;

impl ConsciousnessAudioMapper {
    pub fn new() -> Self { Self }
    pub fn update(&mut self, _time: f64, _beat: &BeatState, _llamas: &[CompatLlamaRenderData]) {}
    pub fn apply_species_modulation(&mut self, sample: f32, _time: f64, _positions: &[Vec2], _counts: &std::collections::HashMap<CompatLlamaSpecies, u32>) -> f32 { sample }
    pub fn get_fundamental_frequency(&self) -> f32 { 440.0 }
    pub fn get_hive_coherence(&self) -> f32 { 0.5 }
}

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

    // User controls for audio parameters
    controls: AudioControls,

    // Consciousness tracking for audio generation
    total_consciousness: f32,
    llama_positions: Vec<Vec2>,
    species_counts: std::collections::HashMap<CompatLlamaSpecies, u32>,

    // Audio environment states
    current_environment: AudioEnvironment,
    environment_transition_state: f32,
    beat_accumulator: f32,
}

/// User-controllable audio modes that change synthesis and effects
#[derive(Debug, Clone, PartialEq)]
pub enum AudioMode {
    Mellow,   // Gentle, ambient synthesis with soft effects
    Active,   // Balanced synthesis with moderate chaos
    Chaotic,  // Extreme synthesis with maximum reality distortion
}

impl AudioMode {
    pub fn from_key_char(c: char) -> Option<Self> {
        match c.to_ascii_lowercase() {
            'm' => Some(Self::Mellow),
            'a' => Some(Self::Active),
            'c' => Some(Self::Chaotic),
            _ => None,
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            Self::Mellow => "MELLOW",
            Self::Active => "ACTIVE",
            Self::Chaotic => "CHAOTIC",
        }
    }
}

impl Default for AudioMode {
    fn default() -> Self {
        Self::Active
    }
}

/// User-controllable audio parameters
#[derive(Debug, Clone)]
pub struct AudioControls {
    pub mode: AudioMode,
    pub volume: f32,        // 0.0 to 1.0
    pub speed: f32,         // 0.1 to 3.0 (speed multiplier)
    pub enabled: bool,      // Master audio on/off
}

impl Default for AudioControls {
    fn default() -> Self {
        Self {
            mode: AudioMode::default(),
            volume: 0.7,      // 70% default volume
            speed: 1.0,       // Normal speed
            enabled: true,    // Audio enabled by default
        }
    }
}

impl AudioControls {
    pub fn adjust_volume(&mut self, delta: f32) {
        self.volume = (self.volume + delta).clamp(0.0, 1.0);
    }

    pub fn adjust_speed(&mut self, delta: f32) {
        self.speed = (self.speed + delta).clamp(0.1, 3.0);
    }

    pub fn toggle_enabled(&mut self) {
        self.enabled = !self.enabled;
    }

    pub fn set_mode(&mut self, mode: AudioMode) {
        self.mode = mode;
    }
}

/// Audio environment types - from zen to full EDM chaos
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
            controls: AudioControls::default(),
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
                  beat_state: &BeatState,
                  llama_data: &[CompatLlamaRenderData],
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

    fn update_llama_tracking(&mut self, llama_data: &[CompatLlamaRenderData]) {
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
        // If audio is disabled, generate silence
        if !self.controls.enabled {
            if let Ok(mut buffer) = self.audio_buffer.lock() {
                for _ in 0..512 {
                    buffer.push_back(0.0);
                    if buffer.len() > 16384 {
                        buffer.pop_front();
                    }
                }
            }
            return;
        }

        // Calculate how many samples we need to generate
        let buffer_size = 512; // Generate in chunks for real-time performance
        let mut samples = Vec::with_capacity(buffer_size);

        // Apply user speed control to time progression
        let speed_factor = self.controls.speed;
        let effective_environment = self.get_effective_environment_for_mode();

        for i in 0..buffer_size {
            let base_sample_time = self.cosmic_time + (i as f64 / self.sample_rate as f64);
            let sample_time = base_sample_time * speed_factor as f64;

            // Generate base consciousness-driven audio
            let base_sample = self.synthesizer.generate_sample(
                sample_time,
                beat_state,
                &effective_environment,
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

            // Apply reality distortion effects with mode-based intensity
            let distortion_intensity = self.get_distortion_intensity_for_mode();
            let distorted_sample = self.distortion_processor.process_sample_with_intensity(
                modulated_sample,
                sample_time,
                beat_state,
                distortion_intensity,
            );

            // Apply environmental effects
            let environmental_sample = self.environment_zones.process_sample(
                distorted_sample,
                sample_time,
                &effective_environment,
            );

            // Apply user volume control
            let volume_adjusted = environmental_sample * self.controls.volume;

            // Final safety limiting
            let safe_sample = self.safety_limiter.limit_sample(volume_adjusted);

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
    pub fn handle_chaos_event(&mut self, event: &CompatChaosEvent) {
        match event {
            CompatChaosEvent::LlamaSpawned { consciousness } => {
                self.synthesizer.trigger_spawn_sound(*consciousness);
            },
            CompatChaosEvent::RealityTear { strength, .. } => {
                self.distortion_processor.trigger_reality_tear(*strength);
            },
            CompatChaosEvent::CrystalHarvested => {
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

    /// User control methods
    pub fn get_controls(&self) -> &AudioControls {
        &self.controls
    }

    pub fn get_controls_mut(&mut self) -> &mut AudioControls {
        &mut self.controls
    }

    pub fn set_audio_mode(&mut self, mode: AudioMode) {
        self.controls.set_mode(mode);
        println!("🎵 Audio mode changed to: {}", mode.to_string());
    }

    pub fn adjust_volume(&mut self, delta: f32) {
        self.controls.adjust_volume(delta);
        println!("🔊 Volume: {:.0}%", self.controls.volume * 100.0);
    }

    pub fn adjust_speed(&mut self, delta: f32) {
        self.controls.adjust_speed(delta);
        println!("⚡ Speed: {:.1}x", self.controls.speed);
    }

    pub fn toggle_audio(&mut self) {
        self.controls.toggle_enabled();
        let status = if self.controls.enabled { "ENABLED" } else { "DISABLED" };
        println!("🎵 Audio: {}", status);
    }

    /// Map user audio mode to environment-like behavior
    fn get_effective_environment_for_mode(&self) -> AudioEnvironment {
        match self.controls.mode {
            AudioMode::Mellow => AudioEnvironment::Meditative,
            AudioMode::Active => {
                // Use natural environment progression
                self.current_environment.clone()
            },
            AudioMode::Chaotic => AudioEnvironment::Electronica,
        }
    }

    /// Get distortion intensity based on user mode
    fn get_distortion_intensity_for_mode(&self) -> f32 {
        match self.controls.mode {
            AudioMode::Mellow => 0.2,   // Minimal distortion
            AudioMode::Active => 1.0,   // Natural distortion progression
            AudioMode::Chaotic => 2.0,  // Maximum chaos
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