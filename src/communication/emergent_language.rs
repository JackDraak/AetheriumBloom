// Emergent language and communication systems module
// Extracted from simple.rs for better modularity

use glam::{Vec2, Vec3};
use std::collections::HashMap;
use crate::entities::Llama;
use crate::simulation::consciousness_systems::DigitalEcosystem;

// === EMERGENT COMMUNICATION SYSTEMS ===
/// Visual language development between entities
#[derive(Debug, Clone)]
pub struct EmergentCommunicationSystems {
    pub signal_vocabulary: SignalVocabulary,
    pub grammar_evolution: GrammarEvolution,
    pub semantic_networks: SemanticNetworks,
    pub cultural_transmission: CulturalTransmission,
    pub active_communications: Vec<VisualMessage>,
}

#[derive(Debug, Clone)]
pub struct SignalVocabulary {
    pub basic_signals: HashMap<String, VisualSignal>,
    pub signal_complexity: f32,
    pub emergence_rate: f32,
    pub signal_evolution_time: f64,
}

#[derive(Debug, Clone)]
pub struct VisualSignal {
    pub signal_type: SignalType,
    pub color_pattern: Vec3,
    pub movement_pattern: MovementPattern,
    pub intensity: f32,
    pub duration: f32,
    pub meaning_strength: f32,
    pub usage_frequency: f32,
}

#[derive(Debug, Clone)]
pub enum SignalType {
    Color(ColorSignal),
    Movement(MovementSignal),
    Pulse(PulseSignal),
    Shape(ShapeSignal),
    Hybrid(Vec<SignalType>),
}

#[derive(Debug, Clone)]
pub struct ColorSignal {
    pub hue_range: (f32, f32),
    pub saturation_range: (f32, f32),
    pub brightness_range: (f32, f32),
    pub transition_speed: f32,
}

#[derive(Debug, Clone)]
pub struct MovementSignal {
    pub pattern_type: MovementPatternType,
    pub amplitude: f32,
    pub frequency: f32,
    pub direction: Vec2,
}

#[derive(Debug, Clone)]
pub enum MovementPatternType {
    Circular,
    Linear,
    Spiral,
    Figure8,
    Random,
    Harmonic,
}

#[derive(Debug, Clone)]
pub struct PulseSignal {
    pub base_intensity: f32,
    pub pulse_frequency: f32,
    pub pulse_amplitude: f32,
    pub pulse_shape: PulseShape,
}

#[derive(Debug, Clone)]
pub enum PulseShape {
    Sine,
    Square,
    Triangle,
    Sawtooth,
    Exponential,
}

#[derive(Debug, Clone)]
pub struct ShapeSignal {
    pub geometric_form: GeometricForm,
    pub size_modulation: f32,
    pub rotation_speed: f32,
    pub morphing_rate: f32,
}

#[derive(Debug, Clone)]
pub enum GeometricForm {
    Circle,
    Triangle,
    Square,
    Pentagon,
    Star,
    Fractal,
}

#[derive(Debug, Clone)]
pub struct MovementPattern {
    pub pattern_type: MovementPatternType,
    pub amplitude: f32,
    pub frequency: f32,
    pub phase_offset: f32,
}

#[derive(Debug, Clone)]
pub struct GrammarEvolution {
    pub grammar_rules: HashMap<String, GrammarRule>,
    pub complexity_level: f32,
    pub rule_emergence_rate: f32,
    pub syntax_coherence: f32,
    pub message_complexity_capacity: f32,
}

#[derive(Debug, Clone)]
pub struct GrammarRule {
    pub rule_type: RuleType,
    pub conditions: Vec<MessageCondition>,
    pub transformations: Vec<MessageTransformation>,
    pub reliability: f32,
    pub usage_context: CommunicationContext,
}

#[derive(Debug, Clone)]
pub enum RuleType {
    Sequence,      // A followed by B creates meaning C
    Combination,   // A and B together create meaning C
    Modulation,    // A modifies the meaning of B
    Negation,      // A negates or reverses meaning of B
    Amplification, // A strengthens meaning of B
}

#[derive(Debug, Clone)]
pub struct MessageCondition {
    pub signal_presence: Option<String>,
    pub intensity_threshold: Option<f32>,
    pub temporal_relationship: Option<TemporalRelation>,
    pub spatial_relationship: Option<SpatialRelation>,
}

