// === REALITY DISTORTION AUDIO PROCESSOR ===
// Bends audio reality through consciousness-driven effects
// When llamas achieve transcendence, audio reality tears apart

use std::collections::VecDeque;
use crate::mathematics::BeatState;

/// Main reality distortion processor - warps audio through consciousness
pub struct RealityDistortionProcessor {
    sample_rate: f32,

    // Reality distortion effects
    frequency_mangler: FrequencyMangler,
    temporal_echo: TemporalEcho,
    consciousness_filter: ConsciousnessFilter,
    reality_crusher: RealityCrusher,

    // Distortion state
    distortion_level: f32,
    reality_tear_intensity: f32,
    chaos_accumulator: f32,

    // Effect chains for extreme consciousness states
    psychedelic_delay_chain: Vec<DelayLine>,
    reality_break_buffer: VecDeque<f32>,
}

/// Frequency domain manipulation - shifts reality through spectral warping
pub struct FrequencyMangler {
    // Frequency shifting and pitch bending
    pitch_shift_buffer: VecDeque<f32>,
    pitch_shift_amount: f32,
    frequency_shift_amount: f32,

    // Spectral filtering
    low_pass_filter: BiquadFilter,
    high_pass_filter: BiquadFilter,
    band_pass_filter: BiquadFilter,

    // Harmonic manipulation
    harmonic_exciter_amount: f32,
    sub_harmonic_generator: f32,

    // Consciousness-driven frequency warping
    consciousness_warp_factor: f32,
}

/// Temporal effects - echoes and delays that bend time
pub struct TemporalEcho {
    delay_lines: Vec<DelayLine>,
    feedback_amounts: Vec<f32>,
    delay_times: Vec<f32>,

    // Consciousness-modulated delay
    consciousness_delay_modulation: f32,
    temporal_displacement: f32,

    // Infinite echo chambers for transcendence states
    infinity_delay: DelayLine,
    infinity_feedback: f32,
}

/// Consciousness-driven filter that responds to collective llama states
struct ConsciousnessFilter {
    resonance: f32,
    cutoff_frequency: f32,
    filter_type: FilterType,
    consciousness_modulation_depth: f32,

    // State variables for filter processing
    low_pass_state: [f32; 2],
    high_pass_state: [f32; 2],
    band_pass_state: [f32; 2],
}

/// Reality crusher - extreme distortion for consciousness overload
struct RealityCrusher {
    bit_crush_depth: f32,
    sample_rate_reduction: f32,
    saturation_amount: f32,
    chaos_injection: f32,

    // Buffer for sample rate reduction
    decimation_buffer: VecDeque<f32>,
    decimation_counter: usize,
}

/// Delay line for temporal effects
struct DelayLine {
    buffer: VecDeque<f32>,
    max_delay_samples: usize,
    current_delay_samples: usize,
}

/// Biquad filter for frequency shaping
struct BiquadFilter {
    a0: f32, a1: f32, a2: f32,
    b1: f32, b2: f32,
    x1: f32, x2: f32,
    y1: f32, y2: f32,
}

#[derive(Debug, Clone)]
enum FilterType {
    LowPass,
    HighPass,
    BandPass,
    Notch,
    Peak,
    ConsciousnessWarp, // Special filter that changes based on consciousness
}

