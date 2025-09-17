// Photosensitive Epilepsy Warning Screen
// Implements mandatory safety warning before AetheriumBloom starts
// Critical safety requirement from EPILEPSY.md Phase 1

use anyhow::Result;
use winit::{
    event::{Event, WindowEvent, ElementState, MouseButton, KeyEvent},
    event_loop::EventLoop,
    window::{Window, WindowAttributes},
    keyboard::{Key, NamedKey},
    dpi::LogicalSize,
};

/// User's response to the safety warning
#[derive(Debug, Clone, PartialEq)]
pub enum WarningResponse {
    Continue,     // User accepts risk and continues
    SafetyMode,   // User wants reduced visual intensity
    Exit,         // User chooses to exit
    Pending,      // No response yet
}

/// Safety warning configuration
#[derive(Debug, Clone)]
pub struct WarningConfig {
    pub auto_timeout_seconds: Option<u32>,
    pub require_explicit_consent: bool,
    pub show_detailed_info: bool,
}

impl Default for WarningConfig {
    fn default() -> Self {
        Self {
            auto_timeout_seconds: None, // No auto-timeout for safety
            require_explicit_consent: true,
            show_detailed_info: true,
        }
    }
}

/// Main warning screen handler
pub struct EpilepsyWarningScreen {
    window: Option<Window>,
    response: WarningResponse,
    config: WarningConfig,
    start_time: std::time::Instant,
    warning_acknowledged: bool,
}

impl EpilepsyWarningScreen {
    pub fn new(config: WarningConfig) -> Self {
        Self {
            window: None,
            response: WarningResponse::Pending,
            config,
            start_time: std::time::Instant::now(),
            warning_acknowledged: false,
        }
    }

    /// Show the warning screen and wait for user response
    pub fn show_warning() -> Result<WarningResponse> {
        let config = WarningConfig::default();
        let mut warning_screen = Self::new(config);
        warning_screen.run()
    }

    /// Show warning with custom configuration
    pub fn show_warning_with_config(config: WarningConfig) -> Result<WarningResponse> {
        let mut warning_screen = Self::new(config);
        warning_screen.run()
    }

