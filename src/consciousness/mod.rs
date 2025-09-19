pub mod llama;
pub mod crystals;

use glam::Vec2;
use crate::core::ecs::{World, EntityId};
use crate::core::events::{EventBus, ChaosEvent, LlamaSpecies, CrystalType};
use crate::mathematics::BeatState;

pub use llama::{PsychedelicLlama, LlamaAI, ConsciousnessLevel};
pub use crystals::{ConsciousnessCrystal, CrystalField};

pub struct LlamaManager {
    world: World,
    crystal_field: CrystalField,
    event_bus: EventBus,
    llama_count: usize,
    total_consciousness: f32,
}

impl LlamaManager {
    pub fn new() -> Self {
        let world = World::new();
        let crystal_field = CrystalField::new(1200.0, 800.0); // Initial screen size
        let event_bus = EventBus::new(1000);

        // Spawn initial llamas for immediate chaos
        let mut manager = Self {
            world,
            crystal_field,
            event_bus,
            llama_count: 0,
            total_consciousness: 0.0,
        };

        // Start with 3 disco llamas
        manager.spawn_initial_llamas();
        manager
    }

    fn spawn_initial_llamas(&mut self) {
        for _ in 0..3 {
            self.spawn_disco_llama(
                Vec2::new(
                    fastrand::f32() * 1200.0,
                    fastrand::f32() * 800.0,
                ),
                1.0,
            );
        }
    }

    pub fn spawn_llamas(&mut self, count: u32, intensity: f32) {
        for _ in 0..count {
            let spawn_pos = Vec2::new(
                fastrand::f32() * 1200.0,
                fastrand::f32() * 800.0,
            );

            // Higher intensity = more likely to spawn exotic species
            if intensity > 0.8 && fastrand::f32() < 0.3 {
                self.spawn_quantum_sheep(spawn_pos, intensity);
            } else if intensity > 0.5 && fastrand::f32() < 0.2 {
                self.spawn_bassdrop_vicuna(spawn_pos, intensity);
            } else {
                self.spawn_disco_llama(spawn_pos, intensity);
            }
        }
    }

    fn spawn_disco_llama(&mut self, position: Vec2, consciousness: f32) -> EntityId {
        let entity = self.world.create_entity();

        let llama = PsychedelicLlama {
            consciousness_level: consciousness,
            trip_intensity: fastrand::f32() * 2.0,
            color_wavelength: Vec2::new(
                fastrand::f32() * 360.0, // Hue
                0.8 + fastrand::f32() * 0.2, // Saturation
            ),
            reality_distortion: 0.1,
            last_explosion_timestamp: 0.0,
            position,
            velocity: Vec2::new(
                (fastrand::f32() - 0.5) * 100.0,
                (fastrand::f32() - 0.5) * 100.0,
            ),
            species: LlamaSpecies::Disco,
        };

        let ai = LlamaAI::new();

        self.world.add_component(entity, llama);
        self.world.add_component(entity, ai);

        self.llama_count += 1;
        self.event_bus.publish(ChaosEvent::LlamaSpawned {
            entity_id: entity,
            consciousness: consciousness,
        });

        entity
    }

    fn spawn_quantum_sheep(&mut self, position: Vec2, consciousness: f32) -> EntityId {
        let entity = self.world.create_entity();

        let llama = PsychedelicLlama {
            consciousness_level: consciousness * 1.5, // Quantum boost
            trip_intensity: fastrand::f32() * 3.0,
            color_wavelength: Vec2::new(
                270.0 + fastrand::f32() * 60.0, // Purple spectrum
                0.9 + fastrand::f32() * 0.1,
            ),
            reality_distortion: 0.3,
            last_explosion_timestamp: 0.0,
            position,
            velocity: Vec2::ZERO, // Quantum sheep exist in multiple states
            species: LlamaSpecies::Quantum,
        };

        let ai = LlamaAI::new_quantum();

        self.world.add_component(entity, llama);
        self.world.add_component(entity, ai);

        self.llama_count += 1;
        entity
    }

