use instant::Instant;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub enum ResonanceEffect {
    Spawn { count: u32, intensity: f32 },
    RealityTear { strength: f32 },
    Ripple { amplitude: f32 },
}

pub struct ConsciousnessResonance {
    click_history: VecDeque<ClickEvent>,
    harmonic_analyzer: HarmonicAnalyzer,
    reality_tear_threshold: f32,
    max_history: usize,
}

#[derive(Debug, Clone)]
struct ClickEvent {
    timestamp: Instant,
    intensity: f32,
}

struct HarmonicAnalyzer {
    frequency_bins: Vec<f32>,
    resonance_patterns: Vec<ResonancePattern>,
}

#[derive(Debug, Clone)]
struct ResonancePattern {
    frequency_range: (f32, f32),
    spawn_multiplier: f32,
    reality_distortion: f32,
    pattern_name: String,
}

impl ConsciousnessResonance {
    pub fn new() -> Self {
        Self {
            click_history: VecDeque::with_capacity(100),
            harmonic_analyzer: HarmonicAnalyzer::new(),
            reality_tear_threshold: 8.0, // Clicks per second threshold
            max_history: 50,
        }
    }

    pub fn process_click(&mut self, timestamp: Instant, button_state: winit::event::ElementState, _button: winit::event::MouseButton) -> ResonanceEffect {
        if matches!(button_state, winit::event::ElementState::Pressed) {
            let click_event = ClickEvent {
                timestamp,
                intensity: 1.0,
            };

            self.add_click(click_event);
            self.analyze_resonance()
        } else {
            ResonanceEffect::Ripple { amplitude: 0.1 }
        }
    }

    fn add_click(&mut self, click: ClickEvent) {
        self.click_history.push_back(click);

        // Remove old clicks (older than 5 seconds)
        let cutoff_time = Instant::now() - std::time::Duration::from_secs(5);
        while let Some(front) = self.click_history.front() {
            if front.timestamp < cutoff_time {
                self.click_history.pop_front();
            } else {
                break;
            }
        }

        // Limit history size
        while self.click_history.len() > self.max_history {
            self.click_history.pop_front();
        }
    }

    fn analyze_resonance(&mut self) -> ResonanceEffect {
        if self.click_history.len() < 2 {
            return ResonanceEffect::Ripple { amplitude: 0.3 };
        }

        let frequency = self.calculate_click_frequency();
        let harmonic = self.harmonic_analyzer.analyze(frequency);
        let rhythm_stability = self.calculate_rhythm_stability();

        // Determine effect based on frequency and patterns
        if frequency > self.reality_tear_threshold {
            // Very fast clicking creates reality tears
            let strength = (frequency / self.reality_tear_threshold - 1.0).min(2.0);
            ResonanceEffect::RealityTear { strength }
        } else if harmonic.is_harmonic() {
            // Harmonic patterns spawn llamas
            let spawn_count = self.calculate_spawn_count(&harmonic, rhythm_stability);
            let intensity = harmonic.intensity * rhythm_stability;
            ResonanceEffect::Spawn {
                count: spawn_count,
                intensity,
            }
        } else {
            // Random clicks create ripples
            let amplitude = (frequency / 3.0).min(1.0);
            ResonanceEffect::Ripple { amplitude }
        }
    }

    fn calculate_click_frequency(&self) -> f32 {
        if self.click_history.len() < 2 {
            return 0.0;
        }

        let now = Instant::now();
        let recent_clicks: Vec<&ClickEvent> = self.click_history
            .iter()
            .filter(|click| now.duration_since(click.timestamp).as_secs_f32() <= 2.0)
            .collect();

        if recent_clicks.len() < 2 {
            return 0.0;
        }

        // Calculate clicks per second over recent period
        let time_span = now.duration_since(recent_clicks[0].timestamp).as_secs_f32();
        if time_span > 0.0 {
            recent_clicks.len() as f32 / time_span
        } else {
            0.0
        }
    }

