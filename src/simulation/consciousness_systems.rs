// Consciousness systems module - core consciousness mechanics
// Extracted from simple.rs for better modularity

use glam::Vec2;
use std::collections::HashMap;
use crate::entities::{SpeciesType, ConsciousnessLevel, Llama};

// ========== PHASE 3: ECOSYSTEM EMERGENCE ==========

/// Consciousness Crystal - harvestable nodes that enhance abilities
#[derive(Debug, Clone)]
pub struct ConsciousnessCrystal {
    pub position: Vec2,
    pub consciousness_energy: f32,    // Amount of consciousness stored
    pub resonance_frequency: f32,     // Prime-based frequency for interactions
    pub visual_intensity: f32,        // Current visual brightness/pulsing
    pub growth_rate: f32,             // How fast it accumulates energy
    pub harvest_radius: f32,          // Range for llama interaction
    pub age: f32,                     // How long it has existed
    pub crystal_type: CrystalType,    // Different types with different properties
}

#[derive(Debug, Clone, PartialEq)]
pub enum CrystalType {
    Resonance,    // Amplifies harmonic resonance
    Chaos,        // Increases chaos acceptance
    Memory,       // Enhances memory formation
    Social,       // Strengthens social bonds
    Quantum,      // Affects quantum state (for Quantum Sheep)
}

impl ConsciousnessCrystal {
    pub fn new(position: Vec2, crystal_type: CrystalType) -> Self {
        let (base_energy, base_frequency, base_growth) = match crystal_type {
            CrystalType::Resonance => (0.3, 7.0, 0.02),   // High frequency, moderate growth
            CrystalType::Chaos => (0.5, 11.0, 0.03),      // High energy, prime frequency
            CrystalType::Memory => (0.2, 5.0, 0.015),     // Low energy, steady growth
            CrystalType::Social => (0.4, 13.0, 0.025),    // Social prime frequency
            CrystalType::Quantum => (0.6, 17.0, 0.01),    // High energy, slow growth
        };

        Self {
            position,
            consciousness_energy: base_energy,
            resonance_frequency: base_frequency,
            visual_intensity: 0.5,
            growth_rate: base_growth,
            harvest_radius: 25.0 + fastrand::f32() * 15.0, // 25-40 radius
            age: 0.0,
            crystal_type,
        }
    }

    pub fn update(&mut self, dt: f32, beat_intensity: f32, cosmic_time: f64) {
        self.age += dt;

        // Crystals grow in consciousness energy over time
        let growth_multiplier = 1.0 + beat_intensity * 0.5;
        self.consciousness_energy += self.growth_rate * dt * growth_multiplier;
        self.consciousness_energy = self.consciousness_energy.min(2.0); // Cap at 2.0

        // Visual intensity pulses based on prime frequency and beat
        let frequency_phase = (cosmic_time as f32 * self.resonance_frequency * 0.1).sin();
        let beat_phase = beat_intensity * 0.5;
        self.visual_intensity = 0.3 + frequency_phase.abs() * 0.4 + beat_phase * 0.3;

        // Older crystals have larger harvest radius
        self.harvest_radius = (25.0 + self.age * 2.0).min(60.0);
    }

    pub fn get_harvest_amount(&self) -> f32 {
        // More energy = more harvest potential
        self.consciousness_energy * 0.3
    }

    pub fn harvest(&mut self, amount: f32) -> f32 {
        let harvested = amount.min(self.consciousness_energy);
        self.consciousness_energy -= harvested;
        harvested
    }

    pub fn get_color(&self) -> glam::Vec3 {
        use crate::engine::safety::hsv_to_rgb_vec3;

        let base_hue = match self.crystal_type {
            CrystalType::Resonance => 180.0, // Cyan
            CrystalType::Chaos => 300.0,     // Magenta
            CrystalType::Memory => 120.0,    // Green
            CrystalType::Social => 60.0,     // Yellow
            CrystalType::Quantum => 240.0,   // Blue
        };

        let saturation = 0.8 + self.consciousness_energy * 0.2;
        let brightness = 0.4 + self.visual_intensity * 0.6;

        hsv_to_rgb_vec3(glam::Vec3::new(base_hue, saturation, brightness))
    }

    /// Check if conditions are met to trigger mutation events
    pub fn should_trigger_mutation(&self) -> bool {
        self.chaos_accumulation > self.mutation_threshold
    }

    /// Reset chaos accumulation after mutation event
    pub fn reset_chaos_for_mutation(&mut self) {
        self.chaos_accumulation = 0.0;
    }

    /// Get territorial effects for entities in different zones
    pub fn get_territory_effects(&self, position: glam::Vec2) -> f32 {
        // Find which territory zone this position is in
        for zone in &self.territory_zones {
            let distance = position.distance(zone.center);
            if distance <= zone.radius {
                return zone.consciousness_amplification;
            }
        }
        1.0 // Default no amplification
    }
}

/// Environmental consciousness field - affects entity behavior
#[derive(Debug, Clone)]
pub struct ConsciousnessField {
    pub grid_size: usize,                    // Resolution of the field grid
    pub consciousness_density: Vec<Vec<f32>>, // 2D grid of consciousness values
    pub width: f32,                          // World width
    pub height: f32,                         // World height
}

impl ConsciousnessField {
    pub fn new(width: f32, height: f32, grid_size: usize) -> Self {
        let consciousness_density = vec![vec![0.1; grid_size]; grid_size]; // Start with low consciousness

        Self {
            grid_size,
            consciousness_density,
            width,
            height,
        }
    }

    pub fn get_consciousness_at(&self, position: Vec2) -> f32 {
        let x = ((position.x / self.width) * self.grid_size as f32) as usize;
        let y = ((position.y / self.height) * self.grid_size as f32) as usize;

        if x < self.grid_size && y < self.grid_size {
            self.consciousness_density[y][x]
        } else {
            0.1 // Default consciousness outside grid
        }
    }

