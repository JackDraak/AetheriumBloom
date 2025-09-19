use glam::Vec2;

pub struct RealityField {
    pub distortion_grid: Vec<Vec<f32>>,
    pub consciousness_density: Vec<Vec<f32>>,
    pub width: usize,
    pub height: usize,
    pub cell_size: f32,
    pub tear_locations: Vec<RealityTear>,
}

#[derive(Debug, Clone)]
pub struct RealityTear {
    pub position: Vec2,
    pub strength: f32,
    pub radius: f32,
    pub creation_time: f64,
    pub tear_type: TearType,
}

#[derive(Debug, Clone)]
pub enum TearType {
    ConsciousnessExplosion,
    QuantumTeleportation,
    BeatDropRift,
    ChaosVortex,
}

impl RealityField {
    pub fn new(screen_width: f32, screen_height: f32, cell_size: f32) -> Self {
        let width = (screen_width / cell_size) as usize + 1;
        let height = (screen_height / cell_size) as usize + 1;

        let distortion_grid = vec![vec![0.0; height]; width];
        let consciousness_density = vec![vec![0.0; height]; width];

        Self {
            distortion_grid,
            consciousness_density,
            width,
            height,
            cell_size,
            tear_locations: Vec::new(),
        }
    }

    pub fn update(&mut self, cosmic_time: f64, delta_time: f32) {
        // Update reality tears
        self.update_tears(cosmic_time, delta_time);

        // Apply tear influences to distortion grid
        self.apply_tear_distortions();

        // Propagate distortions using wave equation
        self.propagate_distortions(delta_time);

        // Decay consciousness density
        self.decay_consciousness(delta_time);

        // Add background mathematical noise
        self.add_mathematical_noise(cosmic_time);
    }

    fn update_tears(&mut self, cosmic_time: f64, _delta_time: f32) {
        // Update existing tears
        for tear in &mut self.tear_locations {
            let age = cosmic_time - tear.creation_time;

            match tear.tear_type {
                TearType::ConsciousnessExplosion => {
                    // Expand rapidly then contract
                    if age < 2.0 {
                        tear.radius = age as f32 * 30.0;
                        tear.strength = (1.0 - age as f32 * 0.5).max(0.0);
                    } else {
                        tear.strength *= 0.95;
                    }
                }
                TearType::QuantumTeleportation => {
                    // Quick flash then fade
                    tear.strength = if age < 0.5 {
                        1.0 - age as f32 * 2.0
                    } else {
                        0.0
                    };
                }
                TearType::BeatDropRift => {
                    // Pulse with beat rhythm
                    tear.strength = (((age * 4.0).sin().abs() * 0.5 + 0.5) *
                                  (1.0 - (age * 0.2).min(1.0))) as f32;
                }
                TearType::ChaosVortex => {
                    // Chaotic swirling motion
                    tear.radius = 20.0 + (age * 3.0).sin() * 10.0;
                    tear.strength = (((age * 7.0).cos().abs() * 0.3 + 0.7) *
                                  (1.0 - (age * 0.1).min(1.0))) as f32;
                }
            }
        }

        // Remove expired tears
        self.tear_locations.retain(|tear| tear.strength > 0.01);
    }

