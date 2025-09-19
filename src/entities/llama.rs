// Llama entity definition and core behavior
// Extracted from simple.rs for better modularity

use glam::Vec2;
use crate::entities::species::{SpeciesType, ConsciousnessLevel};
use crate::engine::ChaosDecisionEngine;
use crate::simulation::{CrystalType, ZoneType, ConsciousnessField, ConsciousnessCrystal, TerritoryEffects};

/// Main llama entity with all consciousness and behavioral systems
#[derive(Clone)]
pub struct Llama {
    // Core properties
    pub position: Vec2,
    pub velocity: Vec2,
    pub color: Vec2, // hue, saturation
    pub consciousness: f32,
    pub trip_intensity: f32,

    // Phase 1: Consciousness Depth Enhancement
    pub awareness_level: f32,           // 0.0-1.0 consciousness depth
    pub memory_fragments: Vec<Vec2>,    // Historical position memories
    pub social_bonds: Vec<usize>,       // Connections to other llamas
    pub personality_matrix: [f32; 7],   // 7-dimensional personality traits
    pub reality_distortion: f32,        // Local space-time manipulation factor

    // Internal consciousness state
    pub emotional_state: f32,           // Current emotional resonance
    pub memory_intensity: f32,          // How strongly memories are forming
    pub social_attraction: f32,         // Desire to be near other llamas
    pub exploration_drive: f32,         // Tendency to seek new areas

    // Phase 2: Mathematical Chaos Engine
    pub species: SpeciesType,           // Species determines behavior patterns
    pub chaos_engine: ChaosDecisionEngine, // 11D decision engine
    pub quantum_state: f32,             // Quantum superposition factor
    pub harmonic_resonance: f32,        // Musical mathematics coupling
    pub prime_chaos_factor: f32,        // Current prime number influence

    // Phase 3: Ecosystem Emergence
    pub harvested_crystals: Vec<CrystalType>, // Types of crystals harvested
    pub mutation_count: u32,            // How many mutations this llama has undergone
    pub environmental_consciousness: f32, // Consciousness absorbed from environment
    pub territory_affinity: Option<ZoneType>, // Preferred territory type

    // Phase 5: Consciousness Multiplication
    pub consciousness_level: ConsciousnessLevel, // Individual/Pack/Hive/Meta hierarchy
    pub collective_id: Option<usize>,     // ID of collective consciousness if part of one
    pub territorial_dominance: f32,       // Individual territorial control strength
    pub warfare_participation: f32,       // How actively engaged in species warfare
    pub absorption_resistance: f32,       // Resistance to consciousness predation
    pub hive_connection_strength: f32,    // Strength of connection to hive mind
    pub predation_target: Option<usize>,  // Current target for consciousness absorption
    pub extinction_pressure: f32,         // Environmental pressure affecting this entity
    pub war_efficiency: f32,              // Combat effectiveness in consciousness warfare
}

impl Llama {
    /// Create a new llama with default species
    pub fn new(position: Vec2) -> Self {
        Self::new_with_species(position, SpeciesType::DiscoLlama)
    }

