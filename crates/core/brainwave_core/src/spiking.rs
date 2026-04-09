use rand::Rng;
use std::collections::VecDeque;

/// Спайковый нейрон с мембранным потенциалом
#[derive(Clone, Debug)]
pub struct SpikingNeuron {
    pub weights: Vec<f64>,
    pub bias: f64,
    pub membrane_potential: f64,
    pub threshold: f64,
    pub refractory_period: usize,
    pub refractory_counter: usize,
    pub spike_history: VecDeque<bool>,
    pub potential_history: VecDeque<f64>,
}

impl SpikingNeuron {
    pub fn new(input_size: usize) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            weights: (0..input_size).map(|_| rng.gen_range(-1.0..1.0)).collect(),
            bias: rng.gen_range(-0.5..0.5),
            membrane_potential: 0.0,
            threshold: 1.0,
            refractory_period: 3,
            refractory_counter: 0,
            spike_history: VecDeque::with_capacity(50),
            potential_history: VecDeque::with_capacity(50),
        }
    }

    /// Leaky Integrate-and-Fire модель
    pub fn process(&mut self, input_spikes: &[bool], modulation: f64) -> bool {
        // Рефрактерный период
        if self.refractory_counter > 0 {
            self.refractory_counter -= 1;
            self.membrane_potential *= 0.5;
            self.record_state(false);
            return false;
        }

        // Утечка мембранного потенциала
        self.membrane_potential *= 0.85;

        // Интеграция входных спайков с модуляцией
        for (i, &spike) in input_spikes.iter().enumerate() {
            if spike && i < self.weights.len() {
                self.membrane_potential += self.weights[i] * modulation;
            }
        }

        self.membrane_potential += self.bias * modulation;

        // Проверка порога с учетом модуляции
        let effective_threshold = self.threshold / modulation.max(0.5);
        let fired = self.membrane_potential >= effective_threshold;

        if fired {
            self.membrane_potential = 0.0;
            self.refractory_counter = self.refractory_period;
        }

        self.record_state(fired);
        fired
    }

    fn record_state(&mut self, fired: bool) {
        self.spike_history.push_back(fired);
        if self.spike_history.len() > 50 {
            self.spike_history.pop_front();
        }

        self.potential_history.push_back(self.membrane_potential);
        if self.potential_history.len() > 50 {
            self.potential_history.pop_front();
        }
    }

    pub fn get_spike_rate(&self) -> f64 {
        if self.spike_history.is_empty() {
            return 0.0;
        }
        self.spike_history.iter().filter(|&&s| s).count() as f64 / self.spike_history.len() as f64
    }

    pub fn get_avg_potential(&self) -> f64 {
        if self.potential_history.is_empty() {
            return 0.0;
        }
        self.potential_history.iter().sum::<f64>() / self.potential_history.len() as f64
    }
}

/// Спайковая нейросеть с волновой модуляцией
pub struct SpikingBrainNetwork {
    pub layers: Vec<Vec<SpikingNeuron>>,
    pub layer_spike_rates: Vec<f64>,
}

impl SpikingBrainNetwork {
    pub fn new(layer_sizes: &[usize]) -> Self {
        let mut layers = Vec::new();

        for i in 1..layer_sizes.len() {
            let layer: Vec<SpikingNeuron> = (0..layer_sizes[i])
                .map(|_| SpikingNeuron::new(layer_sizes[i - 1]))
                .collect();
            layers.push(layer);
        }

        Self {
            layers,
            layer_spike_rates: vec![0.0; layer_sizes.len() - 1],
        }
    }

    /// Обработка с модуляцией от мозговых волн
    pub fn forward(&mut self, inputs: &[f64], wave_modulation: f64) -> Vec<bool> {
        // Конвертируем аналоговые входы в спайки
        let mut current_spikes: Vec<bool> = inputs.iter().map(|&x| x > 0.5).collect();

        for (layer_idx, layer) in self.layers.iter_mut().enumerate() {
            current_spikes = layer
                .iter_mut()
                .map(|neuron| neuron.process(&current_spikes, wave_modulation))
                .collect();

            // Вычисляем спайк-рейт слоя
            let spike_rate = layer.iter()
                .map(|n| n.get_spike_rate())
                .sum::<f64>() / layer.len() as f64;
            self.layer_spike_rates[layer_idx] = spike_rate;
        }

        current_spikes
    }

    pub fn get_total_spike_rate(&self) -> f64 {
        if self.layer_spike_rates.is_empty() {
            return 0.0;
        }
        self.layer_spike_rates.iter().sum::<f64>() / self.layer_spike_rates.len() as f64
    }

    pub fn get_network_activity(&self) -> f64 {
        let mut total_activity = 0.0;
        let mut count = 0;

        for layer in &self.layers {
            for neuron in layer {
                total_activity += neuron.get_avg_potential().abs();
                count += 1;
            }
        }

        if count > 0 {
            total_activity / count as f64
        } else {
            0.0
        }
    }
}