#[derive(Debug, Clone)]
pub enum TemporalRelation {
    Simultaneous,
    Sequential(f32), // time delay
    Overlapping(f32), // overlap duration
    Rhythmic(f32),   // rhythmic pattern
}

#[derive(Debug, Clone)]
pub enum SpatialRelation {
    Adjacent,
    Distant(f32),
    Surrounding,
    Directional(Vec2),
}

#[derive(Debug, Clone)]
pub struct MessageTransformation {
    pub transform_type: TransformType,
    pub parameters: Vec<f32>,
    pub strength: f32,
}

#[derive(Debug, Clone)]
pub enum TransformType {
    ColorShift,
    IntensityChange,
    MovementModification,
    TemporalAdjustment,
    SpatialDisplacement,
}

#[derive(Debug, Clone)]
pub enum CommunicationContext {
    Greeting,
    Warning,
    Cooperation,
    Territorial,
    Emotional,
    Informational,
    Social,
    Emergency,
}

#[derive(Debug, Clone)]
pub struct SemanticNetworks {
    pub concept_associations: HashMap<String, ConceptNode>,
    pub meaning_strength_matrix: Vec<Vec<f32>>,
    pub context_modifiers: HashMap<CommunicationContext, f32>,
    pub emergence_pathways: Vec<EmergencePathway>,
}

#[derive(Debug, Clone)]
pub struct ConceptNode {
    pub concept_id: String,
    pub base_meaning: String,
    pub associated_signals: Vec<String>,
    pub meaning_confidence: f32,
    pub usage_frequency: f32,
    pub context_variations: HashMap<CommunicationContext, f32>,
    pub emergence_timestamp: f64,
}

#[derive(Debug, Clone)]
pub struct EmergencePathway {
    pub source_concepts: Vec<String>,
    pub target_concept: String,
    pub pathway_strength: f32,
    pub emergence_probability: f32,
    pub required_conditions: Vec<EmergenceCondition>,
}

#[derive(Debug, Clone)]
pub struct EmergenceCondition {
    pub condition_type: ConditionType,
    pub threshold: f32,
    pub current_value: f32,
}

#[derive(Debug, Clone)]
pub enum ConditionType {
    ConsciousnessLevel,
    CommunicationFrequency,
    SemanticComplexity,
    CulturalCoherence,
    EnvironmentalStability,
}

#[derive(Debug, Clone)]
pub struct CulturalTransmission {
    pub transmission_networks: Vec<TransmissionNetwork>,
    pub language_variants: HashMap<String, LanguageVariant>,
    pub cultural_drift_rate: f32,
    pub innovation_rate: f32,
    pub adoption_threshold: f32,
}

#[derive(Debug, Clone)]
pub struct TransmissionNetwork {
    pub network_id: String,
    pub participating_entities: Vec<usize>,
    pub transmission_efficiency: f32,
    pub network_stability: f32,
    pub dominant_language_variant: String,
    pub cultural_coherence: f32,
}

#[derive(Debug, Clone)]
pub struct LanguageVariant {
    pub variant_id: String,
    pub signal_modifications: HashMap<String, SignalModification>,
    pub grammar_differences: HashMap<String, GrammarModification>,
    pub prevalence: f32,
    pub innovation_rate: f32,
    pub cultural_fitness: f32,
}

#[derive(Debug, Clone)]
pub struct SignalModification {
    pub original_signal: String,
    pub modified_properties: HashMap<String, f32>,
    pub modification_strength: f32,
}

#[derive(Debug, Clone)]
pub struct GrammarModification {
    pub rule_id: String,
    pub modification_type: GrammarModificationType,
    pub parameters: Vec<f32>,
}

#[derive(Debug, Clone)]
pub enum GrammarModificationType {
    RuleStrengthening,
    RuleWeakening,
    RuleInversion,
    RuleComplication,
    RuleSimplification,
}

#[derive(Debug, Clone)]
pub struct VisualMessage {
    pub sender_id: usize,
    pub receiver_ids: Vec<usize>,
    pub signal_sequence: Vec<ActiveSignal>,
    pub message_intent: CommunicationContext,
    pub spatial_manifestation: SpatialManifestation,
    pub temporal_properties: TemporalProperties,
    pub transmission_effectiveness: f32,
    pub cultural_variant: String,
}

#[derive(Debug, Clone)]
pub struct ActiveSignal {
    pub signal_id: String,
    pub current_intensity: f32,
    pub position: Vec2,
    pub visual_state: VisualState,
    pub duration_remaining: f32,
    pub interaction_radius: f32,
}

