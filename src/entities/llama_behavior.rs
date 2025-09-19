// Llama behavior module - comprehensive movement, social, and interaction systems
// Extracted from simple.rs for better modularity

use glam::Vec2;
use crate::entities::{SpeciesType, Llama};
use crate::engine::{ChaosDecisionEngine, LlamaSnapshot, DecisionVector};

/// Comprehensive llama behavior system implementation
impl Llama {
    /// Main behavior update function that orchestrates all llama behaviors
    pub fn update_behavior(&mut self, dt: f32, beat_intensity: f32, all_llamas: &[Llama],
                          my_index: usize, cosmic_time: f64) {
        // === Phase 2: Mathematical Chaos Engine Update ===
        let snapshot = LlamaSnapshot {
            color: self.color,
            awareness_level: self.awareness_level,
            social_attraction: self.social_attraction,
            exploration_drive: self.exploration_drive,
            reality_distortion: self.reality_distortion,
            emotional_state: self.emotional_state,
            memory_intensity: self.memory_intensity,
            consciousness: self.consciousness,
        };

        self.chaos_engine.update(snapshot, beat_intensity, cosmic_time);
        let decision_vector = self.chaos_engine.get_decision_vector();
        self.prime_chaos_factor = self.chaos_engine.calculate_prime_chaos(cosmic_time);

        // Update quantum state for Quantum Sheep
        if self.species == SpeciesType::QuantumSheep {
            self.quantum_state = (self.quantum_state + dt * 2.0 + self.prime_chaos_factor) % 1.0;
        }

        // Update harmonic resonance
        self.harmonic_resonance = self.chaos_engine.harmonic_resonance.iter().sum::<f32>() / 7.0;

        // Update all behavior systems
        self.update_consciousness_evolution(dt, beat_intensity, decision_vector);
        self.update_memory_system(dt, beat_intensity);
        self.update_movement_behavior(dt, all_llamas, my_index, decision_vector, cosmic_time);
        self.update_color_psychology(dt, decision_vector, cosmic_time);
        self.update_trip_intensity(beat_intensity, decision_vector);
    }

    /// Enhanced consciousness evolution with 11D decision factors
    fn update_consciousness_evolution(&mut self, dt: f32, beat_intensity: f32,
                                    decision_vector: DecisionVector) {
        // Awareness grows with time, beat intensity, and 11D decision factors
        let base_growth = 0.01 * (1.0 + beat_intensity) * self.personality_matrix[0]; // Curiosity trait
        let chaos_growth = self.prime_chaos_factor * 0.005; // Prime chaos accelerates consciousness
        let decision_growth = decision_vector.chaos_acceptance * 0.003; // Chaos acceptance helps growth

        let total_growth = base_growth + chaos_growth + decision_growth;
        self.awareness_level = (self.awareness_level + total_growth).min(1.0);

        // Consciousness affects overall awareness with species modulation
        let species_multiplier = match self.species {
            SpeciesType::DiscoLlama => 1.0,
            SpeciesType::QuantumSheep => 1.3,
            SpeciesType::HypnoCamel => 0.8,
        };
        self.consciousness += total_growth * species_multiplier;
    }

    /// Memory formation and management system
    fn update_memory_system(&mut self, dt: f32, beat_intensity: f32) {
        // Form memories of interesting locations (high beat intensity or social interactions)
        let memory_threshold = 0.7 + self.personality_matrix[3] * 0.3; // Memory strength affects threshold
        if beat_intensity > memory_threshold || self.social_attraction > 0.8 {
            // Only store memory if this location is significantly different from existing memories
            let should_remember = self.memory_fragments.is_empty() ||
                self.memory_fragments.iter().all(|mem| mem.distance(self.position) > 50.0);

            if should_remember && self.memory_fragments.len() < 10 {
                self.memory_fragments.push(self.position);
                self.memory_intensity = (self.memory_intensity + 0.1).min(1.0);
            }
        }
    }