impl RealityDistortionProcessor {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            frequency_mangler: FrequencyMangler::new(sample_rate),
            temporal_echo: TemporalEcho::new(sample_rate),
            consciousness_filter: ConsciousnessFilter::new(sample_rate),
            reality_crusher: RealityCrusher::new(),
            distortion_level: 0.0,
            reality_tear_intensity: 0.0,
            chaos_accumulator: 0.0,
            psychedelic_delay_chain: vec![
                DelayLine::new((sample_rate * 0.1) as usize),  // 100ms
                DelayLine::new((sample_rate * 0.23) as usize), // 230ms
                DelayLine::new((sample_rate * 0.37) as usize), // 370ms
                DelayLine::new((sample_rate * 0.61) as usize), // 610ms (golden ratio intervals)
            ],
            reality_break_buffer: VecDeque::with_capacity((sample_rate * 2.0) as usize),
        }
    }

    /// Update distortion parameters based on consciousness state
    pub fn update(&mut self, cosmic_time: f64, beat_state: &BeatState) {
        // Build distortion based on beat intensity and prime factors
        self.distortion_level = (beat_state.intensity + beat_state.prime_factor) * 0.5;

        // Chaos accumulates over time when beats are intense
        if beat_state.is_beat_drop && beat_state.intensity > 0.7 {
            self.chaos_accumulator += 0.05;
        }
        self.chaos_accumulator *= 0.998; // Slow decay
        self.chaos_accumulator = self.chaos_accumulator.min(1.0);

        // Update all effect modules
        self.frequency_mangler.update(cosmic_time, beat_state, self.distortion_level);
        self.temporal_echo.update(cosmic_time, beat_state);
        self.consciousness_filter.update(cosmic_time, beat_state);
        self.reality_crusher.update(self.chaos_accumulator);
    }

    /// Process audio sample through reality distortion
    pub fn process_sample(&mut self,
                         input_sample: f32,
                         sample_time: f64,
                         beat_state: &BeatState) -> f32 {
        self.process_sample_with_intensity(input_sample, sample_time, beat_state, 1.0)
    }

    /// Process audio sample with custom intensity multiplier for user control
    pub fn process_sample_with_intensity(&mut self,
                                       input_sample: f32,
                                       sample_time: f64,
                                       beat_state: &BeatState,
                                       intensity_multiplier: f32) -> f32 {

        let mut sample = input_sample;
        let effective_distortion_level = self.distortion_level * intensity_multiplier;

        // Apply effects in consciousness-determined order
        match effective_distortion_level {
            level if level < 0.2 => {
                // Minimal distortion - just slight filtering
                sample = self.consciousness_filter.process(sample);
            },
            level if level < 0.5 => {
                // Light psychedelic effects
                sample = self.consciousness_filter.process(sample);
                sample = self.temporal_echo.process_light(sample);
                sample = self.frequency_mangler.process_light(sample);
            },
            level if level < 0.8 => {
                // Heavy psychedelic processing
                sample = self.frequency_mangler.process_heavy(sample);
                sample = self.temporal_echo.process_heavy(sample);
                sample = self.consciousness_filter.process(sample);

                // Add chaos injection
                if self.chaos_accumulator > 0.3 {
                    sample = self.reality_crusher.process_light(sample);
                }
            },
            _ => {
                // MAXIMUM REALITY DISTORTION
                sample = self.reality_crusher.process_heavy(sample);
                sample = self.frequency_mangler.process_extreme(sample);
                sample = self.temporal_echo.process_infinite(sample);
                sample = self.consciousness_filter.process_transcendent(sample);
            }
        }

        // Apply reality tear effects if active
        if self.reality_tear_intensity > 0.0 {
            sample = self.apply_reality_tear_effects(sample, sample_time);
        }

        // Store in reality break buffer for potential reverse effects
        self.reality_break_buffer.push_back(sample);
        if self.reality_break_buffer.len() > (self.sample_rate * 2.0) as usize {
            self.reality_break_buffer.pop_front();
        }

        sample
    }

    fn apply_reality_tear_effects(&mut self, sample: f32, sample_time: f64) -> f32 {
        let mut torn_sample = sample;

        // Frequency tearing - split audio into multiple frequency bands that drift apart
        let tear_modulation = (sample_time * 13.0).sin() as f32 * self.reality_tear_intensity;
        torn_sample *= 1.0 + tear_modulation * 0.3;

        // Temporal tearing - introduce glitches and stutters
        if fastrand::f32() < self.reality_tear_intensity * 0.1 {
            // Glitch: replace with older sample
            if let Some(&old_sample) = self.reality_break_buffer.get(fastrand::usize(0..self.reality_break_buffer.len())) {
                torn_sample = old_sample * 0.7 + torn_sample * 0.3;
            }
        }

        // Reality fragmentation - split sample across multiple processing paths
        let fragment_1 = torn_sample * (1.0 + (sample_time * 7.0).sin() as f32 * 0.2);
        let fragment_2 = torn_sample * (1.0 + (sample_time * 11.0).cos() as f32 * 0.15);
        let fragment_3 = torn_sample * (1.0 + (sample_time * 17.0).sin() as f32 * 0.1);

        // Recombine fragments with consciousness-driven mixing
        let mix_factor = self.reality_tear_intensity;
        torn_sample = fragment_1 * (1.0 - mix_factor) +
                     (fragment_2 + fragment_3) * 0.5 * mix_factor;

        torn_sample
    }

    /// Trigger reality tear effect
    pub fn trigger_reality_tear(&mut self, strength: f32) {
        self.reality_tear_intensity = strength.min(1.0);
    }

    /// Trigger complete reality break
    pub fn trigger_reality_break(&mut self) {
        self.reality_tear_intensity = 1.0;
        self.chaos_accumulator = 1.0;
        self.distortion_level = 1.0;
    }

    /// Get current distortion level for visualization
    pub fn get_distortion_level(&self) -> f32 {
        self.distortion_level
    }
}

