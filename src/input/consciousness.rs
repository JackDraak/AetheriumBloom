use instant::Instant;
use winit::event::{ElementState, MouseButton};
use crate::input::ResonanceEffect;

pub struct ConsciousnessResonance {
    inner: crate::mathematics::ConsciousnessResonance,
}

impl ConsciousnessResonance {
    pub fn new() -> Self {
        Self {
            inner: crate::mathematics::ConsciousnessResonance::new(),
        }
    }

    pub fn process_click(&mut self, timestamp: Instant, state: ElementState, button: MouseButton) -> ResonanceEffect {
        self.inner.process_click(timestamp, state, button)
    }
}