    pub fn add_consciousness_at(&mut self, position: Vec2, amount: f32) {
        let x = ((position.x / self.width) * self.grid_size as f32) as usize;
        let y = ((position.y / self.height) * self.grid_size as f32) as usize;

        if x < self.grid_size && y < self.grid_size {
            self.consciousness_density[y][x] = (self.consciousness_density[y][x] + amount).min(2.0);
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Gradual diffusion and decay of consciousness
        for y in 0..self.grid_size {
            for x in 0..self.grid_size {
                // Decay consciousness over time
                self.consciousness_density[y][x] *= 0.999;

                // Simple diffusion with neighbors
                let mut neighbor_sum = 0.0;
                let mut neighbor_count = 0;

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 { continue; }
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;

                        if nx >= 0 && nx < self.grid_size as i32 && ny >= 0 && ny < self.grid_size as i32 {
                            neighbor_sum += self.consciousness_density[ny as usize][nx as usize];
                            neighbor_count += 1;
                        }
                    }
                }

                if neighbor_count > 0 {
                    let neighbor_avg = neighbor_sum / neighbor_count as f32;
                    // Slight diffusion toward neighbors
                    let diffusion_rate = 0.01 * dt;
                    self.consciousness_density[y][x] =
                        self.consciousness_density[y][x] * (1.0 - diffusion_rate) +
                        neighbor_avg * diffusion_rate;
                }
            }
        }
    }
}

/// Reality Tear - visual glitch representing consciousness breakthrough
#[derive(Debug, Clone)]
pub struct RealityTear {
    pub position: Vec2,
    pub size: f32,
    pub intensity: f32,
    pub age: f32,
    pub tear_type: TearType,
}

#[derive(Debug, Clone)]
pub enum TearType {
    Static,      // Stationary glitch
    Moving,      // Travels across screen
    Pulsing,     // Grows and shrinks
    Fragmenting, // Splits into smaller tears
}

impl RealityTear {
    pub fn new(position: Vec2, tear_type: TearType) -> Self {
        Self {
            position,
            size: 5.0 + fastrand::f32() * 15.0,
            intensity: 0.7 + fastrand::f32() * 0.3,
            age: 0.0,
            tear_type,
        }
    }

    pub fn update(&mut self, dt: f32, cosmic_time: f64) {
        self.age += dt;

        match self.tear_type {
            TearType::Static => {
                // Just age and fade
                self.intensity *= 0.995;
            },
            TearType::Moving => {
                // Move in chaotic pattern
                let move_speed = 50.0;
                let chaos_angle = (cosmic_time as f32 * 3.0 + self.position.length() * 0.01).sin() * 6.28;
                self.position.x += chaos_angle.cos() * move_speed * dt;
                self.position.y += chaos_angle.sin() * move_speed * dt;
                self.intensity *= 0.99;
            },
            TearType::Pulsing => {
                // Pulsing size
                let pulse = (cosmic_time as f32 * 5.0).sin().abs();
                self.size = (5.0 + fastrand::f32() * 15.0) * (0.5 + pulse * 0.5);
                self.intensity *= 0.995;
            },
            TearType::Fragmenting => {
                // Grows then fragments (handled in ecosystem update)
                if self.age < 2.0 {
                    self.size += dt * 10.0;
                }
                self.intensity *= 0.99;
            },
        }

        // Keep within bounds
        if self.position.x < 0.0 { self.position.x = 1200.0; }
        if self.position.x > 1200.0 { self.position.x = 0.0; }
        if self.position.y < 0.0 { self.position.y = 800.0; }
        if self.position.y > 800.0 { self.position.y = 0.0; }
    }

    pub fn should_remove(&self) -> bool {
        self.intensity < 0.1 || self.age > 10.0
    }
}

/// Territory Zone - regions with different consciousness properties
#[derive(Debug, Clone)]
pub struct TerritoryZone {
    pub center: Vec2,
    pub radius: f32,
    pub zone_type: ZoneType,
    pub strength: f32,
    pub age: f32,
}

#[derive(Debug, Clone)]
pub enum ZoneType {
    Harmonic,    // Enhances resonance and social bonding
    Chaotic,     // Increases chaos and reality distortion
    Meditative,  // Calms entities, increases memory formation
    Quantum,     // Quantum effects amplified
}

impl TerritoryZone {
    pub fn new(center: Vec2, zone_type: ZoneType) -> Self {
        let radius = match zone_type {
            ZoneType::Harmonic => 80.0 + fastrand::f32() * 40.0,
            ZoneType::Chaotic => 60.0 + fastrand::f32() * 60.0,
            ZoneType::Meditative => 100.0 + fastrand::f32() * 50.0,
            ZoneType::Quantum => 70.0 + fastrand::f32() * 30.0,
        };

        Self {
            center,
            radius,
            zone_type,
            strength: 0.3 + fastrand::f32() * 0.4,
            age: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32, cosmic_time: f64) {
        self.age += dt;

        // Zones slowly grow and change strength
        let growth_rate = match self.zone_type {
            ZoneType::Harmonic => 0.5,
            ZoneType::Chaotic => 1.0,
            ZoneType::Meditative => 0.3,
            ZoneType::Quantum => 0.8,
        };

        self.radius += growth_rate * dt;
        self.radius = self.radius.min(150.0);

        // Strength oscillates
        let oscillation = (cosmic_time as f32 * 0.5 + self.center.length() * 0.001).sin() * 0.1;
        self.strength = (0.3 + fastrand::f32() * 0.4 + oscillation).clamp(0.1, 0.8);
    }

    pub fn affects_position(&self, position: Vec2) -> f32 {
        let distance = self.center.distance(position);
        if distance < self.radius {
            let factor = 1.0 - (distance / self.radius);
            factor * self.strength
        } else {
            0.0
        }
    }
}

/// Digital Ecosystem containing crystals and consciousness fields
#[derive(Debug)]
pub struct DigitalEcosystem {
    pub consciousness_fields: ConsciousnessField,
    pub crystal_formations: Vec<ConsciousnessCrystal>,
    pub chaos_accumulation: f32,              // Global chaos level from clicks
    pub mutation_threshold: f32,              // When mutations trigger
    pub reality_tears: Vec<RealityTear>,      // Visual glitches
    pub territory_zones: Vec<TerritoryZone>,  // Different environmental regions
}

impl DigitalEcosystem {
    pub fn new() -> Self {
        let consciousness_fields = ConsciousnessField::new(1200.0, 800.0, 40); // 40x40 grid

        // Start with a few crystals
        let mut crystal_formations = Vec::new();
        for _ in 0..3 {
            let position = Vec2::new(fastrand::f32() * 1200.0, fastrand::f32() * 800.0);
            let crystal_type = match fastrand::usize(0..5) {
                0 => CrystalType::Resonance,
                1 => CrystalType::Chaos,
                2 => CrystalType::Memory,
                3 => CrystalType::Social,
                _ => CrystalType::Quantum,
            };
            crystal_formations.push(ConsciousnessCrystal::new(position, crystal_type));
        }

        // Start with one territory zone
        let mut territory_zones = Vec::new();
        let zone_center = Vec2::new(fastrand::f32() * 1200.0, fastrand::f32() * 800.0);
        let zone_type = match fastrand::usize(0..4) {
            0 => ZoneType::Harmonic,
            1 => ZoneType::Chaotic,
            2 => ZoneType::Meditative,
            _ => ZoneType::Quantum,
        };
        territory_zones.push(TerritoryZone::new(zone_center, zone_type));

        Self {
            consciousness_fields,
            crystal_formations,
            chaos_accumulation: 0.0,
            mutation_threshold: 3.0, // Mutations trigger when chaos reaches this level
            reality_tears: Vec::new(),
            territory_zones,
        }
    }

