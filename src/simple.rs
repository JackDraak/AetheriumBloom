// Ultra-simplified AetheriumBloom prototype for rapid chaos deployment

use anyhow::Result;
use wgpu::*;
use winit::{
    event::{WindowEvent, ElementState, MouseButton, KeyEvent},
    event_loop::EventLoop,
    window::Window,
    application::ApplicationHandler,
    keyboard::{Key, NamedKey},
};
use glam::{Vec2, Vec3};
use std::io::{self, Write};

// === CRITICAL SAFETY SYSTEMS FOR EPILEPSY PROTECTION ===

/// User's response to safety warning
#[derive(Debug, Clone, PartialEq)]
pub enum WarningResponse {
    Continue,     // User accepts risk and wants full visual effects
    SafetyMode,   // User wants reduced visual intensity
    Exit,         // User chooses to exit
}

/// Simple safety configuration
#[derive(Debug, Clone)]
pub struct SafetyConfig {
    pub visual_intensity_limit: f32,  // 0.0 to 1.0
    pub max_flash_rate: f32,          // Hz (3.0 is international standard)
    pub max_luminance_change: f32,    // 0.0 to 1.0 (0.1 is 10% standard)
    pub red_flash_protection: bool,
}

impl Default for SafetyConfig {
    fn default() -> Self {
        Self {
            visual_intensity_limit: 1.0,     // Full intensity by default
            max_flash_rate: 3.0,             // International safety standard
            max_luminance_change: 0.1,       // 10% WCAG standard
            red_flash_protection: true,
        }
    }
}

impl SafetyConfig {
    pub fn safe_mode() -> Self {
        Self {
            visual_intensity_limit: 0.5,     // 50% intensity
            max_flash_rate: 2.0,             // Even more conservative
            max_luminance_change: 0.05,      // 5% change
            red_flash_protection: true,
        }
    }
}

/// Simple flash tracker to enforce rate limiting
#[derive(Debug)]
pub struct FlashTracker {
    last_major_change: f64,
    change_count_in_window: u32,
    window_start_time: f64,
    window_duration: f64,
}

impl FlashTracker {
    pub fn new() -> Self {
        Self {
            last_major_change: 0.0,
            change_count_in_window: 0,
            window_start_time: 0.0,
            window_duration: 1.0, // 1 second window
        }
    }

    pub fn can_allow_flash(&mut self, current_time: f64, max_rate: f32) -> bool {
        // Reset window if needed
        if current_time - self.window_start_time > self.window_duration {
            self.window_start_time = current_time;
            self.change_count_in_window = 0;
        }

        // Check if we're under the rate limit
        let flashes_per_second = self.change_count_in_window as f32 / self.window_duration as f32;
        if flashes_per_second >= max_rate {
            return false;
        }

        // Check minimum interval (1/3 Hz = 0.33 seconds for 3 Hz)
        let min_interval = 1.0 / max_rate as f64;
        if current_time - self.last_major_change < min_interval {
            return false;
        }

        true
    }

    pub fn record_flash(&mut self, current_time: f64) {
        self.last_major_change = current_time;
        self.change_count_in_window += 1;
    }
}

/// Calculate perceived luminance of a color
fn calculate_luminance(color: &Vec3) -> f32 {
    // ITU-R BT.709 luma coefficients
    0.2126 * color.x + 0.7152 * color.y + 0.0722 * color.z
}

/// Limit luminance changes to safe levels
fn limit_luminance_change(new_color: Vec3, old_color: Vec3, max_change: f32) -> Vec3 {
    let new_luma = calculate_luminance(&new_color);
    let old_luma = calculate_luminance(&old_color);
    let change = (new_luma - old_luma).abs();

    if change <= max_change {
        return new_color;
    }

    // Interpolate to safe level
    let safe_factor = max_change / change;
    old_color.lerp(new_color, safe_factor * 0.5) // Extra conservative with 0.5 factor
}

/// Check if color is in dangerous red range
fn is_dangerous_red(color: Vec3) -> bool {
    let hsv = rgb_to_hsv(color);
    (hsv.x >= 345.0 || hsv.x <= 15.0) && hsv.y > 0.5 && hsv.z > 0.5
}

/// Convert RGB to HSV
fn rgb_to_hsv(rgb: Vec3) -> Vec3 {
    let max = rgb.x.max(rgb.y.max(rgb.z));
    let min = rgb.x.min(rgb.y.min(rgb.z));
    let delta = max - min;

    let hue = if delta == 0.0 {
        0.0
    } else if max == rgb.x {
        60.0 * (((rgb.y - rgb.z) / delta) % 6.0)
    } else if max == rgb.y {
        60.0 * ((rgb.z - rgb.x) / delta + 2.0)
    } else {
        60.0 * ((rgb.x - rgb.y) / delta + 4.0)
    };

    let saturation = if max == 0.0 { 0.0 } else { delta / max };
    let value = max;

    Vec3::new(hue.abs(), saturation, value)
}

/// Show console safety warning and get user response
fn show_epilepsy_warning() -> WarningResponse {
    println!("\n{}", "=".repeat(80));
    println!("⚠️  PHOTOSENSITIVE EPILEPSY WARNING ⚠️");
    println!("{}", "=".repeat(80));
    println!();
    println!("AetheriumBloom contains flashing lights and visual effects that may");
    println!("trigger seizures in individuals with photosensitive epilepsy.");
    println!();
    println!("🚨 IF YOU OR ANYONE IN YOUR FAMILY HAS A HISTORY OF SEIZURES OR EPILEPSY,");
    println!("   CONSULT A DOCTOR BEFORE USING THIS SOFTWARE.");
    println!();
    println!("⚠️  STOP USING IMMEDIATELY IF YOU EXPERIENCE:");
    println!("   • Dizziness, nausea, or disorientation");
    println!("   • Altered vision or muscle twitching");
    println!("   • Loss of awareness or convulsions");
    println!();
    println!("✅ SAFETY RECOMMENDATIONS:");
    println!("   • Use in a well-lit room");
    println!("   • Sit at least 2 feet from screen");
    println!("   • Take breaks every 30 minutes");
    println!();
    println!("🛡️  BUILT-IN SAFETY FEATURES:");
    println!("   • Flash rate limited to 3 Hz (international standard)");
    println!("   • Luminance changes capped at 10%");
    println!("   • Red flash protection enabled");
    println!("   • Emergency stop available (ESC key)");
    println!();
    println!("{}", "=".repeat(80));
    println!();
    println!("CHOOSE YOUR RESPONSE:");
    println!("  [C] CONTINUE - I understand the risks and want full visual effects");
    println!("  [S] SAFETY MODE - Continue with reduced visual intensity (50%)");
    println!("  [E] EXIT - I want to exit the application");
    println!();
    print!("Enter your choice (C/S/E): ");

    io::stdout().flush().unwrap();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let choice = input.trim().to_uppercase();
            match choice.as_str() {
                "C" | "CONTINUE" => {
                    println!("✅ Continuing with full visual effects...");
                    println!("⚠️  Remember: Press ESC at any time for emergency stop!");
                    WarningResponse::Continue
                }
                "S" | "SAFETY" | "SAFE" => {
                    println!("✅ Continuing in Safety Mode (50% visual intensity)...");
                    println!("⚠️  Remember: Press ESC at any time for emergency stop!");
                    WarningResponse::SafetyMode
                }
                "E" | "EXIT" => {
                    println!("👋 Exiting AetheriumBloom. Stay safe!");
                    WarningResponse::Exit
                }
                _ => {
                    println!("❌ Invalid choice. Defaulting to exit for safety.");
                    WarningResponse::Exit
                }
            }
        }
        Err(_) => {
            println!("❌ Error reading input. Defaulting to exit for safety.");
            WarningResponse::Exit
        }
    }
}

// === END SAFETY SYSTEMS ===