impl FrequencyMangler {
    fn new(sample_rate: f32) -> Self {
        Self {
            pitch_shift_buffer: VecDeque::with_capacity((sample_rate * 0.1) as usize),
            pitch_shift_amount: 0.0,
            frequency_shift_amount: 0.0,
            low_pass_filter: BiquadFilter::new_lowpass(sample_rate, 8000.0, 0.7),
            high_pass_filter: BiquadFilter::new_highpass(sample_rate, 100.0, 0.7),
            band_pass_filter: BiquadFilter::new_bandpass(sample_rate, 1000.0, 2.0),
            harmonic_exciter_amount: 0.0,
            sub_harmonic_generator: 0.0,
            consciousness_warp_factor: 0.0,
        }
    }

    fn update(&mut self, cosmic_time: f64, beat_state: &BeatState, distortion_level: f32) {
        // Update pitch shifting based on consciousness
        self.pitch_shift_amount = (cosmic_time * 0.1).sin() as f32 * distortion_level * 0.3;

        // Update frequency filters based on beat state
        let filter_freq = 1000.0 + beat_state.cosmic_frequency * distortion_level;
        self.low_pass_filter.update_lowpass(filter_freq.min(20000.0), 0.7 + distortion_level * 0.3);

        // Consciousness warp factor
        self.consciousness_warp_factor = beat_state.prime_factor * distortion_level;
    }

    fn process_light(&mut self, sample: f32) -> f32 {
        // Light frequency processing
        let filtered = self.low_pass_filter.process(sample);
        filtered * (1.0 + self.consciousness_warp_factor * 0.1)
    }

    fn process_heavy(&mut self, sample: f32) -> f32 {
        // Heavy frequency manipulation
        let mut processed = sample;

        // Apply pitch shifting
        processed = self.apply_pitch_shift(processed);

        // Multi-band filtering
        let low = self.low_pass_filter.process(processed);
        let high = self.high_pass_filter.process(processed);
        let band = self.band_pass_filter.process(processed);

        // Mix with consciousness warping
        processed = low * 0.4 + high * 0.3 + band * 0.3;
        processed *= 1.0 + self.consciousness_warp_factor * 0.3;

        processed
    }

