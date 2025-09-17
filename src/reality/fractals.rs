// Fractal generation for psychedelic backgrounds

use glam::Vec2;

pub struct FractalGenerator {
    julia_c: Vec2,
    mandelbrot_zoom: f32,
    time_offset: f32,
    chaos_factor: f32,
}

impl FractalGenerator {
    pub fn new() -> Self {
        Self {
            julia_c: Vec2::new(-0.7, 0.27015),
            mandelbrot_zoom: 1.0,
            time_offset: 0.0,
            chaos_factor: 0.0,
        }
    }

    pub fn update(&mut self, cosmic_time: f64, consciousness_level: f32, reality_distortion: f32) {
        self.time_offset = cosmic_time as f32 * 0.1;

        // Consciousness affects Julia set parameters
        self.julia_c = Vec2::new(
            -0.7 + (consciousness_level * 0.3 + self.time_offset * 0.1).sin() * 0.2,
            0.27015 + (consciousness_level * 0.4 + self.time_offset * 0.07).cos() * 0.1,
        );

        // Reality distortion affects zoom and chaos
        self.mandelbrot_zoom = 1.0 + reality_distortion * 2.0;
        self.chaos_factor = reality_distortion;
    }

    pub fn generate_julia_field(&self, width: usize, height: usize, offset: Vec2) -> Vec<f32> {
        let mut field = vec![0.0; width * height];

        for y in 0..height {
            for x in 0..width {
                let u = (x as f32 / width as f32 - 0.5) * 2.0;
                let v = (y as f32 / height as f32 - 0.5) * 2.0;

                let mut z = Vec2::new(u, v) + offset;
                let mut iterations = 0;
                let max_iterations = 50;

                for _ in 0..max_iterations {
                    if z.length_squared() > 4.0 {
                        break;
                    }
                    z = self.julia_iteration(z) + self.julia_c;
                    iterations += 1;
                }

                let t = iterations as f32 / max_iterations as f32;
                field[y * width + x] = t;
            }
        }

        field
    }

    pub fn generate_mandelbrot_field(&self, width: usize, height: usize, center: Vec2) -> Vec<f32> {
        let mut field = vec![0.0; width * height];

        for y in 0..height {
            for x in 0..width {
                let u = (x as f32 / width as f32 - 0.5) * 4.0 / self.mandelbrot_zoom + center.x;
                let v = (y as f32 / height as f32 - 0.5) * 4.0 / self.mandelbrot_zoom + center.y;

                let c = Vec2::new(u, v);
                let mut z = Vec2::ZERO;
                let mut iterations = 0;
                let max_iterations = 100;

                for _ in 0..max_iterations {
                    if z.length_squared() > 4.0 {
                        break;
                    }
                    z = self.mandelbrot_iteration(z) + c;
                    iterations += 1;
                }

                let t = iterations as f32 / max_iterations as f32;
                field[y * width + x] = t;
            }
        }

        field
    }

    pub fn generate_chaos_fractal(&self, width: usize, height: usize) -> Vec<f32> {
        let mut field = vec![0.0; width * height];

        for y in 0..height {
            for x in 0..width {
                let u = x as f32 / width as f32;
                let v = y as f32 / height as f32;

                // Create chaotic fractal using multiple mathematical functions
                let chaos1 = (u * 10.0 + self.time_offset).sin() * (v * 8.0 + self.time_offset * 0.7).cos();
                let chaos2 = (u * 6.0 + self.time_offset * 1.3).cos() * (v * 12.0 + self.time_offset * 0.9).sin();
                let chaos3 = ((u + v) * 15.0 + self.time_offset * 0.5).sin();

                let combined_chaos = (chaos1 + chaos2 + chaos3) / 3.0;
                let fractal_value = (combined_chaos * self.chaos_factor).sin() * 0.5 + 0.5;

                field[y * width + x] = fractal_value;
            }
        }

        field
    }

    fn julia_iteration(&self, z: Vec2) -> Vec2 {
        // Standard Julia set iteration: z² with chaos modifications
        let mut result = Vec2::new(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y);

        // Add chaos factor for reality distortion
        if self.chaos_factor > 0.1 {
            let chaos_mod = Vec2::new(
                (z.x * 3.0 + self.time_offset).sin() * self.chaos_factor * 0.1,
                (z.y * 3.0 + self.time_offset * 0.7).cos() * self.chaos_factor * 0.1,
            );
            result += chaos_mod;
        }

        result
    }

    fn mandelbrot_iteration(&self, z: Vec2) -> Vec2 {
        // Standard Mandelbrot iteration with time-based modifications
        let mut result = Vec2::new(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y);

        // Time-based parameter drift
        let time_drift = Vec2::new(
            (self.time_offset * 0.1).sin() * 0.01,
            (self.time_offset * 0.07).cos() * 0.01,
        );
        result += time_drift;

        result
    }

    pub fn get_fractal_color(&self, fractal_value: f32, position: Vec2) -> glam::Vec3 {
        // Convert fractal iteration count to psychedelic colors
        let hue = (fractal_value * 360.0 + self.time_offset * 50.0) % 360.0;
        let saturation = 0.7 + fractal_value * 0.3;
        let brightness = 0.4 + fractal_value * 0.4;

        // Add spatial color variation
        let spatial_hue_shift = (position.x * 0.01 + position.y * 0.007).sin() * 30.0;

        crate::reality::hsv_to_rgb(hue + spatial_hue_shift, saturation, brightness)
    }

    pub fn sample_fractal_at_point(&self, point: Vec2, fractal_type: FractalType) -> f32 {
        match fractal_type {
            FractalType::Julia => {
                let mut z = point;
                let mut iterations = 0;
                let max_iterations = 30;

                for _ in 0..max_iterations {
                    if z.length_squared() > 4.0 {
                        break;
                    }
                    z = self.julia_iteration(z) + self.julia_c;
                    iterations += 1;
                }

                iterations as f32 / max_iterations as f32
            }
            FractalType::Mandelbrot => {
                let c = point;
                let mut z = Vec2::ZERO;
                let mut iterations = 0;
                let max_iterations = 50;

                for _ in 0..max_iterations {
                    if z.length_squared() > 4.0 {
                        break;
                    }
                    z = self.mandelbrot_iteration(z) + c;
                    iterations += 1;
                }

                iterations as f32 / max_iterations as f32
            }
            FractalType::Chaos => {
                let chaos = (point.x * 8.0 + self.time_offset).sin() *
                           (point.y * 6.0 + self.time_offset * 0.8).cos() *
                           self.chaos_factor;
                (chaos * 3.0).sin() * 0.5 + 0.5
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FractalType {
    Julia,
    Mandelbrot,
    Chaos,
}