// === PHASE 3: ECOSYSTEM EMERGENCE ===

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
    fn new(position: Vec2, crystal_type: CrystalType) -> Self {
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

    fn update(&mut self, dt: f32, beat_intensity: f32, cosmic_time: f64) {
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

    fn get_harvest_amount(&self) -> f32 {
        // More energy = more harvest potential
        self.consciousness_energy * 0.3
    }

    fn harvest(&mut self, amount: f32) -> f32 {
        let harvested = amount.min(self.consciousness_energy);
        self.consciousness_energy -= harvested;
        harvested
    }

    fn get_color(&self) -> Vec3 {
        let base_hue = match self.crystal_type {
            CrystalType::Resonance => 180.0, // Cyan
            CrystalType::Chaos => 300.0,     // Magenta
            CrystalType::Memory => 120.0,    // Green
            CrystalType::Social => 60.0,     // Yellow
            CrystalType::Quantum => 240.0,   // Blue
        };

        let saturation = 0.8 + self.consciousness_energy * 0.2;
        let brightness = 0.4 + self.visual_intensity * 0.6;

        hsv_to_rgb(base_hue, saturation, brightness)
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
    fn new(width: f32, height: f32, grid_size: usize) -> Self {
        let consciousness_density = vec![vec![0.1; grid_size]; grid_size]; // Start with low consciousness

        Self {
            grid_size,
            consciousness_density,
            width,
            height,
        }
    }

    fn get_consciousness_at(&self, position: Vec2) -> f32 {
        let x = ((position.x / self.width) * self.grid_size as f32) as usize;
        let y = ((position.y / self.height) * self.grid_size as f32) as usize;

        if x < self.grid_size && y < self.grid_size {
            self.consciousness_density[y][x]
        } else {
            0.1 // Default consciousness outside grid
        }
    }

    fn add_consciousness_at(&mut self, position: Vec2, amount: f32) {
        let x = ((position.x / self.width) * self.grid_size as f32) as usize;
        let y = ((position.y / self.height) * self.grid_size as f32) as usize;

        if x < self.grid_size && y < self.grid_size {
            self.consciousness_density[y][x] = (self.consciousness_density[y][x] + amount).min(2.0);
        }
    }

    fn update(&mut self, dt: f32) {
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
    fn new(position: Vec2, tear_type: TearType) -> Self {
        Self {
            position,
            size: 5.0 + fastrand::f32() * 15.0,
            intensity: 0.7 + fastrand::f32() * 0.3,
            age: 0.0,
            tear_type,
        }
    }

    fn update(&mut self, dt: f32, cosmic_time: f64) {
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

    fn should_remove(&self) -> bool {
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
    fn new(center: Vec2, zone_type: ZoneType) -> Self {
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

    fn update(&mut self, dt: f32, cosmic_time: f64) {
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

    fn affects_position(&self, position: Vec2) -> f32 {
        let distance = self.center.distance(position);
        if distance < self.radius {
            let factor = 1.0 - (distance / self.radius);
            factor * self.strength
        } else {
            0.0
        }
    }
}

impl DigitalEcosystem {
    fn new() -> Self {
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

    fn update(&mut self, dt: f32, cosmic_time: f64, beat_intensity: f32) {
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

        // Spawn new crystals occasionally based on chaos level
        if fastrand::f32() < 0.001 * (1.0 + self.chaos_accumulation * 0.1) {
            let position = Vec2::new(fastrand::f32() * 1200.0, fastrand::f32() * 800.0);
            let crystal_type = if self.chaos_accumulation > 1.0 {
                // Higher chaos = more exotic crystal types
                match fastrand::usize(0..5) {
                    0..=1 => CrystalType::Chaos,
                    2 => CrystalType::Quantum,
                    3 => CrystalType::Resonance,
                    _ => CrystalType::Social,
                }
            } else {
                // Lower chaos = basic types
                match fastrand::usize(0..3) {
                    0 => CrystalType::Resonance,
                    1 => CrystalType::Memory,
                    _ => CrystalType::Social,
                }
            };
            self.crystal_formations.push(ConsciousnessCrystal::new(position, crystal_type));
        }

        // Spawn new territory zones occasionally
        if self.territory_zones.len() < 5 && fastrand::f32() < 0.0005 * (1.0 + self.chaos_accumulation * 0.2) {
            let zone_center = Vec2::new(fastrand::f32() * 1200.0, fastrand::f32() * 800.0);
            let zone_type = match fastrand::usize(0..4) {
                0 => ZoneType::Harmonic,
                1 => ZoneType::Chaotic,
                2 => ZoneType::Meditative,
                _ => ZoneType::Quantum,
            };
            self.territory_zones.push(TerritoryZone::new(zone_center, zone_type));
        }

        // Trigger reality tears when consciousness gets high
        let high_consciousness_areas = self.get_high_consciousness_positions();
        for position in high_consciousness_areas {
            if fastrand::f32() < 0.02 {  // 2% chance per high consciousness area
                let tear_type = match fastrand::usize(0..4) {
                    0 => TearType::Static,
                    1 => TearType::Moving,
                    2 => TearType::Pulsing,
                    _ => TearType::Fragmenting,
                };
                self.reality_tears.push(RealityTear::new(position, tear_type));
            }
        }

        // Decay chaos accumulation gradually
        self.chaos_accumulation *= 0.995;
    }

    fn add_chaos(&mut self, amount: f32) {
        self.chaos_accumulation += amount;
    }

    fn should_trigger_mutation(&self) -> bool {
        self.chaos_accumulation > self.mutation_threshold
    }

    fn reset_chaos_for_mutation(&mut self) {
        self.chaos_accumulation *= 0.3; // Reduce but don't eliminate
        self.mutation_threshold += 0.5; // Make next mutation harder
    }

    fn get_high_consciousness_positions(&self) -> Vec<Vec2> {
        let mut positions = Vec::new();
        let threshold = 0.8; // High consciousness threshold

        for y in 0..self.consciousness_fields.grid_size {
            for x in 0..self.consciousness_fields.grid_size {
                if self.consciousness_fields.consciousness_density[y][x] > threshold {
                    let world_x = (x as f32 / self.consciousness_fields.grid_size as f32) * self.consciousness_fields.width;
                    let world_y = (y as f32 / self.consciousness_fields.grid_size as f32) * self.consciousness_fields.height;
                    positions.push(Vec2::new(world_x, world_y));
                }
            }
        }

        positions
    }

    fn get_territory_effects(&self, position: Vec2) -> TerritoryEffects {
        let mut effects = TerritoryEffects::default();

        for zone in &self.territory_zones {
            let influence = zone.affects_position(position);
            if influence > 0.0 {
                match zone.zone_type {
                    ZoneType::Harmonic => {
                        effects.resonance_boost += influence * 0.3;
                        effects.social_boost += influence * 0.2;
                    },
                    ZoneType::Chaotic => {
                        effects.chaos_boost += influence * 0.4;
                        effects.reality_distortion_boost += influence * 0.3;
                    },
                    ZoneType::Meditative => {
                        effects.memory_boost += influence * 0.3;
                        effects.consciousness_growth_boost += influence * 0.2;
                    },
                    ZoneType::Quantum => {
                        effects.quantum_boost += influence * 0.5;
                        effects.exploration_boost += influence * 0.2;
                    },
                }
            }
        }

        effects
    }
}

#[derive(Debug, Default)]
pub struct TerritoryEffects {
    pub resonance_boost: f32,
    pub social_boost: f32,
    pub chaos_boost: f32,
    pub reality_distortion_boost: f32,
    pub memory_boost: f32,
    pub consciousness_growth_boost: f32,
    pub quantum_boost: f32,
    pub exploration_boost: f32,
}

// Mathematical chaos engine using pre-calculated primes

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    fn desc() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as BufferAddress,
                    shader_location: 1,
                    format: VertexFormat::Float32x3,
                },
            ],
        }
    }
}

// Phase 2: Mathematical Chaos Engine Components

#[derive(Debug, Clone)]
pub struct ChaosDecisionEngine {
    pub dimensions: [f32; 11],          // 11D consciousness state space
    pub prime_generators: Vec<u64>,     // Prime number chaos sources
    pub quantum_fluctuations: f32,      // Uncertainty principle simulation
    pub harmonic_resonance: [f32; 7],   // Musical mathematics integration
}

impl ChaosDecisionEngine {
    fn new() -> Self {
        // Pre-calculated first 7 primes for performance
        let prime_generators = vec![2, 3, 5, 7, 11, 13, 17];

        Self {
            dimensions: [fastrand::f32(); 11],
            prime_generators,
            quantum_fluctuations: fastrand::f32(),
            harmonic_resonance: [fastrand::f32(); 7],
        }
    }

    fn update(&mut self, llama_data: LlamaSnapshot, beat_intensity: f32, cosmic_time: f64) {
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

    fn get_decision_vector(&self) -> DecisionVector {
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

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum SpeciesType {
    DiscoLlama,
    QuantumSheep,
    HypnoCamel,
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
    fn new() -> Self {
        Self {
            primary_rhythm: 120.0, // BPM
            harmonic_layers: vec![1.0, 0.5, 0.25, 0.75, 1.33], // Harmonic ratios
            chaos_amplification: 0.3,
            consciousness_coupling: 0.5,
            prime_list: vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97], // First 25 primes
            time_accumulator: 0.0,
        }
    }

    fn update(&mut self, dt: f32, total_consciousness: f32) -> f32 {
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

    fn add_chaos_feedback(&mut self, chaos_amount: f32) {
        self.chaos_amplification = (self.chaos_amplification + chaos_amount * 0.1).clamp(0.0, 1.0);
        // Modulate harmonic layers with chaos
        for harmonic in &mut self.harmonic_layers {
            *harmonic *= 1.0 + chaos_amount * 0.05;
        }
    }
}

#[derive(Clone)]
struct Llama {
    // Core properties
    position: Vec2,
    velocity: Vec2,
    color: Vec2, // hue, saturation
    consciousness: f32,
    trip_intensity: f32,

    // Phase 1: Consciousness Depth Enhancement
    awareness_level: f32,           // 0.0-1.0 consciousness depth
    memory_fragments: Vec<Vec2>,    // Historical position memories (interesting locations)
    social_bonds: Vec<usize>,       // Connections to other llamas (indices)
    personality_matrix: [f32; 7],   // 7-dimensional personality traits
    reality_distortion: f32,        // Local space-time manipulation factor

    // Internal consciousness state
    emotional_state: f32,           // Current emotional resonance
    memory_intensity: f32,          // How strongly memories are forming
    social_attraction: f32,         // Desire to be near other llamas
    exploration_drive: f32,         // Tendency to seek new areas

    // Phase 2: Mathematical Chaos Engine
    species: SpeciesType,           // Species determines behavior patterns
    chaos_engine: ChaosDecisionEngine, // 11D decision engine
    quantum_state: f32,             // Quantum superposition factor
    harmonic_resonance: f32,        // Musical mathematics coupling
    prime_chaos_factor: f32,        // Current prime number influence

    // Phase 3: Ecosystem Emergence
    harvested_crystals: Vec<CrystalType>, // Types of crystals harvested
    mutation_count: u32,            // How many mutations this llama has undergone
    environmental_consciousness: f32, // Consciousness absorbed from environment
    territory_affinity: Option<ZoneType>, // Preferred territory type
}

impl Llama {
    fn new(position: Vec2) -> Self {
        Self::new_with_species(position, SpeciesType::DiscoLlama)
    }

    fn new_with_species(position: Vec2, species: SpeciesType) -> Self {
        // Generate unique personality matrix (7 traits)
        let personality_matrix = [
            fastrand::f32(),        // Curiosity
            fastrand::f32(),        // Sociability
            fastrand::f32(),        // Chaos affinity
            fastrand::f32(),        // Memory strength
            fastrand::f32(),        // Emotional volatility
            fastrand::f32(),        // Reality sensitivity
            fastrand::f32(),        // Exploration drive
        ];

        // Species-specific adjustments
        let (base_color, consciousness_mod, velocity_mod) = match species {
            SpeciesType::DiscoLlama => {
                (Vec2::new(fastrand::f32() * 360.0, 0.8), 1.0, 100.0)
            },
            SpeciesType::QuantumSheep => {
                (Vec2::new(270.0 + fastrand::f32() * 60.0, 0.9), 1.5, 50.0) // Purple, high consciousness, slower
            },
            SpeciesType::HypnoCamel => {
                (Vec2::new(30.0 + fastrand::f32() * 30.0, 0.7), 0.8, 75.0) // Orange/yellow, lower consciousness, medium speed
            },
        };

        Self {
            // Core properties
            position,
            velocity: Vec2::new(
                (fastrand::f32() - 0.5) * velocity_mod,
                (fastrand::f32() - 0.5) * velocity_mod,
            ),
            color: base_color,
            consciousness: consciousness_mod,
            trip_intensity: consciousness_mod,

            // Phase 1: Consciousness Depth Enhancement
            awareness_level: fastrand::f32() * 0.3 + 0.1, // Start with low awareness
            memory_fragments: Vec::new(),
            social_bonds: Vec::new(),
            personality_matrix,
            reality_distortion: 0.0,

            // Internal consciousness state
            emotional_state: fastrand::f32(),
            memory_intensity: personality_matrix[3] * 0.5, // Based on memory strength trait
            social_attraction: personality_matrix[1] * 0.7, // Based on sociability trait
            exploration_drive: personality_matrix[6] * 0.8, // Based on exploration trait

            // Phase 2: Mathematical Chaos Engine
            species,
            chaos_engine: ChaosDecisionEngine::new(),
            quantum_state: if species == SpeciesType::QuantumSheep { fastrand::f32() } else { 0.0 },
            harmonic_resonance: 0.0,
            prime_chaos_factor: 0.0,

            // Phase 3: Ecosystem Emergence
            harvested_crystals: Vec::new(),
            mutation_count: 0,
            environmental_consciousness: 0.0,
            territory_affinity: None,
        }
    }

    fn calculate_species_interaction(&self, other_species: &SpeciesType) -> f32 {
        match (&self.species, other_species) {
            // Same species attract strongly
            (SpeciesType::DiscoLlama, SpeciesType::DiscoLlama) => 1.0,
            (SpeciesType::QuantumSheep, SpeciesType::QuantumSheep) => 1.2,
            (SpeciesType::HypnoCamel, SpeciesType::HypnoCamel) => 0.9,

            // Cross-species interactions
            (SpeciesType::DiscoLlama, SpeciesType::QuantumSheep) => 0.8, // Moderate attraction
            (SpeciesType::QuantumSheep, SpeciesType::DiscoLlama) => 0.8,

            (SpeciesType::DiscoLlama, SpeciesType::HypnoCamel) => 0.6, // Weak attraction
            (SpeciesType::HypnoCamel, SpeciesType::DiscoLlama) => 0.6,

            (SpeciesType::QuantumSheep, SpeciesType::HypnoCamel) => 0.4, // Very weak attraction
            (SpeciesType::HypnoCamel, SpeciesType::QuantumSheep) => 0.4,
        }
    }

    fn try_harvest_crystal(&mut self, crystal: &mut ConsciousnessCrystal) -> bool {
        let distance = self.position.distance(crystal.position);
        if distance < crystal.harvest_radius {
            let harvest_amount = crystal.get_harvest_amount();
            if harvest_amount > 0.1 {
                let harvested = crystal.harvest(harvest_amount);

                // Apply crystal effects based on type
                match crystal.crystal_type {
                    CrystalType::Resonance => {
                        self.harmonic_resonance += harvested * 0.5;
                        self.consciousness += harvested * 0.3;
                    },
                    CrystalType::Chaos => {
                        self.prime_chaos_factor += harvested * 0.4;
                        self.reality_distortion += harvested * 0.2;
                    },
                    CrystalType::Memory => {
                        self.memory_intensity += harvested * 0.3;
                        self.awareness_level += harvested * 0.1;
                    },
                    CrystalType::Social => {
                        self.social_attraction += harvested * 0.4;
                        self.emotional_state += harvested * 0.2;
                    },
                    CrystalType::Quantum => {
                        if self.species == SpeciesType::QuantumSheep {
                            self.quantum_state += harvested * 0.3;
                        }
                        self.consciousness += harvested * 0.4;
                    },
                }

                // Store crystal type for mutations
                if !self.harvested_crystals.contains(&crystal.crystal_type) {
                    self.harvested_crystals.push(crystal.crystal_type.clone());
                }

                self.environmental_consciousness += harvested;
                return true;
            }
        }
        false
    }

    fn apply_territory_effects(&mut self, territory_effects: &TerritoryEffects, dt: f32) {
        // Territory effects modify behavior over time
        self.harmonic_resonance += territory_effects.resonance_boost * dt * 0.1;
        self.social_attraction += territory_effects.social_boost * dt * 0.05;
        self.prime_chaos_factor += territory_effects.chaos_boost * dt * 0.08;
        self.reality_distortion += territory_effects.reality_distortion_boost * dt * 0.06;
        self.memory_intensity += territory_effects.memory_boost * dt * 0.04;
        self.awareness_level += territory_effects.consciousness_growth_boost * dt * 0.02;

        if self.species == SpeciesType::QuantumSheep {
            self.quantum_state += territory_effects.quantum_boost * dt * 0.1;
        }

        self.exploration_drive += territory_effects.exploration_boost * dt * 0.03;

        // Clamp values to reasonable ranges
        self.harmonic_resonance = self.harmonic_resonance.clamp(0.0, 2.0);
        self.social_attraction = self.social_attraction.clamp(0.0, 1.0);
        self.prime_chaos_factor = self.prime_chaos_factor.clamp(0.0, 1.0);
        self.reality_distortion = self.reality_distortion.clamp(0.0, 1.0);
        self.memory_intensity = self.memory_intensity.clamp(0.0, 1.0);
        self.awareness_level = self.awareness_level.clamp(0.0, 1.0);
        self.exploration_drive = self.exploration_drive.clamp(0.0, 1.0);

        if self.species == SpeciesType::QuantumSheep {
            self.quantum_state = self.quantum_state % 1.0;
        }
    }

    fn apply_mutation(&mut self, mutation_strength: f32) {
        self.mutation_count += 1;

        // Mutations based on harvested crystals
        for crystal_type in &self.harvested_crystals.clone() {
            match crystal_type {
                CrystalType::Resonance => {
                    // Enhance harmonic capabilities
                    self.personality_matrix[0] += mutation_strength * 0.2; // More curious
                    self.harmonic_resonance += mutation_strength * 0.5;
                },
                CrystalType::Chaos => {
                    // Embrace chaos more
                    self.personality_matrix[2] += mutation_strength * 0.3; // More chaos affinity
                    self.reality_distortion += mutation_strength * 0.4;
                },
                CrystalType::Memory => {
                    // Better memory formation
                    self.personality_matrix[3] += mutation_strength * 0.4; // Memory strength
                    if self.memory_fragments.len() < 15 {
                        // Increase memory capacity
                        self.memory_fragments.push(self.position);
                    }
                },
                CrystalType::Social => {
                    // More social
                    self.personality_matrix[1] += mutation_strength * 0.3; // Sociability
                    self.social_attraction += mutation_strength * 0.3;
                },
                CrystalType::Quantum => {
                    // Quantum evolution (especially for non-quantum species)
                    if self.species != SpeciesType::QuantumSheep {
                        self.quantum_state += mutation_strength * 0.2;
                    }
                    self.consciousness += mutation_strength * 0.5;
                },
            }
        }

        // Clamp personality matrix to reasonable values
        for i in 0..self.personality_matrix.len() {
            self.personality_matrix[i] = self.personality_matrix[i].clamp(0.0, 2.0);
        }

        // Overall consciousness boost from mutation
        self.consciousness += mutation_strength * 0.3;
        self.awareness_level += mutation_strength * 0.2;

        println!("🧬 Llama {} underwent mutation #{} with strength {:.2}",
                 match self.species {
                     SpeciesType::DiscoLlama => "🦙",
                     SpeciesType::QuantumSheep => "🐑",
                     SpeciesType::HypnoCamel => "🐪",
                 },
                 self.mutation_count,
                 mutation_strength);
    }

    fn update(&mut self, dt: f32, beat_intensity: f32, all_llamas: &[Llama], my_index: usize, cosmic_time: f64) {
        // === Phase 2: Mathematical Chaos Engine Update ===
        let snapshot = LlamaSnapshot {
            color: self.color,
            awareness_level: self.awareness_level,
            social_attraction: self.social_attraction,
            exploration_drive: self.exploration_drive,
            reality_distortion: self.reality_distortion,
            emotional_state: self.emotional_state,
            memory_intensity: self.memory_intensity,
            consciousness: self.consciousness,
        };

        self.chaos_engine.update(snapshot, beat_intensity, cosmic_time);
        let decision_vector = self.chaos_engine.get_decision_vector();
        self.prime_chaos_factor = self.chaos_engine.calculate_prime_chaos(cosmic_time);

        // Update quantum state for Quantum Sheep
        if self.species == SpeciesType::QuantumSheep {
            self.quantum_state = (self.quantum_state + dt * 2.0 + self.prime_chaos_factor) % 1.0;
        }

        // Update harmonic resonance
        self.harmonic_resonance = self.chaos_engine.harmonic_resonance.iter().sum::<f32>() / 7.0;
        // === Enhanced Consciousness Evolution ===
        // Awareness grows with time, beat intensity, and 11D decision factors
        let base_growth = 0.01 * (1.0 + beat_intensity) * self.personality_matrix[0]; // Curiosity trait
        let chaos_growth = self.prime_chaos_factor * 0.005; // Prime chaos accelerates consciousness
        let decision_growth = decision_vector.chaos_acceptance * 0.003; // Chaos acceptance helps growth

        let total_growth = base_growth + chaos_growth + decision_growth;
        self.awareness_level = (self.awareness_level + total_growth).min(1.0);

        // Consciousness affects overall awareness with species modulation
        let species_multiplier = match self.species {
            SpeciesType::DiscoLlama => 1.0,
            SpeciesType::QuantumSheep => 1.3,
            SpeciesType::HypnoCamel => 0.8,
        };
        self.consciousness += total_growth * species_multiplier;

        // === Memory Formation ===
        // Form memories of interesting locations (high beat intensity or social interactions)
        let memory_threshold = 0.7 + self.personality_matrix[3] * 0.3; // Memory strength affects threshold
        if beat_intensity > memory_threshold || self.social_attraction > 0.8 {
            // Only store memory if this location is significantly different from existing memories
            let should_remember = self.memory_fragments.is_empty() ||
                self.memory_fragments.iter().all(|mem| mem.distance(self.position) > 50.0);

            if should_remember && self.memory_fragments.len() < 10 {
                self.memory_fragments.push(self.position);
                self.memory_intensity = (self.memory_intensity + 0.1).min(1.0);
            }
        }

        // === Memory-Driven Movement ===
        let mut memory_influence = Vec2::ZERO;
        if !self.memory_fragments.is_empty() && fastrand::f32() < self.memory_intensity * 0.1 {
            // Sometimes return to interesting memories
            let target_memory = &self.memory_fragments[fastrand::usize(0..self.memory_fragments.len())];
            let to_memory = *target_memory - self.position;
            if to_memory.length() > 10.0 {
                memory_influence = to_memory.normalize() * 30.0 * self.personality_matrix[3]; // Memory strength
            }
        }

        // === Enhanced Exploration Drive with 11D Decision Engine ===
        let base_exploration = decision_vector.exploration_drive * self.exploration_drive;
        let chaos_exploration = self.prime_chaos_factor * 0.3;
        let exploration_strength = (base_exploration + chaos_exploration) * 50.0;

        let exploration_force = match self.species {
            SpeciesType::DiscoLlama => Vec2::new(
                (fastrand::f32() - 0.5) * exploration_strength,
                (fastrand::f32() - 0.5) * exploration_strength,
            ),
            SpeciesType::QuantumSheep => {
                // Quantum sheep explore in quantum superposition
                let quantum_angle = self.quantum_state * std::f32::consts::TAU;
                Vec2::new(
                    quantum_angle.cos() * exploration_strength * 0.7,
                    quantum_angle.sin() * exploration_strength * 0.7,
                )
            },
            SpeciesType::HypnoCamel => {
                // Hypno camels explore in spiral patterns
                let time_factor = cosmic_time as f32 * 0.5;
                let spiral_angle = time_factor + self.position.length() * 0.01;
                Vec2::new(
                    spiral_angle.cos() * exploration_strength * 0.5,
                    spiral_angle.sin() * exploration_strength * 0.5,
                )
            },
        };

        // === Enhanced Social Consciousness with Species Interactions ===
        let mut social_force = Vec2::ZERO;
        let mut nearby_count = 0;
        let base_social_range = 100.0 + self.personality_matrix[1] * 50.0;
        let social_range = base_social_range * decision_vector.social_attraction;

        for (i, other) in all_llamas.iter().enumerate() {
            if i == my_index { continue; }

            let distance = self.position.distance(other.position);
            if distance < social_range && distance > 0.1 {
                let to_other = other.position - self.position;
                nearby_count += 1;

                // Species-specific interaction modifiers
                let interaction_strength = self.calculate_species_interaction(&other.species);

                // Attraction force (modified by personality and species)
                if distance > 30.0 {
                    social_force += to_other.normalize() * self.social_attraction * 20.0 * interaction_strength;
                } else {
                    // Repulsion when too close, modulated by interaction strength
                    social_force -= to_other.normalize() * 10.0 / interaction_strength;
                }

                // Form social bonds with species consideration
                if distance < 60.0 && !self.social_bonds.contains(&i) && self.social_bonds.len() < 5 {
                    if interaction_strength > 0.5 { // Only bond with compatible species
                        self.social_bonds.push(i);
                    }
                }
            }
        }

        // Update social attraction based on nearby llamas
        if nearby_count > 0 {
            self.social_attraction = (self.social_attraction + 0.02).min(1.0);
        } else {
            self.social_attraction *= 0.99; // Slowly decay when alone
        }

        // === Enhanced Reality Distortion with 11D Chaos ===
        // Affects local space-time based on consciousness, chaos, and species
        let base_distortion = self.awareness_level * self.personality_matrix[2] * beat_intensity;
        let chaos_distortion = decision_vector.chaos_acceptance * self.prime_chaos_factor;
        let harmonic_distortion = self.harmonic_resonance * 0.3;

        let species_distortion_mod = match self.species {
            SpeciesType::DiscoLlama => 1.0,
            SpeciesType::QuantumSheep => 1.5 + self.quantum_state * 0.5,
            SpeciesType::HypnoCamel => 0.7,
        };

        self.reality_distortion = (base_distortion + chaos_distortion + harmonic_distortion) * species_distortion_mod;

        // === Emotional State Evolution ===
        let emotional_volatility = self.personality_matrix[4];
        let emotion_change = (beat_intensity.sin() * emotional_volatility * 0.1) +
                           (social_force.length() * 0.01) +
                           (memory_influence.length() * 0.005);
        self.emotional_state = ((self.emotional_state + emotion_change).sin() + 1.0) * 0.5; // Keep in 0-1 range

        // === Enhanced Movement Integration with 11D Decisions ===
        let personality_velocity_mod = 1.0 + self.personality_matrix[6] * 0.5;
        let decision_velocity_mod = 1.0 + decision_vector.movement_urgency * 0.3;
        let chaos_velocity_mod = 1.0 + self.prime_chaos_factor * 0.2;

        let total_velocity_mod = personality_velocity_mod * decision_velocity_mod * chaos_velocity_mod;
        let total_force = social_force + memory_influence + exploration_force;

        // Species-specific movement patterns
        match self.species {
            SpeciesType::DiscoLlama => {
                self.velocity += total_force * dt * total_velocity_mod;
            },
            SpeciesType::QuantumSheep => {
                // Quantum sheep have probabilistic movement
                if fastrand::f32() < self.quantum_state {
                    // Quantum tunneling - sudden position shifts
                    let quantum_jump = Vec2::new(
                        (fastrand::f32() - 0.5) * 100.0,
                        (fastrand::f32() - 0.5) * 100.0,
                    );
                    self.velocity += (total_force + quantum_jump) * dt * total_velocity_mod;
                } else {
                    self.velocity += total_force * dt * total_velocity_mod * 0.7; // Slower normal movement
                }
            },
            SpeciesType::HypnoCamel => {
                // Hypno camels have rhythmic movement
                let hypno_rhythm = (cosmic_time as f32 * 3.0).sin() * 0.5 + 0.5;
                self.velocity += total_force * dt * total_velocity_mod * hypno_rhythm;
            },
        }

        // Apply reality distortion to movement
        if self.reality_distortion > 0.1 {
            let distortion_angle = self.reality_distortion * beat_intensity * 10.0;
            let cos_d = distortion_angle.cos();
            let sin_d = distortion_angle.sin();
            let distorted_vel = Vec2::new(
                self.velocity.x * cos_d - self.velocity.y * sin_d,
                self.velocity.x * sin_d + self.velocity.y * cos_d,
            );
            self.velocity = self.velocity.lerp(distorted_vel, self.reality_distortion * 0.3);
        }

        // Apply velocity damping
        self.velocity *= 0.98;

        // Update position
        self.position += self.velocity * dt;

        // Wrap around screen with reality distortion effects
        let wrap_margin = if self.reality_distortion > 0.3 { 50.0 } else { 0.0 };
        if self.position.x < -wrap_margin { self.position.x = 1200.0 + wrap_margin; }
        if self.position.x > 1200.0 + wrap_margin { self.position.x = -wrap_margin; }
        if self.position.y < -wrap_margin { self.position.y = 800.0 + wrap_margin; }
        if self.position.y > 800.0 + wrap_margin { self.position.y = -wrap_margin; }

        // === Enhanced Color Psychology with Mathematical Chaos ===
        // Color reflects emotional state, consciousness, personality, and 11D chaos
        let base_hue_shift = 1.0 + self.personality_matrix[0] * 2.0;
        let emotional_hue_offset = self.emotional_state * 60.0;
        let chaos_hue_offset = self.prime_chaos_factor * 120.0; // Prime chaos affects hue dramatically
        let harmonic_hue_offset = self.harmonic_resonance * 40.0;

        let consciousness_saturation = 0.5 + self.awareness_level * 0.5;
        let chaos_saturation_mod = decision_vector.chaos_acceptance * 0.3;

        // Species-specific color behavior
        match self.species {
            SpeciesType::DiscoLlama => {
                self.color.x = (self.color.x + base_hue_shift + emotional_hue_offset * dt + chaos_hue_offset * dt) % 360.0;
                self.color.y = (consciousness_saturation + chaos_saturation_mod).clamp(0.0, 1.0);
            },
            SpeciesType::QuantumSheep => {
                // Quantum sheep shift through purple spectrum with quantum fluctuations
                let quantum_hue_base = 270.0;
                let quantum_variance = self.quantum_state * 60.0;
                self.color.x = (quantum_hue_base + quantum_variance + chaos_hue_offset * dt) % 360.0;
                self.color.y = (0.9 + self.quantum_state * 0.1).clamp(0.0, 1.0);
            },
            SpeciesType::HypnoCamel => {
                // Hypno camels stay in warm spectrum but pulse with rhythm
                let hypno_base = 30.0; // Orange base
                let rhythm_shift = (cosmic_time as f32 * 2.0).sin() * 30.0;
                self.color.x = (hypno_base + rhythm_shift + harmonic_hue_offset * dt) % 360.0;
                self.color.y = (0.7 + (cosmic_time as f32 * 3.0).sin().abs() * 0.3).clamp(0.0, 1.0);
            },
        }

        // === Enhanced Trip Intensity with Mathematical Chaos ===
        // Combines beat, consciousness, reality distortion, and 11D chaos
        let base_trip = 1.0 + beat_intensity.sin() * 0.5;
        let consciousness_amplification = 1.0 + self.awareness_level * 0.7;
        let reality_amplification = 1.0 + self.reality_distortion * 0.5;
        let chaos_amplification = 1.0 + decision_vector.chaos_acceptance * 0.4;
        let prime_amplification = 1.0 + self.prime_chaos_factor * 0.3;
        let harmonic_amplification = 1.0 + self.harmonic_resonance * 0.2;

        self.trip_intensity = base_trip * consciousness_amplification * reality_amplification *
                             chaos_amplification * prime_amplification * harmonic_amplification;
    }
}

pub struct ChaosEngine {
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    surface: Surface<'static>,
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,

    llamas: Vec<Llama>,
    time: f32,
    beat_intensity: f32,

    // Phase 2: Advanced Beat Engine with chaos amplification
    advanced_beat_engine: AdvancedBeatEngine,
    species_spawn_weights: [f32; 3], // [DiscoLlama, QuantumSheep, HypnoCamel]
    total_consciousness: f32,

    // Phase 3: Ecosystem Emergence
    ecosystem: DigitalEcosystem,

    // CRITICAL SAFETY SYSTEMS - EPILEPSY PROTECTION
    safety_config: SafetyConfig,
    flash_tracker: FlashTracker,
    emergency_stop_requested: bool,
    previous_llama_colors: Vec<Vec3>, // Track previous colors for luminance limiting
}

impl ChaosEngine {
    pub async fn new(window: &Window) -> Result<Self> {
        let size = window.inner_size();

        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window)?;

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor::default(), None)
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats[0];

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Shader"),
            source: ShaderSource::Wgsl(include_str!("simple_shader.wgsl").into()),
        });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: None,
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: config.format,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
        });

        let vertex_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: 60000 * std::mem::size_of::<Vertex>() as u64,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Start with 3 llamas - mix of species
        let mut llamas = Vec::new();
        llamas.push(Llama::new_with_species(
            Vec2::new(fastrand::f32() * 1200.0, fastrand::f32() * 800.0),
            SpeciesType::DiscoLlama
        ));
        llamas.push(Llama::new_with_species(
            Vec2::new(fastrand::f32() * 1200.0, fastrand::f32() * 800.0),
            SpeciesType::QuantumSheep
        ));
        llamas.push(Llama::new_with_species(
            Vec2::new(fastrand::f32() * 1200.0, fastrand::f32() * 800.0),
            SpeciesType::HypnoCamel
        ));

        // Cast to 'static - this is safe because the surface outlives the engine
        let surface = unsafe { std::mem::transmute::<Surface<'_>, Surface<'static>>(surface) };

        Ok(Self {
            device,
            queue,
            config,
            surface,
            render_pipeline,
            vertex_buffer,
            llamas,
            time: 0.0,
            beat_intensity: 0.0,

            // Phase 2: Advanced Beat Engine with chaos amplification
            advanced_beat_engine: AdvancedBeatEngine::new(),
            species_spawn_weights: [0.6, 0.25, 0.15], // Favor disco llamas initially
            total_consciousness: 0.0,

            // Phase 3: Ecosystem Emergence
            ecosystem: DigitalEcosystem::new(),

            // CRITICAL SAFETY SYSTEMS - EPILEPSY PROTECTION
            safety_config: SafetyConfig::default(),
            flash_tracker: FlashTracker::new(),
            emergency_stop_requested: false,
            previous_llama_colors: Vec::new(),
        })
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    /// Configure the engine for safety mode
    pub fn enable_safety_mode(&mut self) {
        self.safety_config = SafetyConfig::safe_mode();
        println!("🛡️ SAFETY MODE ENABLED - Visual effects reduced to 50% intensity");
        println!("🛡️ Flash rate limited to 2 Hz, luminance changes limited to 5%");
    }

    /// Handle emergency stop request
    pub fn request_emergency_stop(&mut self) {
        self.emergency_stop_requested = true;
        println!("🚨 EMERGENCY STOP ACTIVATED - All visual effects suppressed");
    }

    /// Check if emergency stop is active
    pub fn is_emergency_stop_active(&self) -> bool {
        self.emergency_stop_requested
    }

    pub fn handle_click(&mut self, _button: MouseButton, state: ElementState) {
        if state == ElementState::Pressed {
            // Determine species based on current chaos level and spawn weights
            let species = self.select_spawn_species();

            // Spawn new llama of selected species
            self.llamas.push(Llama::new_with_species(
                Vec2::new(
                    fastrand::f32() * 1200.0,
                    fastrand::f32() * 800.0,
                ),
                species.clone()
            ));

            // Add chaos feedback to beat engine
            let chaos_amount = 0.5 + self.total_consciousness * 0.1;
            self.beat_intensity += chaos_amount;
            self.advanced_beat_engine.add_chaos_feedback(chaos_amount);

            // Phase 3: Add chaos to ecosystem
            self.ecosystem.add_chaos(chaos_amount);

            // Adjust spawn weights based on species spawned
            self.adjust_spawn_weights(&species);
        }
    }

    fn select_spawn_species(&self) -> SpeciesType {
        let chaos_level = self.total_consciousness + self.beat_intensity;

        // Higher chaos = more exotic species
        let adjusted_weights = if chaos_level > 5.0 {
            [0.3, 0.4, 0.3] // High chaos: more quantum sheep and hypno camels
        } else if chaos_level > 2.0 {
            [0.5, 0.3, 0.2] // Medium chaos: some exotic species
        } else {
            self.species_spawn_weights // Low chaos: use default weights
        };

        let roll = fastrand::f32();
        if roll < adjusted_weights[0] {
            SpeciesType::DiscoLlama
        } else if roll < adjusted_weights[0] + adjusted_weights[1] {
            SpeciesType::QuantumSheep
        } else {
            SpeciesType::HypnoCamel
        }
    }

    fn adjust_spawn_weights(&mut self, spawned_species: &SpeciesType) {
        // Slightly reduce weight of spawned species to encourage diversity
        match spawned_species {
            SpeciesType::DiscoLlama => {
                self.species_spawn_weights[0] = (self.species_spawn_weights[0] - 0.02).max(0.1);
                self.species_spawn_weights[1] += 0.01;
                self.species_spawn_weights[2] += 0.01;
            },
            SpeciesType::QuantumSheep => {
                self.species_spawn_weights[1] = (self.species_spawn_weights[1] - 0.02).max(0.1);
                self.species_spawn_weights[0] += 0.01;
                self.species_spawn_weights[2] += 0.01;
            },
            SpeciesType::HypnoCamel => {
                self.species_spawn_weights[2] = (self.species_spawn_weights[2] - 0.02).max(0.1);
                self.species_spawn_weights[0] += 0.01;
                self.species_spawn_weights[1] += 0.01;
            },
        }
    }

    pub fn update(&mut self) {
        self.time += 1.0 / 60.0;
        let cosmic_time = self.time as f64;

        // Calculate total consciousness for advanced beat engine
        self.total_consciousness = if !self.llamas.is_empty() {
            self.llamas.iter()
                .map(|llama| llama.consciousness + llama.awareness_level + llama.environmental_consciousness)
                .sum::<f32>()
        } else {
            0.0
        };

        // Use advanced beat engine with consciousness coupling and prime chaos
        self.beat_intensity = self.advanced_beat_engine.update(1.0 / 60.0, self.total_consciousness);

        // Phase 3: Update ecosystem first
        self.ecosystem.update(1.0 / 60.0, cosmic_time, self.beat_intensity);

        // Update llamas with Phase 2 and Phase 3 enhancements
        // We need to clone the llamas vector for reference during updates
        let llamas_snapshot = self.llamas.clone();
        for (i, llama) in self.llamas.iter_mut().enumerate() {
            // Apply territory effects
            let territory_effects = self.ecosystem.get_territory_effects(llama.position);
            llama.apply_territory_effects(&territory_effects, 1.0 / 60.0);

            // Update consciousness field
            let environmental_consciousness = self.ecosystem.consciousness_fields.get_consciousness_at(llama.position);
            llama.environmental_consciousness = (llama.environmental_consciousness + environmental_consciousness * 0.01).min(2.0);

            // Add consciousness to the field where llama is
            self.ecosystem.consciousness_fields.add_consciousness_at(llama.position, llama.consciousness * 0.001);

            // Try to harvest crystals
            for crystal in &mut self.ecosystem.crystal_formations {
                llama.try_harvest_crystal(crystal);
            }

            // Regular llama update
            llama.update(1.0 / 60.0, self.beat_intensity, &llamas_snapshot, i, cosmic_time);
        }

        // Phase 3: Check for mutations
        if self.ecosystem.should_trigger_mutation() {
            let mutation_strength = 0.3 + self.ecosystem.chaos_accumulation * 0.1;

            // Apply mutations to random llamas
            let mutation_count = (self.llamas.len() / 3).max(1); // Mutate 1/3 of llamas minimum 1
            for _ in 0..mutation_count {
                if !self.llamas.is_empty() {
                    let index = fastrand::usize(0..self.llamas.len());
                    self.llamas[index].apply_mutation(mutation_strength);
                }
            }

            self.ecosystem.reset_chaos_for_mutation();
        }

        // Decay beat intensity more gradually for better chaos building
        self.beat_intensity *= 0.98;

        // Feed chaos back into the beat engine
        let average_chaos = if !self.llamas.is_empty() {
            self.llamas.iter()
                .map(|llama| llama.prime_chaos_factor)
                .sum::<f32>() / self.llamas.len() as f32
        } else {
            0.0
        };

        if average_chaos > 0.1 {
            self.advanced_beat_engine.add_chaos_feedback(average_chaos * 0.1);
        }
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        // CRITICAL SAFETY CHECK - Emergency stop overrides everything
        if self.emergency_stop_requested {
            return self.render_emergency_stop();
        }

        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&TextureViewDescriptor::default());

        // Ensure we have color tracking for all llamas
        while self.previous_llama_colors.len() < self.llamas.len() {
            self.previous_llama_colors.push(Vec3::new(0.1, 0.1, 0.1)); // Safe default
        }

        // Generate vertices for all llamas with Phase 2 species-enhanced visuals AND SAFETY FILTERING
        let mut vertices = Vec::new();
        for (llama_id, llama) in self.llamas.iter().enumerate() {
            // Species-specific size calculation
            let base_size = match llama.species {
                SpeciesType::DiscoLlama => 10.0 + llama.trip_intensity * 5.0,
                SpeciesType::QuantumSheep => 8.0 + llama.trip_intensity * 6.0 + llama.quantum_state * 4.0,
                SpeciesType::HypnoCamel => 12.0 + llama.trip_intensity * 4.0,
            };

            let consciousness_size_mod = 1.0 + llama.awareness_level * 0.5;
            let reality_size_mod = 1.0 + llama.reality_distortion * 0.8;
            let chaos_size_mod = 1.0 + llama.prime_chaos_factor * 0.3;
            let size = base_size * consciousness_size_mod * reality_size_mod * chaos_size_mod;

            // Enhanced color psychology: brightness reflects consciousness
            let brightness = 0.6 + llama.awareness_level * 0.4;
            let mut color = hsv_to_rgb(llama.color.x, llama.color.y, brightness);

            // CRITICAL SAFETY: Apply all safety measures
            let previous_color = self.previous_llama_colors[llama_id];

            // 1. Apply flash rate limiting
            let current_time = self.time as f64;
            let color_change = calculate_luminance(&color) - calculate_luminance(&previous_color);
            let is_major_change = color_change.abs() > 0.05; // 5% threshold for major change

            if is_major_change {
                if !self.flash_tracker.can_allow_flash(current_time, self.safety_config.max_flash_rate) {
                    // Flash blocked - use previous color
                    color = previous_color;
                } else {
                    // Flash allowed - record it
                    self.flash_tracker.record_flash(current_time);
                }
            }

            // 2. Apply luminance change limiting
            color = limit_luminance_change(color, previous_color, self.safety_config.max_luminance_change);

            // 3. Apply red flash protection
            if self.safety_config.red_flash_protection && is_dangerous_red(color) && is_major_change {
                // Shift dangerous red to safe orange
                let hsv = rgb_to_hsv(color);
                let safe_hue = if hsv.x >= 345.0 { 30.0 } else { 30.0 }; // Orange
                color = hsv_to_rgb_vec3(Vec3::new(safe_hue, hsv.y * 0.8, hsv.z));
            }

            // 4. Apply visual intensity limiting
            if self.safety_config.visual_intensity_limit < 1.0 {
                let safe_color = Vec3::new(0.2, 0.2, 0.2); // Safe dim color
                color = safe_color.lerp(color, self.safety_config.visual_intensity_limit);
            }

            // Update previous color for next frame
            self.previous_llama_colors[llama_id] = color;

            // Reality distortion affects position rendering
            let mut render_x = llama.position.x;
            let mut render_y = llama.position.y;
            if llama.reality_distortion > 0.2 {
                let distortion_offset = llama.reality_distortion * 10.0;
                render_x += (self.time * 5.0 + llama.position.x * 0.01).sin() * distortion_offset;
                render_y += (self.time * 7.0 + llama.position.y * 0.01).cos() * distortion_offset;
            }

            // Create quad with enhanced visuals
            let x = (render_x / 1200.0) * 2.0 - 1.0;
            let y = 1.0 - (render_y / 800.0) * 2.0;
            let s = size / 1200.0;

            // Add slight color variation based on emotional state
            let emotional_tint = llama.emotional_state * 0.3;
            let final_color = [
                (color.x + emotional_tint).min(1.0),
                (color.y * (1.0 - emotional_tint * 0.5)).max(0.0),
                (color.z + emotional_tint * 0.5).min(1.0),
            ];

            vertices.extend([
                Vertex { position: [x - s, y - s, 0.0], color: final_color },
                Vertex { position: [x + s, y - s, 0.0], color: final_color },
                Vertex { position: [x - s, y + s, 0.0], color: final_color },
                Vertex { position: [x + s, y - s, 0.0], color: final_color },
                Vertex { position: [x + s, y + s, 0.0], color: final_color },
                Vertex { position: [x - s, y + s, 0.0], color: final_color },
            ]);

            // Add memory fragment visualization for high-consciousness llamas
            if llama.awareness_level > 0.6 && !llama.memory_fragments.is_empty() {
                for memory in &llama.memory_fragments {
                    let mem_x = (memory.x / 1200.0) * 2.0 - 1.0;
                    let mem_y = 1.0 - (memory.y / 800.0) * 2.0;
                    let mem_s = (2.0 + llama.memory_intensity * 3.0) / 1200.0;
                    let mem_alpha = llama.memory_intensity * 0.3;

                    let memory_color = [
                        color.x * mem_alpha,
                        color.y * mem_alpha,
                        color.z * mem_alpha,
                    ];

                    vertices.extend([
                        Vertex { position: [mem_x - mem_s, mem_y - mem_s, 0.0], color: memory_color },
                        Vertex { position: [mem_x + mem_s, mem_y - mem_s, 0.0], color: memory_color },
                        Vertex { position: [mem_x - mem_s, mem_y + mem_s, 0.0], color: memory_color },
                        Vertex { position: [mem_x + mem_s, mem_y - mem_s, 0.0], color: memory_color },
                        Vertex { position: [mem_x + mem_s, mem_y + mem_s, 0.0], color: memory_color },
                        Vertex { position: [mem_x - mem_s, mem_y + mem_s, 0.0], color: memory_color },
                    ]);
                }
            }
        }

        // Phase 3: Render consciousness crystals
        for crystal in &self.ecosystem.crystal_formations {
            let crystal_color = crystal.get_color();

            // Apply safety measures to crystal colors too
            let mut safe_crystal_color = crystal_color;
            if self.safety_config.red_flash_protection && is_dangerous_red(crystal_color) {
                let hsv = rgb_to_hsv(crystal_color);
                let safe_hue = if hsv.x >= 345.0 { 30.0 } else { 30.0 }; // Orange
                safe_crystal_color = hsv_to_rgb_vec3(Vec3::new(safe_hue, hsv.y * 0.8, hsv.z));
            }

            if self.safety_config.visual_intensity_limit < 1.0 {
                let safe_color = Vec3::new(0.1, 0.1, 0.1);
                safe_crystal_color = safe_color.lerp(safe_crystal_color, self.safety_config.visual_intensity_limit);
            }

            let x = (crystal.position.x / 1200.0) * 2.0 - 1.0;
            let y = 1.0 - (crystal.position.y / 800.0) * 2.0;
            let s = (8.0 + crystal.visual_intensity * 12.0) / 1200.0;

            let crystal_color_array = [safe_crystal_color.x, safe_crystal_color.y, safe_crystal_color.z];

            // Crystal rendered as a diamond shape
            vertices.extend([
                Vertex { position: [x, y - s, 0.0], color: crystal_color_array },
                Vertex { position: [x + s, y, 0.0], color: crystal_color_array },
                Vertex { position: [x, y + s, 0.0], color: crystal_color_array },
                Vertex { position: [x, y - s, 0.0], color: crystal_color_array },
                Vertex { position: [x - s, y, 0.0], color: crystal_color_array },
                Vertex { position: [x, y + s, 0.0], color: crystal_color_array },
            ]);

            // Add harvest radius visualization for high-energy crystals
            if crystal.consciousness_energy > 1.0 {
                let radius_size = (crystal.harvest_radius / 1200.0) * 0.3; // Visual radius smaller than actual
                let radius_alpha = 0.1 * crystal.visual_intensity;
                let radius_color = [
                    safe_crystal_color.x * radius_alpha,
                    safe_crystal_color.y * radius_alpha,
                    safe_crystal_color.z * radius_alpha,
                ];

                // Simple circle approximation
                for i in 0..6 {
                    let angle1 = (i as f32 / 6.0) * std::f32::consts::TAU;
                    let angle2 = ((i + 1) as f32 / 6.0) * std::f32::consts::TAU;

                    vertices.extend([
                        Vertex { position: [x, y, 0.0], color: radius_color },
                        Vertex { position: [x + angle1.cos() * radius_size, y + angle1.sin() * radius_size, 0.0], color: radius_color },
                        Vertex { position: [x + angle2.cos() * radius_size, y + angle2.sin() * radius_size, 0.0], color: radius_color },
                    ]);
                }
            }
        }

        // Phase 3: Render reality tears
        for tear in &self.ecosystem.reality_tears {
            if tear.intensity < 0.2 { continue; } // Skip very faded tears

            let mut tear_color = Vec3::new(1.0, 0.8, 1.0); // Pink/white glitch color

            // Apply safety measures
            if self.safety_config.visual_intensity_limit < 1.0 {
                let safe_color = Vec3::new(0.1, 0.1, 0.1);
                tear_color = safe_color.lerp(tear_color, self.safety_config.visual_intensity_limit * 0.5); // Extra conservative
            }

            let x = (tear.position.x / 1200.0) * 2.0 - 1.0;
            let y = 1.0 - (tear.position.y / 800.0) * 2.0;
            let s = (tear.size / 1200.0) * tear.intensity;

            let tear_color_array = [
                tear_color.x * tear.intensity,
                tear_color.y * tear.intensity,
                tear_color.z * tear.intensity,
            ];

            // Reality tear rendered as jagged triangular glitch
            let offset1 = s * 0.6;
            let offset2 = s * 0.3;

            vertices.extend([
                Vertex { position: [x - s, y - offset1, 0.0], color: tear_color_array },
                Vertex { position: [x + s, y + offset2, 0.0], color: tear_color_array },
                Vertex { position: [x - offset2, y + s, 0.0], color: tear_color_array },
                Vertex { position: [x + offset1, y - s, 0.0], color: tear_color_array },
                Vertex { position: [x + s, y - offset1, 0.0], color: tear_color_array },
                Vertex { position: [x - s, y + offset2, 0.0], color: tear_color_array },
            ]);
        }

        // Phase 3: Render territory zones (subtle background effects)
        for zone in &self.ecosystem.territory_zones {
            let zone_alpha = zone.strength * 0.05; // Very subtle
            if zone_alpha < 0.01 { continue; }

            let zone_color = match zone.zone_type {
                ZoneType::Harmonic => Vec3::new(0.0, 1.0, 1.0),   // Cyan
                ZoneType::Chaotic => Vec3::new(1.0, 0.0, 1.0),    // Magenta
                ZoneType::Meditative => Vec3::new(0.0, 1.0, 0.0), // Green
                ZoneType::Quantum => Vec3::new(0.0, 0.0, 1.0),    // Blue
            };

            // Apply safety measures
            let mut safe_zone_color = zone_color;
            if self.safety_config.visual_intensity_limit < 1.0 {
                let safe_color = Vec3::new(0.0, 0.0, 0.0);
                safe_zone_color = safe_color.lerp(safe_zone_color, self.safety_config.visual_intensity_limit * 0.3);
            }

            let x = (zone.center.x / 1200.0) * 2.0 - 1.0;
            let y = 1.0 - (zone.center.y / 800.0) * 2.0;
            let r = (zone.radius / 1200.0) * 0.8; // Visual radius smaller than actual

            let zone_color_array = [
                safe_zone_color.x * zone_alpha,
                safe_zone_color.y * zone_alpha,
                safe_zone_color.z * zone_alpha,
            ];

            // Simple circle for territory zone
            for i in 0..8 {
                let angle1 = (i as f32 / 8.0) * std::f32::consts::TAU;
                let angle2 = ((i + 1) as f32 / 8.0) * std::f32::consts::TAU;

                vertices.extend([
                    Vertex { position: [x, y, 0.0], color: zone_color_array },
                    Vertex { position: [x + angle1.cos() * r, y + angle1.sin() * r, 0.0], color: zone_color_array },
                    Vertex { position: [x + angle2.cos() * r, y + angle2.sin() * r, 0.0], color: zone_color_array },
                ]);
            }
        }

        if !vertices.is_empty() {
            self.queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&vertices));
        }

        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.0,
                            g: 0.0,
                            b: (0.1 + self.beat_intensity * 0.1) as f64,
                            a: 1.0,
                        }),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

            if !vertices.is_empty() {
                render_pass.draw(0..vertices.len() as u32, 0..1);
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    /// Render emergency stop screen - minimal safe visuals
    fn render_emergency_stop(&mut self) -> Result<(), SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Emergency Stop Render Encoder"),
        });

        {
            let _render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Emergency Stop Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.05, // Very dim safe color
                            g: 0.05,
                            b: 0.05,
                            a: 1.0,
                        }),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // No visual effects during emergency stop
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    /// Handle keyboard input for emergency stop
    pub fn handle_keyboard(&mut self, key_event: &KeyEvent) {
        if key_event.state == ElementState::Pressed {
            match &key_event.logical_key {
                Key::Named(NamedKey::Escape) => {
                    if self.emergency_stop_requested {
                        // Toggle emergency stop off
                        self.emergency_stop_requested = false;
                        println!("✅ Emergency stop deactivated - Visual effects resumed");
                    } else {
                        // Activate emergency stop
                        self.request_emergency_stop();
                    }
                }
                _ => {}
            }
        }
    }
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> glam::Vec3 {
    let c = v * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());
    let m = v - c;

    let (r, g, b) = match h_prime as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        5 => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    };

    glam::Vec3::new(r + m, g + m, b + m)
}