#[derive(Debug, Clone)]
pub struct VisualState {
    pub current_color: Vec3,
    pub current_size: f32,
    pub current_rotation: f32,
    pub movement_velocity: Vec2,
    pub pulsing_phase: f32,
}

#[derive(Debug, Clone)]
pub struct SpatialManifestation {
    pub manifestation_type: ManifestationType,
    pub primary_position: Vec2,
    pub affected_area: f32,
    pub visual_trail: Vec<Vec2>,
    pub interference_patterns: Vec<InterferenceZone>,
}

#[derive(Debug, Clone)]
pub enum ManifestationType {
    PointSource,
    LinearPath,
    RadialExpansion,
    NetworkNodes,
    FieldDistortion,
}

#[derive(Debug, Clone)]
pub struct InterferenceZone {
    pub center: Vec2,
    pub radius: f32,
    pub interference_strength: f32,
    pub interference_type: InterferenceType,
}

#[derive(Debug, Clone)]
pub enum InterferenceType {
    Constructive,
    Destructive,
    Modulative,
    Harmonic,
}

#[derive(Debug, Clone)]
pub struct TemporalProperties {
    pub message_duration: f32,
    pub transmission_delay: f32,
    pub repetition_pattern: Option<RepetitionPattern>,
    pub decay_rate: f32,
    pub resonance_buildup: f32,
}

#[derive(Debug, Clone)]
pub struct RepetitionPattern {
    pub repetition_interval: f32,
    pub repetition_count: u32,
    pub intensity_decay_per_repetition: f32,
    pub pattern_modification_rate: f32,
}

impl EmergentCommunicationSystems {
    pub fn new() -> Self {
        Self {
            signal_vocabulary: SignalVocabulary::new(),
            grammar_evolution: GrammarEvolution::new(),
            semantic_networks: SemanticNetworks::new(),
            cultural_transmission: CulturalTransmission::new(),
            active_communications: Vec::new(),
        }
    }

    pub fn update(&mut self, dt: f32, llamas: &[Llama], ecosystem: &DigitalEcosystem,
                  consciousness_level: f32, cosmic_time: f64) {
        // Update signal vocabulary evolution
        self.signal_vocabulary.update(dt, consciousness_level, cosmic_time);

        // Evolve grammar based on usage patterns
        self.grammar_evolution.update(dt, &self.active_communications, consciousness_level);

        // Update semantic associations
        self.semantic_networks.update(dt, &self.active_communications, &self.signal_vocabulary);

        // Handle cultural transmission between entities
        self.cultural_transmission.update(dt, llamas, &self.active_communications);

        // Update active communications
        self.update_active_communications(dt, llamas, ecosystem, cosmic_time);

        // Generate new communications based on entity interactions
        self.generate_new_communications(llamas, ecosystem, consciousness_level, cosmic_time);
    }

    fn update_active_communications(&mut self, dt: f32, llamas: &[Llama],
                                   ecosystem: &DigitalEcosystem, cosmic_time: f64) {
        for communication in &mut self.active_communications {
            communication.update(dt, llamas, ecosystem, cosmic_time);
        }

        // Remove completed communications
        self.active_communications.retain(|comm| !comm.is_completed());
    }

    fn generate_new_communications(&mut self, llamas: &[Llama], ecosystem: &DigitalEcosystem,
                                  consciousness_level: f32, cosmic_time: f64) {
        // Generate communications based on llama interactions and consciousness level
        for (i, llama) in llamas.iter().enumerate() {
            if llama.consciousness > 0.7 && cosmic_time as f32 % 2.0 < 0.1 {
                // High consciousness llamas attempt communication
                if let Some(communication) = self.create_communication(i, llamas, ecosystem, cosmic_time) {
                    self.active_communications.push(communication);
                }
            }
        }
    }