    fn process_extreme(&mut self, sample: f32) -> f32 {
        // EXTREME frequency destruction
        let mut mangled = sample;

        // Severe pitch shifting
        mangled = self.apply_pitch_shift(mangled);

        // Harmonic excitation - add higher harmonics
        let harmonic_content = mangled * mangled * mangled; // Cubic distortion for harmonics
        mangled += harmonic_content * self.harmonic_exciter_amount * 0.2;

        // Sub-harmonic generation
        if self.sub_harmonic_generator > 0.0 {
            // Simple sub-harmonic by amplitude modulation
            mangled *= 1.0 + (mangled * 0.5).sin() * self.sub_harmonic_generator * 0.3;
        }

        // Extreme consciousness warping
        mangled *= 1.0 + self.consciousness_warp_factor * 0.8;

        mangled
    }

    fn apply_pitch_shift(&mut self, sample: f32) -> f32 {
        // Simple pitch shifting using buffer delay
        self.pitch_shift_buffer.push_back(sample);

        let delay_samples = (self.pitch_shift_amount * 100.0) as i32;
        let read_index = self.pitch_shift_buffer.len() as i32 - 1 - delay_samples;

        if read_index >= 0 && read_index < self.pitch_shift_buffer.len() as i32 {
            self.pitch_shift_buffer[read_index as usize]
        } else {
            sample
        }
    }
}

impl TemporalEcho {
    fn new(sample_rate: f32) -> Self {
        Self {
            delay_lines: vec![
                DelayLine::new((sample_rate * 0.25) as usize),  // 250ms
                DelayLine::new((sample_rate * 0.5) as usize),   // 500ms
                DelayLine::new((sample_rate * 1.0) as usize),   // 1 second
            ],
            feedback_amounts: vec![0.3, 0.4, 0.2],
            delay_times: vec![0.25, 0.5, 1.0],
            consciousness_delay_modulation: 0.0,
            temporal_displacement: 0.0,
            infinity_delay: DelayLine::new((sample_rate * 4.0) as usize),
            infinity_feedback: 0.0,
        }
    }

    fn update(&mut self, cosmic_time: f64, beat_state: &BeatState) {
        // Modulate delay times with consciousness
        self.consciousness_delay_modulation = (cosmic_time * 0.2).sin() as f32 * beat_state.intensity;

        // Temporal displacement for reality tearing
        self.temporal_displacement = beat_state.prime_factor;
    }

    fn process_light(&mut self, sample: f32) -> f32 {
        // Light echo processing
        let delayed = self.delay_lines[0].read_and_write(sample, 0.3);
        sample * 0.7 + delayed * 0.3
    }

    fn process_heavy(&mut self, sample: f32) -> f32 {
        // Heavy multi-tap delays
        let mut output = sample;

        for (i, delay_line) in self.delay_lines.iter_mut().enumerate() {
            let feedback = self.feedback_amounts[i] * (1.0 + self.consciousness_delay_modulation * 0.2);
            let delayed = delay_line.read_and_write(output, feedback);
            output += delayed * 0.3;
        }

        output * 0.6 // Reduce overall level to prevent clipping
    }

    fn process_infinite(&mut self, sample: f32) -> f32 {
        // Infinite echo chamber for transcendence
        self.infinity_feedback = 0.95 + self.temporal_displacement * 0.04;
        let infinite_echo = self.infinity_delay.read_and_write(sample, self.infinity_feedback);

        sample * 0.3 + infinite_echo * 0.7
    }
}

impl ConsciousnessFilter {
    fn new(sample_rate: f32) -> Self {
        Self {
            resonance: 0.7,
            cutoff_frequency: 1000.0,
            filter_type: FilterType::LowPass,
            consciousness_modulation_depth: 0.0,
            low_pass_state: [0.0; 2],
            high_pass_state: [0.0; 2],
            band_pass_state: [0.0; 2],
        }
    }

    fn update(&mut self, cosmic_time: f64, beat_state: &BeatState) {
        // Consciousness drives filter cutoff
        self.cutoff_frequency = 500.0 + beat_state.cosmic_frequency;
        self.consciousness_modulation_depth = beat_state.intensity;

        // Change filter type based on prime factors
        self.filter_type = match (beat_state.prime_factor * 4.0) as u32 {
            0 => FilterType::LowPass,
            1 => FilterType::HighPass,
            2 => FilterType::BandPass,
            _ => FilterType::ConsciousnessWarp,
        };
    }

