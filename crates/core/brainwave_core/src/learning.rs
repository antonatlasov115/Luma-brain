use rand::Rng;
use std::collections::VecDeque;

/// Спайковый нейрон с STDP обучением
#[derive(Clone, Debug)]
pub struct LearnableSpikingNeuron {
    pub weights: Vec<f64>,
    pub bias: f64,
    pub membrane_potential: f64,
    pub threshold: f64,
    pub refractory_period: usize,
    pub refractory_counter: usize,
    pub spike_history: VecDeque<bool>,
    pub potential_history: VecDeque<f64>,
    pub last_spike_time: Option<f64>,
    pub input_spike_times: Vec<Option<f64>>,
}

impl LearnableSpikingNeuron {
    pub fn new(input_size: usize) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            weights: (0..input_size).map(|_| rng.gen_range(0.1..0.5)).collect(),
            bias: rng.gen_range(-0.3..0.3),
            membrane_potential: 0.0,
            threshold: 1.0,
            refractory_period: 3,
            refractory_counter: 0,
            spike_history: VecDeque::with_capacity(100),
            potential_history: VecDeque::with_capacity(100),
            last_spike_time: None,
            input_spike_times: vec![None; input_size],
        }
    }

    /// Обработка с STDP обучением
    pub fn process_with_learning(&mut self, input_spikes: &[bool], modulation: f64, time: f64, learning_rate: f64) -> bool {
        // Обновляем времена входных спайков
        for (i, &spike) in input_spikes.iter().enumerate() {
            if spike && i < self.input_spike_times.len() {
                self.input_spike_times[i] = Some(time);
            }
        }

        // Рефрактерный период
        if self.refractory_counter > 0 {
            self.refractory_counter -= 1;
            self.membrane_potential *= 0.5;
            self.record_state(false);
            return false;
        }

        // Утечка
        self.membrane_potential *= 0.85;

        // Интеграция входов
        for (i, &spike) in input_spikes.iter().enumerate() {
            if spike && i < self.weights.len() {
                self.membrane_potential += self.weights[i] * modulation;
            }
        }

        self.membrane_potential += self.bias * modulation;

        // Проверка порога
        let effective_threshold = self.threshold / modulation.max(0.5);
        let fired = self.membrane_potential >= effective_threshold;

        if fired {
            self.membrane_potential = 0.0;
            self.refractory_counter = self.refractory_period;
            self.last_spike_time = Some(time);

            // STDP обучение при генерации спайка
            self.apply_stdp(time, learning_rate);
        }

        self.record_state(fired);
        fired
    }

    /// Spike-Timing-Dependent Plasticity (STDP)
    fn apply_stdp(&mut self, current_time: f64, learning_rate: f64) {
        for (i, input_time) in self.input_spike_times.iter().enumerate() {
            if let Some(t_pre) = input_time {
                let delta_t = current_time - t_pre;

                // STDP правило:
                // Если пресинаптический спайк был ДО постсинаптического -> усиление (LTP)
                // Если ПОСЛЕ -> ослабление (LTD)
                let weight_change = if delta_t > 0.0 && delta_t < 20.0 {
                    // Long-Term Potentiation (усиление)
                    learning_rate * (-delta_t / 20.0).exp()
                } else if delta_t < 0.0 && delta_t > -20.0 {
                    // Long-Term Depression (ослабление)
                    -learning_rate * 0.5 * (delta_t / 20.0).exp()
                } else {
                    0.0
                };

                // Обновляем вес с ограничениями
                self.weights[i] = (self.weights[i] + weight_change).clamp(0.0, 2.0);
            }
        }
    }

    fn record_state(&mut self, fired: bool) {
        self.spike_history.push_back(fired);
        if self.spike_history.len() > 100 {
            self.spike_history.pop_front();
        }

        self.potential_history.push_back(self.membrane_potential);
        if self.potential_history.len() > 100 {
            self.potential_history.pop_front();
        }
    }

    pub fn get_spike_rate(&self) -> f64 {
        if self.spike_history.is_empty() {
            return 0.0;
        }
        self.spike_history.iter().filter(|&&s| s).count() as f64 / self.spike_history.len() as f64
    }
}

/// Обучаемая спайковая сеть
#[derive(Clone)]
pub struct LearnableSpikingNetwork {
    pub layers: Vec<Vec<LearnableSpikingNeuron>>,
    pub time: f64,
    pub learning_rate: f64,
}

impl LearnableSpikingNetwork {
    pub fn new(layer_sizes: &[usize], learning_rate: f64) -> Self {
        let mut layers = Vec::new();

        for i in 1..layer_sizes.len() {
            let layer: Vec<LearnableSpikingNeuron> = (0..layer_sizes[i])
                .map(|_| LearnableSpikingNeuron::new(layer_sizes[i - 1]))
                .collect();
            layers.push(layer);
        }

        Self {
            layers,
            time: 0.0,
            learning_rate,
        }
    }

    pub fn forward_with_learning(&mut self, inputs: &[f64], modulation: f64, dt: f64) -> Vec<bool> {
        self.time += dt;

        let mut current_spikes: Vec<bool> = inputs.iter().map(|&x| x > 0.5).collect();

        for layer in &mut self.layers {
            current_spikes = layer
                .iter_mut()
                .map(|neuron| neuron.process_with_learning(&current_spikes, modulation, self.time, self.learning_rate))
                .collect();
        }

        current_spikes
    }

    pub fn get_total_spike_rate(&self) -> f64 {
        let mut total = 0.0;
        let mut count = 0;

        for layer in &self.layers {
            for neuron in layer {
                total += neuron.get_spike_rate();
                count += 1;
            }
        }

        if count > 0 {
            total / count as f64
        } else {
            0.0
        }
    }

    /// Получить силу связи между входом и выходом
    pub fn get_connection_strength(&self, input_idx: usize, output_idx: usize) -> f64 {
        if let Some(first_layer) = self.layers.first() {
            if output_idx < first_layer.len() && input_idx < first_layer[output_idx].weights.len() {
                return first_layer[output_idx].weights[input_idx];
            }
        }
        0.0
    }
}