    /// Comprehensive movement behavior system
    fn update_movement_behavior(&mut self, dt: f32, all_llamas: &[Llama], my_index: usize,
                               decision_vector: DecisionVector, cosmic_time: f64) {
        // Calculate movement forces
        let memory_influence = self.calculate_memory_influence();
        let exploration_force = self.calculate_exploration_force(decision_vector, cosmic_time);
        let social_force = self.calculate_social_force(all_llamas, my_index, decision_vector);

        // Apply species-specific movement patterns
        self.apply_species_movement(dt, memory_influence + exploration_force + social_force,
                                  decision_vector, cosmic_time);

        // Apply reality distortion effects
        self.apply_reality_distortion_to_movement(dt);

        // Update position and handle boundaries
        self.update_position_and_boundaries(dt);

        // Update emotional state based on movement
        self.update_emotional_state(dt, social_force, memory_influence);
    }

    /// Calculate memory-driven movement influence
    fn calculate_memory_influence(&self) -> Vec2 {
        let mut memory_influence = Vec2::ZERO;
        if !self.memory_fragments.is_empty() && fastrand::f32() < self.memory_intensity * 0.1 {
            // Sometimes return to interesting memories
            let target_memory = &self.memory_fragments[fastrand::usize(0..self.memory_fragments.len())];
            let to_memory = *target_memory - self.position;
            if to_memory.length() > 10.0 {
                memory_influence = to_memory.normalize() * 30.0 * self.personality_matrix[3]; // Memory strength
            }
        }
        memory_influence
    }

    /// Calculate exploration force with 11D decision engine
    fn calculate_exploration_force(&self, decision_vector: DecisionVector, cosmic_time: f64) -> Vec2 {
        let base_exploration = decision_vector.exploration_drive * self.exploration_drive;
        let chaos_exploration = self.prime_chaos_factor * 0.3;
        let exploration_strength = (base_exploration + chaos_exploration) * 50.0;

        match self.species {
            SpeciesType::DiscoLlama => Vec2::new(
                (fastrand::f32() - 0.5) * exploration_strength,
                (fastrand::f32() - 0.5) * exploration_strength,
            ),
            SpeciesType::QuantumSheep => {
                // Quantum sheep explore in quantum superposition
                let quantum_angle = self.quantum_state * std::f32::consts::TAU;
                Vec2::new(
                    quantum_angle.cos() * exploration_strength * 0.7,
                    quantum_angle.sin() * exploration_strength * 0.7,
                )
            },
            SpeciesType::HypnoCamel => {
                // Hypno camels explore in spiral patterns
                let time_factor = cosmic_time as f32 * 0.5;
                let spiral_angle = time_factor + self.position.length() * 0.01;
                Vec2::new(
                    spiral_angle.cos() * exploration_strength * 0.5,
                    spiral_angle.sin() * exploration_strength * 0.5,
                )
            },
        }
    }

    /// Calculate social forces with species interactions
    fn calculate_social_force(&mut self, all_llamas: &[Llama], my_index: usize,
                             decision_vector: DecisionVector) -> Vec2 {
        let mut social_force = Vec2::ZERO;
        let mut nearby_count = 0;
        let base_social_range = 100.0 + self.personality_matrix[1] * 50.0;
        let social_range = base_social_range * decision_vector.social_attraction;

        for (i, other) in all_llamas.iter().enumerate() {
            if i == my_index { continue; }

            let distance = self.position.distance(other.position);
            if distance < social_range && distance > 0.1 {
                let to_other = other.position - self.position;
                nearby_count += 1;

                // Species-specific interaction modifiers
                let interaction_strength = self.calculate_species_interaction(&other.species);

                // Attraction force (modified by personality and species)
                if distance > 30.0 {
                    social_force += to_other.normalize() * self.social_attraction * 20.0 * interaction_strength;
                } else {
                    // Repulsion when too close, modulated by interaction strength
                    social_force -= to_other.normalize() * 10.0 / interaction_strength;
                }

                // Form social bonds with species consideration
                if distance < 60.0 && !self.social_bonds.contains(&i) && self.social_bonds.len() < 5 {
                    if interaction_strength > 0.5 { // Only bond with compatible species
                        self.social_bonds.push(i);
                    }
                }
            }
        }

        // Update social attraction based on nearby llamas
        if nearby_count > 0 {
            self.social_attraction = (self.social_attraction + 0.02).min(1.0);
        } else {
            self.social_attraction *= 0.99; // Slowly decay when alone
        }

        social_force
    }