    pub fn update(&mut self, dt: f32, cosmic_time: f64, beat_intensity: f32) {
        // Update consciousness fields
        self.consciousness_fields.update(dt);

        // Update crystals
        for crystal in &mut self.crystal_formations {
            crystal.update(dt, beat_intensity, cosmic_time);
        }

        // Update reality tears
        self.reality_tears.retain_mut(|tear| {
            tear.update(dt, cosmic_time);
            !tear.should_remove()
        });

        // Handle fragmenting tears
        let mut new_tears = Vec::new();
        for tear in &self.reality_tears {
            if matches!(tear.tear_type, TearType::Fragmenting) && tear.age > 2.0 && tear.size > 15.0 {
                // Fragment into smaller tears
                for _ in 0..3 {
                    let offset = Vec2::new(
                        (fastrand::f32() - 0.5) * 30.0,
                        (fastrand::f32() - 0.5) * 30.0,
                    );
                    let fragment_pos = tear.position + offset;
                    let fragment_type = if fastrand::f32() < 0.5 { TearType::Moving } else { TearType::Static };
                    new_tears.push(RealityTear::new(fragment_pos, fragment_type));
                }
            }
        }
        self.reality_tears.extend(new_tears);

        // Update territory zones
        for zone in &mut self.territory_zones {
            zone.update(dt, cosmic_time);
        }

        // Spawn new crystals occasionally
        if fastrand::f32() < 0.002 * dt * (1.0 + beat_intensity) {
            let position = Vec2::new(fastrand::f32() * 1200.0, fastrand::f32() * 800.0);
            let crystal_type = match fastrand::usize(0..5) {
                0 => CrystalType::Resonance,
                1 => CrystalType::Chaos,
                2 => CrystalType::Memory,
                3 => CrystalType::Social,
                _ => CrystalType::Quantum,
            };
            self.crystal_formations.push(ConsciousnessCrystal::new(position, crystal_type));
        }

        // Spawn reality tears from high chaos
        if self.chaos_accumulation > 1.0 && fastrand::f32() < 0.01 * dt {
            let position = Vec2::new(fastrand::f32() * 1200.0, fastrand::f32() * 800.0);
            let tear_type = match fastrand::usize(0..4) {
                0 => TearType::Static,
                1 => TearType::Moving,
                2 => TearType::Pulsing,
                _ => TearType::Fragmenting,
            };
            self.reality_tears.push(RealityTear::new(position, tear_type));
        }

        // Slow chaos decay
        self.chaos_accumulation *= 0.995;
    }