    fn apply_tear_distortions(&mut self) {
        // Reset distortion grid
        for row in &mut self.distortion_grid {
            for cell in row {
                *cell = 0.0;
            }
        }

        // Apply each tear's influence
        for tear in &self.tear_locations {
            let grid_x = (tear.position.x / self.cell_size) as i32;
            let grid_y = (tear.position.y / self.cell_size) as i32;
            let influence_radius = (tear.radius / self.cell_size) as i32;

            for dx in -influence_radius..=influence_radius {
                for dy in -influence_radius..=influence_radius {
                    let x = grid_x + dx;
                    let y = grid_y + dy;

                    if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                        let distance = (dx * dx + dy * dy) as f32;
                        let max_distance = influence_radius as f32;

                        if distance <= max_distance * max_distance {
                            let falloff = 1.0 - (distance.sqrt() / max_distance);
                            let distortion = tear.strength * falloff;

                            self.distortion_grid[x as usize][y as usize] += distortion;
                        }
                    }
                }
            }
        }
    }

    fn propagate_distortions(&mut self, delta_time: f32) {
        // Simple wave propagation using finite differences
        let wave_speed = 100.0; // Units per second
        let damping = 0.98;

        let mut new_grid = self.distortion_grid.clone();

        for x in 1..self.width - 1 {
            for y in 1..self.height - 1 {
                // Calculate Laplacian (discrete)
                let laplacian = self.distortion_grid[x + 1][y] +
                               self.distortion_grid[x - 1][y] +
                               self.distortion_grid[x][y + 1] +
                               self.distortion_grid[x][y - 1] -
                               4.0 * self.distortion_grid[x][y];

                // Wave equation: ∂²u/∂t² = c²∇²u
                let acceleration = wave_speed * wave_speed * laplacian;
                new_grid[x][y] += acceleration * delta_time * delta_time;
                new_grid[x][y] *= damping;
            }
        }

        self.distortion_grid = new_grid;
    }

    fn decay_consciousness(&mut self, delta_time: f32) {
        let decay_rate = 0.5; // Consciousness fades over time

        for row in &mut self.consciousness_density {
            for cell in row {
                *cell *= 1.0 - decay_rate * delta_time;
            }
        }
    }

    fn add_mathematical_noise(&mut self, cosmic_time: f64) {
        // Add subtle mathematical patterns to the reality field
        let noise_amplitude = 0.02;

        for x in 0..self.width {
            for y in 0..self.height {
                let world_x = x as f32 * self.cell_size;
                let world_y = y as f32 * self.cell_size;

                // Use mathematical functions for natural-looking noise
                let noise = (world_x * 0.01 + cosmic_time as f32 * 0.1).sin() *
                           (world_y * 0.01 + cosmic_time as f32 * 0.07).cos() *
                           noise_amplitude;

                self.distortion_grid[x][y] += noise;
            }
        }
    }

    pub fn add_consciousness(&mut self, position: Vec2, amount: f32) {
        let grid_x = (position.x / self.cell_size) as usize;
        let grid_y = (position.y / self.cell_size) as usize;

        if grid_x < self.width && grid_y < self.height {
            self.consciousness_density[grid_x][grid_y] += amount;
        }
    }

    pub fn create_tear(&mut self, position: Vec2, tear_type: TearType, strength: f32, cosmic_time: f64) {
        let radius = match tear_type {
            TearType::ConsciousnessExplosion => 50.0,
            TearType::QuantumTeleportation => 30.0,
            TearType::BeatDropRift => 40.0,
            TearType::ChaosVortex => 35.0,
        };

        let tear = RealityTear {
            position,
            strength,
            radius,
            creation_time: cosmic_time,
            tear_type,
        };

        self.tear_locations.push(tear);
    }

    pub fn get_distortion_at(&self, position: Vec2) -> f32 {
        let grid_x = (position.x / self.cell_size) as usize;
        let grid_y = (position.y / self.cell_size) as usize;

        if grid_x < self.width && grid_y < self.height {
            self.distortion_grid[grid_x][grid_y]
        } else {
            0.0
        }
    }

    pub fn get_consciousness_at(&self, position: Vec2) -> f32 {
        let grid_x = (position.x / self.cell_size) as usize;
        let grid_y = (position.y / self.cell_size) as usize;

        if grid_x < self.width && grid_y < self.height {
            self.consciousness_density[grid_x][grid_y]
        } else {
            0.0
        }
    }

    pub fn resize(&mut self, screen_width: f32, screen_height: f32) {
        let new_width = (screen_width / self.cell_size) as usize + 1;
        let new_height = (screen_height / self.cell_size) as usize + 1;

        // Resize grids, preserving existing data where possible
        self.distortion_grid.resize(new_width, vec![0.0; new_height]);
        self.consciousness_density.resize(new_width, vec![0.0; new_height]);

        for row in &mut self.distortion_grid {
            row.resize(new_height, 0.0);
        }
        for row in &mut self.consciousness_density {
            row.resize(new_height, 0.0);
        }

        self.width = new_width;
        self.height = new_height;

        // Remove tears outside new bounds
        self.tear_locations.retain(|tear| {
            tear.position.x >= 0.0 && tear.position.x <= screen_width &&
            tear.position.y >= 0.0 && tear.position.y <= screen_height
        });
    }
}