    /// Apply species-specific movement patterns
    fn apply_species_movement(&mut self, dt: f32, total_force: Vec2,
                             decision_vector: DecisionVector, cosmic_time: f64) {
        let personality_velocity_mod = 1.0 + self.personality_matrix[6] * 0.5;
        let decision_velocity_mod = 1.0 + decision_vector.movement_urgency * 0.3;
        let chaos_velocity_mod = 1.0 + self.prime_chaos_factor * 0.2;

        let total_velocity_mod = personality_velocity_mod * decision_velocity_mod * chaos_velocity_mod;

        // Species-specific movement patterns
        match self.species {
            SpeciesType::DiscoLlama => {
                self.velocity += total_force * dt * total_velocity_mod;
            },
            SpeciesType::QuantumSheep => {
                // Quantum sheep have probabilistic movement
                if fastrand::f32() < self.quantum_state {
                    // Quantum tunneling - sudden position shifts
                    let quantum_jump = Vec2::new(
                        (fastrand::f32() - 0.5) * 100.0,
                        (fastrand::f32() - 0.5) * 100.0,
                    );
                    self.velocity += (total_force + quantum_jump) * dt * total_velocity_mod;
                } else {
                    self.velocity += total_force * dt * total_velocity_mod * 0.7; // Slower normal movement
                }
            },
            SpeciesType::HypnoCamel => {
                // Hypno camels have rhythmic movement
                let hypno_rhythm = (cosmic_time as f32 * 3.0).sin() * 0.5 + 0.5;
                self.velocity += total_force * dt * total_velocity_mod * hypno_rhythm;
            },
        }
    }

    /// Apply reality distortion effects to movement
    fn apply_reality_distortion_to_movement(&mut self, dt: f32) {
        // Enhanced Reality Distortion with 11D Chaos
        let base_distortion = self.awareness_level * self.personality_matrix[2];
        let chaos_distortion = self.prime_chaos_factor;
        let harmonic_distortion = self.harmonic_resonance * 0.3;

        let species_distortion_mod = match self.species {
            SpeciesType::DiscoLlama => 1.0,
            SpeciesType::QuantumSheep => 1.5 + self.quantum_state * 0.5,
            SpeciesType::HypnoCamel => 0.7,
        };

        self.reality_distortion = (base_distortion + chaos_distortion + harmonic_distortion) * species_distortion_mod;

        // Apply reality distortion to movement
        if self.reality_distortion > 0.1 {
            let distortion_angle = self.reality_distortion * 10.0;
            let cos_d = distortion_angle.cos();
            let sin_d = distortion_angle.sin();
            let distorted_vel = Vec2::new(
                self.velocity.x * cos_d - self.velocity.y * sin_d,
                self.velocity.x * sin_d + self.velocity.y * cos_d,
            );
            self.velocity = self.velocity.lerp(distorted_vel, self.reality_distortion * 0.3);
        }
    }

    /// Update position and handle screen boundaries
    fn update_position_and_boundaries(&mut self, dt: f32) {
        // Apply velocity damping
        self.velocity *= 0.98;

        // Update position
        self.position += self.velocity * dt;

        // Wrap around screen with reality distortion effects
        let wrap_margin = if self.reality_distortion > 0.3 { 50.0 } else { 0.0 };
        if self.position.x < -wrap_margin { self.position.x = 1200.0 + wrap_margin; }
        if self.position.x > 1200.0 + wrap_margin { self.position.x = -wrap_margin; }
        if self.position.y < -wrap_margin { self.position.y = 800.0 + wrap_margin; }
        if self.position.y > 800.0 + wrap_margin { self.position.y = -wrap_margin; }
    }

