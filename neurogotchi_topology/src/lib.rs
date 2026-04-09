//! # NeuroGotchi Topology
//!
//! 5-слойная когнитивная архитектура для AGI-питомца
//!
//! ## Слои
//!
//! ```text
//! L1_Sensory (100 нейронов)
//!     ↓ feedforward
//! L2_Associative (500 нейронов) ← STDP обучение
//!     ↓ feedforward
//! L3_Workspace (200 нейронов) ← Глобальное рабочее пространство
//!     ↓ feedforward
//! L4_InnerVoice (150 нейронов) ← Внутренний монолог
//!     ↓ feedforward (высокий порог)    ↓ feedback (слабая связь)
//! L5_Motor (100 нейронов)          → L1_Sensory (эхо мыслей)
//!     ↓ WTA
//! Речь
//! ```

pub mod layer;
pub mod connection;
pub mod topology;
pub mod innate;
pub mod development;

pub use layer::{Layer, LayerType, Neuron, NeuronType};
pub use connection::{Connection, ConnectionMatrix, ConnectionType, SynapticWeight};
pub use topology::{NeuroGotchiTopology, TopologyStats};
pub use innate::{InnateConnections, InnateVocabulary};
pub use development::{DevelopmentManager, DevelopmentalStage, ImprintingWindow, ImprintType};