    pub fn add_chaos(&mut self, amount: f32) {
        self.chaos_accumulation += amount;
    }
}

// ========== PHASE 5: CONSCIOUSNESS MULTIPLICATION ==========

#[derive(Debug, Clone)]
pub struct ConsciousnessHierarchy {
    pub level: ConsciousnessLevel,
    pub members: Vec<usize>,           // Entity indices that belong to this consciousness
    pub collective_strength: f32,      // Combined consciousness power
    pub territory_control: f32,        // Territorial influence 0.0-1.0
    pub war_efficiency: f32,           // Combat effectiveness modifier
    pub hive_connection_strength: f32, // How strongly hive members are connected
    pub absorption_capacity: f32,      // Ability to absorb other consciousness
}

#[derive(Debug, Clone)]
pub struct WarfareState {
    pub species_populations: [u32; 3],        // [DiscoLlama, QuantumSheep, HypnoCamel]
    pub territorial_dominance: [f32; 3],      // Territory control per species 0.0-1.0
    pub extinction_pressure: [f32; 3],        // Extinction threat level per species
    pub consciousness_crystals_controlled: [u32; 3], // Resource control
    pub active_conflicts: Vec<SpeciesConflict>,
}

#[derive(Debug, Clone)]
pub struct SpeciesConflict {
    pub attacker_species: SpeciesType,
    pub defender_species: SpeciesType,
    pub conflict_intensity: f32,       // 0.0-1.0 intensity of the conflict
    pub territory_contested: Vec2,     // Center point of contested territory
    pub duration: f32,                 // How long the conflict has lasted
    pub victory_threshold: f32,        // Consciousness advantage needed to win
}

#[derive(Debug, Clone)]
pub struct HiveMind {
    pub collective_id: usize,
    pub member_entities: Vec<usize>,   // Indices of entities in the hive
    pub collective_consciousness: f32,  // Combined consciousness level
    pub hive_center: Vec2,             // Geometric center of the hive
    pub connection_network: Vec<(usize, usize)>, // Pairs of connected entities
    pub shared_memories: Vec<Vec2>,    // Collective memory fragments
    pub collective_decision_weight: f32, // How much the hive influences individual decisions
    pub emergence_timestamp: f32,      // When this hive mind formed
}

#[derive(Debug, Clone)]
pub struct ConsciousnessPredation {
    pub predator_id: usize,
    pub prey_id: usize,
    pub absorption_progress: f32,      // 0.0-1.0 progress of consumption
    pub resistance_strength: f32,      // How much the prey is fighting back
    pub visual_effect_intensity: f32,  // Visual feedback for absorption event
}

#[derive(Debug, Clone)]
pub struct MetaConsciousnessObserver {
    pub observer_position: Vec2,
    pub awareness_radius: f32,         // How far the observer can see
    pub intervention_power: f32,       // Ability to influence consciousness wars
    pub observation_intensity: f32,    // Current focus level
    pub last_intervention: f32,        // Time since last intervention
    pub consciousness_analysis: ConsciousnessAnalysis,
}

#[derive(Debug, Clone)]
pub struct ConsciousnessAnalysis {
    pub total_individual_entities: u32,
    pub total_pack_collectives: u32,
    pub total_hive_minds: u32,
    pub dominant_species: SpeciesType,
    pub extinction_imminent: Option<SpeciesType>, // Species about to go extinct
    pub consciousness_distribution: [f32; 3],     // Consciousness per species
    pub warfare_intensity: f32,       // Overall conflict level
    pub ecosystem_stability: f32,     // 0.0-1.0 stability measure
}

#[derive(Debug, Clone)]
pub struct ConsciousnessMultiplicationSystem {
    pub hierarchy_levels: Vec<ConsciousnessHierarchy>,
    pub hive_minds: Vec<HiveMind>,
    pub active_predations: Vec<ConsciousnessPredation>,
    pub warfare_state: WarfareState,
    pub meta_observer: MetaConsciousnessObserver,
    pub evolution_pressure_accumulator: f32,
    pub next_hive_id: usize,
    pub consciousness_crystal_spawn_rate: f32,
    pub territorial_conflict_threshold: f32,
}

impl ConsciousnessMultiplicationSystem {
    pub fn new() -> Self {
        Self {
            hierarchy_levels: Vec::new(),
            hive_minds: Vec::new(),
            active_predations: Vec::new(),
            warfare_state: WarfareState {
                species_populations: [0, 0, 0],
                territorial_dominance: [0.33, 0.33, 0.34], // Start balanced
                extinction_pressure: [0.0, 0.0, 0.0],
                consciousness_crystals_controlled: [0, 0, 0],
                active_conflicts: Vec::new(),
            },
            meta_observer: MetaConsciousnessObserver {
                observer_position: Vec2::new(600.0, 400.0), // Center of screen
                awareness_radius: 800.0,
                intervention_power: 1.0,
                observation_intensity: 0.5,
                last_intervention: 0.0,
                consciousness_analysis: ConsciousnessAnalysis {
                    total_individual_entities: 0,
                    total_pack_collectives: 0,
                    total_hive_minds: 0,
                    dominant_species: SpeciesType::DiscoLlama,
                    extinction_imminent: None,
                    consciousness_distribution: [0.33, 0.33, 0.34],
                    warfare_intensity: 0.0,
                    ecosystem_stability: 1.0,
                },
            },
            evolution_pressure_accumulator: 0.0,
            next_hive_id: 1,
            consciousness_crystal_spawn_rate: 1.0,
            territorial_conflict_threshold: 0.7,
        }
    }

    pub fn update(&mut self, dt: f32, llamas: &mut [Llama], cosmic_time: f32) {
        // Update meta observer consciousness analysis
        self.update_consciousness_analysis(llamas);

        // Process consciousness hierarchy formation and dissolution
        self.process_consciousness_hierarchies(llamas, dt);

        // Handle hive mind emergence and collective behavior
        self.process_hive_mind_emergence(llamas, dt, cosmic_time);

        // Execute consciousness predation events
        self.process_consciousness_predation(llamas, dt);

        // Run species warfare and territorial conflicts
        self.process_species_warfare(llamas, dt, cosmic_time);

        // Apply evolution pressure and extinction dynamics
        self.process_evolution_pressure(llamas, dt);

        // Meta-consciousness observer interventions
        self.process_meta_observer_interventions(llamas, dt, cosmic_time);

        // Update warfare state and population tracking
        self.update_warfare_state(llamas);
    }

    fn update_consciousness_analysis(&mut self, llamas: &[Llama]) {
        let mut analysis = &mut self.meta_observer.consciousness_analysis;

        // Count consciousness levels
        analysis.total_individual_entities = llamas.iter()
            .filter(|l| l.consciousness_level == ConsciousnessLevel::Individual)
            .count() as u32;

        analysis.total_pack_collectives = llamas.iter()
            .filter(|l| l.consciousness_level == ConsciousnessLevel::Pack)
            .count() as u32;

        analysis.total_hive_minds = self.hive_minds.len() as u32;

        // Calculate consciousness distribution by species
        let mut species_consciousness = [0.0f32; 3];
        let mut species_counts = [0u32; 3];

        for llama in llamas {
            let species_idx = match llama.species {
                SpeciesType::DiscoLlama => 0,
                SpeciesType::QuantumSheep => 1,
                SpeciesType::HypnoCamel => 2,
            };
            species_consciousness[species_idx] += llama.consciousness;
            species_counts[species_idx] += 1;
        }

        let total_consciousness: f32 = species_consciousness.iter().sum();
        if total_consciousness > 0.0 {
            analysis.consciousness_distribution = [
                species_consciousness[0] / total_consciousness,
                species_consciousness[1] / total_consciousness,
                species_consciousness[2] / total_consciousness,
            ];
        }

        // Determine dominant species
        let max_consciousness_idx = analysis.consciousness_distribution
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0);

