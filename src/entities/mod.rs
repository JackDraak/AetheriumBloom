// Entities module containing llamas, species, and consciousness systems

pub mod llama;
pub mod llama_behavior;
pub mod species;

pub use llama::Llama;
pub use llama_behavior::*;
pub use species::{SpeciesType, SpeciesConfig, ConsciousnessLevel};