    fn create_communication(&self, sender_id: usize, llamas: &[Llama],
                           ecosystem: &DigitalEcosystem, cosmic_time: f64) -> Option<VisualMessage> {
        // Create a new visual message based on current context
        let sender = &llamas[sender_id];

        // Find nearby llamas as potential receivers
        let nearby_llamas: Vec<usize> = llamas.iter().enumerate()
            .filter(|(i, llama)| *i != sender_id &&
                    (sender.position - llama.position).length() < 200.0)
            .map(|(i, _)| i)
            .collect();

        if nearby_llamas.is_empty() {
            return None;
        }

        // Determine communication intent based on sender's state
        let intent = if sender.consciousness > 0.9 {
            CommunicationContext::Social
        } else if sender.prime_chaos_factor > 0.8 {
            CommunicationContext::Emotional
        } else {
            CommunicationContext::Informational
        };

        // Create signal based on available vocabulary
        let signal_sequence = vec![ActiveSignal {
            signal_id: "basic_greeting".to_string(),
            current_intensity: sender.consciousness,
            position: sender.position,
            visual_state: VisualState {
                current_color: Vec3::new(sender.color.x, sender.color.y, 0.6),
                current_size: 20.0 + sender.consciousness * 30.0,
                current_rotation: 0.0,
                movement_velocity: Vec2::ZERO,
                pulsing_phase: cosmic_time as f32,
            },
            duration_remaining: 2.0,
            interaction_radius: 50.0,
        }];

        Some(VisualMessage {
            sender_id,
            receiver_ids: nearby_llamas,
            signal_sequence,
            message_intent: intent,
            spatial_manifestation: SpatialManifestation {
                manifestation_type: ManifestationType::RadialExpansion,
                primary_position: sender.position,
                affected_area: 100.0,
                visual_trail: vec![sender.position],
                interference_patterns: Vec::new(),
            },
            temporal_properties: TemporalProperties {
                message_duration: 2.0,
                transmission_delay: 0.1,
                repetition_pattern: None,
                decay_rate: 0.5,
                resonance_buildup: 0.0,
            },
            transmission_effectiveness: 0.8,
            cultural_variant: "default".to_string(),
        })
    }
}

impl SignalVocabulary {
    pub fn new() -> Self {
        let mut basic_signals = HashMap::new();

        // Initialize basic signals
        basic_signals.insert("basic_greeting".to_string(), VisualSignal {
            signal_type: SignalType::Color(ColorSignal {
                hue_range: (120.0, 180.0), // Green-cyan range
                saturation_range: (0.7, 1.0),
                brightness_range: (0.6, 1.0),
                transition_speed: 2.0,
            }),
            color_pattern: Vec3::new(0.3, 0.8, 0.6),
            movement_pattern: MovementPattern {
                pattern_type: MovementPatternType::Circular,
                amplitude: 20.0,
                frequency: 1.0,
                phase_offset: 0.0,
            },
            intensity: 0.8,
            duration: 2.0,
            meaning_strength: 0.9,
            usage_frequency: 0.0,
        });

        Self {
            basic_signals,
            signal_complexity: 0.1,
            emergence_rate: 0.01,
            signal_evolution_time: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32, consciousness_level: f32, cosmic_time: f64) {
        self.signal_evolution_time += dt as f64;

        // Evolve signal complexity based on consciousness
        self.signal_complexity += dt * consciousness_level * 0.01;
        self.signal_complexity = self.signal_complexity.min(1.0);

        // Increase emergence rate with higher consciousness
        self.emergence_rate = 0.01 + consciousness_level * 0.02;

        // Potentially evolve existing signals
        for signal in self.basic_signals.values_mut() {
            signal.evolve(dt, consciousness_level);
        }

        // Create new signals if complexity threshold is met
        if self.signal_complexity > 0.5 && self.basic_signals.len() < 10 {
            self.generate_new_signal(consciousness_level, cosmic_time);
        }
    }

    fn generate_new_signal(&mut self, consciousness_level: f32, cosmic_time: f64) {
        let signal_id = format!("evolved_signal_{}", self.basic_signals.len());

        let new_signal = VisualSignal {
            signal_type: SignalType::Hybrid(vec![
                SignalType::Color(ColorSignal {
                    hue_range: (cosmic_time as f32 % 360.0, (cosmic_time as f32 + 60.0) % 360.0),
                    saturation_range: (0.6, 1.0),
                    brightness_range: (0.5, 0.9),
                    transition_speed: 1.0 + consciousness_level,
                }),
                SignalType::Movement(MovementSignal {
                    pattern_type: MovementPatternType::Spiral,
                    amplitude: 15.0 + consciousness_level * 20.0,
                    frequency: 0.5 + consciousness_level * 1.5,
                    direction: Vec2::new(consciousness_level.cos(), consciousness_level.sin()),
                }),
            ]),
            color_pattern: Vec3::new(
                (cosmic_time as f32 * 0.1).sin() * 0.5 + 0.5,
                (cosmic_time as f32 * 0.15).cos() * 0.5 + 0.5,
                consciousness_level,
            ),
            movement_pattern: MovementPattern {
                pattern_type: MovementPatternType::Spiral,
                amplitude: 25.0,
                frequency: 1.2,
                phase_offset: cosmic_time as f32,
            },
            intensity: consciousness_level,
            duration: 3.0,
            meaning_strength: 0.5,
            usage_frequency: 0.0,
        };

        self.basic_signals.insert(signal_id, new_signal);
    }
}

impl VisualSignal {
    pub fn evolve(&mut self, dt: f32, consciousness_level: f32) {
        // Evolve the signal properties based on usage and consciousness
        self.meaning_strength += dt * consciousness_level * 0.01;
        self.meaning_strength = self.meaning_strength.min(1.0);

        // Adjust intensity based on consciousness
        self.intensity = (self.intensity + consciousness_level * dt * 0.1).min(1.0);

        // Evolve color pattern
        self.color_pattern = self.color_pattern.lerp(
            Vec3::new(consciousness_level, 0.8, 0.9),
            dt * 0.05
        );
    }
}

impl GrammarEvolution {
    pub fn new() -> Self {
        Self {
            grammar_rules: HashMap::new(),
            complexity_level: 0.1,
            rule_emergence_rate: 0.005,
            syntax_coherence: 0.8,
            message_complexity_capacity: 0.2,
        }
    }