    fn process(&mut self, sample: f32) -> f32 {
        match self.filter_type {
            FilterType::LowPass => self.process_lowpass(sample),
            FilterType::HighPass => self.process_highpass(sample),
            FilterType::BandPass => self.process_bandpass(sample),
            FilterType::ConsciousnessWarp => self.process_consciousness_warp(sample),
            _ => sample,
        }
    }

    fn process_transcendent(&mut self, sample: f32) -> f32 {
        // Transcendent filter that shifts through all types rapidly
        let type_modulation = ((sample as f64 * 1000.0) % 4.0) as u32;

        match type_modulation {
            0 => self.process_lowpass(sample),
            1 => self.process_highpass(sample),
            2 => self.process_bandpass(sample),
            _ => self.process_consciousness_warp(sample),
        }
    }

    fn process_lowpass(&mut self, sample: f32) -> f32 {
        // Simple one-pole lowpass
        self.low_pass_state[0] += (sample - self.low_pass_state[0]) * 0.1;
        self.low_pass_state[0]
    }

    fn process_highpass(&mut self, sample: f32) -> f32 {
        // Simple one-pole highpass
        self.high_pass_state[0] = sample - self.high_pass_state[1];
        self.high_pass_state[1] += self.high_pass_state[0] * 0.1;
        self.high_pass_state[0]
    }

    fn process_bandpass(&mut self, sample: f32) -> f32 {
        // Bandpass as combination of high and low
        let low = self.process_lowpass(sample);
        let high = self.process_highpass(sample);
        (low + high) * 0.5
    }

    fn process_consciousness_warp(&mut self, sample: f32) -> f32 {
        // Special consciousness-driven filter
        let warp_factor = self.consciousness_modulation_depth;
        let warped = sample * (1.0 + (sample * warp_factor * 10.0).sin() * 0.3);
        warped
    }
}

impl RealityCrusher {
    fn new() -> Self {
        Self {
            bit_crush_depth: 0.0,
            sample_rate_reduction: 0.0,
            saturation_amount: 0.0,
            chaos_injection: 0.0,
            decimation_buffer: VecDeque::new(),
            decimation_counter: 0,
        }
    }

    fn update(&mut self, chaos_level: f32) {
        self.bit_crush_depth = chaos_level * 8.0; // Up to 8-bit crushing
        self.sample_rate_reduction = chaos_level * 0.8; // Up to 80% sample rate reduction
        self.saturation_amount = chaos_level;
        self.chaos_injection = chaos_level * 0.3;
    }

    fn process_light(&mut self, sample: f32) -> f32 {
        // Light crushing
        let mut crushed = sample;

        if self.saturation_amount > 0.0 {
            // Soft saturation
            crushed = crushed.tanh() * self.saturation_amount + crushed * (1.0 - self.saturation_amount);
        }

        crushed
    }

    fn process_heavy(&mut self, sample: f32) -> f32 {
        // Heavy reality crushing
        let mut crushed = sample;

        // Bit crushing
        if self.bit_crush_depth > 0.0 {
            let quantization = 2.0_f32.powi(self.bit_crush_depth as i32);
            crushed = (crushed * quantization).round() / quantization;
        }

        // Sample rate reduction
        if self.sample_rate_reduction > 0.0 {
            self.decimation_counter += 1;
            let skip_samples = (self.sample_rate_reduction * 10.0) as usize;

            if self.decimation_counter % (skip_samples + 1) != 0 {
                // Return last held sample
                if let Some(&last_sample) = self.decimation_buffer.back() {
                    return last_sample;
                }
            } else {
                self.decimation_buffer.push_back(crushed);
                if self.decimation_buffer.len() > 10 {
                    self.decimation_buffer.pop_front();
                }
            }
        }

        // Saturation
        crushed = crushed.tanh();

        // Chaos injection
        if self.chaos_injection > 0.0 {
            let chaos = (fastrand::f32() - 0.5) * 2.0;
            crushed += chaos * self.chaos_injection * 0.1;
        }

        crushed
    }
}

