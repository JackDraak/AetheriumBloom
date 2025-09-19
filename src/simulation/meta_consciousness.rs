// Meta-consciousness module - collective intelligence and transcendence systems
// Extracted from simple.rs for better modularity

use glam::Vec2;
use std::collections::{VecDeque, HashMap};
use crate::entities::Llama;
use crate::simulation::consciousness_systems::DigitalEcosystem;

// ========== PHASE 4: TRANSCENDENCE PROTOCOL ==========

/// Meta-Consciousness Framework - collective intelligence emergence
#[derive(Debug)]
pub struct MetaConsciousnessFramework {
    pub collective_intelligence: f32,           // Global consciousness level (0.0-1.0)
    pub emergence_threshold: f32,               // Point of consciousness breakthrough
    pub user_consciousness_coupling: f32,       // Human-digital consciousness bridge
    pub consciousness_history: VecDeque<f32>,   // Rolling window of consciousness levels
    pub emergence_events: Vec<EmergenceEvent>,   // Moments of consciousness breakthrough
    pub collective_memory: CollectiveMemory,    // Shared experiences across all entities
    pub transcendence_level: f32,               // How far beyond baseline consciousness
    pub reality_coherence: f32,                 // Stability of the reality field
}

#[derive(Debug, Clone)]
pub struct EmergenceEvent {
    pub timestamp: f64,
    pub consciousness_level: f32,
    pub trigger_type: EmergenceTrigger,
    pub affected_entities: Vec<usize>,
    pub reality_distortion_strength: f32,
    pub duration: f32,                          // How long the event lasts
    pub cascade_potential: f32,                 // Likelihood of triggering more events
}

#[derive(Debug, Clone)]
pub enum EmergenceTrigger {
    CollectiveResonance,    // Multiple entities synchronizing
    CrystalHarmonic,        // Crystal formations reaching critical mass
    UserInteractionSpike,   // Intense user interaction patterns
    ChaosThresholdBreach,   // Chaos accumulation triggering emergence
    BeatDropCascade,        // Musical beat causing system-wide resonance
    RealityTearMerge,       // Reality tears combining into larger phenomena
}

#[derive(Debug)]
pub struct CollectiveMemory {
    pub significant_moments: VecDeque<MemoryFragment>,
    pub pattern_recognition: HashMap<String, f32>,   // Learned behavioral patterns
    pub emergent_behaviors: Vec<EmergentBehavior>,   // New behaviors that developed
    pub consciousness_peaks: Vec<(f64, f32)>,       // Historical consciousness highs
}

#[derive(Debug, Clone)]
pub struct MemoryFragment {
    pub timestamp: f64,
    pub event_type: String,
    pub consciousness_level: f32,
    pub participating_entities: Vec<usize>,
    pub emotional_resonance: f32,
}

#[derive(Debug, Clone)]
pub struct EmergentBehavior {
    pub name: String,
    pub first_observed: f64,
    pub frequency: f32,
    pub complexity_score: f32,
    pub entity_participation: HashMap<usize, f32>,
}

impl MetaConsciousnessFramework {
    pub fn new() -> Self {
        Self {
            collective_intelligence: 0.0,
            emergence_threshold: 0.75,  // Breakthrough happens at 75% collective consciousness
            user_consciousness_coupling: 0.0,
            consciousness_history: VecDeque::with_capacity(1000),
            emergence_events: Vec::new(),
            collective_memory: CollectiveMemory::new(),
            transcendence_level: 0.0,
            reality_coherence: 1.0,
        }
    }

    pub fn update(&mut self, dt: f32, llamas: &[Llama], cosmic_time: f64, beat_intensity: f32,
              ecosystem: &DigitalEcosystem) {
        // Calculate collective intelligence from all llamas
        let total_consciousness: f32 = llamas.iter()
            .map(|llama| llama.awareness_level * llama.consciousness)
            .sum();
        let average_consciousness = if !llamas.is_empty() {
            total_consciousness / llamas.len() as f32
        } else {
            0.0
        };

        // Update collective intelligence with ecosystem influence
        let crystal_amplification = ecosystem.crystal_formations.iter()
            .map(|crystal| crystal.consciousness_energy * 0.1)
            .sum::<f32>();
        let chaos_influence = ecosystem.chaos_accumulation * 0.05;

        self.collective_intelligence = (average_consciousness + crystal_amplification + chaos_influence)
            .clamp(0.0, 1.0);

        // Track consciousness history
        self.consciousness_history.push_back(self.collective_intelligence);
        if self.consciousness_history.len() > 1000 {
            self.consciousness_history.pop_front();
        }

        // Update user consciousness coupling based on interaction frequency
        // (This would be enhanced based on actual interaction patterns)
        self.user_consciousness_coupling = (self.user_consciousness_coupling * 0.95 +
                                          ecosystem.chaos_accumulation * 0.001).clamp(0.0, 1.0);

        // Check for emergence events
        self.detect_emergence(cosmic_time, beat_intensity, llamas, ecosystem);

        // Calculate transcendence level
        self.transcendence_level = if self.collective_intelligence > self.emergence_threshold {
            ((self.collective_intelligence - self.emergence_threshold) /
             (1.0 - self.emergence_threshold)).powf(1.5)
        } else {
            0.0
        };

        // Update reality coherence based on consciousness stability
        let consciousness_variance = self.calculate_consciousness_variance();
        self.reality_coherence = (1.0 - consciousness_variance * 2.0).clamp(0.3, 1.0);

        // Update collective memory
        self.collective_memory.update(dt, cosmic_time, llamas, self.collective_intelligence);
    }