    /// Create a new llama with specific species
    pub fn new_with_species(position: Vec2, species: SpeciesType) -> Self {
        // Generate unique personality matrix (7 traits)
        let personality_matrix = [
            fastrand::f32(),        // Curiosity
            fastrand::f32(),        // Sociability
            fastrand::f32(),        // Chaos affinity
            fastrand::f32(),        // Memory strength
            fastrand::f32(),        // Emotional volatility
            fastrand::f32(),        // Reality sensitivity
            fastrand::f32(),        // Exploration drive
        ];

        // Get species configuration
        let config = species.get_base_config();
        let hue = fastrand::f32() * (config.base_hue_range.1 - config.base_hue_range.0) + config.base_hue_range.0;
        let base_color = Vec2::new(hue, config.base_saturation);

        Self {
            // Core properties
            position,
            velocity: Vec2::new(
                (fastrand::f32() - 0.5) * config.velocity_modifier,
                (fastrand::f32() - 0.5) * config.velocity_modifier,
            ),
            color: base_color,
            consciousness: fastrand::f32() * 0.5 + 0.3, // 0.3-0.8 base consciousness
            trip_intensity: fastrand::f32() * 0.8,

            // Phase 1: Consciousness Depth Enhancement
            awareness_level: personality_matrix[0] * config.consciousness_modifier, // Based on curiosity
            memory_fragments: Vec::new(),
            social_bonds: Vec::new(),
            personality_matrix,
            reality_distortion: personality_matrix[5] * 0.5, // Based on reality sensitivity

            // Internal consciousness state
            emotional_state: personality_matrix[4] * 0.7,     // Based on emotional volatility
            memory_intensity: personality_matrix[3] * 0.6,    // Based on memory strength
            social_attraction: personality_matrix[1] * 0.8,   // Based on sociability
            exploration_drive: personality_matrix[6] * 0.8,   // Based on exploration trait

            // Phase 2: Mathematical Chaos Engine
            species,
            chaos_engine: ChaosDecisionEngine::new(),
            quantum_state: if config.quantum_affinity { fastrand::f32() } else { 0.0 },
            harmonic_resonance: 0.0,
            prime_chaos_factor: 0.0,

            // Phase 3: Ecosystem Emergence
            harvested_crystals: Vec::new(),
            mutation_count: 0,
            environmental_consciousness: 0.0,
            territory_affinity: None,

            // Phase 5: Consciousness Multiplication
            consciousness_level: ConsciousnessLevel::Individual,
            collective_id: None,
            territorial_dominance: 0.0,
            warfare_participation: 0.0,
            absorption_resistance: config.consciousness_modifier + personality_matrix[4] * 0.3,
            hive_connection_strength: 0.0,
            predation_target: None,
            extinction_pressure: 0.0,
            war_efficiency: config.war_efficiency,
        }
    }

    /// Calculate interaction strength with another species
    pub fn calculate_species_interaction(&self, other_species: &SpeciesType) -> f32 {
        self.species.calculate_interaction_strength(other_species)
    }

    /// Try to harvest a consciousness crystal
    pub fn try_harvest_crystal(&mut self, crystal: &mut ConsciousnessCrystal) -> bool {
        let distance = self.position.distance(crystal.position);
        if distance < crystal.harvest_radius {
            let harvest_amount = crystal.get_harvest_amount();
            if harvest_amount > 0.1 {
                let harvested = crystal.harvest(harvest_amount);

                // Apply crystal effects based on type
                match crystal.crystal_type {
                    CrystalType::Memory => {
                        self.memory_intensity += harvested * 0.3;
                        self.awareness_level += harvested * 0.2;
                        if !self.memory_fragments.contains(&crystal.position) {
                            self.memory_fragments.push(crystal.position);
                        }
                    },
                    CrystalType::Social => {
                        self.social_attraction += harvested * 0.4;
                        self.emotional_state += harvested * 0.2;
                    },
                    CrystalType::Quantum => {
                        if self.species.get_base_config().quantum_affinity {
                            self.quantum_state += harvested * 0.3;
                        }
                        self.consciousness += harvested * 0.4;
                    },
                    CrystalType::Resonance => {
                        self.harmonic_resonance += harvested * 0.5;
                        self.trip_intensity += harvested * 0.3;
                        self.consciousness += harvested * 0.2;
                    },
                    CrystalType::Chaos => {
                        self.exploration_drive += harvested * 0.4;
                        self.reality_distortion += harvested * 0.3;
                        self.prime_chaos_factor += harvested * 0.2;
                    },
                }

                // Store crystal type for mutations
                if !self.harvested_crystals.contains(&crystal.crystal_type) {
                    self.harvested_crystals.push(crystal.crystal_type.clone());
                }

                return true;
            }
        }
        false
    }