    fn run(&mut self) -> Result<WarningResponse> {
        // Print warning to console immediately
        self.print_console_warning();

        let event_loop = EventLoop::new()?;

        let window = event_loop.create_window(
            WindowAttributes::default()
                .with_title("⚠️ PHOTOSENSITIVE EPILEPSY WARNING - AetheriumBloom")
                .with_inner_size(LogicalSize::new(800.0, 600.0))
                .with_resizable(false)
        )?;

        self.window = Some(window);

        // Run the event loop
        let mut response = WarningResponse::Pending;

        event_loop.run(move |event, elwt| {
            match event {
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::CloseRequested => {
                            response = WarningResponse::Exit;
                            elwt.exit();
                        }
                        WindowEvent::KeyboardInput { event: KeyEvent { logical_key: key, state: ElementState::Pressed, .. }, .. } => {
                            match key {
                                Key::Named(NamedKey::Enter) | Key::Character(c) if c == "c" || c == "C" => {
                                    response = WarningResponse::Continue;
                                    elwt.exit();
                                }
                                Key::Character(c) if c == "s" || c == "S" => {
                                    response = WarningResponse::SafetyMode;
                                    elwt.exit();
                                }
                                Key::Named(NamedKey::Escape) | Key::Character(c) if c == "e" || c == "E" => {
                                    response = WarningResponse::Exit;
                                    elwt.exit();
                                }
                                _ => {}
                            }
                        }
                        WindowEvent::MouseInput { button: MouseButton::Left, state: ElementState::Pressed, .. } => {
                            // For now, left click continues (in a full implementation, this would check click position)
                            response = WarningResponse::Continue;
                            elwt.exit();
                        }
                        WindowEvent::RedrawRequested => {
                            // In a full implementation, this would render the GUI warning
                            // For now, we rely on console output
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        })?;

        self.response = response.clone();
        Ok(response)
    }

    fn print_console_warning(&self) {
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
        println!("   • Consider Safety Mode for reduced visual intensity");
        println!();
        println!("🛡️  SAFETY FEATURES ACTIVE:");
        println!("   • Flash rate limited to 3 Hz (international standard)");
        println!("   • Luminance changes capped at 10%");
        println!("   • Red flash protection enabled");
        println!("   • Emergency stop available (ESC key)");
        println!();
        println!("{}", "=".repeat(80));
        println!();

        if self.config.require_explicit_consent {
            println!("CHOOSE YOUR RESPONSE:");
            println!("  [C] CONTINUE - I understand the risks and want full visual effects");
            println!("  [S] SAFETY MODE - Continue with reduced visual intensity");
            println!("  [E] EXIT - I want to exit the application");
            println!();
            println!("Press the corresponding key or close this window to exit.");
        } else {
            println!("Press any key to continue or close window to exit...");
        }

        println!("{}", "=".repeat(80));
    }

    /// Get the user's response (for external polling)
    pub fn get_response(&self) -> WarningResponse {
        self.response.clone()
    }

    /// Check if user has acknowledged the warning
    pub fn is_acknowledged(&self) -> bool {
        self.warning_acknowledged || self.response != WarningResponse::Pending
    }

    /// Get elapsed time since warning was shown
    pub fn get_elapsed_time(&self) -> std::time::Duration {
        self.start_time.elapsed()
    }
}

/// Simple console-only warning for environments without windowing
pub fn show_console_warning_only() -> WarningResponse {
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
    println!("   • Emergency stop available (ESC key during execution)");
    println!();
    println!("{}", "=".repeat(80));
    println!();
    println!("CHOOSE YOUR RESPONSE:");
    println!("  [C] CONTINUE - I understand the risks and want full visual effects");
    println!("  [S] SAFETY MODE - Continue with reduced visual intensity (50%)");
    println!("  [E] EXIT - I want to exit the application");
    println!();
    print!("Enter your choice (C/S/E): ");

    use std::io::{self, Write};
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

/// Configuration for reduced safety mode
#[derive(Debug, Clone)]
pub struct SafetyModeConfig {
    pub visual_intensity_limit: f32,  // 0.0 to 1.0
    pub max_flash_rate: f32,          // Hz
    pub max_luminance_change: f32,    // 0.0 to 1.0
    pub disable_red_colors: bool,
    pub enable_break_reminders: bool,
    pub break_reminder_interval_minutes: u32,
}

impl Default for SafetyModeConfig {
    fn default() -> Self {
        Self {
            visual_intensity_limit: 0.5,     // 50% intensity
            max_flash_rate: 2.0,             // Even more conservative than 3 Hz
            max_luminance_change: 0.05,      // 5% instead of 10%
            disable_red_colors: true,
            enable_break_reminders: true,
            break_reminder_interval_minutes: 15, // Every 15 minutes
        }
    }
}

impl SafetyModeConfig {
    /// Create an ultra-safe configuration
    pub fn ultra_safe() -> Self {
        Self {
            visual_intensity_limit: 0.3,     // 30% intensity
            max_flash_rate: 1.0,             // 1 Hz maximum
            max_luminance_change: 0.03,      // 3% change
            disable_red_colors: true,
            enable_break_reminders: true,
            break_reminder_interval_minutes: 10, // Every 10 minutes
        }
    }

    /// Convert to SafetySettings for the main engine
    pub fn to_safety_settings(&self) -> crate::core::safety::SafetySettings {
        crate::core::safety::SafetySettings {
            enabled: true,
            max_flash_rate: self.max_flash_rate,
            max_luminance_change: self.max_luminance_change,
            red_flash_protection: self.disable_red_colors,
            chaos_dampening: true,
            visual_intensity_limit: self.visual_intensity_limit,
            emergency_stop_enabled: true,
        }
    }
}

/// Break reminder system for safety mode
pub struct BreakReminder {
    last_reminder: std::time::Instant,
    interval: std::time::Duration,
    reminders_shown: u32,
}

impl BreakReminder {
    pub fn new(interval_minutes: u32) -> Self {
        Self {
            last_reminder: std::time::Instant::now(),
            interval: std::time::Duration::from_secs(interval_minutes as u64 * 60),
            reminders_shown: 0,
        }
    }

    pub fn check_and_remind(&mut self) -> bool {
        if self.last_reminder.elapsed() >= self.interval {
            self.show_break_reminder();
            self.last_reminder = std::time::Instant::now();
            self.reminders_shown += 1;
            true
        } else {
            false
        }
    }

    fn show_break_reminder(&self) {
        println!("\n🌟 BREAK REMINDER #{} 🌟", self.reminders_shown + 1);
        println!("You've been viewing psychedelic effects for {} minutes.",
                (self.reminders_shown + 1) * (self.interval.as_secs() / 60) as u32);
        println!("Consider taking a break:");
        println!("• Look away from the screen");
        println!("• Focus on distant objects");
        println!("• Blink and rest your eyes");
        println!("• Move around and stretch");
        println!("Press ESC at any time to stop visual effects.");
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safety_mode_config() {
        let config = SafetyModeConfig::default();
        assert!(config.visual_intensity_limit <= 1.0);
        assert!(config.visual_intensity_limit >= 0.0);
        assert!(config.max_flash_rate <= 3.0); // Should be more conservative than standard
    }

    #[test]
    fn test_ultra_safe_config() {
        let config = SafetyModeConfig::ultra_safe();
        assert!(config.visual_intensity_limit <= 0.5); // Even more conservative
        assert!(config.max_flash_rate <= 2.0);
        assert!(config.disable_red_colors);
    }

    #[test]
    fn test_break_reminder() {
        let mut reminder = BreakReminder::new(1); // 1 minute for testing

        // Should not remind immediately
        assert!(!reminder.check_and_remind());
    }
}