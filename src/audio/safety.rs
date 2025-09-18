// === AUDIO SAFETY SYSTEMS ===
// Protects users from dangerous audio levels while preserving psychedelic experience
// "Maximum chaos, safe decibels" - The Minter Agent

use std::collections::VecDeque;

/// Audio safety limiter - ensures safe listening levels while maintaining psychedelic intensity
pub struct AudioSafetyLimiter {
    sample_rate: f32,

    // Volume and amplitude limiting
    volume_envelope: VolumeEnvelope,
    frequency_guard: FrequencyGuard,
    dynamic_range_compressor: DynamicRangeCompressor,

    // Safety thresholds
    max_output_level: f32,      // Peak amplitude limit (0.0 to 1.0)
    rms_safety_threshold: f32,  // RMS level safety limit
    frequency_safety_ranges: Vec<(f32, f32)>, // Safe frequency ranges

    // Real-time monitoring
    peak_detector: PeakDetector,
    rms_detector: RmsDetector,
    frequency_analyzer: FrequencyAnalyzer,

    // Emergency limiting
    emergency_limiter_active: bool,
    emergency_gain_reduction: f32,
    safety_violation_count: u32,
}

/// Volume envelope processor for smooth gain changes
pub struct VolumeEnvelope {
    current_gain: f32,
    target_gain: f32,
    attack_time_samples: f32,
    release_time_samples: f32,
    envelope_state: EnvelopeState,
}

/// Frequency guard to prevent dangerous frequency content
pub struct FrequencyGuard {
    // Dangerous frequency detection and filtering
    high_frequency_limiter: HighFrequencyLimiter,
    low_frequency_limiter: LowFrequencyLimiter,
    resonance_detector: ResonanceDetector,

    // Safety filters
    dc_blocker: DcBlocker,
    subsonic_filter: SubsonicFilter,
    ultrasonic_filter: UltrasonicFilter,
}

/// Dynamic range compressor for consistent levels
struct DynamicRangeCompressor {
    threshold: f32,
    ratio: f32,
    attack_samples: f32,
    release_samples: f32,
    envelope_follower: f32,
    gain_reduction: f32,
}

/// Peak level detector
struct PeakDetector {
    current_peak: f32,
    peak_hold_samples: usize,
    peak_hold_counter: usize,
    decay_rate: f32,
}

/// RMS (Root Mean Square) level detector
struct RmsDetector {
    sample_buffer: VecDeque<f32>,
    buffer_size: usize,
    current_rms: f32,
}

/// Simple frequency analyzer for safety monitoring
struct FrequencyAnalyzer {
    // Simple band-pass filters for frequency analysis
    low_band_filter: SimpleBandpassFilter,
    mid_band_filter: SimpleBandpassFilter,
    high_band_filter: SimpleBandpassFilter,

    // Energy levels in each band
    low_energy: f32,
    mid_energy: f32,
    high_energy: f32,
}

/// High frequency limiter to prevent ear damage
struct HighFrequencyLimiter {
    cutoff_frequency: f32,
    filter: SimpleFilter,
    limiter_active: bool,
}

/// Low frequency limiter to prevent speaker damage
struct LowFrequencyLimiter {
    cutoff_frequency: f32,
    filter: SimpleFilter,
    limiter_active: bool,
}

/// Detects dangerous resonances
struct ResonanceDetector {
    resonance_threshold: f32,
    detection_window: VecDeque<f32>,
    window_size: usize,
}

/// DC blocking filter
struct DcBlocker {
    previous_input: f32,
    previous_output: f32,
    pole: f32,
}

/// Subsonic filter (removes frequencies below hearing range)
struct SubsonicFilter {
    cutoff_frequency: f32,
    filter: SimpleFilter,
}

/// Ultrasonic filter (removes frequencies above hearing range)
struct UltrasonicFilter {
    cutoff_frequency: f32,
    filter: SimpleFilter,
}

/// Simple first-order filter
struct SimpleFilter {
    a: f32,
    previous_input: f32,
    previous_output: f32,
}

/// Simple bandpass filter for frequency analysis
struct SimpleBandpassFilter {
    low_pass: SimpleFilter,
    high_pass: SimpleFilter,
}