        analysis.dominant_species = match max_consciousness_idx {
            0 => SpeciesType::DiscoLlama,
            1 => SpeciesType::QuantumSheep,
            _ => SpeciesType::HypnoCamel,
        };

        // Check for extinction threats
        analysis.extinction_imminent = None;
        for (i, &count) in species_counts.iter().enumerate() {
            if count < 3 && count > 0 { // Less than 3 entities remaining
                analysis.extinction_imminent = Some(match i {
                    0 => SpeciesType::DiscoLlama,
                    1 => SpeciesType::QuantumSheep,
                    _ => SpeciesType::HypnoCamel,
                });
                break;
            }
        }

        // Calculate warfare intensity
        analysis.warfare_intensity = self.warfare_state.active_conflicts.len() as f32 * 0.2
            + self.active_predations.len() as f32 * 0.1;
        analysis.warfare_intensity = analysis.warfare_intensity.min(1.0);

        // Calculate ecosystem stability
        let consciousness_balance = 1.0 - (analysis.consciousness_distribution[0] - 0.33).abs()
            - (analysis.consciousness_distribution[1] - 0.33).abs()
            - (analysis.consciousness_distribution[2] - 0.33).abs();

        analysis.ecosystem_stability = (consciousness_balance * 0.6 + (1.0 - analysis.warfare_intensity) * 0.4)
            .clamp(0.0, 1.0);
    }

    fn process_consciousness_hierarchies(&mut self, llamas: &mut [Llama], _dt: f32) {
        // Clear existing hierarchies to rebuild them
        self.hierarchy_levels.clear();

        // Find entities that should form pack consciousness
        let mut processed = vec![false; llamas.len()];

        for i in 0..llamas.len() {
            if processed[i] { continue; }

            let mut pack_members = vec![i];
            let llama = &llamas[i];

            // Find nearby llamas of the same species for pack formation
            for j in (i + 1)..llamas.len() {
                if processed[j] { continue; }

                let other = &llamas[j];
                if llama.species == other.species {
                    let distance = llama.position.distance(other.position);
                    let pack_threshold = 80.0 + llama.social_attraction * 40.0;

                    if distance < pack_threshold && pack_members.len() < 8 {
                        pack_members.push(j);
                        processed[j] = true;
                    }
                }
            }

            processed[i] = true;

            // Determine consciousness level based on pack size
            let consciousness_level = if pack_members.len() >= 9 {
                ConsciousnessLevel::Hive
            } else if pack_members.len() >= 2 {
                ConsciousnessLevel::Pack
            } else {
                ConsciousnessLevel::Individual
            };

            // Calculate collective strength
            let collective_strength: f32 = pack_members.iter()
                .map(|&idx| llamas[idx].consciousness)
                .sum();

            // Update llama consciousness levels
            for &member_idx in &pack_members {
                llamas[member_idx].consciousness_level = consciousness_level;
                if pack_members.len() > 1 {
                    llamas[member_idx].collective_id = Some(self.hierarchy_levels.len());
                } else {
                    llamas[member_idx].collective_id = None;
                }
            }

            // Create hierarchy entry
            self.hierarchy_levels.push(ConsciousnessHierarchy {
                level: consciousness_level,
                members: pack_members.clone(),
                collective_strength,
                territory_control: collective_strength / (pack_members.len() as f32 + 1.0),
                war_efficiency: match consciousness_level {
                    ConsciousnessLevel::Individual => 1.0,
                    ConsciousnessLevel::Pack => 1.2 + (pack_members.len() as f32 - 2.0) * 0.1,
                    ConsciousnessLevel::Hive => 1.5 + (pack_members.len() as f32 - 9.0) * 0.05,
                    ConsciousnessLevel::Meta => 2.0,
                },
                hive_connection_strength: if consciousness_level == ConsciousnessLevel::Hive {
                    0.8 + fastrand::f32() * 0.2
                } else {
                    0.0
                },
                absorption_capacity: collective_strength * 0.1,
            });
        }
    }

    fn process_hive_mind_emergence(&mut self, llamas: &mut [Llama], dt: f32, cosmic_time: f32) {
        // Check for new hive mind formation
        for hierarchy in &self.hierarchy_levels {
            if hierarchy.level == ConsciousnessLevel::Hive {
                // Check if this hive already exists
                let hive_exists = self.hive_minds.iter()
                    .any(|hive| hive.member_entities == hierarchy.members);

                if !hive_exists && hierarchy.members.len() >= 9 {
                    // Calculate hive center
                    let hive_center = {
                        let sum: Vec2 = hierarchy.members.iter()
                            .map(|&idx| llamas[idx].position)
                            .fold(Vec2::ZERO, |acc, pos| acc + pos);
                        sum / hierarchy.members.len() as f32
                    };

                    // Create connection network (each entity connected to 2-3 others)
                    let mut connection_network = Vec::new();
                    for (i, &member_a) in hierarchy.members.iter().enumerate() {
                        for (j, &member_b) in hierarchy.members.iter().enumerate().skip(i + 1) {
                            if fastrand::f32() < 0.3 { // 30% chance of connection
                                connection_network.push((member_a, member_b));
                            }
                        }
                    }

                    // Collect shared memories from all members
                    let mut shared_memories = Vec::new();
                    for &member_idx in &hierarchy.members {
                        shared_memories.extend(llamas[member_idx].memory_fragments.clone());
                    }

                    // Create new hive mind
                    let hive_mind = HiveMind {
                        collective_id: self.next_hive_id,
                        member_entities: hierarchy.members.clone(),
                        collective_consciousness: hierarchy.collective_strength,
                        hive_center,
                        connection_network,
                        shared_memories,
                        collective_decision_weight: 0.7 + fastrand::f32() * 0.3,
                        emergence_timestamp: cosmic_time,
                    };

                    self.hive_minds.push(hive_mind);
                    self.next_hive_id += 1;
                }
            }
        }

        // Update existing hive minds
        self.hive_minds.retain_mut(|hive| {
            // Check if hive members still exist and are close enough
            let valid_members: Vec<usize> = hive.member_entities.iter()
                .filter(|&&idx| idx < llamas.len())
                .copied()
                .collect();

            if valid_members.len() < 5 { // Hive dissolves if too few members
                // Mark former hive members as individuals
                for &member_idx in &valid_members {
                    if member_idx < llamas.len() {
                        llamas[member_idx].consciousness_level = ConsciousnessLevel::Individual;
                        llamas[member_idx].collective_id = None;
                        llamas[member_idx].hive_connection_strength = 0.0;
                    }
                }
                return false; // Remove this hive
            }

            hive.member_entities = valid_members;

            // Update hive center
            let new_center = {
                let sum: Vec2 = hive.member_entities.iter()
                    .map(|&idx| llamas[idx].position)
                    .fold(Vec2::ZERO, |acc, pos| acc + pos);
                sum / hive.member_entities.len() as f32
            };
            hive.hive_center = new_center;

            // Update collective consciousness
            hive.collective_consciousness = hive.member_entities.iter()
                .map(|&idx| llamas[idx].consciousness)
                .sum();

            // Apply hive mind effects to members
            for &member_idx in &hive.member_entities {
                if member_idx < llamas.len() {
                    let llama = &mut llamas[member_idx];
                    llama.hive_connection_strength = (llama.hive_connection_strength + dt * 0.5).min(1.0);

                    // Hive members share consciousness boost
                    let consciousness_boost = hive.collective_consciousness * 0.01;
                    llama.consciousness = (llama.consciousness + consciousness_boost * dt).min(3.0);

                    // Influence movement toward hive center
                    let to_center = hive.hive_center - llama.position;
                    if to_center.length() > 50.0 {
                        let influence = hive.collective_decision_weight * llama.hive_connection_strength;
                        llama.velocity += to_center.normalize() * influence * dt * 20.0;
                    }
                }
            }

            true // Keep this hive
        });
    }

