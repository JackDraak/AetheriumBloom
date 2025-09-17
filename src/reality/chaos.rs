// Chaos effects system for visual madness

use glam::Vec2;

pub struct ChaosEffects {
    screen_distortion: Vec<Vec2>,
    chaos_particles: Vec<ChaosParticle>,
    reality_tears: Vec<RealityTear>,
    time_accumulator: f32,
}

#[derive(Debug, Clone)]
struct ChaosParticle {
    position: Vec2,
    velocity: Vec2,
    color: glam::Vec3,
    lifetime: f32,
    max_lifetime: f32,
    particle_type: ParticleType,
}

#[derive(Debug, Clone)]
enum ParticleType {
    ConsciousnessFragment,
    RealityRipple,
    BeatEcho,
    ChaosStatic,
}

#[derive(Debug, Clone)]
struct RealityTear {
    center: Vec2,
    radius: f32,
    intensity: f32,
    rotation: f32,
    creation_time: f32,
}

impl ChaosEffects {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        let grid_size = 50;
        let mut screen_distortion = Vec::new();

        // Initialize distortion grid
        for x in 0..grid_size {
            for y in 0..grid_size {
                let grid_x = (x as f32 / grid_size as f32) * screen_width;
                let grid_y = (y as f32 / grid_size as f32) * screen_height;
                screen_distortion.push(Vec2::new(grid_x, grid_y));
            }
        }

        Self {
            screen_distortion,
            chaos_particles: Vec::with_capacity(1000),
            reality_tears: Vec::new(),
            time_accumulator: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32, cosmic_time: f64, reality_distortion: f32, beat_intensity: f32) {
        self.time_accumulator += delta_time;

        // Update screen distortion
        self.update_screen_distortion(cosmic_time, reality_distortion);

        // Update particles
        self.update_particles(delta_time);

        // Update reality tears
        self.update_reality_tears(delta_time);

        // Spawn new effects based on state
        self.spawn_effects(cosmic_time, reality_distortion, beat_intensity);
    }

    fn update_screen_distortion(&mut self, cosmic_time: f64, reality_distortion: f32) {
        let time = cosmic_time as f32;

        for (i, distortion_point) in self.screen_distortion.iter_mut().enumerate() {
            let grid_x = i % 50;
            let grid_y = i / 50;

            // Create wave-based distortion
            let wave1 = (grid_x as f32 * 0.1 + time * 0.5).sin();
            let wave2 = (grid_y as f32 * 0.08 + time * 0.3).cos();
            let combined_wave = (wave1 + wave2) * reality_distortion * 5.0;

            // Apply mathematical distortion patterns
            let spiral_factor = (grid_x as f32 - 25.0).atan2(grid_y as f32 - 25.0) + time * 0.1;
            let spiral_distortion = Vec2::new(
                spiral_factor.cos() * combined_wave,
                spiral_factor.sin() * combined_wave,
            );

            *distortion_point += spiral_distortion * 0.1;

            // Decay distortion to prevent infinite accumulation
            *distortion_point *= 0.99;
        }
    }

    fn update_particles(&mut self, delta_time: f32) {
        // Update existing particles
        for particle in &mut self.chaos_particles {
            particle.position += particle.velocity * delta_time;
            particle.lifetime -= delta_time;

            // Update particle behavior based on type
            match particle.particle_type {
                ParticleType::ConsciousnessFragment => {
                    // Consciousness fragments spiral and fade
                    let spiral_speed = 2.0;
                    let angle = particle.lifetime * spiral_speed;
                    particle.velocity = Vec2::new(angle.cos(), angle.sin()) * 20.0;
                }
                ParticleType::RealityRipple => {
                    // Reality ripples expand outward
                    particle.velocity *= 1.05; // Accelerate outward
                }
                ParticleType::BeatEcho => {
                    // Beat echoes bounce and vibrate
                    if particle.position.x < 0.0 || particle.position.x > 1200.0 {
                        particle.velocity.x *= -0.8;
                    }
                    if particle.position.y < 0.0 || particle.position.y > 800.0 {
                        particle.velocity.y *= -0.8;
                    }
                }
                ParticleType::ChaosStatic => {
                    // Chaos static moves randomly
                    particle.velocity += Vec2::new(
                        (fastrand::f32() - 0.5) * 100.0,
                        (fastrand::f32() - 0.5) * 100.0,
                    ) * delta_time;
                    particle.velocity *= 0.9; // Damping
                }
            }

            // Fade color based on lifetime
            let fade_factor = particle.lifetime / particle.max_lifetime;
            particle.color *= fade_factor;
        }

        // Remove dead particles
        self.chaos_particles.retain(|p| p.lifetime > 0.0);
    }

    fn update_reality_tears(&mut self, delta_time: f32) {
        for tear in &mut self.reality_tears {
            tear.rotation += delta_time * 2.0; // Rotate tears
            tear.intensity *= 0.98; // Fade over time
        }

        // Remove weak tears
        self.reality_tears.retain(|t| t.intensity > 0.01);
    }

