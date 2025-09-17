// 11-dimensional mathematical space for llama consciousness
use glam::Vec3;

#[derive(Debug, Clone)]
pub struct ElevenDimensionalSpace {
    pub dimensions: [f32; 11],
    pub names: [&'static str; 11],
}

impl ElevenDimensionalSpace {
    pub fn new() -> Self {
        Self {
            dimensions: [0.0; 11],
            names: [
                "Color Temperature of Dreams",
                "Prime Number Fluctuations",
                "Mathematical Mercury Retrograde",
                "Collective Llama Consciousness",
                "Reality Stability Index",
                "Trip Intensity Resonance",
                "Beat Synchronization Level",
                "Spatial Harmony Coefficient",
                "Temporal Flow Direction",
                "Consciousness Recursion Depth",
                "Pure Chaos Injection"
            ],
        }
    }

    pub fn from_context(
        color_hue: f32,
        prime_factor: f32,
        cosmic_time: f64,
        consciousness_level: f32,
        reality_distortion: f32,
        trip_intensity: f32,
        beat_drop: bool,
        position: glam::Vec2,
    ) -> Self {
        let mut space = Self::new();

        // Dimension 0: Color temperature of nearby dreams
        space.dimensions[0] = (color_hue / 360.0 * std::f32::consts::TAU).sin();

        // Dimension 1: Prime number fluctuations
        space.dimensions[1] = prime_factor.clamp(0.0, 1.0);

        // Dimension 2: Mathematical Mercury retrograde
        space.dimensions[2] = (((cosmic_time as f32) * 0.01).sin() + 1.0) * 0.5;

        // Dimension 3: Collective llama consciousness
        space.dimensions[3] = (consciousness_level / 10.0).clamp(0.0, 1.0);

        // Dimension 4: Reality stability
        space.dimensions[4] = (1.0 - reality_distortion).clamp(0.0, 1.0);

        // Dimension 5: Trip intensity resonance
        space.dimensions[5] = (trip_intensity / 5.0).clamp(0.0, 1.0);

        // Dimension 6: Beat synchronization
        space.dimensions[6] = if beat_drop { 1.0 } else { 0.0 };

        // Dimension 7: Spatial harmony coefficient
        space.dimensions[7] = (position.length() / 1000.0).sin().abs();

        // Dimension 8: Temporal flow direction
        space.dimensions[8] = (((cosmic_time as f32) * 0.1).cos() + 1.0) * 0.5;

        // Dimension 9: Consciousness recursion depth
        space.dimensions[9] = if consciousness_level > 0.0 {
            (consciousness_level.ln() / 10.0).clamp(0.0, 1.0)
        } else {
            0.0
        };

        // Dimension 10: Pure chaos injection
        space.dimensions[10] = fastrand::f32();

        space
    }

    pub fn distance_to(&self, other: &ElevenDimensionalSpace) -> f32 {
        let mut sum_sq = 0.0;
        for i in 0..11 {
            let diff = self.dimensions[i] - other.dimensions[i];
            sum_sq += diff * diff;
        }
        sum_sq.sqrt()
    }

    pub fn dot_product(&self, other: &ElevenDimensionalSpace) -> f32 {
        let mut sum = 0.0;
        for i in 0..11 {
            sum += self.dimensions[i] * other.dimensions[i];
        }
        sum
    }

    pub fn magnitude(&self) -> f32 {
        let sum_sq: f32 = self.dimensions.iter().map(|&x| x * x).sum();
        sum_sq.sqrt()
    }

    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        if mag > 0.0 {
            for dim in &mut self.dimensions {
                *dim /= mag;
            }
        }
    }

    pub fn project_to_3d(&self) -> Vec3 {
        // Project 11D space to 3D for visualization
        // Use principal component analysis approximation

        // X: Combination of color and spatial dimensions
        let x = self.dimensions[0] * 0.7 + self.dimensions[7] * 0.3;

        // Y: Combination of consciousness and chaos
        let y = self.dimensions[3] * 0.6 + self.dimensions[10] * 0.4;

        // Z: Combination of time and reality
        let z = self.dimensions[8] * 0.5 + self.dimensions[4] * 0.5;

        Vec3::new(x, y, z)
    }