    fn process_consciousness_predation(&mut self, llamas: &mut [Llama], dt: f32) {
        // Start new predation events
        for i in 0..llamas.len() {
            let llama = &llamas[i];

            // High consciousness entities can target lower consciousness ones for absorption
            if llama.consciousness > 1.5 && llama.predation_target.is_none() && fastrand::f32() < 0.01 {
                // Find suitable prey within range
                for j in 0..llamas.len() {
                    if i == j { continue; }

                    let prey = &llamas[j];
                    let distance = llama.position.distance(prey.position);

                    if distance < 60.0 && prey.consciousness < llama.consciousness * 0.7 {
                        // Start predation event
                        let predation = ConsciousnessPredation {
                            predator_id: i,
                            prey_id: j,
                            absorption_progress: 0.0,
                            resistance_strength: prey.absorption_resistance,
                            visual_effect_intensity: 0.0,
                        };

                        self.active_predations.push(predation);
                        break;
                    }
                }
            }
        }

        // Process active predations
        self.active_predations.retain_mut(|predation| {
            if predation.predator_id >= llamas.len() || predation.prey_id >= llamas.len() {
                return false; // Invalid indices
            }

            let distance = llamas[predation.predator_id].position
                .distance(llamas[predation.prey_id].position);

            if distance > 100.0 {
                return false; // Too far apart, predation fails
            }

            // Calculate absorption rate
            let predator_strength = llamas[predation.predator_id].consciousness;
            let absorption_rate = (predator_strength - predation.resistance_strength).max(0.0) * dt * 0.3;

            predation.absorption_progress += absorption_rate;
            predation.visual_effect_intensity = (predation.absorption_progress * 2.0).min(1.0);

            // Apply effects during absorption
            if predation.absorption_progress < 1.0 {
                // Prey loses consciousness gradually
                let consciousness_drain = absorption_rate * 0.5;
                llamas[predation.prey_id].consciousness =
                    (llamas[predation.prey_id].consciousness - consciousness_drain).max(0.1);

                // Predator gains consciousness
                llamas[predation.predator_id].consciousness += consciousness_drain * 0.3;

                // Visual effects - make prey fade and predator glow
                llamas[predation.prey_id].color.y *= 1.0 - absorption_rate * 0.1; // Reduce saturation

                true // Continue predation
            } else {
                // Absorption complete
                let absorbed_consciousness = llamas[predation.prey_id].consciousness;
                llamas[predation.predator_id].consciousness += absorbed_consciousness * 0.5;

                // Remove prey from simulation by setting consciousness to 0
                llamas[predation.prey_id].consciousness = 0.0;
                llamas[predation.prey_id].color.y = 0.0; // Make invisible

                false // Remove predation event
            }
        });
    }

