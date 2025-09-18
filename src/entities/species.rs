// Species types and behavior patterns
// Extracted from simple.rs for better modularity

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum SpeciesType {
    DiscoLlama,
    QuantumSheep,
    HypnoCamel,
}

impl SpeciesType {
    /// Get species-specific base configuration
    pub fn get_base_config(&self) -> SpeciesConfig {
        match self {
            SpeciesType::DiscoLlama => SpeciesConfig {
                base_hue_range: (0.0, 360.0),
                base_saturation: 0.8,
                consciousness_modifier: 1.0,
                velocity_modifier: 100.0,
                war_efficiency: 1.0,
                quantum_affinity: false,
            },
            SpeciesType::QuantumSheep => SpeciesConfig {
                base_hue_range: (270.0, 330.0), // Purple range
                base_saturation: 0.9,
                consciousness_modifier: 1.5,
                velocity_modifier: 50.0,
                war_efficiency: 1.3,
                quantum_affinity: true,
            },
            SpeciesType::HypnoCamel => SpeciesConfig {
                base_hue_range: (30.0, 60.0), // Orange/yellow range
                base_saturation: 0.7,
                consciousness_modifier: 0.8,
                velocity_modifier: 75.0,
                war_efficiency: 0.8,
                quantum_affinity: false,
            },
        }
    }

    /// Calculate interaction strength between species
    pub fn calculate_interaction_strength(&self, other: &SpeciesType) -> f32 {
        match (self, other) {
            // Same species attract strongly
            (SpeciesType::DiscoLlama, SpeciesType::DiscoLlama) => 1.0,
            (SpeciesType::QuantumSheep, SpeciesType::QuantumSheep) => 1.2,
            (SpeciesType::HypnoCamel, SpeciesType::HypnoCamel) => 0.9,

            // Cross-species interactions
            (SpeciesType::DiscoLlama, SpeciesType::QuantumSheep) |
            (SpeciesType::QuantumSheep, SpeciesType::DiscoLlama) => 0.8,

            (SpeciesType::DiscoLlama, SpeciesType::HypnoCamel) |
            (SpeciesType::HypnoCamel, SpeciesType::DiscoLlama) => 0.6,

            (SpeciesType::QuantumSheep, SpeciesType::HypnoCamel) |
            (SpeciesType::HypnoCamel, SpeciesType::QuantumSheep) => 0.4,
        }
    }

    /// Map species to shader ID for psychedelic effects
    pub fn to_shader_id(&self) -> f32 {
        match self {
            SpeciesType::DiscoLlama => 0.0,
            SpeciesType::QuantumSheep => 1.0,
            SpeciesType::HypnoCamel => 2.0,
        }
    }

    /// Get species index for arrays
    pub fn to_index(&self) -> usize {
        match self {
            SpeciesType::DiscoLlama => 0,
            SpeciesType::QuantumSheep => 1,
            SpeciesType::HypnoCamel => 2,
        }
    }

    /// Get species from index
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => SpeciesType::DiscoLlama,
            1 => SpeciesType::QuantumSheep,
            _ => SpeciesType::HypnoCamel,
        }
    }
}

/// Species configuration parameters
#[derive(Debug, Clone)]
pub struct SpeciesConfig {
    pub base_hue_range: (f32, f32),
    pub base_saturation: f32,
    pub consciousness_modifier: f32,
    pub velocity_modifier: f32,
    pub war_efficiency: f32,
    pub quantum_affinity: bool,
}

/// Consciousness hierarchy levels for evolution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsciousnessLevel {
    Individual,    // Single entity consciousness
    Pack,          // Small group collective
    Hive,          // Large collective mind
    Meta,          // Transcendent consciousness
}