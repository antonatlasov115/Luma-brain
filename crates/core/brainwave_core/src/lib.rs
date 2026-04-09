pub mod brainwaves;
pub mod wave_network;
pub mod spiking;
pub mod learning;
pub mod vocabulary;

pub use brainwaves::{BrainwaveType, BrainwaveSpectrum, ConsciousnessState};
pub use wave_network::{BrainwaveNetwork, WaveModulatedNeuron};
pub use spiking::{SpikingNeuron, SpikingBrainNetwork};
pub use learning::{LearnableSpikingNeuron, LearnableSpikingNetwork};
pub use vocabulary::PetVocabulary;