    fn spawn_effects(&mut self, cosmic_time: f64, reality_distortion: f32, beat_intensity: f32) {
        let spawn_rate = reality_distortion * 10.0 + beat_intensity * 5.0;

        // Spawn consciousness fragments
        if fastrand::f32() < spawn_rate * 0.01 {
            self.spawn_consciousness_fragment();
        }

        // Spawn reality ripples on beat
        if beat_intensity > 0.7 && fastrand::f32() < 0.3 {
            self.spawn_reality_ripple();
        }

        // Spawn chaos static when reality is heavily distorted
        if reality_distortion > 0.5 && fastrand::f32() < 0.1 {
            self.spawn_chaos_static();
        }

        // Occasionally spawn reality tears
        if fastrand::f32() < 0.001 {
            self.spawn_reality_tear();
        }
    }

    fn spawn_consciousness_fragment(&mut self) {
        let particle = ChaosParticle {
            position: Vec2::new(
                fastrand::f32() * 1200.0,
                fastrand::f32() * 800.0,
            ),
            velocity: Vec2::new(
                (fastrand::f32() - 0.5) * 50.0,
                (fastrand::f32() - 0.5) * 50.0,
            ),
            color: glam::Vec3::new(1.0, 0.5, 1.0), // Purple-pink
            lifetime: 3.0,
            max_lifetime: 3.0,
            particle_type: ParticleType::ConsciousnessFragment,
        };

        if self.chaos_particles.len() < 1000 {
            self.chaos_particles.push(particle);
        }
    }

    fn spawn_reality_ripple(&mut self) {
        let particle = ChaosParticle {
            position: Vec2::new(600.0, 400.0), // Screen center
            velocity: Vec2::new(
                (fastrand::f32() - 0.5) * 200.0,
                (fastrand::f32() - 0.5) * 200.0,
            ),
            color: glam::Vec3::new(0.0, 1.0, 1.0), // Cyan
            lifetime: 2.0,
            max_lifetime: 2.0,
            particle_type: ParticleType::RealityRipple,
        };

        if self.chaos_particles.len() < 1000 {
            self.chaos_particles.push(particle);
        }
    }

    fn spawn_chaos_static(&mut self) {
        for _ in 0..5 {
            let particle = ChaosParticle {
                position: Vec2::new(
                    fastrand::f32() * 1200.0,
                    fastrand::f32() * 800.0,
                ),
                velocity: Vec2::new(
                    (fastrand::f32() - 0.5) * 300.0,
                    (fastrand::f32() - 0.5) * 300.0,
                ),
                color: glam::Vec3::new(
                    fastrand::f32(),
                    fastrand::f32(),
                    fastrand::f32(),
                ),
                lifetime: 0.5,
                max_lifetime: 0.5,
                particle_type: ParticleType::ChaosStatic,
            };

            if self.chaos_particles.len() < 1000 {
                self.chaos_particles.push(particle);
            }
        }
    }

    fn spawn_reality_tear(&mut self) {
        let tear = RealityTear {
            center: Vec2::new(
                fastrand::f32() * 1200.0,
                fastrand::f32() * 800.0,
            ),
            radius: 20.0 + fastrand::f32() * 50.0,
            intensity: 1.0,
            rotation: 0.0,
            creation_time: self.time_accumulator,
        };

        self.reality_tears.push(tear);
    }

    pub fn add_beat_echo(&mut self, position: Vec2, intensity: f32) {
        let particle = ChaosParticle {
            position,
            velocity: Vec2::ZERO,
            color: glam::Vec3::new(1.0, 1.0, 0.0) * intensity, // Yellow
            lifetime: 1.0,
            max_lifetime: 1.0,
            particle_type: ParticleType::BeatEcho,
        };

        if self.chaos_particles.len() < 1000 {
            self.chaos_particles.push(particle);
        }
    }

    pub fn add_consciousness_explosion(&mut self, position: Vec2, count: u32) {
        for _ in 0..count {
            let angle = fastrand::f32() * std::f32::consts::TAU;
            let speed = 50.0 + fastrand::f32() * 100.0;

            let particle = ChaosParticle {
                position,
                velocity: Vec2::new(angle.cos() * speed, angle.sin() * speed),
                color: glam::Vec3::new(1.0, 0.0, 1.0), // Magenta
                lifetime: 2.0,
                max_lifetime: 2.0,
                particle_type: ParticleType::ConsciousnessFragment,
            };

            if self.chaos_particles.len() < 1000 {
                self.chaos_particles.push(particle);
            }
        }
    }

    pub fn get_screen_distortion(&self) -> &Vec<Vec2> {
        &self.screen_distortion
    }

    pub fn get_particles(&self) -> &Vec<ChaosParticle> {
        &self.chaos_particles
    }

    pub fn get_reality_tears(&self) -> &Vec<RealityTear> {
        &self.reality_tears
    }
}