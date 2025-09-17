use glam::Vec2;
use crate::core::events::LlamaSpecies;
use crate::mathematics::BeatState;

#[derive(Debug, Clone)]
pub struct PsychedelicLlama {
    pub consciousness_level: f32,
    pub trip_intensity: f32,
    pub color_wavelength: Vec2,  // Hue, Saturation in psychedelic space
    pub reality_distortion: f32,
    pub last_explosion_timestamp: f64,
    pub position: Vec2,
    pub velocity: Vec2,
    pub species: LlamaSpecies,
}

impl PsychedelicLlama {
    pub fn update(&mut self, cosmic_time: f64, beat_state: &BeatState) {
        // Consciousness growth over time
        self.consciousness_level += 0.01 * (1.0 + beat_state.intensity);

        // Update position based on species behavior
        match self.species {
            LlamaSpecies::Disco => {
                // Disco llamas dance to the beat
                if beat_state.is_beat_drop {
                    self.velocity = Vec2::new(
                        (fastrand::f32() - 0.5) * 200.0,
                        (fastrand::f32() - 0.5) * 200.0,
                    );
                    self.trip_intensity = (self.trip_intensity + 1.0).min(5.0);
                }
                self.update_disco_movement(cosmic_time);
            }
            LlamaSpecies::Quantum => {
                // Quantum sheep exist in probability clouds
                self.update_quantum_movement(cosmic_time, beat_state);
            }
            LlamaSpecies::BassDrop => {
                // Bass drop vicuñas sync to mathematical rhythms
                if beat_state.is_beat_drop {
                    self.consciousness_level += 0.5;
                    self.reality_distortion += 0.1;
                }
                self.update_bassdrop_movement(beat_state);
            }
            _ => {
                // Default movement
                self.update_default_movement(cosmic_time);
            }
        }

        // Update color based on consciousness level
        self.update_color_consciousness();

        // Decay trip intensity
        self.trip_intensity *= 0.98;
        self.trip_intensity = self.trip_intensity.max(0.1);

        // Decay reality distortion
        self.reality_distortion *= 0.99;
    }

    fn update_disco_movement(&mut self, cosmic_time: f64) {
        // Circular dancing motion with randomness
        let dance_frequency = 2.0 + self.trip_intensity * 0.5;
        let dance_radius = 30.0 + self.consciousness_level * 5.0;

        let dance_offset = Vec2::new(
            (cosmic_time as f32 * dance_frequency).cos() * dance_radius,
            (cosmic_time as f32 * dance_frequency).sin() * dance_radius,
        );

        self.velocity = self.velocity * 0.9 + dance_offset * 0.1;
        self.position += self.velocity * 0.016; // ~60 FPS

        // Wrap around screen
        self.wrap_position(1200.0, 800.0);
    }

    fn update_quantum_movement(&mut self, cosmic_time: f64, beat_state: &BeatState) {
        // Quantum sheep teleport and exist in multiple dimensions
        if fastrand::f32() < 0.02 * beat_state.intensity {
            // Quantum teleportation
            self.position = Vec2::new(
                fastrand::f32() * 1200.0,
                fastrand::f32() * 800.0,
            );
            self.reality_distortion += 0.2;
        } else {
            // Probability wave movement
            let wave_x = (cosmic_time as f32 * 3.0 + self.position.y * 0.01).sin() * 2.0;
            let wave_y = (cosmic_time as f32 * 2.0 + self.position.x * 0.01).cos() * 2.0;
            self.velocity = Vec2::new(wave_x, wave_y);
            self.position += self.velocity;
        }

        self.wrap_position(1200.0, 800.0);
    }

    fn update_bassdrop_movement(&mut self, beat_state: &BeatState) {
        // Bass drop vicuñas move in sync with mathematical beats
        if beat_state.is_beat_drop {
            // Explosive movement on beat drop
            self.velocity = Vec2::new(
                (fastrand::f32() - 0.5) * 300.0 * beat_state.intensity,
                (fastrand::f32() - 0.5) * 300.0 * beat_state.intensity,
            );
        } else {
            // Gentle drift between beats
            self.velocity *= 0.95;
        }

        self.position += self.velocity * 0.016;
        self.wrap_position(1200.0, 800.0);
    }

    fn update_default_movement(&mut self, cosmic_time: f64) {
        // Simple wandering behavior
        let wander_strength = 0.5;
        let wander_x = (cosmic_time as f32 * 1.5 + self.position.y * 0.005).sin() * wander_strength;
        let wander_y = (cosmic_time as f32 * 1.2 + self.position.x * 0.005).cos() * wander_strength;

        self.velocity += Vec2::new(wander_x, wander_y);
        self.velocity *= 0.98; // Damping
        self.position += self.velocity * 0.016;

        self.wrap_position(1200.0, 800.0);
    }

    fn update_color_consciousness(&mut self) {
        // Hue shifts based on consciousness level
        let hue_shift = self.consciousness_level * 10.0;
        self.color_wavelength.x = (self.color_wavelength.x + hue_shift) % 360.0;

        // Saturation based on trip intensity
        self.color_wavelength.y = (0.5 + self.trip_intensity * 0.1).min(1.0);
    }

    fn wrap_position(&mut self, width: f32, height: f32) {
        if self.position.x < 0.0 {
            self.position.x = width;
        } else if self.position.x > width {
            self.position.x = 0.0;
        }

        if self.position.y < 0.0 {
            self.position.y = height;
        } else if self.position.y > height {
            self.position.y = 0.0;
        }
    }
}