#[derive(Debug, Clone)]
enum EnvelopeState {
    Attack,
    Sustain,
    Release,
}

impl AudioSafetyLimiter {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            volume_envelope: VolumeEnvelope::new(sample_rate),
            frequency_guard: FrequencyGuard::new(sample_rate),
            dynamic_range_compressor: DynamicRangeCompressor::new(sample_rate),
            max_output_level: 0.8,       // 80% of full scale for safety
            rms_safety_threshold: 0.3,   // 30% RMS for comfortable listening
            frequency_safety_ranges: vec![
                (20.0, 20000.0),         // Full audible range
                (80.0, 8000.0),          // Extra safe range for extended listening
            ],
            peak_detector: PeakDetector::new(sample_rate),
            rms_detector: RmsDetector::new(sample_rate),
            frequency_analyzer: FrequencyAnalyzer::new(sample_rate),
            emergency_limiter_active: false,
            emergency_gain_reduction: 1.0,
            safety_violation_count: 0,
        }
    }

    /// Main safety processing - ensures audio is safe while preserving character
    pub fn limit_sample(&mut self, input_sample: f32) -> f32 {
        let mut safe_sample = input_sample;

        // First, apply frequency guard to remove dangerous frequencies
        safe_sample = self.frequency_guard.process_sample(safe_sample);

        // Update safety monitoring
        self.peak_detector.update(safe_sample.abs());
        self.rms_detector.update(safe_sample);
        self.frequency_analyzer.update(safe_sample);

        // Check for safety violations
        self.check_safety_violations();

        // Apply dynamic range compression for consistent levels
        safe_sample = self.dynamic_range_compressor.process(safe_sample);

        // Apply volume envelope for smooth gain changes
        safe_sample = self.volume_envelope.process(safe_sample);

        // Final hard limiting for absolute safety
        safe_sample = self.apply_hard_limiting(safe_sample);

        // Emergency limiting if safety violations detected
        if self.emergency_limiter_active {
            safe_sample *= self.emergency_gain_reduction;
        }

        safe_sample
    }

    fn check_safety_violations(&mut self) {
        let mut violations_detected = false;

        // Check peak levels
        if self.peak_detector.current_peak > self.max_output_level {
            violations_detected = true;
        }

        // Check RMS levels
        if self.rms_detector.current_rms > self.rms_safety_threshold {
            violations_detected = true;
        }

        // Check frequency content
        let total_energy = self.frequency_analyzer.low_energy +
                          self.frequency_analyzer.mid_energy +
                          self.frequency_analyzer.high_energy;

        if total_energy > 2.0 {
            violations_detected = true;
        }

        // Handle violations
        if violations_detected {
            self.safety_violation_count += 1;

            // Activate emergency limiting after multiple violations
            if self.safety_violation_count > 10 {
                self.emergency_limiter_active = true;
                self.emergency_gain_reduction *= 0.95; // Gradually reduce gain
            }
        } else {
            // Reset violation count and gradually restore gain
            if self.safety_violation_count > 0 {
                self.safety_violation_count -= 1;
            }

            if self.safety_violation_count == 0 && self.emergency_limiter_active {
                self.emergency_gain_reduction = (self.emergency_gain_reduction + 0.001).min(1.0);

                // Deactivate emergency limiting when gain is restored
                if self.emergency_gain_reduction >= 0.95 {
                    self.emergency_limiter_active = false;
                    self.emergency_gain_reduction = 1.0;
                }
            }
        }
    }

    fn apply_hard_limiting(&self, sample: f32) -> f32 {
        // Hard limit to prevent clipping while maintaining waveform character
        if sample.abs() > self.max_output_level {
            // Soft clipping using tanh for musical distortion
            let sign = if sample >= 0.0 { 1.0 } else { -1.0 };
            let limited = (sample.abs() / self.max_output_level).tanh() * self.max_output_level;
            limited * sign
        } else {
            sample
        }
    }

    /// Set user safety preferences
    pub fn set_safety_level(&mut self, safety_level: SafetyLevel) {
        match safety_level {
            SafetyLevel::Conservative => {
                self.max_output_level = 0.6;
                self.rms_safety_threshold = 0.2;
                self.volume_envelope.set_conservative_settings();
            },
            SafetyLevel::Standard => {
                self.max_output_level = 0.8;
                self.rms_safety_threshold = 0.3;
                self.volume_envelope.set_standard_settings();
            },
            SafetyLevel::Aggressive => {
                self.max_output_level = 0.95;
                self.rms_safety_threshold = 0.4;
                self.volume_envelope.set_aggressive_settings();
            },
        }
    }

    /// Get current safety status for monitoring
    pub fn get_safety_status(&self) -> SafetyStatus {
        SafetyStatus {
            current_peak_level: self.peak_detector.current_peak,
            current_rms_level: self.rms_detector.current_rms,
            emergency_limiting_active: self.emergency_limiter_active,
            emergency_gain_reduction: self.emergency_gain_reduction,
            safety_violation_count: self.safety_violation_count,
            frequency_analysis: FrequencyAnalysis {
                low_energy: self.frequency_analyzer.low_energy,
                mid_energy: self.frequency_analyzer.mid_energy,
                high_energy: self.frequency_analyzer.high_energy,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum SafetyLevel {
    Conservative,  // Extra safe for extended listening
    Standard,      // Normal safety settings
    Aggressive,    // Maximum intensity with minimal safety limiting
}

#[derive(Debug, Clone)]
pub struct SafetyStatus {
    pub current_peak_level: f32,
    pub current_rms_level: f32,
    pub emergency_limiting_active: bool,
    pub emergency_gain_reduction: f32,
    pub safety_violation_count: u32,
    pub frequency_analysis: FrequencyAnalysis,
}

#[derive(Debug, Clone)]
pub struct FrequencyAnalysis {
    pub low_energy: f32,
    pub mid_energy: f32,
    pub high_energy: f32,
}

impl VolumeEnvelope {
    fn new(sample_rate: f32) -> Self {
        Self {
            current_gain: 1.0,
            target_gain: 1.0,
            attack_time_samples: sample_rate * 0.01,  // 10ms attack
            release_time_samples: sample_rate * 0.1,  // 100ms release
            envelope_state: EnvelopeState::Sustain,
        }
    }

    fn process(&mut self, input_sample: f32) -> f32 {
        // Update gain based on envelope state
        match self.envelope_state {
            EnvelopeState::Attack => {
                self.current_gain += (self.target_gain - self.current_gain) / self.attack_time_samples;
                if (self.current_gain - self.target_gain).abs() < 0.001 {
                    self.envelope_state = EnvelopeState::Sustain;
                }
            },
            EnvelopeState::Release => {
                self.current_gain += (self.target_gain - self.current_gain) / self.release_time_samples;
                if (self.current_gain - self.target_gain).abs() < 0.001 {
                    self.envelope_state = EnvelopeState::Sustain;
                }
            },
            EnvelopeState::Sustain => {
                // No change needed
            }
        }

        input_sample * self.current_gain
    }

    fn set_target_gain(&mut self, target: f32) {
        self.target_gain = target.clamp(0.0, 1.0);

        if target > self.current_gain {
            self.envelope_state = EnvelopeState::Attack;
        } else if target < self.current_gain {
            self.envelope_state = EnvelopeState::Release;
        }
    }

    fn set_conservative_settings(&mut self) {
        self.attack_time_samples *= 2.0;  // Slower changes
        self.release_time_samples *= 2.0;
    }

    fn set_standard_settings(&mut self) {
        // Keep default settings
    }

    fn set_aggressive_settings(&mut self) {
        self.attack_time_samples *= 0.5;  // Faster changes
        self.release_time_samples *= 0.5;
    }
}

impl FrequencyGuard {
    fn new(sample_rate: f32) -> Self {
        Self {
            high_frequency_limiter: HighFrequencyLimiter::new(sample_rate),
            low_frequency_limiter: LowFrequencyLimiter::new(sample_rate),
            resonance_detector: ResonanceDetector::new(sample_rate),
            dc_blocker: DcBlocker::new(),
            subsonic_filter: SubsonicFilter::new(sample_rate),
            ultrasonic_filter: UltrasonicFilter::new(sample_rate),
        }
    }

    fn process_sample(&mut self, input_sample: f32) -> f32 {
        let mut safe_sample = input_sample;

        // Remove DC offset
        safe_sample = self.dc_blocker.process(safe_sample);

        // Remove subsonic content (below 20 Hz)
        safe_sample = self.subsonic_filter.process(safe_sample);

        // Remove ultrasonic content (above 20 kHz)
        safe_sample = self.ultrasonic_filter.process(safe_sample);

        // Check for dangerous resonances
        if self.resonance_detector.detect_resonance(safe_sample) {
            // Apply gentle limiting if resonance detected
            safe_sample *= 0.8;
        }

        // Apply frequency range limiting
        safe_sample = self.high_frequency_limiter.process(safe_sample);
        safe_sample = self.low_frequency_limiter.process(safe_sample);

        safe_sample
    }
}

impl DynamicRangeCompressor {
    fn new(sample_rate: f32) -> Self {
        Self {
            threshold: 0.7,
            ratio: 4.0,
            attack_samples: sample_rate * 0.001,  // 1ms attack
            release_samples: sample_rate * 0.1,   // 100ms release
            envelope_follower: 0.0,
            gain_reduction: 1.0,
        }
    }

    fn process(&mut self, input_sample: f32) -> f32 {
        let input_level = input_sample.abs();

        // Update envelope follower
        if input_level > self.envelope_follower {
            // Attack
            self.envelope_follower += (input_level - self.envelope_follower) / self.attack_samples;
        } else {
            // Release
            self.envelope_follower += (input_level - self.envelope_follower) / self.release_samples;
        }

        // Calculate gain reduction
        if self.envelope_follower > self.threshold {
            let over_threshold = self.envelope_follower - self.threshold;
            let compressed_over = over_threshold / self.ratio;
            let target_level = self.threshold + compressed_over;
            self.gain_reduction = target_level / self.envelope_follower;
        } else {
            self.gain_reduction = 1.0;
        }

        input_sample * self.gain_reduction
    }
}

impl PeakDetector {
    fn new(sample_rate: f32) -> Self {
        Self {
            current_peak: 0.0,
            peak_hold_samples: (sample_rate * 0.1) as usize, // 100ms hold
            peak_hold_counter: 0,
            decay_rate: 0.999,
        }
    }

    fn update(&mut self, input_level: f32) {
        if input_level > self.current_peak {
            self.current_peak = input_level;
            self.peak_hold_counter = self.peak_hold_samples;
        } else if self.peak_hold_counter > 0 {
            self.peak_hold_counter -= 1;
        } else {
            self.current_peak *= self.decay_rate;
        }
    }
}

impl RmsDetector {
    fn new(sample_rate: f32) -> Self {
        let buffer_size = (sample_rate * 0.1) as usize; // 100ms window
        Self {
            sample_buffer: VecDeque::with_capacity(buffer_size),
            buffer_size,
            current_rms: 0.0,
        }
    }

    fn update(&mut self, input_sample: f32) {
        self.sample_buffer.push_back(input_sample * input_sample);

        if self.sample_buffer.len() > self.buffer_size {
            self.sample_buffer.pop_front();
        }

        // Calculate RMS
        let sum_of_squares: f32 = self.sample_buffer.iter().sum();
        self.current_rms = (sum_of_squares / self.sample_buffer.len() as f32).sqrt();
    }
}

impl FrequencyAnalyzer {
    fn new(sample_rate: f32) -> Self {
        Self {
            low_band_filter: SimpleBandpassFilter::new(sample_rate, 80.0, 300.0),
            mid_band_filter: SimpleBandpassFilter::new(sample_rate, 300.0, 3000.0),
            high_band_filter: SimpleBandpassFilter::new(sample_rate, 3000.0, 8000.0),
            low_energy: 0.0,
            mid_energy: 0.0,
            high_energy: 0.0,
        }
    }

    fn update(&mut self, input_sample: f32) {
        let low_sample = self.low_band_filter.process(input_sample);
        let mid_sample = self.mid_band_filter.process(input_sample);
        let high_sample = self.high_band_filter.process(input_sample);

        // Update energy levels with simple peak following
        self.low_energy = (self.low_energy * 0.99).max(low_sample.abs());
        self.mid_energy = (self.mid_energy * 0.99).max(mid_sample.abs());
        self.high_energy = (self.high_energy * 0.99).max(high_sample.abs());
    }
}

// Simplified implementations for basic safety filtering

impl SimpleFilter {
    fn new(sample_rate: f32, cutoff_frequency: f32) -> Self {
        let rc = 1.0 / (std::f32::consts::TAU * cutoff_frequency);
        let dt = 1.0 / sample_rate;
        let a = dt / (rc + dt);

        Self {
            a,
            previous_input: 0.0,
            previous_output: 0.0,
        }
    }

    fn process(&mut self, input: f32) -> f32 {
        let output = self.a * input + (1.0 - self.a) * self.previous_output;
        self.previous_input = input;
        self.previous_output = output;
        output
    }
}

impl SimpleBandpassFilter {
    fn new(sample_rate: f32, low_cutoff: f32, high_cutoff: f32) -> Self {
        Self {
            low_pass: SimpleFilter::new(sample_rate, high_cutoff),
            high_pass: SimpleFilter::new(sample_rate, low_cutoff),
        }
    }

    fn process(&mut self, input: f32) -> f32 {
        let low_passed = self.low_pass.process(input);
        // Simple high-pass implementation
        let high_passed = input - low_passed;
        high_passed
    }
}

impl DcBlocker {
    fn new() -> Self {
        Self {
            previous_input: 0.0,
            previous_output: 0.0,
            pole: 0.995,
        }
    }

    fn process(&mut self, input: f32) -> f32 {
        let output = input - self.previous_input + self.pole * self.previous_output;
        self.previous_input = input;
        self.previous_output = output;
        output
    }
}

impl HighFrequencyLimiter {
    fn new(sample_rate: f32) -> Self {
        Self {
            cutoff_frequency: 8000.0, // Limit high frequencies for ear safety
            filter: SimpleFilter::new(sample_rate, 8000.0),
            limiter_active: true,
        }
    }

    fn process(&mut self, input: f32) -> f32 {
        if self.limiter_active {
            self.filter.process(input)
        } else {
            input
        }
    }
}

impl LowFrequencyLimiter {
    fn new(sample_rate: f32) -> Self {
        Self {
            cutoff_frequency: 40.0, // Remove very low frequencies that can cause speaker damage
            filter: SimpleFilter::new(sample_rate, 40.0),
            limiter_active: true,
        }
    }

    fn process(&mut self, input: f32) -> f32 {
        if self.limiter_active {
            // High-pass filter implementation
            input - self.filter.process(input)
        } else {
            input
        }
    }
}

impl SubsonicFilter {
    fn new(sample_rate: f32) -> Self {
        Self {
            cutoff_frequency: 20.0,
            filter: SimpleFilter::new(sample_rate, 20.0),
        }
    }

    fn process(&mut self, input: f32) -> f32 {
        // High-pass filter to remove subsonic content
        input - self.filter.process(input)
    }
}

impl UltrasonicFilter {
    fn new(sample_rate: f32) -> Self {
        Self {
            cutoff_frequency: 20000.0,
            filter: SimpleFilter::new(sample_rate, 20000.0),
        }
    }

    fn process(&mut self, input: f32) -> f32 {
        // Low-pass filter to remove ultrasonic content
        self.filter.process(input)
    }
}

impl ResonanceDetector {
    fn new(sample_rate: f32) -> Self {
        let window_size = (sample_rate * 0.05) as usize; // 50ms window
        Self {
            resonance_threshold: 0.8,
            detection_window: VecDeque::with_capacity(window_size),
            window_size,
        }
    }

    fn detect_resonance(&mut self, input_sample: f32) -> bool {
        self.detection_window.push_back(input_sample.abs());

        if self.detection_window.len() > self.window_size {
            self.detection_window.pop_front();
        }

        // Simple resonance detection: check for sustained high levels
        if self.detection_window.len() == self.window_size {
            let average_level: f32 = self.detection_window.iter().sum::<f32>() / self.window_size as f32;
            return average_level > self.resonance_threshold;
        }

        false
    }
}