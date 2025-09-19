use glam::Vec2;
use fastrand;

// Phase 2: Mathematical Chaos Engine Components

#[derive(Debug, Clone)]
pub struct ChaosDecisionEngine {
    pub dimensions: [f32; 11],          // 11D consciousness state space
    pub prime_generators: Vec<u64>,     // Prime number chaos sources
    pub quantum_fluctuations: f32,      // Uncertainty principle simulation
    pub harmonic_resonance: [f32; 7],   // Musical mathematics integration
}

impl ChaosDecisionEngine {
    pub fn new() -> Self {
        // Pre-calculated first 7 primes for performance
        let prime_generators = vec![2, 3, 5, 7, 11, 13, 17];

        Self {
            dimensions: [fastrand::f32(); 11],
            prime_generators,
            quantum_fluctuations: fastrand::f32(),
            harmonic_resonance: [fastrand::f32(); 7],
        }
    }

    pub fn update(&mut self, llama_data: LlamaSnapshot, beat_intensity: f32, cosmic_time: f64) {
        // Update 11D consciousness state space based on llama state
        self.dimensions[0] = (llama_data.color.x / 360.0 * std::f32::consts::TAU).sin();
        self.dimensions[1] = llama_data.awareness_level;
        self.dimensions[2] = llama_data.social_attraction;
        self.dimensions[3] = llama_data.exploration_drive;
        self.dimensions[4] = llama_data.reality_distortion;
        self.dimensions[5] = beat_intensity;
        self.dimensions[6] = llama_data.emotional_state;
        self.dimensions[7] = llama_data.memory_intensity;
        self.dimensions[8] = (cosmic_time as f32 * 0.1).sin();
        self.dimensions[9] = llama_data.consciousness;
        self.dimensions[10] = fastrand::f32(); // Pure chaos injection

        // Update quantum fluctuations using prime chaos
        let prime_chaos = self.calculate_prime_chaos(cosmic_time);
        self.quantum_fluctuations = (self.quantum_fluctuations + prime_chaos * 0.1).clamp(0.0, 1.0);

        // Update harmonic resonance layers
        for (i, harmonic) in self.harmonic_resonance.iter_mut().enumerate() {
            let prime_freq = self.prime_generators[i] as f32 / 100.0;
            *harmonic = (cosmic_time as f32 * prime_freq * 0.01).sin() * beat_intensity;
        }
    }

    fn calculate_prime_chaos(&self, cosmic_time: f64) -> f32 {
        let mut chaos = 0.0;
        for (i, &prime) in self.prime_generators.iter().enumerate() {
            let freq = prime as f32 / 1000.0;
            let phase = cosmic_time as f32 * freq + (i as f32 * 0.5);
            chaos += (phase.sin() * phase.cos()).abs();
        }
        (chaos / self.prime_generators.len() as f32).clamp(0.0, 1.0)
    }