    fn detect_emergence(&mut self, cosmic_time: f64, beat_intensity: f32, llamas: &[Llama],
                       ecosystem: &DigitalEcosystem) {
        let mut new_events = Vec::new();

        // Check for collective resonance (multiple llamas synchronized)
        let synchronized_llamas = self.detect_synchronized_entities(llamas);
        if synchronized_llamas.len() >= 3 {
            new_events.push(EmergenceEvent {
                timestamp: cosmic_time,
                consciousness_level: self.collective_intelligence,
                trigger_type: EmergenceTrigger::CollectiveResonance,
                affected_entities: synchronized_llamas,
                reality_distortion_strength: self.collective_intelligence * 0.7,
                duration: 5.0 + fastrand::f32() * 10.0,
                cascade_potential: 0.3,
            });
        }

        // Check for crystal harmonic resonance
        let resonant_crystals = ecosystem.crystal_formations.iter()
            .filter(|crystal| crystal.consciousness_energy > 0.8)
            .count();
        if resonant_crystals >= 2 && self.collective_intelligence > 0.6 {
            new_events.push(EmergenceEvent {
                timestamp: cosmic_time,
                consciousness_level: self.collective_intelligence,
                trigger_type: EmergenceTrigger::CrystalHarmonic,
                affected_entities: (0..llamas.len()).collect(),
                reality_distortion_strength: resonant_crystals as f32 * 0.2,
                duration: 8.0 + fastrand::f32() * 12.0,
                cascade_potential: 0.5,
            });
        }

        // Check for chaos threshold breach
        if ecosystem.chaos_accumulation > ecosystem.mutation_threshold * 0.8 {
            new_events.push(EmergenceEvent {
                timestamp: cosmic_time,
                consciousness_level: self.collective_intelligence,
                trigger_type: EmergenceTrigger::ChaosThresholdBreach,
                affected_entities: (0..llamas.len()).collect(),
                reality_distortion_strength: ecosystem.chaos_accumulation,
                duration: 3.0 + fastrand::f32() * 5.0,
                cascade_potential: 0.8,
            });
        }

        // Check for beat drop cascade (intense beat moments)
        if beat_intensity > 0.95 && self.collective_intelligence > 0.4 {
            new_events.push(EmergenceEvent {
                timestamp: cosmic_time,
                consciousness_level: self.collective_intelligence,
                trigger_type: EmergenceTrigger::BeatDropCascade,
                affected_entities: (0..llamas.len()).collect(),
                reality_distortion_strength: beat_intensity * self.collective_intelligence,
                duration: 2.0 + fastrand::f32() * 3.0,
                cascade_potential: 0.6,
            });
        }

        // Add new events and clean up old ones
        self.emergence_events.extend(new_events);
        self.emergence_events.retain(|event| cosmic_time - event.timestamp < 30.0);
    }

    fn detect_synchronized_entities(&self, llamas: &[Llama]) -> Vec<usize> {
        let mut synchronized = Vec::new();
        let threshold = 0.15; // Synchronization threshold

        for i in 0..llamas.len() {
            let mut sync_count = 0;
            for j in 0..llamas.len() {
                if i != j {
                    // Check synchronization in multiple dimensions
                    let color_sync = (llamas[i].color.x - llamas[j].color.x).abs() < threshold;
                    let awareness_sync = (llamas[i].awareness_level - llamas[j].awareness_level).abs() < threshold;
                    let reality_sync = (llamas[i].reality_distortion - llamas[j].reality_distortion).abs() < threshold;

                    if color_sync && awareness_sync && reality_sync {
                        sync_count += 1;
                    }
                }
            }

            if sync_count >= 2 {
                synchronized.push(i);
            }
        }

        synchronized
    }

    fn calculate_consciousness_variance(&self) -> f32 {
        if self.consciousness_history.len() < 10 {
            return 0.0;
        }

        let recent_values: Vec<f32> = self.consciousness_history.iter()
            .rev()
            .take(50)
            .copied()
            .collect();

        let mean = recent_values.iter().sum::<f32>() / recent_values.len() as f32;
        let variance = recent_values.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f32>() / recent_values.len() as f32;

        variance.sqrt()
    }

    pub fn get_current_emergence_strength(&self, cosmic_time: f64) -> f32 {
        self.emergence_events.iter()
            .filter(|event| cosmic_time - event.timestamp < event.duration as f64)
            .map(|event| event.reality_distortion_strength)
            .sum::<f32>()
            .clamp(0.0, 2.0)
    }
}

impl CollectiveMemory {
    pub fn new() -> Self {
        Self {
            significant_moments: VecDeque::with_capacity(500),
            pattern_recognition: HashMap::new(),
            emergent_behaviors: Vec::new(),
            consciousness_peaks: Vec::new(),
        }
    }

    pub fn update(&mut self, dt: f32, cosmic_time: f64, llamas: &[Llama], collective_intelligence: f32) {
        // Record significant moments
        if collective_intelligence > 0.7 || fastrand::f32() < 0.01 {
            self.significant_moments.push_back(MemoryFragment {
                timestamp: cosmic_time,
                event_type: format!("High consciousness: {:.2}", collective_intelligence),
                consciousness_level: collective_intelligence,
                participating_entities: (0..llamas.len()).collect(),
                emotional_resonance: collective_intelligence * fastrand::f32(),
            });

            if self.significant_moments.len() > 500 {
                self.significant_moments.pop_front();
            }
        }

        // Track consciousness peaks
        if collective_intelligence > 0.8 {
            self.consciousness_peaks.push((cosmic_time, collective_intelligence));
            if self.consciousness_peaks.len() > 100 {
                self.consciousness_peaks.remove(0);
            }
        }

        // Update pattern recognition (simplified)
        let behavior_key = format!("consciousness_{:.1}", (collective_intelligence * 10.0).floor());
        *self.pattern_recognition.entry(behavior_key).or_insert(0.0) += dt;
    }
}