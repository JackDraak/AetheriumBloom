use primes::{PrimeSet, Sieve};

#[derive(Debug, Clone)]
pub struct BeatState {
    pub is_beat_drop: bool,
    pub intensity: f32,
    pub phase: f32,
    pub prime_factor: f32,
    pub cosmic_frequency: f32,
}

pub struct BeatEngine {
    prime_sieve: Sieve,
    cosmic_tempo: f32,
    reality_stability: f32,
    last_prime_index: usize,
    beat_accumulator: f32,
    phase_offset: f32,
}

impl BeatEngine {
    pub fn new() -> Self {
        Self {
            prime_sieve: Sieve::new(),
            cosmic_tempo: 120.0, // BPM in mathematical space
            reality_stability: 0.7,
            last_prime_index: 0,
            beat_accumulator: 0.0,
            phase_offset: fastrand::f32() * std::f32::consts::TAU,
        }
    }

    pub fn update(&mut self, cosmic_time: f64) -> BeatState {
        // Calculate prime-based rhythm
        let prime_factor = self.calculate_prime_influence(cosmic_time);

        // Update cosmic tempo based on prime harmonics
        self.cosmic_tempo = 120.0 + prime_factor * 60.0;

        // Calculate beat phase
        let beat_frequency = self.cosmic_tempo / 60.0; // Convert BPM to Hz
        let phase = ((cosmic_time as f32 * beat_frequency) + self.phase_offset) % 1.0;

        // Determine if this is a beat drop
        let is_beat_drop = self.calculate_beat_drop(phase, prime_factor);

        // Calculate intensity based on mathematical harmonics
        let intensity = self.calculate_intensity(phase, prime_factor, cosmic_time);

        // Update beat accumulator for chaos effects
        if is_beat_drop {
            self.beat_accumulator += intensity;
        }
        self.beat_accumulator *= 0.95; // Decay

        // Calculate cosmic frequency for other systems
        let cosmic_frequency = self.calculate_cosmic_frequency(cosmic_time, prime_factor);

        BeatState {
            is_beat_drop,
            intensity,
            phase,
            prime_factor,
            cosmic_frequency,
        }
    }

    fn calculate_prime_influence(&mut self, cosmic_time: f64) -> f32 {
        // Use time to generate prime sequence position
        let time_scaled = (cosmic_time * 0.5) as usize;

        // Get next prime if we've advanced
        if time_scaled > self.last_prime_index {
            self.last_prime_index = time_scaled;
        }

        // Generate prime at current position
        let prime = self.prime_sieve.iter().nth(self.last_prime_index % 1000).unwrap_or(2); // Cycle through first 1000 primes

        // Convert prime to influence factor (0.0 to 1.0)
        let normalized_prime = (prime % 100) as f32 / 100.0;

        // Add some chaos through prime modular arithmetic
        let prime_chaos = ((prime % 7) as f32 / 7.0) * 0.3;

        (normalized_prime + prime_chaos).min(1.0)
    }

    fn calculate_beat_drop(&self, phase: f32, prime_factor: f32) -> bool {
        // Beat drops occur at phase boundaries with prime influence
        let beat_threshold = self.reality_stability - prime_factor * 0.3;

        // Primary beat on phase cycle
        let primary_beat = phase < 0.1 || phase > 0.9;

        // Secondary beats based on prime harmonics
        let harmonic_beat = {
            let harmonic_phase = (phase * 4.0) % 1.0;
            harmonic_phase < 0.05 && prime_factor > 0.6
        };

        // Chaos beats when prime factor is very high
        let chaos_beat = prime_factor > 0.9 && fastrand::f32() < 0.1;

        primary_beat || harmonic_beat || chaos_beat
    }

    fn calculate_intensity(&self, phase: f32, prime_factor: f32, cosmic_time: f64) -> f32 {
        // Base intensity from sine wave
        let base_intensity = (phase * std::f32::consts::TAU).sin().abs();

        // Prime modulation
        let prime_modulation = 0.5 + prime_factor * 0.5;

        // Cosmic chaos injection
        let chaos_factor = ((cosmic_time * 0.1).sin() * (cosmic_time * 0.07).cos()).abs();

        // Accumulator influence
        let accumulator_boost = (self.beat_accumulator * 0.1).min(0.5);

        (base_intensity * prime_modulation + chaos_factor as f32 * 0.3 + accumulator_boost).min(1.0)
    }

    fn calculate_cosmic_frequency(&self, cosmic_time: f64, prime_factor: f32) -> f32 {
        // Fundamental frequency based on mathematical constants
        let base_freq = 440.0; // A4 note

        // Prime harmonic scaling
        let prime_harmonic = 1.0 + prime_factor * 0.5;

        // Time-based modulation using golden ratio
        let golden_ratio = 1.618033988749;
        let time_mod = ((cosmic_time * golden_ratio * 0.1) % 1.0) as f32;

        base_freq * prime_harmonic * (1.0 + time_mod * 0.1)
    }

    pub fn current_intensity(&self) -> f32 {
        self.beat_accumulator
    }

    pub fn set_reality_stability(&mut self, stability: f32) {
        self.reality_stability = stability.clamp(0.0, 1.0);
    }

    pub fn add_chaos(&mut self, chaos_amount: f32) {
        self.beat_accumulator += chaos_amount;
        self.phase_offset += chaos_amount * 0.1;
        self.phase_offset %= std::f32::consts::TAU;
    }

    // Get mathematical harmonics for other systems
    pub fn get_harmonic_series(&self, fundamental: f32, count: usize) -> Vec<f32> {
        let mut harmonics = Vec::with_capacity(count);

        for i in 1..=count {
            let harmonic = fundamental * i as f32;

            // Apply prime number filtering
            let prime_index = i % 100;
            let prime = self.prime_sieve.iter().nth(prime_index).unwrap_or(2);
            let prime_factor = (prime % 12) as f32 / 12.0;

            let modulated_harmonic = harmonic * (1.0 + prime_factor * 0.1);
            harmonics.push(modulated_harmonic);
        }

        harmonics
    }

    // Generate mathematical beat pattern for visualization
    pub fn get_beat_pattern(&self, samples: usize, time_window: f32) -> Vec<f32> {
        let mut pattern = Vec::with_capacity(samples);

        for i in 0..samples {
            let t = (i as f32 / samples as f32) * time_window;
            let phase = (t * self.cosmic_tempo / 60.0) % 1.0;
            let intensity = (phase * std::f32::consts::TAU).sin().abs();
            pattern.push(intensity);
        }

        pattern
    }
}