    /// Update emotional state based on movement and social interactions
    fn update_emotional_state(&mut self, dt: f32, social_force: Vec2, memory_influence: Vec2) {
        let emotional_volatility = self.personality_matrix[4];
        let emotion_change = (emotional_volatility * 0.1) +
                           (social_force.length() * 0.01) +
                           (memory_influence.length() * 0.005);
        self.emotional_state = ((self.emotional_state + emotion_change).sin() + 1.0) * 0.5; // Keep in 0-1 range
    }

    /// Enhanced color psychology with mathematical chaos
    fn update_color_psychology(&mut self, dt: f32, decision_vector: DecisionVector, cosmic_time: f64) {
        // Color reflects emotional state, consciousness, personality, and 11D chaos
        let base_hue_shift = 1.0 + self.personality_matrix[0] * 2.0;
        let emotional_hue_offset = self.emotional_state * 60.0;
        let chaos_hue_offset = self.prime_chaos_factor * 120.0; // Prime chaos affects hue dramatically
        let harmonic_hue_offset = self.harmonic_resonance * 40.0;

        let consciousness_saturation = 0.5 + self.awareness_level * 0.5;
        let chaos_saturation_mod = decision_vector.chaos_acceptance * 0.3;

        // Species-specific color behavior
        match self.species {
            SpeciesType::DiscoLlama => {
                self.color.x = (self.color.x + base_hue_shift + emotional_hue_offset * dt + chaos_hue_offset * dt) % 360.0;
                self.color.y = (consciousness_saturation + chaos_saturation_mod).clamp(0.0, 1.0);
            },
            SpeciesType::QuantumSheep => {
                // Quantum sheep shift through purple spectrum with quantum fluctuations
                let quantum_hue_base = 270.0;
                let quantum_variance = self.quantum_state * 60.0;
                self.color.x = (quantum_hue_base + quantum_variance + chaos_hue_offset * dt) % 360.0;
                self.color.y = (0.9 + self.quantum_state * 0.1).clamp(0.0, 1.0);
            },
            SpeciesType::HypnoCamel => {
                // Hypno camels stay in warm spectrum but pulse with rhythm
                let hypno_base = 30.0; // Orange base
                let rhythm_shift = (cosmic_time as f32 * 2.0).sin() * 30.0;
                self.color.x = (hypno_base + rhythm_shift + harmonic_hue_offset * dt) % 360.0;
                self.color.y = (0.7 + (cosmic_time as f32 * 3.0).sin().abs() * 0.3).clamp(0.0, 1.0);
            },
        }
    }

    /// Enhanced trip intensity calculation with mathematical chaos
    fn update_trip_intensity(&mut self, beat_intensity: f32, decision_vector: DecisionVector) {
        // Combines beat, consciousness, reality distortion, and 11D chaos
        let base_trip = 1.0 + beat_intensity.sin() * 0.5;
        let consciousness_amplification = 1.0 + self.awareness_level * 0.7;
        let reality_amplification = 1.0 + self.reality_distortion * 0.5;
        let chaos_amplification = 1.0 + decision_vector.chaos_acceptance * 0.4;
        let prime_amplification = 1.0 + self.prime_chaos_factor * 0.3;
        let harmonic_amplification = 1.0 + self.harmonic_resonance * 0.2;

        self.trip_intensity = base_trip * consciousness_amplification * reality_amplification *
                             chaos_amplification * prime_amplification * harmonic_amplification;
    }

    /// Social bonding behavior system
    pub fn update_social_bonds(&mut self, all_llamas: &[Llama], my_index: usize) {
        // Remove bonds that are too far away or no longer exist
        self.social_bonds.retain(|&bond_idx| {
            if bond_idx >= all_llamas.len() || bond_idx == my_index {
                return false;
            }

            let distance = self.position.distance(all_llamas[bond_idx].position);
            distance < 150.0 // Maximum bond distance
        });

        // Strengthen existing bonds over time
        for &bond_idx in &self.social_bonds {
            if bond_idx < all_llamas.len() {
                let other = &all_llamas[bond_idx];
                let distance = self.position.distance(other.position);

                // Bonds strengthen when llamas are near each other
                if distance < 80.0 {
                    self.social_attraction = (self.social_attraction + 0.001).min(1.0);
                }
            }
        }
    }