    /// Apply territory effects to the llama
    pub fn apply_territory_effects(&mut self, territory_effects: &TerritoryEffects, dt: f32) {
        self.harmonic_resonance += territory_effects.harmonic_boost * dt * 0.1;
        self.social_attraction += territory_effects.social_boost * dt * 0.05;
        self.prime_chaos_factor += territory_effects.chaos_boost * dt * 0.08;
        self.reality_distortion += territory_effects.reality_distortion_boost * dt * 0.06;
        self.memory_intensity += territory_effects.memory_boost * dt * 0.04;
        self.awareness_level += territory_effects.consciousness_growth_boost * dt * 0.02;

        if self.species.get_base_config().quantum_affinity {
            self.quantum_state += territory_effects.quantum_boost * dt * 0.1;
        }

        self.exploration_drive += territory_effects.exploration_boost * dt * 0.03;

        // Clamp values to reasonable ranges
        self.harmonic_resonance = self.harmonic_resonance.clamp(0.0, 2.0);
        self.social_attraction = self.social_attraction.clamp(0.0, 1.0);
        self.prime_chaos_factor = self.prime_chaos_factor.clamp(0.0, 1.0);
        self.reality_distortion = self.reality_distortion.clamp(0.0, 1.0);
        self.memory_intensity = self.memory_intensity.clamp(0.0, 1.0);
        self.awareness_level = self.awareness_level.clamp(0.0, 1.0);
        self.exploration_drive = self.exploration_drive.clamp(0.0, 1.0);

        if self.species.get_base_config().quantum_affinity {
            self.quantum_state = self.quantum_state % 1.0;
        }
    }

    /// Apply mutation based on harvested crystals
    pub fn apply_mutation(&mut self, mutation_strength: f32) {
        self.mutation_count += 1;

        // Mutations based on harvested crystals
        for crystal_type in &self.harvested_crystals.clone() {
            match crystal_type {
                CrystalType::Memory => {
                    // Memory evolution
                    self.personality_matrix[3] += mutation_strength * 0.3; // Memory strength
                    self.memory_intensity += mutation_strength * 0.4;
                },
                CrystalType::Social => {
                    // Social evolution
                    self.personality_matrix[1] += mutation_strength * 0.3; // Sociability
                    self.social_attraction += mutation_strength * 0.3;
                },
                CrystalType::Quantum => {
                    // Quantum evolution (especially for non-quantum species)
                    if !self.species.get_base_config().quantum_affinity {
                        self.quantum_state += mutation_strength * 0.2;
                    }
                    self.consciousness += mutation_strength * 0.5;
                },
                CrystalType::Resonance => {
                    // Harmonic resonance evolution
                    self.personality_matrix[5] += mutation_strength * 0.3; // Artistic/musical affinity
                    self.harmonic_resonance += mutation_strength * 0.4;
                    self.trip_intensity += mutation_strength * 0.2;
                },
                CrystalType::Chaos => {
                    // Chaos acceptance evolution
                    self.personality_matrix[0] += mutation_strength * 0.3; // Adventurousness
                    self.exploration_drive += mutation_strength * 0.4;
                    self.reality_distortion += mutation_strength * 0.3;
                },
            }
        }

        // Clamp personality matrix to reasonable values
        for trait_value in &mut self.personality_matrix {
            *trait_value = trait_value.clamp(0.0, 1.5);
        }
    }

    /// Get rendering data for the psychedelic shader
    pub fn get_render_data(&self) -> LlamaRenderData {
        LlamaRenderData {
            position: self.position,
            color: self.color,
            species_id: self.species.to_shader_id(),
            consciousness: self.awareness_level,
            trip_intensity: self.trip_intensity,
            reality_distortion: self.reality_distortion,
        }
    }

    /// Main update method called from simulation loop
    pub fn update(&mut self, dt: f32, beat_intensity: f32, all_llamas: &[Llama],
                  my_index: usize, cosmic_time: f64) {
        self.update_behavior(dt, beat_intensity, all_llamas, my_index, cosmic_time);
    }
}

/// Data structure for rendering llamas
#[derive(Debug, Clone)]
pub struct LlamaRenderData {
    pub position: Vec2,
    pub color: Vec2,
    pub species_id: f32,
    pub consciousness: f32,
    pub trip_intensity: f32,
    pub reality_distortion: f32,
}