impl DelayLine {
    fn new(max_delay_samples: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(max_delay_samples),
            max_delay_samples,
            current_delay_samples: max_delay_samples / 2,
        }
    }

    fn read_and_write(&mut self, input: f32, feedback: f32) -> f32 {
        // Fill buffer initially
        if self.buffer.len() < self.max_delay_samples {
            self.buffer.push_back(0.0);
            return 0.0;
        }

        // Read delayed sample
        let delayed = self.buffer[0];

        // Write new sample with feedback
        let write_sample = input + delayed * feedback;
        self.buffer.pop_front();
        self.buffer.push_back(write_sample);

        delayed
    }
}

impl BiquadFilter {
    fn new_lowpass(sample_rate: f32, frequency: f32, q: f32) -> Self {
        let mut filter = Self {
            a0: 0.0, a1: 0.0, a2: 0.0,
            b1: 0.0, b2: 0.0,
            x1: 0.0, x2: 0.0,
            y1: 0.0, y2: 0.0,
        };
        filter.update_lowpass(frequency, q);
        filter
    }

    fn new_highpass(sample_rate: f32, frequency: f32, q: f32) -> Self {
        let mut filter = Self {
            a0: 0.0, a1: 0.0, a2: 0.0,
            b1: 0.0, b2: 0.0,
            x1: 0.0, x2: 0.0,
            y1: 0.0, y2: 0.0,
        };
        filter.update_highpass(frequency, q);
        filter
    }

    fn new_bandpass(sample_rate: f32, frequency: f32, q: f32) -> Self {
        let mut filter = Self {
            a0: 0.0, a1: 0.0, a2: 0.0,
            b1: 0.0, b2: 0.0,
            x1: 0.0, x2: 0.0,
            y1: 0.0, y2: 0.0,
        };
        filter.update_bandpass(frequency, q);
        filter
    }

    fn update_lowpass(&mut self, frequency: f32, q: f32) {
        let omega = std::f32::consts::TAU * frequency / 44100.0;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let alpha = sin_omega / (2.0 * q);

        let b0 = 1.0 + alpha;
        self.a0 = (1.0 - cos_omega) / 2.0 / b0;
        self.a1 = (1.0 - cos_omega) / b0;
        self.a2 = (1.0 - cos_omega) / 2.0 / b0;
        self.b1 = -2.0 * cos_omega / b0;
        self.b2 = (1.0 - alpha) / b0;
    }

    fn update_highpass(&mut self, frequency: f32, q: f32) {
        let omega = std::f32::consts::TAU * frequency / 44100.0;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let alpha = sin_omega / (2.0 * q);

        let b0 = 1.0 + alpha;
        self.a0 = (1.0 + cos_omega) / 2.0 / b0;
        self.a1 = -(1.0 + cos_omega) / b0;
        self.a2 = (1.0 + cos_omega) / 2.0 / b0;
        self.b1 = -2.0 * cos_omega / b0;
        self.b2 = (1.0 - alpha) / b0;
    }

    fn update_bandpass(&mut self, frequency: f32, q: f32) {
        let omega = std::f32::consts::TAU * frequency / 44100.0;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let alpha = sin_omega / (2.0 * q);

        let b0 = 1.0 + alpha;
        self.a0 = alpha / b0;
        self.a1 = 0.0;
        self.a2 = -alpha / b0;
        self.b1 = -2.0 * cos_omega / b0;
        self.b2 = (1.0 - alpha) / b0;
    }

    fn process(&mut self, input: f32) -> f32 {
        let output = self.a0 * input + self.a1 * self.x1 + self.a2 * self.x2
                   - self.b1 * self.y1 - self.b2 * self.y2;

        self.x2 = self.x1;
        self.x1 = input;
        self.y2 = self.y1;
        self.y1 = output;

        output
    }
}