    fn spawn_bassdrop_vicuna(&mut self, position: Vec2, consciousness: f32) -> EntityId {
        let entity = self.world.create_entity();

        let llama = PsychedelicLlama {
            consciousness_level: consciousness,
            trip_intensity: 1.0,
            color_wavelength: Vec2::new(
                0.0, // Red spectrum for bass
                1.0,
            ),
            reality_distortion: 0.05,
            last_explosion_timestamp: 0.0,
            position,
            velocity: Vec2::new(
                (fastrand::f32() - 0.5) * 50.0,
                (fastrand::f32() - 0.5) * 50.0,
            ),
            species: LlamaSpecies::BassDrop,
        };

        let ai = LlamaAI::new_bassdrop();

        self.world.add_component(entity, llama);
        self.world.add_component(entity, ai);

        self.llama_count += 1;
        entity
    }

    pub fn update(&mut self, cosmic_time: f64, beat_state: &BeatState) {
        self.total_consciousness = 0.0;

        // Update crystal field
        self.crystal_field.update(cosmic_time);

        // Update all llamas
        let entities: Vec<EntityId> = self.world.query::<PsychedelicLlama>()
            .iter()
            .map(|(id, _)| *id)
            .collect();

        for entity_id in entities {
            // Update AI first
            if let Some(ai) = self.world.get_component_mut::<LlamaAI>(entity_id) {
                if let Some(llama) = self.world.get_component::<PsychedelicLlama>(entity_id) {
                    ai.update(cosmic_time, beat_state, llama);
                }
            }

            // Then update llama
            if let Some(llama) = self.world.get_component_mut::<PsychedelicLlama>(entity_id) {
                // Update llama physics and consciousness
                llama.update(cosmic_time, beat_state);

                // Check for crystal harvesting
                if let Some(crystal) = self.crystal_field.check_harvest(llama.position, 20.0) {
                    llama.consciousness_level += crystal.consciousness_value;
                    self.event_bus.publish(ChaosEvent::CrystalHarvested {
                        llama_id: entity_id,
                        crystal_type: crystal.crystal_type,
                    });
                }

                self.total_consciousness += llama.consciousness_level;

                // Check for consciousness explosion (spawning)
                if llama.consciousness_level > 10.0 &&
                   (cosmic_time - llama.last_explosion_timestamp) > 5.0 {
                    let explosion_pos = llama.position;
                    self.trigger_consciousness_explosion(entity_id, explosion_pos);
                    llama.consciousness_level *= 0.5; // Reset but keep some consciousness
                    llama.last_explosion_timestamp = cosmic_time;
                }
            }
        }
    }

    fn trigger_consciousness_explosion(&mut self, source_id: EntityId, position: Vec2) {
        // Spawn 2-3 new llamas in explosion pattern
        let spawn_count = 2 + (fastrand::u32(0..2));
        let explosion_radius = 50.0;

        for i in 0..spawn_count {
            let angle = (i as f32 / spawn_count as f32) * std::f32::consts::TAU;
            let spawn_pos = position + Vec2::new(
                angle.cos() * explosion_radius,
                angle.sin() * explosion_radius,
            );

            self.spawn_disco_llama(spawn_pos, 1.0);
        }

        self.event_bus.publish(ChaosEvent::RealityTear {
            position,
            strength: 0.5,
        });
    }

    pub fn llama_count(&self) -> usize {
        self.llama_count
    }

    pub fn total_consciousness(&self) -> f32 {
        self.total_consciousness
    }

    pub fn get_render_data(&self) -> Vec<LlamaRenderData> {
        self.world.query::<PsychedelicLlama>()
            .iter()
            .map(|(_, llama)| LlamaRenderData {
                position: llama.position,
                color_wavelength: llama.color_wavelength,
                trip_intensity: llama.trip_intensity,
                reality_distortion: llama.reality_distortion,
                species: llama.species.clone(),
            })
            .collect()
    }

    pub fn get_crystal_render_data(&self) -> &CrystalField {
        &self.crystal_field
    }
}

#[derive(Debug, Clone)]
pub struct LlamaRenderData {
    pub position: Vec2,
    pub color_wavelength: Vec2, // Hue, Saturation
    pub trip_intensity: f32,
    pub reality_distortion: f32,
    pub species: LlamaSpecies,
}