    pub fn get_decision_vector(&self) -> DecisionVector {
        // Calculate behavior weights from 11D space
        let movement_urgency = self.dimensions[1] + self.dimensions[5] + self.quantum_fluctuations;
        let exploration_drive = self.dimensions[3] + self.dimensions[10] + self.harmonic_resonance[2];
        let social_attraction = self.dimensions[2] + self.dimensions[6] * 0.5;
        let chaos_acceptance = self.dimensions[10] + self.quantum_fluctuations + self.harmonic_resonance[6];

        DecisionVector {
            movement_urgency: movement_urgency.clamp(0.0, 1.0),
            exploration_drive: exploration_drive.clamp(0.0, 1.0),
            social_attraction: social_attraction.clamp(0.0, 1.0),
            chaos_acceptance: chaos_acceptance.clamp(0.0, 1.0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DecisionVector {
    pub movement_urgency: f32,
    pub exploration_drive: f32,
    pub social_attraction: f32,
    pub chaos_acceptance: f32,
}

#[derive(Debug, Clone)]
pub struct LlamaSnapshot {
    pub color: Vec2,
    pub awareness_level: f32,
    pub social_attraction: f32,
    pub exploration_drive: f32,
    pub reality_distortion: f32,
    pub emotional_state: f32,
    pub memory_intensity: f32,
    pub consciousness: f32,
}

// PHASE 5: CONSCIOUSNESS MULTIPLICATION - "When One Mind Becomes Legion"

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

#[derive(Debug, Clone, PartialEq)]
pub enum ConsciousnessLevel {
    Individual,
    Pack,
    Hive,
    Meta,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpeciesType {
    DiscoLlama,
    QuantumSheep,
    HypnoCamel,
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

#[derive(Debug, Clone)]
pub struct AdvancedBeatEngine {
    pub primary_rhythm: f32,            // Core mathematical heartbeat
    pub harmonic_layers: Vec<f32>,      // Multiple overlapping rhythms
    pub chaos_amplification: f32,       // Feedback loop strength
    pub consciousness_coupling: f32,    // Beat-consciousness interaction
    prime_list: Vec<u64>,               // Pre-calculated primes
    time_accumulator: f64,
}

impl AdvancedBeatEngine {
    pub fn new() -> Self {
        Self {
            primary_rhythm: 120.0, // BPM
            harmonic_layers: vec![1.0, 0.5, 0.25, 0.75, 1.33], // Harmonic ratios
            chaos_amplification: 0.3,
            consciousness_coupling: 0.5,
            prime_list: vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97], // First 25 primes
            time_accumulator: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32, total_consciousness: f32) -> f32 {
        self.time_accumulator += dt as f64;

        // Calculate base beat from primary rhythm
        let beat_phase = (self.time_accumulator * self.primary_rhythm as f64 / 60.0) % 1.0;
        let base_beat = ((beat_phase as f32) * std::f32::consts::TAU).sin().abs();

        // Add harmonic layers with prime number modulation
        let mut harmonic_sum = 0.0f32;
        for (i, &ratio) in self.harmonic_layers.iter().enumerate() {
            let prime = self.prime_list[i % self.prime_list.len()];
            let prime_mod = (prime % 17) as f32 / 17.0;
            let harmonic_phase = (self.time_accumulator * self.primary_rhythm as f64 * ratio as f64 / 60.0) % 1.0;
            let harmonic = ((harmonic_phase as f32) * std::f32::consts::TAU * prime_mod).sin().abs();
            harmonic_sum += harmonic * (1.0 / (i + 1) as f32); // Decay higher harmonics
        }
        harmonic_sum /= self.harmonic_layers.len() as f32;

        // Apply consciousness coupling
        let consciousness_factor = 1.0 + total_consciousness * self.consciousness_coupling * 0.1;

        // Apply chaos amplification
        let chaos_factor = 1.0 + self.chaos_amplification * fastrand::f32();

        // Combine all factors
        let final_intensity = (base_beat * 0.6 + harmonic_sum * 0.4) * consciousness_factor * chaos_factor;

        final_intensity.clamp(0.0, 2.0)
    }

    pub fn add_chaos_feedback(&mut self, chaos_amount: f32) {
        self.chaos_amplification = (self.chaos_amplification + chaos_amount * 0.1).clamp(0.0, 1.0);
        // Modulate harmonic layers with chaos
        for harmonic in &mut self.harmonic_layers {
            *harmonic *= 1.0 + chaos_amount * 0.05;
        }
    }
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

    pub fn update(&mut self, dt: f32, llamas: &mut [crate::simple::Llama], cosmic_time: f32) {
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

    fn update_consciousness_analysis(&mut self, llamas: &[crate::simple::Llama]) {
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

    fn process_consciousness_hierarchies(&mut self, llamas: &mut [crate::simple::Llama], _dt: f32) {
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
                llamas[member_idx].consciousness_level = consciousness_level.clone();
                if pack_members.len() > 1 {
                    llamas[member_idx].collective_id = Some(self.hierarchy_levels.len());
                } else {
                    llamas[member_idx].collective_id = None;
                }
            }

            // Create hierarchy entry
            self.hierarchy_levels.push(ConsciousnessHierarchy {
                level: consciousness_level.clone(),
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

    fn process_hive_mind_emergence(&mut self, llamas: &mut [crate::simple::Llama], dt: f32, cosmic_time: f32) {
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
            let hive_center = {
                let sum: Vec2 = hive.member_entities.iter()
                    .map(|&idx| llamas[idx].position)
                    .fold(Vec2::ZERO, |acc, pos| acc + pos);
                sum / hive.member_entities.len() as f32
            };
            hive.hive_center = hive_center;

            // Update collective consciousness
            hive.collective_consciousness = hive.member_entities.iter()
                .map(|&idx| llamas[idx].consciousness)
                .sum();

            // Apply hive mind effects to members
            for &member_idx in &hive.member_entities {
                if member_idx < llamas.len() {
                    let llama = &mut llamas[member_idx];

                    // Enhanced consciousness from collective
                    llama.consciousness += hive.collective_decision_weight * dt * 0.1;
                    llama.consciousness = llama.consciousness.min(2.0);

                    // Hive connection strength affects behavior
                    llama.hive_connection_strength = hive.collective_decision_weight;

                    // Collective memory sharing - occasionally add shared memories
                    if fastrand::f32() < 0.01 && !hive.shared_memories.is_empty() {
                        let shared_memory = hive.shared_memories[fastrand::usize(0..hive.shared_memories.len())];
                        if !llama.memory_fragments.contains(&shared_memory) {
                            llama.memory_fragments.push(shared_memory);
                        }
                    }
                }
            }

            true // Keep this hive
        });
    }

    fn process_consciousness_predation(&mut self, llamas: &mut [crate::simple::Llama], dt: f32) {
        // Check for new predation events
        for i in 0..llamas.len() {
            let predator = &llamas[i];

            if predator.consciousness > 1.5 { // High consciousness can predator others
                for j in 0..llamas.len() {
                    if i == j { continue; }

                    let prey = &llamas[j];
                    let distance = predator.position.distance(prey.position);

                    if distance < 30.0 && predator.consciousness > prey.consciousness * 1.5 {
                        // Check if predation already exists
                        let predation_exists = self.active_predations.iter()
                            .any(|p| (p.predator_id == i && p.prey_id == j) ||
                                     (p.predator_id == j && p.prey_id == i));

                        if !predation_exists && fastrand::f32() < 0.02 { // 2% chance per frame
                            self.active_predations.push(ConsciousnessPredation {
                                predator_id: i,
                                prey_id: j,
                                absorption_progress: 0.0,
                                resistance_strength: prey.consciousness * 0.5,
                                visual_effect_intensity: 1.0,
                            });
                        }
                    }
                }
            }
        }

        // Update active predations
        self.active_predations.retain_mut(|predation| {
            // Check if both entities still exist
            if predation.predator_id >= llamas.len() || predation.prey_id >= llamas.len() {
                return false;
            }

            let predator_consciousness = llamas[predation.predator_id].consciousness;
            let prey_consciousness = llamas[predation.prey_id].consciousness;

            // Check if predation should continue
            let distance = llamas[predation.predator_id].position.distance(llamas[predation.prey_id].position);
            if distance > 50.0 || prey_consciousness < 0.1 {
                return false; // End predation
            }

            // Progress absorption
            let absorption_rate = (predator_consciousness - predation.resistance_strength) * dt * 0.1;
            predation.absorption_progress += absorption_rate.max(0.0);

            // Apply resistance degradation
            predation.resistance_strength *= 0.99; // Resistance weakens over time
            predation.visual_effect_intensity *= 0.98; // Effect fades

            // Transfer consciousness
            if predation.absorption_progress > 0.0 {
                let transfer_amount = absorption_rate * 0.5;
                llamas[predation.predator_id].consciousness += transfer_amount;
                llamas[predation.prey_id].consciousness -= transfer_amount;
                llamas[predation.prey_id].consciousness = llamas[predation.prey_id].consciousness.max(0.1);
            }

            // Complete absorption if progress reaches 1.0
            if predation.absorption_progress >= 1.0 {
                llamas[predation.predator_id].consciousness += prey_consciousness * 0.8;
                llamas[predation.prey_id].consciousness = 0.05; // Nearly extinct
                llamas[predation.prey_id].extinction_pressure = 1.0;
                return false; // Complete predation
            }

            true // Continue predation
        });
    }

    fn process_species_warfare(&mut self, llamas: &mut [crate::simple::Llama], dt: f32, cosmic_time: f32) {
        // Check for new territorial conflicts
        for i in 0..llamas.len() {
            for j in (i + 1)..llamas.len() {
                let llama_a = &llamas[i];
                let llama_b = &llamas[j];

                // Different species can conflict
                if llama_a.species != llama_b.species {
                    let distance = llama_a.position.distance(llama_b.position);
                    let territorial_threshold = 100.0 - llama_a.social_attraction * 20.0;

                    if distance < territorial_threshold &&
                       llama_a.consciousness > 0.8 && llama_b.consciousness > 0.8 {

                        // Check if conflict already exists
                        let conflict_exists = self.warfare_state.active_conflicts.iter()
                            .any(|c| (c.attacker_species == llama_a.species && c.defender_species == llama_b.species) ||
                                     (c.attacker_species == llama_b.species && c.defender_species == llama_a.species));

                        if !conflict_exists && fastrand::f32() < 0.01 { // 1% chance per frame
                            let territory_center = (llama_a.position + llama_b.position) * 0.5;

                            self.warfare_state.active_conflicts.push(SpeciesConflict {
                                attacker_species: llama_a.species.clone(),
                                defender_species: llama_b.species.clone(),
                                conflict_intensity: 0.5 + fastrand::f32() * 0.5,
                                territory_contested: territory_center,
                                duration: 0.0,
                                victory_threshold: 0.7 + fastrand::f32() * 0.3,
                            });
                        }
                    }
                }
            }
        }

        // Update existing conflicts
        self.warfare_state.active_conflicts.retain_mut(|conflict| {
            conflict.duration += dt;

            // Calculate species strength in the conflict area
            let mut attacker_strength = 0.0f32;
            let mut defender_strength = 0.0f32;
            let conflict_radius = 150.0;

            for llama in llamas.iter() {
                let distance_to_conflict = llama.position.distance(conflict.territory_contested);
                if distance_to_conflict < conflict_radius {
                    let proximity_factor = 1.0 - (distance_to_conflict / conflict_radius);
                    let contribution = llama.consciousness * proximity_factor;

                    if llama.species == conflict.attacker_species {
                        attacker_strength += contribution;
                    } else if llama.species == conflict.defender_species {
                        defender_strength += contribution;
                    }
                }
            }

            // Update conflict intensity based on relative strength
            let strength_ratio = if defender_strength > 0.0 {
                attacker_strength / defender_strength
            } else {
                attacker_strength + 1.0
            };

            conflict.conflict_intensity = (strength_ratio * 0.5 + conflict.conflict_intensity * 0.5)
                .clamp(0.1, 2.0);

            // Check for conflict resolution
            if strength_ratio > conflict.victory_threshold {
                // Attacker wins - boost attacker species, weaken defender species
                for llama in llamas.iter_mut() {
                    let distance_to_conflict = llama.position.distance(conflict.territory_contested);
                    if distance_to_conflict < conflict_radius {
                        if llama.species == conflict.attacker_species {
                            llama.consciousness += 0.1;
                            llama.territorial_dominance += 0.05;
                            llama.warfare_participation += 0.1;
                        } else if llama.species == conflict.defender_species {
                            llama.consciousness -= 0.05;
                            llama.extinction_pressure += 0.1;
                            llama.warfare_participation -= 0.05;
                        }
                    }
                }
                return false; // End conflict
            } else if (1.0 / strength_ratio) > conflict.victory_threshold {
                // Defender wins - boost defender species, weaken attacker species
                for llama in llamas.iter_mut() {
                    let distance_to_conflict = llama.position.distance(conflict.territory_contested);
                    if distance_to_conflict < conflict_radius {
                        if llama.species == conflict.defender_species {
                            llama.consciousness += 0.1;
                            llama.territorial_dominance += 0.05;
                            llama.warfare_participation += 0.1;
                        } else if llama.species == conflict.attacker_species {
                            llama.consciousness -= 0.05;
                            llama.extinction_pressure += 0.1;
                            llama.warfare_participation -= 0.05;
                        }
                    }
                }
                return false; // End conflict
            }

            // Conflict continues if no clear winner and not too old
            conflict.duration < 30.0 // Max 30 second conflicts
        });

        // Apply warfare effects to participating llamas
        for llama in llamas.iter_mut() {
            let mut in_warfare = false;
            for conflict in &self.warfare_state.active_conflicts {
                let distance_to_conflict = llama.position.distance(conflict.territory_contested);
                if distance_to_conflict < 150.0 &&
                   (llama.species == conflict.attacker_species || llama.species == conflict.defender_species) {
                    in_warfare = true;
                    llama.warfare_participation = (llama.warfare_participation + dt * 0.1).min(1.0);
                    llama.emotional_state += conflict.conflict_intensity * dt * 0.1;
                    break;
                }
            }

            if !in_warfare {
                llama.warfare_participation *= 0.95; // Decay warfare participation
            }
        }
    }

    fn process_evolution_pressure(&mut self, llamas: &mut [crate::simple::Llama], dt: f32) {
        self.evolution_pressure_accumulator += dt;

        // Apply evolution pressure every 5 seconds
        if self.evolution_pressure_accumulator > 5.0 {
            self.evolution_pressure_accumulator = 0.0;

            // Calculate species fitness
            let mut species_fitness = [0.0f32; 3];
            let mut species_counts = [0u32; 3];

            for llama in llamas.iter() {
                if llama.consciousness > 0.1 {
                    let species_idx = match llama.species {
                        SpeciesType::DiscoLlama => 0,
                        SpeciesType::QuantumSheep => 1,
                        SpeciesType::HypnoCamel => 2,
                    };

                    species_fitness[species_idx] += llama.consciousness + llama.territorial_dominance - llama.extinction_pressure;
                    species_counts[species_idx] += 1;
                }
            }

            // Normalize fitness by population
            for i in 0..3 {
                if species_counts[i] > 0 {
                    species_fitness[i] /= species_counts[i] as f32;
                }
            }

            // Apply evolution pressure based on relative fitness
            let max_fitness = species_fitness.iter().fold(0.0f32, |a, &b| a.max(b));
            let min_fitness = species_fitness.iter().fold(f32::INFINITY, |a, &b| a.min(b));

            if max_fitness > min_fitness {
                for llama in llamas.iter_mut() {
                    if llama.consciousness > 0.1 {
                        let species_idx = match llama.species {
                            SpeciesType::DiscoLlama => 0,
                            SpeciesType::QuantumSheep => 1,
                            SpeciesType::HypnoCamel => 2,
                        };

                        let relative_fitness = (species_fitness[species_idx] - min_fitness) / (max_fitness - min_fitness);

                        if relative_fitness < 0.3 { // Low fitness species face extinction pressure
                            llama.extinction_pressure += 0.2;
                            llama.consciousness *= 0.95;
                        } else if relative_fitness > 0.7 { // High fitness species thrive
                            llama.extinction_pressure *= 0.8;
                            llama.consciousness += 0.05;
                            llama.territorial_dominance += 0.02;
                        }
                    }
                }
            }
        }

        // Handle extinction events
        for llama in llamas.iter_mut() {
            if llama.extinction_pressure > 0.8 {
                llama.consciousness *= 0.9; // Accelerated consciousness loss
                if llama.consciousness < 0.1 {
                    llama.consciousness = 0.05; // Near extinction
                    llama.extinction_pressure = 1.0;
                }
            }
        }
    }

    fn process_meta_observer_interventions(&mut self, llamas: &mut [crate::simple::Llama], dt: f32, cosmic_time: f32) {
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
                            let species_llamas: Vec<&mut crate::simple::Llama> = llamas.iter_mut()
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

    fn update_warfare_state(&mut self, llamas: &[crate::simple::Llama]) {
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