    pub fn update(&mut self, dt: f32, active_communications: &[VisualMessage], consciousness_level: f32) {
        // Evolve grammar complexity
        self.complexity_level += dt * consciousness_level * 0.005;
        self.complexity_level = self.complexity_level.min(1.0);

        // Update rule emergence rate
        self.rule_emergence_rate = 0.005 + consciousness_level * 0.01;

        // Analyze active communications for pattern emergence
        self.analyze_communication_patterns(active_communications);

        // Generate new rules if complexity allows
        if self.complexity_level > 0.3 && self.grammar_rules.len() < 20 {
            self.generate_new_rule(consciousness_level);
        }
    }

    fn analyze_communication_patterns(&mut self, communications: &[VisualMessage]) {
        // Simple pattern analysis - look for repeated signal combinations
        for comm in communications {
            if comm.signal_sequence.len() > 1 {
                // This is a compound message - potential for grammar rule
                self.syntax_coherence += 0.001;
            }
        }
        self.syntax_coherence = self.syntax_coherence.min(1.0);
    }

    fn generate_new_rule(&mut self, consciousness_level: f32) {
        let rule_id = format!("rule_{}", self.grammar_rules.len());

        let new_rule = GrammarRule {
            rule_type: RuleType::Sequence,
            conditions: vec![MessageCondition {
                signal_presence: Some("basic_greeting".to_string()),
                intensity_threshold: Some(0.5),
                temporal_relationship: Some(TemporalRelation::Sequential(0.5)),
                spatial_relationship: None,
            }],
            transformations: vec![MessageTransformation {
                transform_type: TransformType::IntensityChange,
                parameters: vec![1.2], // Amplify by 20%
                strength: consciousness_level,
            }],
            reliability: 0.7,
            usage_context: CommunicationContext::Social,
        };

        self.grammar_rules.insert(rule_id, new_rule);
    }
}

impl SemanticNetworks {
    pub fn new() -> Self {
        Self {
            concept_associations: HashMap::new(),
            meaning_strength_matrix: Vec::new(),
            context_modifiers: HashMap::new(),
            emergence_pathways: Vec::new(),
        }
    }

    pub fn update(&mut self, dt: f32, communications: &[VisualMessage], vocabulary: &SignalVocabulary) {
        // Update concept associations based on communication patterns
        for comm in communications {
            self.strengthen_associations(&comm.message_intent, &comm.signal_sequence);
        }

        // Update emergence pathways
        self.update_emergence_pathways(dt);

        // Decay unused associations
        self.decay_unused_associations(dt);
    }

    fn strengthen_associations(&mut self, intent: &CommunicationContext, signals: &[ActiveSignal]) {
        for signal in signals {
            let concept_id = format!("{:?}_{}", intent, signal.signal_id);
            self.concept_associations.entry(concept_id.clone())
                .or_insert_with(|| ConceptNode {
                    concept_id: concept_id.clone(),
                    base_meaning: format!("{:?}", intent),
                    associated_signals: vec![signal.signal_id.clone()],
                    meaning_confidence: 0.5,
                    usage_frequency: 0.0,
                    context_variations: HashMap::new(),
                    emergence_timestamp: 0.0,
                })
                .usage_frequency += 0.1;
        }
    }