    pub fn calculate_decision_weights(&self, target_behavior: BehaviorType) -> DecisionWeights {
        match target_behavior {
            BehaviorType::Harvest => DecisionWeights {
                movement_urgency: self.dimensions[1] + self.dimensions[6], // Prime + beat
                exploration_drive: self.dimensions[10] + self.dimensions[2], // Chaos + mercury
                social_attraction: self.dimensions[3], // Collective consciousness
                chaos_acceptance: self.dimensions[10] + self.dimensions[5], // Pure chaos + trip
            },
            BehaviorType::Dance => DecisionWeights {
                movement_urgency: self.dimensions[6] + self.dimensions[5], // Beat + trip
                exploration_drive: self.dimensions[0], // Color dreams
                social_attraction: self.dimensions[3] + self.dimensions[6], // Consciousness + beat
                chaos_acceptance: self.dimensions[10] + self.dimensions[4], // Chaos + reality
            },
            BehaviorType::Meditate => DecisionWeights {
                movement_urgency: 1.0 - self.dimensions[6], // Anti-beat
                exploration_drive: self.dimensions[9], // Recursion depth
                social_attraction: self.dimensions[3] * 0.5, // Reduced social
                chaos_acceptance: self.dimensions[4], // Reality stability
            },
            BehaviorType::Chaos => DecisionWeights {
                movement_urgency: self.dimensions[10] + self.dimensions[1], // Chaos + prime
                exploration_drive: self.dimensions[10] + self.dimensions[2], // Chaos + mercury
                social_attraction: 1.0 - self.dimensions[3], // Anti-social
                chaos_acceptance: self.dimensions[10] + self.dimensions[5] + self.dimensions[1], // Max chaos
            },
        }
    }

    pub fn get_dominant_dimensions(&self, count: usize) -> Vec<(usize, f32, &'static str)> {
        let mut indexed_dims: Vec<(usize, f32, &'static str)> = self.dimensions
            .iter()
            .enumerate()
            .map(|(i, &val)| (i, val, self.names[i]))
            .collect();

        indexed_dims.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        indexed_dims.into_iter().take(count).collect()
    }

    pub fn interpolate(&self, other: &ElevenDimensionalSpace, t: f32) -> ElevenDimensionalSpace {
        let mut result = Self::new();
        for i in 0..11 {
            result.dimensions[i] = self.dimensions[i] * (1.0 - t) + other.dimensions[i] * t;
        }
        result
    }

    pub fn add_chaos(&mut self, amount: f32) {
        // Add controlled chaos to all dimensions
        for dim in &mut self.dimensions {
            *dim += (fastrand::f32() - 0.5) * amount * 2.0;
            *dim = dim.clamp(0.0, 1.0);
        }

        // Boost the pure chaos dimension
        self.dimensions[10] = (self.dimensions[10] + amount).min(1.0);
    }
}

#[derive(Debug, Clone)]
pub enum BehaviorType {
    Harvest,
    Dance,
    Meditate,
    Chaos,
}

#[derive(Debug, Clone)]
pub struct DecisionWeights {
    pub movement_urgency: f32,
    pub exploration_drive: f32,
    pub social_attraction: f32,
    pub chaos_acceptance: f32,
}

impl DecisionWeights {
    pub fn normalize(&mut self) {
        let sum = self.movement_urgency + self.exploration_drive +
                  self.social_attraction + self.chaos_acceptance;
        if sum > 0.0 {
            self.movement_urgency /= sum;
            self.exploration_drive /= sum;
            self.social_attraction /= sum;
            self.chaos_acceptance /= sum;
        }
    }

    pub fn strongest_drive(&self) -> BehaviorDrive {
        let weights = [
            ("movement", self.movement_urgency),
            ("exploration", self.exploration_drive),
            ("social", self.social_attraction),
            ("chaos", self.chaos_acceptance),
        ];

        let max_weight = weights.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        match max_weight.map(|(name, _)| *name).unwrap_or("movement") {
            "movement" => BehaviorDrive::Movement,
            "exploration" => BehaviorDrive::Exploration,
            "social" => BehaviorDrive::Social,
            "chaos" => BehaviorDrive::Chaos,
            _ => BehaviorDrive::Movement,
        }
    }
}

#[derive(Debug, Clone)]
pub enum BehaviorDrive {
    Movement,
    Exploration,
    Social,
    Chaos,
}