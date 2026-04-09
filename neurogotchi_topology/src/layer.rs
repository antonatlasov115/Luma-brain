//! Слои нейронов

use serde::{Deserialize, Serialize};

/// Тип слоя
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayerType {
    /// L1: Сенсорный слой (врата восприятия)
    Sensory,

    /// L2: Ассоциативный слой (грибовидные тела, память)
    Associative,

    /// L3: Глобальное рабочее пространство (ядро сознания)
    Workspace,

    /// L4: Внутренний голос (премоторная кора)
    InnerVoice,

    /// L5: Моторно-речевой центр
    Motor,
}

/// Нейрон в слое
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Neuron {
    /// ID нейрона (глобальный)
    pub id: usize,

    /// Мембранный потенциал
    pub membrane_potential: f64,

    /// Порог срабатывания
    pub threshold: f64,

    /// Рефрактерный период (оставшееся время)
    pub refractory_counter: u32,

    /// Рефрактерный период (полный)
    pub refractory_period: u32,

    /// История спайков (последние 50)
    pub spike_history: Vec<f64>,

    /// Тип нейрона
    pub neuron_type: NeuronType,
}

/// Тип нейрона
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeuronType {
    /// Возбуждающий (глутамат)
    Excitatory,

    /// Тормозящий (ГАМК)
    Inhibitory,
}

impl Neuron {
    pub fn new(id: usize, neuron_type: NeuronType) -> Self {
        Self {
            id,
            membrane_potential: 0.0,
            threshold: 1.0,
            refractory_counter: 0,
            refractory_period: 3,
            spike_history: Vec::new(),
            neuron_type,
        }
    }

    /// Обработать входные спайки
    pub fn process(&mut self, input_current: f64, modulation: f64) -> bool {
        // Рефрактерный период
        if self.refractory_counter > 0 {
            self.refractory_counter -= 1;
            self.membrane_potential *= 0.5;
            return false;
        }

        // Утечка
        self.membrane_potential *= 0.85;

        // Добавляем входной ток
        self.membrane_potential += input_current * modulation;

        // Проверяем порог (модулируется)
        let effective_threshold = self.threshold / modulation.max(0.5);

        if self.membrane_potential >= effective_threshold {
            self.refractory_counter = self.refractory_period;
            self.membrane_potential = 0.0;
            true
        } else {
            false
        }
    }

    /// Записать спайк в историю
    pub fn record_spike(&mut self, time: f64) {
        self.spike_history.push(time);
        if self.spike_history.len() > 50 {
            self.spike_history.remove(0);
        }
    }
}

/// Слой нейронов
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    /// Тип слоя
    pub layer_type: LayerType,

    /// Нейроны в слое
    pub neurons: Vec<Neuron>,

    /// Размер слоя
    pub size: usize,

    /// Соотношение возбуждающих/тормозящих (обычно 80/20)
    pub excitatory_ratio: f64,
}

impl Layer {
    pub fn new(layer_type: LayerType, size: usize, excitatory_ratio: f64) -> Self {
        let mut neurons = Vec::with_capacity(size);
        let excitatory_count = (size as f64 * excitatory_ratio) as usize;

        for i in 0..size {
            let neuron_type = if i < excitatory_count {
                NeuronType::Excitatory
            } else {
                NeuronType::Inhibitory
            };

            neurons.push(Neuron::new(i, neuron_type));
        }

        Self {
            layer_type,
            neurons,
            size,
            excitatory_ratio,
        }
    }

    /// Получить активность слоя (средняя частота спайков)
    pub fn get_activity(&self, current_time: f64, window: f64) -> f64 {
        let mut total_spikes = 0;

        for neuron in &self.neurons {
            for &spike_time in &neuron.spike_history {
                if current_time - spike_time < window {
                    total_spikes += 1;
                }
            }
        }

        if self.size == 0 || window == 0.0 {
            return 0.0;
        }

        (total_spikes as f64) / (self.size as f64 * window / 1000.0)
    }

    /// Получить средний мембранный потенциал
    pub fn get_average_potential(&self) -> f64 {
        if self.neurons.is_empty() {
            return 0.0;
        }

        let sum: f64 = self.neurons.iter().map(|n| n.membrane_potential).sum();
        sum / self.neurons.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neuron_creation() {
        let neuron = Neuron::new(0, NeuronType::Excitatory);
        assert_eq!(neuron.id, 0);
        assert_eq!(neuron.membrane_potential, 0.0);
    }

    #[test]
    fn test_neuron_spike() {
        let mut neuron = Neuron::new(0, NeuronType::Excitatory);

        // Подаем достаточный ток
        let fired = neuron.process(2.0, 1.0);
        assert!(fired);

        // В рефрактерном периоде не должен спайкать
        let fired2 = neuron.process(2.0, 1.0);
        assert!(!fired2);
    }

    #[test]
    fn test_layer_creation() {
        let layer = Layer::new(LayerType::Sensory, 100, 0.8);
        assert_eq!(layer.size, 100);
        assert_eq!(layer.neurons.len(), 100);

        // Проверяем соотношение возбуждающих/тормозящих
        let excitatory_count = layer.neurons.iter()
            .filter(|n| n.neuron_type == NeuronType::Excitatory)
            .count();
        assert_eq!(excitatory_count, 80);
    }

    #[test]
    fn test_layer_activity() {
        let mut layer = Layer::new(LayerType::Sensory, 10, 0.8);

        // Записываем спайки
        for neuron in &mut layer.neurons {
            neuron.record_spike(0.0);
        }

        let activity = layer.get_activity(100.0, 200.0);
        assert!(activity > 0.0);
    }
}