    fn update_emergence_pathways(&mut self, dt: f32) {
        for pathway in &mut self.emergence_pathways {
            pathway.pathway_strength += dt * 0.01;
            pathway.pathway_strength = pathway.pathway_strength.min(1.0);
        }
    }

    fn decay_unused_associations(&mut self, dt: f32) {
        for concept in self.concept_associations.values_mut() {
            concept.meaning_confidence -= dt * 0.001;
            concept.meaning_confidence = concept.meaning_confidence.max(0.0);
        }
    }
}

impl CulturalTransmission {
    pub fn new() -> Self {
        Self {
            transmission_networks: Vec::new(),
            language_variants: HashMap::new(),
            cultural_drift_rate: 0.01,
            innovation_rate: 0.005,
            adoption_threshold: 0.3,
        }
    }

    pub fn update(&mut self, dt: f32, llamas: &[Llama], communications: &[VisualMessage]) {
        // Update transmission networks based on llama interactions
        self.update_networks(llamas, communications);

        // Apply cultural drift
        self.apply_cultural_drift(dt);

        // Process innovation
        self.process_innovation(dt);
    }

    fn update_networks(&mut self, llamas: &[Llama], communications: &[VisualMessage]) {
        // Create or update transmission networks based on active communications
        for comm in communications {
            let network_id = format!("network_{}", comm.sender_id);
            if !self.transmission_networks.iter().any(|n| n.network_id == network_id) {
                self.transmission_networks.push(TransmissionNetwork {
                    network_id,
                    participating_entities: vec![comm.sender_id],
                    transmission_efficiency: 0.7,
                    network_stability: 0.8,
                    dominant_language_variant: "default".to_string(),
                    cultural_coherence: 0.6,
                });
            }
        }
    }

    fn apply_cultural_drift(&mut self, dt: f32) {
        for variant in self.language_variants.values_mut() {
            variant.innovation_rate += dt * self.cultural_drift_rate * 0.1;
        }
    }

    fn process_innovation(&mut self, dt: f32) {
        // Generate new language variants based on innovation rate
        if self.language_variants.len() < 5 && dt * self.innovation_rate > 0.01 {
            let variant_id = format!("variant_{}", self.language_variants.len());
            self.language_variants.insert(variant_id.clone(), LanguageVariant {
                variant_id,
                signal_modifications: HashMap::new(),
                grammar_differences: HashMap::new(),
                prevalence: 0.1,
                innovation_rate: self.innovation_rate,
                cultural_fitness: 0.5,
            });
        }
    }
}

impl VisualMessage {
    pub fn update(&mut self, dt: f32, llamas: &[Llama], ecosystem: &DigitalEcosystem, cosmic_time: f64) {
        // Update signal states
        for signal in &mut self.signal_sequence {
            signal.update(dt, cosmic_time);
        }

        // Update temporal properties
        self.temporal_properties.message_duration -= dt;
        self.temporal_properties.resonance_buildup += dt * 0.1;

        // Update spatial manifestation
        self.spatial_manifestation.update(dt, llamas, ecosystem);
    }

    pub fn is_completed(&self) -> bool {
        self.temporal_properties.message_duration <= 0.0
    }
}

impl ActiveSignal {
    pub fn update(&mut self, dt: f32, cosmic_time: f64) {
        self.duration_remaining -= dt;

        // Update visual state
        self.visual_state.pulsing_phase += dt * 2.0;
        self.visual_state.current_rotation += dt * 0.5;

        // Fade out as duration decreases
        if self.duration_remaining < 1.0 {
            self.current_intensity *= 0.95;
        }
    }
}

impl SpatialManifestation {
    pub fn update(&mut self, dt: f32, llamas: &[Llama], ecosystem: &DigitalEcosystem) {
        // Update manifestation based on type
        match self.manifestation_type {
            ManifestationType::RadialExpansion => {
                self.affected_area += dt * 50.0; // Expand outward
            }
            ManifestationType::LinearPath => {
                // Move along a path
                if let Some(last_pos) = self.visual_trail.last() {
                    let new_pos = *last_pos + Vec2::new(dt * 100.0, 0.0);
                    self.visual_trail.push(new_pos);
                }
            }
            _ => {}
        }

        // Update interference patterns
        for pattern in &mut self.interference_patterns {
            pattern.interference_strength *= 0.99; // Gradual decay
        }
    }
}