    fn calculate_rhythm_stability(&self) -> f32 {
        if self.click_history.len() < 3 {
            return 0.5;
        }

        // Calculate variance in click intervals
        let mut intervals = Vec::new();
        for i in 1..self.click_history.len() {
            let interval = self.click_history[i].timestamp
                .duration_since(self.click_history[i - 1].timestamp)
                .as_secs_f32();
            intervals.push(interval);
        }

        if intervals.is_empty() {
            return 0.5;
        }

        // Calculate coefficient of variation (lower = more stable)
        let mean = intervals.iter().sum::<f32>() / intervals.len() as f32;
        let variance = intervals.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f32>() / intervals.len() as f32;
        let std_dev = variance.sqrt();

        let cv = if mean > 0.0 { std_dev / mean } else { 1.0 };

        // Convert to stability (0.0 = chaotic, 1.0 = perfectly stable)
        (1.0 - cv.min(1.0)).max(0.0)
    }

    fn calculate_spawn_count(&self, harmonic: &HarmonicResult, stability: f32) -> u32 {
        let base_count = match harmonic.pattern {
            Some(ref pattern) => pattern.spawn_multiplier,
            None => 1.0,
        };

        let frequency_bonus = (self.calculate_click_frequency() / 2.0).min(2.0);
        let stability_bonus = stability * 2.0;

        let total = base_count + frequency_bonus + stability_bonus;
        (total as u32).max(1).min(10) // Cap at 10 llamas per resonance
    }
}

impl HarmonicAnalyzer {
    fn new() -> Self {
        let resonance_patterns = vec![
            ResonancePattern {
                frequency_range: (2.0, 3.0),
                spawn_multiplier: 2.0,
                reality_distortion: 0.3,
                pattern_name: "Zen Rhythm".to_string(),
            },
            ResonancePattern {
                frequency_range: (4.0, 6.0),
                spawn_multiplier: 3.0,
                reality_distortion: 0.5,
                pattern_name: "Disco Beat".to_string(),
            },
            ResonancePattern {
                frequency_range: (7.0, 9.0),
                spawn_multiplier: 4.0,
                reality_distortion: 0.8,
                pattern_name: "Chaos Drums".to_string(),
            },
            ResonancePattern {
                frequency_range: (1.0, 1.5),
                spawn_multiplier: 1.5,
                reality_distortion: 0.2,
                pattern_name: "Meditative Pulse".to_string(),
            },
        ];

        Self {
            frequency_bins: vec![0.0; 64],
            resonance_patterns,
        }
    }

    fn analyze(&mut self, frequency: f32) -> HarmonicResult {
        // Find matching pattern
        let pattern = self.resonance_patterns
            .iter()
            .find(|p| frequency >= p.frequency_range.0 && frequency <= p.frequency_range.1)
            .cloned();

        // Calculate intensity based on how well frequency matches pattern
        let intensity = if let Some(ref p) = pattern {
            let center = (p.frequency_range.0 + p.frequency_range.1) / 2.0;
            let distance = (frequency - center).abs();
            let max_distance = (p.frequency_range.1 - p.frequency_range.0) / 2.0;
            1.0 - (distance / max_distance).min(1.0)
        } else {
            0.3 // Default intensity for non-harmonic
        };

        // Add mathematical harmonics check
        let is_mathematical_harmonic = self.check_mathematical_harmonics(frequency);

        HarmonicResult {
            frequency,
            intensity,
            pattern,
            is_mathematical_harmonic,
        }
    }

    fn check_mathematical_harmonics(&self, frequency: f32) -> bool {
        // Check if frequency matches mathematical constants or their multiples
        let constants = [
            std::f32::consts::PI,
            std::f32::consts::E,
            1.618033988749, // Golden ratio
            1.414213562373, // Square root of 2
        ];

        for &constant in &constants {
            for multiplier in 1..=5 {
                let target = constant * multiplier as f32;
                if (frequency - target).abs() < 0.2 {
                    return true;
                }
            }
        }

        // Check for Fibonacci ratios
        let fib_ratios = [1.0, 1.618, 2.618, 4.236];
        for &ratio in &fib_ratios {
            if (frequency - ratio).abs() < 0.1 {
                return true;
            }
        }

        false
    }
}

#[derive(Debug)]
struct HarmonicResult {
    frequency: f32,
    intensity: f32,
    pattern: Option<ResonancePattern>,
    is_mathematical_harmonic: bool,
}

impl HarmonicResult {
    fn is_harmonic(&self) -> bool {
        self.pattern.is_some() || self.is_mathematical_harmonic
    }
}