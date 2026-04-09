//! # Token-Spike Interface
//!
//! Преобразование между языковыми токенами и спайковыми паттернами.
//!
//! ## Концепция
//!
//! LLM мыслят токенами (кусочками слов). Спайковые сети мыслят импульсами во времени.
//! Этот модуль - транслятор между двумя мирами.
//!
//! ## Архитектура
//!
//! ```text
//! Слово "Привет"
//!     ↓
//! Фонемы: ['п', 'р', 'и', 'в', 'е', 'т']
//!     ↓
//! Частоты: [40Hz, 35Hz, 42Hz, 38Hz, 40Hz, 37Hz]
//!     ↓
//! Спайковый паттерн: [(0ms, n1), (25ms, n1), (50ms, n1), ...]
//!     ↓
//! Нейронная сеть
//!     ↓
//! Резонансный анализ
//!     ↓
//! Декодированное слово
//! ```

pub mod encoder;
pub mod decoder;
pub mod resonance;
pub mod patterns;
pub mod sensory_input;
pub mod motor_output;
pub mod thought_stream;

use serde::{Deserialize, Serialize};

/// Временная метка спайка в миллисекундах
pub type SpikeTime = f64;

/// ID нейрона
pub type NeuronId = usize;

/// Один спайк в определенный момент времени от определенного нейрона
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Spike {
    pub time: SpikeTime,
    pub neuron_id: NeuronId,
}

/// Паттерн спайков (последовательность спайков)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpikePattern {
    pub spikes: Vec<Spike>,
    pub duration: f64, // Общая длительность паттерна в мс
}

impl SpikePattern {
    pub fn new() -> Self {
        Self {
            spikes: Vec::new(),
            duration: 0.0,
        }
    }

    pub fn add_spike(&mut self, spike: Spike) {
        self.spikes.push(spike);
        if spike.time > self.duration {
            self.duration = spike.time;
        }
    }

    /// Получить спайки в временном окне
    pub fn get_spikes_in_window(&self, start: f64, end: f64) -> Vec<Spike> {
        self.spikes
            .iter()
            .filter(|s| s.time >= start && s.time < end)
            .copied()
            .collect()
    }

    /// Вычислить среднюю частоту спайков (Hz)
    pub fn average_frequency(&self) -> f64 {
        if self.duration == 0.0 || self.spikes.is_empty() {
            return 0.0;
        }
        (self.spikes.len() as f64) / (self.duration / 1000.0)
    }
}

/// Характеристики фонемы
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhonemeCharacteristics {
    pub phoneme: char,
    pub frequency: f64,    // Характерная частота в Hz
    pub duration: f64,     // Длительность в мс
    pub neuron_id: NeuronId,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spike_pattern_creation() {
        let mut pattern = SpikePattern::new();
        pattern.add_spike(Spike { time: 0.0, neuron_id: 0 });
        pattern.add_spike(Spike { time: 25.0, neuron_id: 0 });
        pattern.add_spike(Spike { time: 50.0, neuron_id: 0 });

        assert_eq!(pattern.spikes.len(), 3);
        assert_eq!(pattern.duration, 50.0);
        assert!((pattern.average_frequency() - 60.0).abs() < 1.0); // ~60 Hz
    }

    #[test]
    fn test_get_spikes_in_window() {
        let mut pattern = SpikePattern::new();
        pattern.add_spike(Spike { time: 10.0, neuron_id: 0 });
        pattern.add_spike(Spike { time: 30.0, neuron_id: 0 });
        pattern.add_spike(Spike { time: 50.0, neuron_id: 0 });

        let window_spikes = pattern.get_spikes_in_window(20.0, 40.0);
        assert_eq!(window_spikes.len(), 1);
        assert_eq!(window_spikes[0].time, 30.0);
    }
}