    /// Territorial behavior system
    pub fn update_territorial_behavior(&mut self, all_llamas: &[Llama], my_index: usize, dt: f32) {
        let mut territorial_pressure = 0.0;
        let territory_range = 120.0 + self.territorial_dominance * 80.0;

        for (i, other) in all_llamas.iter().enumerate() {
            if i == my_index { continue; }

            let distance = self.position.distance(other.position);
            if distance < territory_range {
                // Different species create more territorial pressure
                let species_pressure = if other.species != self.species { 2.0 } else { 0.5 };
                let proximity_pressure = (territory_range - distance) / territory_range;
                territorial_pressure += species_pressure * proximity_pressure;
            }
        }

        // Adjust territorial dominance based on pressure
        if territorial_pressure > 1.0 {
            self.territorial_dominance = (self.territorial_dominance + dt * 0.1).min(1.0);
        } else {
            self.territorial_dominance *= 0.999; // Slow decay when not pressured
        }

        // Territorial dominance affects behavior
        if self.territorial_dominance > 0.7 {
            // More aggressive movement when highly territorial
            self.exploration_drive = (self.exploration_drive + dt * 0.05).min(1.0);
            self.social_attraction *= 0.95; // Less social when territorial
        }
    }

    /// Flocking behavior for pack consciousness
    pub fn update_flocking_behavior(&mut self, all_llamas: &[Llama], my_index: usize) -> Vec2 {
        let mut separation = Vec2::ZERO;
        let mut alignment = Vec2::ZERO;
        let mut cohesion = Vec2::ZERO;
        let mut neighbor_count = 0;

        let flock_radius = 100.0 + self.social_attraction * 50.0;

        for (i, other) in all_llamas.iter().enumerate() {
            if i == my_index { continue; }

            let distance = self.position.distance(other.position);
            if distance < flock_radius && distance > 0.1 {
                neighbor_count += 1;

                // Separation: steer to avoid crowding local flockmates
                if distance < 30.0 {
                    let diff = (self.position - other.position).normalize_or_zero();
                    separation += diff / distance; // Weight by distance
                }

                // Alignment: steer towards the average heading of neighbors
                alignment += other.velocity;

                // Cohesion: steer to move toward the average position of neighbors
                cohesion += other.position;
            }
        }

        if neighbor_count > 0 {
            // Average the vectors
            alignment /= neighbor_count as f32;
            cohesion /= neighbor_count as f32;

            // Calculate steering forces
            let cohesion_force = (cohesion - self.position).normalize_or_zero() * 10.0;
            let alignment_force = alignment.normalize_or_zero() * 5.0;
            let separation_force = separation.normalize_or_zero() * 20.0;

            separation_force + alignment_force * 0.5 + cohesion_force * 0.3
        } else {
            Vec2::ZERO
        }
    }

    /// Predator-prey behavior for consciousness predation
    pub fn update_predator_behavior(&mut self, all_llamas: &[Llama], my_index: usize) -> Vec2 {
        let mut predator_force = Vec2::ZERO;

        // High consciousness llamas become predators
        if self.consciousness > 1.5 && self.predation_target.is_none() {
            // Look for suitable prey
            for (i, potential_prey) in all_llamas.iter().enumerate() {
                if i == my_index { continue; }

                let distance = self.position.distance(potential_prey.position);
                if distance < 100.0 && potential_prey.consciousness < self.consciousness * 0.7 {
                    // Target found - move towards prey
                    let to_prey = potential_prey.position - self.position;
                    predator_force += to_prey.normalize_or_zero() * 15.0;
                    break;
                }
            }
        }

        // Low consciousness llamas flee from predators
        if self.consciousness < 1.0 {
            for (i, potential_predator) in all_llamas.iter().enumerate() {
                if i == my_index { continue; }

                if potential_predator.consciousness > self.consciousness * 1.3 {
                    let distance = self.position.distance(potential_predator.position);
                    if distance < 80.0 {
                        // Flee from predator
                        let away_from_predator = (self.position - potential_predator.position).normalize_or_zero();
                        predator_force += away_from_predator * 25.0 / (distance + 1.0);
                    }
                }
            }
        }

        predator_force
    }

