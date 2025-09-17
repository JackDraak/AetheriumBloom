// Event system for chaos coordination
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub enum ChaosEvent {
    LlamaSpawned { entity_id: u32, consciousness: f32 },
    CrystalHarvested { llama_id: u32, crystal_type: CrystalType },
    BeatDrop { intensity: f32, cosmic_time: f64 },
    RealityTear { position: glam::Vec2, strength: f32 },
    ConsciousnessResonance { frequency: f32, harmonic: u8 },
    SpeciesMutation { from_id: u32, to_species: LlamaSpecies },
}

#[derive(Debug, Clone)]
pub enum CrystalType {
    PurpleHaze,
    NeonDream,
    MathematicalHoney,
    VoidSprinkles,
    CosmicGiggle,
}

#[derive(Debug, Clone)]
pub enum LlamaSpecies {
    Disco,
    Quantum,
    Hypno,
    Fractal,
    BassDrop,
}

pub struct EventBus {
    events: VecDeque<ChaosEvent>,
    max_events: usize,
}

impl EventBus {
    pub fn new(max_events: usize) -> Self {
        Self {
            events: VecDeque::with_capacity(max_events),
            max_events,
        }
    }

    pub fn publish(&mut self, event: ChaosEvent) {
        if self.events.len() >= self.max_events {
            self.events.pop_front();
        }
        self.events.push_back(event);
    }

    pub fn consume_events(&mut self) -> impl Iterator<Item = ChaosEvent> + '_ {
        self.events.drain(..)
    }

    pub fn peek_events(&self) -> impl Iterator<Item = &ChaosEvent> {
        self.events.iter()
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}