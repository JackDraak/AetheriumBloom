use glam::Vec2;
use crate::core::events::CrystalType;

#[derive(Debug, Clone)]
pub struct ConsciousnessCrystal {
    pub position: Vec2,
    pub crystal_type: CrystalType,
    pub consciousness_value: f32,
    pub spawn_time: f64,
    pub visual_intensity: f32,
    pub harvested: bool,
}

impl ConsciousnessCrystal {
    pub fn new(position: Vec2, crystal_type: CrystalType, spawn_time: f64) -> Self {
        let consciousness_value = match crystal_type {
            CrystalType::PurpleHaze => 0.5,
            CrystalType::NeonDream => 1.0,
            CrystalType::MathematicalHoney => 1.5,
            CrystalType::VoidSprinkles => 2.0,
            CrystalType::CosmicGiggle => 3.0,
        };

        Self {
            position,
            crystal_type,
            consciousness_value,
            spawn_time,
            visual_intensity: 1.0,
            harvested: false,
        }
    }

    pub fn update(&mut self, cosmic_time: f64) {
        // Crystals pulse with mathematical frequencies
        let age = cosmic_time - self.spawn_time;
        self.visual_intensity = (age as f32 * 2.0).sin().abs() * 0.5 + 0.5;

        // Some crystal types have special behaviors
        match self.crystal_type {
            CrystalType::VoidSprinkles => {
                // Void sprinkles occasionally disappear and reappear
                if age > 0.0 && (age * 10.0).sin() < -0.9 {
                    self.visual_intensity = 0.1;
                }
            }
            CrystalType::CosmicGiggle => {
                // Cosmic giggles grow in power over time
                self.consciousness_value = 3.0 + (age as f32 * 0.1).min(2.0);
            }
            _ => {}
        }
    }

    pub fn get_color(&self) -> Vec2 {
        match self.crystal_type {
            CrystalType::PurpleHaze => Vec2::new(270.0, 0.8), // Purple
            CrystalType::NeonDream => Vec2::new(180.0, 1.0),  // Cyan
            CrystalType::MathematicalHoney => Vec2::new(45.0, 0.9), // Orange
            CrystalType::VoidSprinkles => Vec2::new(0.0, 0.0), // Black (void)
            CrystalType::CosmicGiggle => Vec2::new(300.0, 1.0), // Magenta
        }
    }
}

pub struct CrystalField {
    crystals: Vec<ConsciousnessCrystal>,
    width: f32,
    height: f32,
    spawn_timer: f64,
    next_spawn_time: f64,
}

impl CrystalField {
    pub fn new(width: f32, height: f32) -> Self {
        let mut field = Self {
            crystals: Vec::with_capacity(100),
            width,
            height,
            spawn_timer: 0.0,
            next_spawn_time: 2.0, // First spawn in 2 seconds
        };

        // Spawn initial crystals for immediate interaction
        field.spawn_initial_crystals();
        field
    }

    fn spawn_initial_crystals(&mut self) {
        for _ in 0..10 {
            let position = Vec2::new(
                fastrand::f32() * self.width,
                fastrand::f32() * self.height,
            );

            let crystal_type = self.random_crystal_type(0.0); // Basic crystals initially
            let crystal = ConsciousnessCrystal::new(position, crystal_type, 0.0);
            self.crystals.push(crystal);
        }
    }

    pub fn update(&mut self, cosmic_time: f64) {
        self.spawn_timer = cosmic_time;

        // Update existing crystals
        for crystal in &mut self.crystals {
            if !crystal.harvested {
                crystal.update(cosmic_time);
            }
        }

        // Remove harvested crystals
        self.crystals.retain(|c| !c.harvested);

        // Spawn new crystals periodically
        if cosmic_time >= self.next_spawn_time {
            self.spawn_random_crystal(cosmic_time);
            self.next_spawn_time = cosmic_time + self.calculate_spawn_interval();
        }

        // Maintain reasonable crystal count
        while self.crystals.len() > 50 {
            // Remove oldest unharvested crystal
            if let Some(pos) = self.crystals.iter().position(|c| !c.harvested) {
                self.crystals.remove(pos);
            } else {
                break;
            }
        }
    }

    fn spawn_random_crystal(&mut self, cosmic_time: f64) {
        let position = Vec2::new(
            fastrand::f32() * self.width,
            fastrand::f32() * self.height,
        );

        let crystal_type = self.random_crystal_type(cosmic_time);
        let crystal = ConsciousnessCrystal::new(position, crystal_type, cosmic_time);
        self.crystals.push(crystal);
    }

    fn random_crystal_type(&self, cosmic_time: f64) -> CrystalType {
        // Crystal rarity increases over time
        let rarity_boost = (cosmic_time as f32 / 60.0).min(1.0); // Max boost after 1 minute
        let roll = fastrand::f32();

        match roll {
            x if x < 0.4 - rarity_boost * 0.1 => CrystalType::PurpleHaze,
            x if x < 0.7 - rarity_boost * 0.1 => CrystalType::NeonDream,
            x if x < 0.85 => CrystalType::MathematicalHoney,
            x if x < 0.95 => CrystalType::VoidSprinkles,
            _ => CrystalType::CosmicGiggle,
        }
    }

    fn calculate_spawn_interval(&self) -> f64 {
        // Spawn rate increases with fewer crystals
        let base_interval = 3.0;
        let density_factor = (self.crystals.len() as f32 / 30.0).min(1.0);
        base_interval * (0.5 + density_factor as f64 * 0.5)
    }

    pub fn check_harvest(&mut self, position: Vec2, radius: f32) -> Option<ConsciousnessCrystal> {
        let radius_sq = radius * radius;

        for crystal in &mut self.crystals {
            if !crystal.harvested && crystal.position.distance_squared(position) <= radius_sq {
                crystal.harvested = true;
                return Some(crystal.clone());
            }
        }

        None
    }

    pub fn get_visible_crystals(&self) -> impl Iterator<Item = &ConsciousnessCrystal> {
        self.crystals.iter().filter(|c| !c.harvested && c.visual_intensity > 0.1)
    }

    pub fn crystal_count(&self) -> usize {
        self.crystals.iter().filter(|c| !c.harvested).count()
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;

        // Remove crystals outside new bounds
        self.crystals.retain(|c| {
            c.position.x >= 0.0 && c.position.x <= width &&
            c.position.y >= 0.0 && c.position.y <= height
        });
    }
}