    fn process_species_warfare(&mut self, llamas: &mut [Llama], dt: f32, cosmic_time: f32) {
        // Check for new conflicts based on territorial overlap
        let mut new_conflicts = Vec::new();

        // Analyze territorial tensions between species
        for hierarchy_a in &self.hierarchy_levels {
            for hierarchy_b in &self.hierarchy_levels {
                if hierarchy_a.members == hierarchy_b.members { continue; }

                // Check if different species are competing for same territory
                if let (Some(&a_idx), Some(&b_idx)) = (hierarchy_a.members.first(), hierarchy_b.members.first()) {
                    if a_idx >= llamas.len() || b_idx >= llamas.len() { continue; }

                    let species_a = llamas[a_idx].species;
                    let species_b = llamas[b_idx].species;

                    if species_a != species_b {
                        // Calculate average positions for territorial centers
                        let center_a: Vec2 = hierarchy_a.members.iter()
                            .map(|&idx| llamas[idx].position)
                            .fold(Vec2::ZERO, |acc, pos| acc + pos) / hierarchy_a.members.len() as f32;

                        let center_b: Vec2 = hierarchy_b.members.iter()
                            .map(|&idx| llamas[idx].position)
                            .fold(Vec2::ZERO, |acc, pos| acc + pos) / hierarchy_b.members.len() as f32;

                        let distance = center_a.distance(center_b);
                        let territorial_range = 150.0 + hierarchy_a.territory_control * 50.0;

                        if distance < territorial_range {
                            // Check if conflict already exists
                            let conflict_exists = self.warfare_state.active_conflicts.iter()
                                .any(|c| (c.attacker_species == species_a && c.defender_species == species_b) ||
                                        (c.attacker_species == species_b && c.defender_species == species_a));

                            if !conflict_exists && fastrand::f32() < 0.005 { // 0.5% chance per frame
                                let stronger_hierarchy = if hierarchy_a.collective_strength > hierarchy_b.collective_strength {
                                    hierarchy_a
                                } else {
                                    hierarchy_b
                                };

                                let weaker_hierarchy = if hierarchy_a.collective_strength <= hierarchy_b.collective_strength {
                                    hierarchy_a
                                } else {
                                    hierarchy_b
                                };

                                new_conflicts.push(SpeciesConflict {
                                    attacker_species: if stronger_hierarchy.members == hierarchy_a.members { species_a } else { species_b },
                                    defender_species: if weaker_hierarchy.members == hierarchy_a.members { species_a } else { species_b },
                                    conflict_intensity: 0.3 + fastrand::f32() * 0.4,
                                    territory_contested: (center_a + center_b) / 2.0,
                                    duration: 0.0,
                                    victory_threshold: 0.7 + fastrand::f32() * 0.3,
                                });
                            }
                        }
                    }
                }
            }
        }

        // Add new conflicts
        self.warfare_state.active_conflicts.extend(new_conflicts);

        // Process existing conflicts
        self.warfare_state.active_conflicts.retain_mut(|conflict| {
            conflict.duration += dt;

            // Apply warfare effects to participating llamas
            for llama in llamas.iter_mut() {
                if llama.species == conflict.attacker_species || llama.species == conflict.defender_species {
                    llama.warfare_participation = (llama.warfare_participation + dt * 0.2).min(1.0);

                    // Warfare participation affects consciousness and behavior
                    let warfare_stress = conflict.conflict_intensity * 0.1 * dt;
                    llama.consciousness = (llama.consciousness - warfare_stress).max(0.5);
                    llama.emotional_state = (llama.emotional_state + warfare_stress * 2.0).min(2.0);

                    // Entities in warfare move more aggressively
                    let aggression_factor = llama.warfare_participation * llama.war_efficiency;
                    llama.velocity *= 1.0 + aggression_factor * 0.1;
                }
            }

            // Check for conflict resolution
            if conflict.duration > 30.0 + fastrand::f32() * 20.0 { // 30-50 second conflicts
                // Determine winner based on current species strength
                let attacker_strength: f32 = llamas.iter()
                    .filter(|l| l.species == conflict.attacker_species)
                    .map(|l| l.consciousness * l.war_efficiency)
                    .sum();

                let defender_strength: f32 = llamas.iter()
                    .filter(|l| l.species == conflict.defender_species)
                    .map(|l| l.consciousness * l.war_efficiency)
                    .sum();

                let attacker_advantage = attacker_strength / (defender_strength + 0.1);

                if attacker_advantage > conflict.victory_threshold {
                    // Attacker wins - apply extinction pressure to defender
                    let species_idx = match conflict.defender_species {
                        SpeciesType::DiscoLlama => 0,
                        SpeciesType::QuantumSheep => 1,
                        SpeciesType::HypnoCamel => 2,
                    };
                    self.warfare_state.extinction_pressure[species_idx] += 0.2;

                    // Winner gains territorial dominance
                    let winner_idx = match conflict.attacker_species {
                        SpeciesType::DiscoLlama => 0,
                        SpeciesType::QuantumSheep => 1,
                        SpeciesType::HypnoCamel => 2,
                    };
                    self.warfare_state.territorial_dominance[winner_idx] += 0.1;
                    self.warfare_state.territorial_dominance[species_idx] -= 0.1;

                    // Normalize territorial dominance
                    let sum: f32 = self.warfare_state.territorial_dominance.iter().sum();
                    if sum > 0.0 {
                        for dominance in &mut self.warfare_state.territorial_dominance {
                            *dominance /= sum;
                        }
                    }
                }

                false // End conflict
            } else {
                true // Continue conflict
            }
        });
    }

    fn process_evolution_pressure(&mut self, llamas: &mut [Llama], dt: f32) {
        self.evolution_pressure_accumulator += dt;

        // Apply evolution pressure every few seconds
        if self.evolution_pressure_accumulator > 5.0 {
            self.evolution_pressure_accumulator = 0.0;

            // Count species populations
            let mut species_counts = [0u32; 3];
            for llama in llamas.iter() {
                if llama.consciousness > 0.1 { // Only count living llamas
                    let idx = match llama.species {
                        SpeciesType::DiscoLlama => 0,
                        SpeciesType::QuantumSheep => 1,
                        SpeciesType::HypnoCamel => 2,
                    };
                    species_counts[idx] += 1;
                }
            }

            // Apply extinction pressure
            for (i, &pressure) in self.warfare_state.extinction_pressure.iter().enumerate() {
                if pressure > 0.5 && species_counts[i] > 0 {
                    // Find weakest entities of this species for extinction
                    let mut species_llamas: Vec<(usize, f32)> = llamas.iter()
                        .enumerate()
                        .filter(|(_, l)| {
                            let species_idx = match l.species {
                                SpeciesType::DiscoLlama => 0,
                                SpeciesType::QuantumSheep => 1,
                                SpeciesType::HypnoCamel => 2,
                            };
                            species_idx == i && l.consciousness > 0.1
                        })
                        .map(|(idx, l)| (idx, l.consciousness))
                        .collect();

                    species_llamas.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

                    // Remove weakest entities (up to 1/3 of population)
                    let removal_count = ((species_llamas.len() as f32 * pressure * 0.3) as usize).max(1);
                    for i in 0..removal_count.min(species_llamas.len()) {
                        let (idx, _) = species_llamas[i];
                        llamas[idx].consciousness = 0.0;
                        llamas[idx].color.y = 0.0; // Make invisible
                        llamas[idx].extinction_pressure = 1.0;
                    }
                }
            }

            // Successful species multiply - add consciousness boost to surviving members
            for (i, &dominance) in self.warfare_state.territorial_dominance.iter().enumerate() {
                if dominance > 0.4 && species_counts[i] > 0 {
                    for llama in llamas.iter_mut() {
                        let species_idx = match llama.species {
                            SpeciesType::DiscoLlama => 0,
                            SpeciesType::QuantumSheep => 1,
                            SpeciesType::HypnoCamel => 2,
                        };
                        if species_idx == i && llama.consciousness > 0.1 {
                            llama.consciousness += dominance * 0.2;
                            llama.territorial_dominance += dominance * 0.1;
                        }
                    }
                }
            }

            // Reduce extinction pressure over time
            for pressure in &mut self.warfare_state.extinction_pressure {
                *pressure *= 0.9;
            }
        }
    }