// 11-dimensional AI consciousness for llama decision making
#[derive(Debug)]
pub struct LlamaAI {
    pub decision_dimensions: [f32; 11],
    pub cosmic_mood: CosmicMood,
    pub last_decision_time: f64,
    pub chaos_accumulator: f32,
}

#[derive(Debug, Clone)]
pub enum CosmicMood {
    Euphoric,
    Contemplative,
    Chaotic,
    Harmonious,
    Transcendent,
}

impl LlamaAI {
    pub fn new() -> Self {
        Self {
            decision_dimensions: [fastrand::f32(); 11],
            cosmic_mood: CosmicMood::Harmonious,
            last_decision_time: 0.0,
            chaos_accumulator: 0.0,
        }
    }

    pub fn new_quantum() -> Self {
        let mut ai = Self::new();
        ai.cosmic_mood = CosmicMood::Transcendent;
        // Quantum sheep have higher dimensional variance
        for dim in &mut ai.decision_dimensions {
            *dim = (*dim - 0.5) * 2.0; // Range -1 to 1
        }
        ai
    }

    pub fn new_bassdrop() -> Self {
        let mut ai = Self::new();
        ai.cosmic_mood = CosmicMood::Chaotic;
        // Bass drop focus on rhythm dimensions
        ai.decision_dimensions[0] = 0.9; // Beat sensitivity
        ai.decision_dimensions[1] = 0.8; // Rhythm alignment
        ai
    }

    pub fn update(&mut self, cosmic_time: f64, beat_state: &BeatState, llama: &PsychedelicLlama) {
        if cosmic_time - self.last_decision_time < 0.1 {
            return; // Don't update too frequently
        }

        self.last_decision_time = cosmic_time;

        // Update decision dimensions based on 11D consciousness space
        self.update_decision_dimensions(cosmic_time, beat_state, llama);

        // Update cosmic mood based on reality state
        self.update_cosmic_mood(beat_state, llama);

        // Accumulate chaos for emergent behaviors
        self.chaos_accumulator += beat_state.intensity * 0.1;
        if self.chaos_accumulator > 1.0 {
            self.trigger_chaos_behavior();
            self.chaos_accumulator = 0.0;
        }
    }

    fn update_decision_dimensions(&mut self, cosmic_time: f64, beat_state: &BeatState, llama: &PsychedelicLlama) {
        // Dimension 0: Color temperature of nearby dreams
        self.decision_dimensions[0] = (llama.color_wavelength.x / 360.0).sin();

        // Dimension 1: Prime number fluctuations
        self.decision_dimensions[1] = beat_state.prime_factor * 0.1;

        // Dimension 2: Mercury retrograde (mathematical)
        self.decision_dimensions[2] = ((cosmic_time * 0.01).sin() + 1.0) * 0.5;

        // Dimension 3: Collective llama mood (simplified)
        self.decision_dimensions[3] = llama.consciousness_level / 10.0;

        // Dimension 4: Reality stability
        self.decision_dimensions[4] = 1.0 - llama.reality_distortion;

        // Dimension 5: Trip intensity influence
        self.decision_dimensions[5] = llama.trip_intensity / 5.0;

        // Dimension 6: Beat synchronization
        self.decision_dimensions[6] = if beat_state.is_beat_drop { 1.0 } else { 0.0 };

        // Dimension 7: Spatial harmony
        self.decision_dimensions[7] = (llama.position.length() / 1000.0).sin();

        // Dimension 8: Temporal flow
        self.decision_dimensions[8] = (cosmic_time * 0.1).cos();

        // Dimension 9: Consciousness recursion
        self.decision_dimensions[9] = (llama.consciousness_level.ln() / 10.0).max(0.0);

        // Dimension 10: Pure chaos injection
        self.decision_dimensions[10] = fastrand::f32();
    }

    fn update_cosmic_mood(&mut self, beat_state: &BeatState, llama: &PsychedelicLlama) {
        let mood_factor = self.decision_dimensions.iter().sum::<f32>() / 11.0;

        self.cosmic_mood = match mood_factor {
            x if x > 0.8 => CosmicMood::Transcendent,
            x if x > 0.6 => CosmicMood::Euphoric,
            x if x > 0.4 => CosmicMood::Harmonious,
            x if x > 0.2 => CosmicMood::Contemplative,
            _ => CosmicMood::Chaotic,
        };

        // Beat drops can force chaotic mood
        if beat_state.is_beat_drop && beat_state.intensity > 0.7 {
            self.cosmic_mood = CosmicMood::Chaotic;
        }
    }

    fn trigger_chaos_behavior(&mut self) {
        // Randomize some dimensions for emergent behavior
        for i in 0..3 {
            let idx = fastrand::usize(0..11);
            self.decision_dimensions[idx] = fastrand::f32();
        }

        // Force mood change
        self.cosmic_mood = match fastrand::u8(0..5) {
            0 => CosmicMood::Euphoric,
            1 => CosmicMood::Contemplative,
            2 => CosmicMood::Chaotic,
            3 => CosmicMood::Harmonious,
            _ => CosmicMood::Transcendent,
        };
    }
}

#[derive(Debug, Clone)]
pub enum ConsciousnessLevel {
    Dormant,
    Awakening,
    Enlightened,
    Transcendent,
    CosmicUnity,
}