// Entities module containing llamas, species, and consciousness systems

pub mod species;
pub mod llama;

pub use species::{SpeciesType, SpeciesConfig, ConsciousnessLevel};
pub use llama::Llama;