    fn process_meta_observer_interventions(&mut self, llamas: &mut [Llama], dt: f32, cosmic_time: f32) {
        let observer = &mut self.meta_observer;
        observer.last_intervention += dt;

        // Observer analyzes the ecosystem and decides whether to intervene
        let analysis = &observer.consciousness_analysis;

        // Increase observation intensity based on warfare and instability
        observer.observation_intensity = (analysis.warfare_intensity * 0.5 +
                                        (1.0 - analysis.ecosystem_stability) * 0.5)
                                        .clamp(0.1, 1.0);

        // Decide on intervention
        let should_intervene = observer.last_intervention > 10.0 && // At least 10 seconds between interventions
                             (analysis.ecosystem_stability < 0.3 || // Very unstable
                              analysis.extinction_imminent.is_some() || // Species about to go extinct
                              analysis.warfare_intensity > 0.8); // Intense warfare

        if should_intervene && fastrand::f32() < 0.1 { // 10% chance when conditions are met
            observer.last_intervention = 0.0;

            // Meta-consciousness observer intervention
            match fastrand::u32(0..4) {
                0 => {
                    // Consciousness blessing - boost weakest species
                    if let Some(extinct_species) = analysis.extinction_imminent {
                        for llama in llamas.iter_mut() {
                            if llama.species == extinct_species && llama.consciousness > 0.1 {
                                llama.consciousness += 0.5;
                                llama.consciousness_level = ConsciousnessLevel::Pack; // Temporary boost
                                llama.extinction_pressure = 0.0;
                            }
                        }
                    }
                },
                1 => {
                    // Force peace - end all conflicts
                    self.warfare_state.active_conflicts.clear();
                    for llama in llamas.iter_mut() {
                        llama.warfare_participation *= 0.5;
                        llama.emotional_state *= 0.7;
                    }
                },
                2 => {
                    // Reality distortion - scramble positions to break territorial deadlocks
                    for llama in llamas.iter_mut() {
                        if fastrand::f32() < 0.3 {
                            llama.position += Vec2::new(
                                (fastrand::f32() - 0.5) * 200.0,
                                (fastrand::f32() - 0.5) * 200.0
                            );
                            // Clamp to screen bounds
                            llama.position.x = llama.position.x.clamp(50.0, 1150.0);
                            llama.position.y = llama.position.y.clamp(50.0, 750.0);
                        }
                    }
                },
                _ => {
                    // Consciousness redistribution - balance species consciousness
                    let total_consciousness: f32 = llamas.iter()
                        .filter(|l| l.consciousness > 0.1)
                        .map(|l| l.consciousness).sum();

                    if total_consciousness > 0.0 {
                        let target_per_species = total_consciousness / 3.0;

                        for species_type in [SpeciesType::DiscoLlama, SpeciesType::QuantumSheep, SpeciesType::HypnoCamel] {
                            let species_llamas: Vec<&mut Llama> = llamas.iter_mut()
                                .filter(|l| l.species == species_type && l.consciousness > 0.1)
                                .collect();

                            if !species_llamas.is_empty() {
                                let current_total: f32 = species_llamas.iter()
                                    .map(|l| l.consciousness).sum();
                                let adjustment = (target_per_species - current_total) / species_llamas.len() as f32;

                                for llama in species_llamas {
                                    llama.consciousness = (llama.consciousness + adjustment * 0.5).max(0.5);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Observer position slowly drifts to maintain omnipresence
        observer.observer_position += Vec2::new(
            (cosmic_time * 0.1).sin() * dt * 10.0,
            (cosmic_time * 0.07).cos() * dt * 8.0
        );

        // Keep observer within screen bounds
        observer.observer_position.x = observer.observer_position.x.clamp(100.0, 1100.0);
        observer.observer_position.y = observer.observer_position.y.clamp(100.0, 700.0);
    }

    fn update_warfare_state(&mut self, llamas: &[Llama]) {
        // Update species populations
        let mut populations = [0u32; 3];
        for llama in llamas {
            if llama.consciousness > 0.1 { // Only count living entities
                let idx = match llama.species {
                    SpeciesType::DiscoLlama => 0,
                    SpeciesType::QuantumSheep => 1,
                    SpeciesType::HypnoCamel => 2,
                };
                populations[idx] += 1;
            }
        }
        self.warfare_state.species_populations = populations;

        // Update consciousness crystals controlled (placeholder - would need crystal system integration)
        // This would be calculated based on territorial control near crystal formations
        for i in 0..3 {
            self.warfare_state.consciousness_crystals_controlled[i] =
                (self.warfare_state.territorial_dominance[i] * 10.0) as u32;
        }
    }
}

/// Territory effects that can be applied to entities
#[derive(Debug, Default)]
pub struct TerritoryEffects {
    pub harmonic_boost: f32,
    pub social_boost: f32,
    pub chaos_boost: f32,
    pub reality_distortion_boost: f32,
    pub memory_boost: f32,
    pub consciousness_growth_boost: f32,
    pub quantum_boost: f32,
    pub exploration_boost: f32,
}