    /// Emergent behavior patterns based on consciousness level
    pub fn update_emergent_behaviors(&mut self, cosmic_time: f64) {
        // High consciousness entities develop new behaviors
        if self.consciousness > 1.2 && fastrand::f32() < 0.001 {
            // Consciousness breakthrough moments
            self.awareness_level = (self.awareness_level + 0.1).min(1.0);

            // Reality distortion events
            if self.consciousness > 1.5 {
                self.reality_distortion += 0.2;
            }
        }

        // Rhythmic behaviors emerge at specific consciousness levels
        if self.consciousness > 0.8 {
            let rhythm_phase = (cosmic_time as f32 * self.consciousness * 2.0).sin();
            if rhythm_phase > 0.9 {
                // Synchronization pulses
                self.trip_intensity += 0.3;
            }
        }

        // Collective behaviors emerge when multiple systems align
        if self.harmonic_resonance > 0.7 && self.social_attraction > 0.8 && self.awareness_level > 0.9 {
            // Transcendent state - enhanced all abilities
            self.exploration_drive = (self.exploration_drive + 0.05).min(1.0);
            self.memory_intensity = (self.memory_intensity + 0.03).min(1.0);
            self.social_attraction = (self.social_attraction + 0.02).min(1.0);
        }
    }
}

/// Behavioral analysis utilities
impl Llama {
    /// Calculate behavioral complexity score
    pub fn calculate_behavioral_complexity(&self) -> f32 {
        let personality_diversity = self.personality_matrix.iter()
            .map(|&x| (x - 0.5).abs())
            .sum::<f32>() / 7.0;

        let consciousness_factor = self.consciousness.min(1.0);
        let social_factor = self.social_bonds.len() as f32 * 0.1;
        let memory_factor = self.memory_fragments.len() as f32 * 0.05;
        let chaos_factor = self.prime_chaos_factor;

        (personality_diversity + consciousness_factor + social_factor + memory_factor + chaos_factor) / 5.0
    }

    /// Get current behavior state as a descriptive string
    pub fn get_behavior_state_description(&self) -> String {
        let mut behaviors = Vec::new();

        if self.consciousness > 1.5 {
            behaviors.push("Highly Conscious");
        }
        if self.social_attraction > 0.8 {
            behaviors.push("Social");
        }
        if self.exploration_drive > 0.8 {
            behaviors.push("Exploratory");
        }
        if self.territorial_dominance > 0.7 {
            behaviors.push("Territorial");
        }
        if self.reality_distortion > 0.5 {
            behaviors.push("Reality-Bending");
        }
        if self.memory_intensity > 0.8 {
            behaviors.push("Memory-Rich");
        }
        if self.prime_chaos_factor > 0.8 {
            behaviors.push("Chaos-Influenced");
        }

        if behaviors.is_empty() {
            "Baseline".to_string()
        } else {
            behaviors.join(", ")
        }
    }

    /// Check if llama is in a transcendent state
    pub fn is_transcendent(&self) -> bool {
        self.consciousness > 1.8 &&
        self.awareness_level > 0.9 &&
        self.reality_distortion > 0.7 &&
        self.harmonic_resonance > 0.8
    }

    /// Calculate influence radius for consciousness effects
    pub fn get_consciousness_influence_radius(&self) -> f32 {
        let base_radius = 50.0;
        let consciousness_multiplier = 1.0 + self.consciousness;
        let awareness_multiplier = 1.0 + self.awareness_level * 0.5;
        let reality_multiplier = 1.0 + self.reality_distortion * 0.3;

        base_radius * consciousness_multiplier * awareness_multiplier * reality_multiplier
    }
}