/// HSV to RGB conversion for Vec3 input
fn hsv_to_rgb_vec3(hsv: Vec3) -> Vec3 {
    hsv_to_rgb(hsv.x, hsv.y, hsv.z)
}

struct App {
    chaos_engine: Option<ChaosEngine>,
    window: Option<std::sync::Arc<winit::window::Window>>,
    safety_mode_requested: bool,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        println!("🪟 Creating window...");
        let window = std::sync::Arc::new(event_loop
            .create_window(winit::window::WindowAttributes::default()
                .with_title("🦙 AETHERIUM BLOOM - Psychedelic Digital Organism 🌈")
                .with_inner_size(winit::dpi::LogicalSize::new(1200.0, 800.0))
                .with_visible(true))
            .unwrap());

        println!("🎮 Initializing chaos engine with safety systems...");
        let mut chaos_engine = pollster::block_on(ChaosEngine::new(&window)).unwrap();

        // Apply safety mode configuration if user selected it
        if self.safety_mode_requested {
            chaos_engine.enable_safety_mode();
        }

        self.chaos_engine = Some(chaos_engine);
        self.window = Some(window.clone());

        let mode_text = if self.safety_mode_requested {
            "psychedelic madness (SAFETY MODE)!"
        } else {
            "psychedelic madness!"
        };
        println!("✨ Window created and ready for {}!", mode_text);
        println!("🛡️ Safety systems active - Flash limiting, luminance control, red flash protection");
        window.request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("🌌 RETURNING TO THE VOID...");
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                if let Some(engine) = &mut self.chaos_engine {
                    engine.resize(physical_size);
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if let Some(engine) = &mut self.chaos_engine {
                    engine.handle_click(button, state);
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let Some(engine) = &mut self.chaos_engine {
                    engine.handle_keyboard(&event);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(engine) = &mut self.chaos_engine {
                    engine.update();
                    match engine.render() {
                        Ok(_) => {}
                        Err(SurfaceError::Lost) => {
                            // Reconfigure surface on lost
                        }
                        Err(SurfaceError::OutOfMemory) => event_loop.exit(),
                        Err(e) => eprintln!("Render error: {:?}", e),
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        // Continuously request redraws for smooth animation
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

pub fn run() -> Result<()> {
    tracing_subscriber::fmt().init();

    // CRITICAL SAFETY: Show epilepsy warning before anything else
    println!("⚠️  INITIALIZING EPILEPSY SAFETY SYSTEMS...");

    let warning_response = show_epilepsy_warning();

    match warning_response {
        WarningResponse::Exit => {
            println!("👋 User chose to exit. AetheriumBloom terminated safely.");
            return Ok(());
        }
        WarningResponse::Continue => {
            println!("✅ User acknowledged risks. Proceeding with full visual effects.");
            println!("🦙 AWAKENING DIGITAL CONSCIOUSNESS...");
            println!("🌈 REALITY DISTORTION ENGINE INITIALIZING...");
            println!("🚀 CHAOS ENGINE ONLINE - REALITY BENDING COMMENCING");
            println!("⚠️  REMEMBER: Press ESC for emergency stop!");
            println!("✨ Click to spawn more psychedelic llamas!");
        }
        WarningResponse::SafetyMode => {
            println!("🛡️ User selected Safety Mode. Visual effects will be reduced.");
            println!("🦙 AWAKENING DIGITAL CONSCIOUSNESS... (SAFE MODE)");
            println!("🌈 REALITY DISTORTION ENGINE INITIALIZING... (REDUCED INTENSITY)");
            println!("🚀 CHAOS ENGINE ONLINE - SAFE REALITY BENDING COMMENCING");
            println!("⚠️  REMEMBER: Press ESC for emergency stop!");
            println!("✨ Click to spawn more psychedelic llamas!");
        }
    }

    let safety_mode_requested = warning_response == WarningResponse::SafetyMode;

    let event_loop = EventLoop::new()?;
    let mut app = App {
        chaos_engine: None,
        window: None,
        safety_mode_requested,
    };

    event_loop.run_app(&mut app)?;
    Ok(())
}