use rand::Rng;
use std::collections::VecDeque;
use crate::brainwaves::{BrainwaveSpectrum, BrainwaveType, ConsciousnessState};

/// Нейрон, модулируемый мозговыми волнами
#[derive(Clone, Debug)]
pub struct WaveModulatedNeuron {
    pub weights: Vec<f64>,
    pub bias: f64,
    pub membrane_potential: f64,
    pub threshold: f64,
    pub spike_history: VecDeque<bool>,
    pub wave_sensitivity: [f64; 5], // Чувствительность к каждому типу волн
    pub current_modulation: f64,
}

impl WaveModulatedNeuron {
    pub fn new(input_size: usize) -> Self {
        let mut rng = rand::thread_rng();

        // Каждый нейрон имеет разную чувствительность к волнам
        let wave_sensitivity = [
            rng.gen_range(0.0..1.0), // Delta
            rng.gen_range(0.0..1.0), // Theta
            rng.gen_range(0.0..1.0), // Alpha
            rng.gen_range(0.0..1.0), // Beta
            rng.gen_range(0.0..1.0), // Gamma
        ];

        Self {
            weights: (0..input_size).map(|_| rng.gen_range(-1.0..1.0)).collect(),
            bias: rng.gen_range(-0.5..0.5),
            membrane_potential: 0.0,
            threshold: 1.0,
            spike_history: VecDeque::with_capacity(50),
            wave_sensitivity,
            current_modulation: 1.0,
        }
    }

    /// Обновить модуляцию на основе текущего спектра волн
    pub fn update_modulation(&mut self, spectrum: &BrainwaveSpectrum) {
        let powers = [
            spectrum.delta_power,
            spectrum.theta_power,
            spectrum.alpha_power,
            spectrum.beta_power,
            spectrum.gamma_power,
        ];

        // Модуляция = взвешенная сумма волн по чувствительности нейрона
        self.current_modulation = powers
            .iter()
            .zip(self.wave_sensitivity.iter())
            .map(|(power, sensitivity)| power * sensitivity)
            .sum::<f64>() / 5.0;

        // Нормализуем в диапазон [0.5, 1.5] чтобы не убить активность
        self.current_modulation = 0.5 + self.current_modulation;
    }

    /// Обработка с учетом модуляции волнами
    pub fn process(&mut self, input_spikes: &[bool]) -> bool {
        // Утечка
        self.membrane_potential *= 0.85;

        // Интеграция входов с модуляцией
        for (i, &spike) in input_spikes.iter().enumerate() {
            if spike && i < self.weights.len() {
                self.membrane_potential += self.weights[i] * self.current_modulation;
            }
        }

        self.membrane_potential += self.bias * self.current_modulation;

        // Порог также модулируется
        let effective_threshold = self.threshold / self.current_modulation;
        let fired = self.membrane_potential >= effective_threshold;

        if fired {
            self.membrane_potential = 0.0;
        }

        self.spike_history.push_back(fired);
        if self.spike_history.len() > 50 {
            self.spike_history.pop_front();
        }

        fired
    }

    pub fn get_spike_rate(&self) -> f64 {
        if self.spike_history.is_empty() {
            return 0.0;
        }
        self.spike_history.iter().filter(|&&s| s).count() as f64 / self.spike_history.len() as f64
    }

    /// Определить к какой волне нейрон наиболее чувствителен
    pub fn dominant_wave_sensitivity(&self) -> BrainwaveType {
        let max_idx = self.wave_sensitivity
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(idx, _)| idx)
            .unwrap_or(0);

        BrainwaveType::all()[max_idx]
    }
}

/// Нейросеть с модуляцией мозговыми волнами
pub struct BrainwaveNetwork {
    pub layers: Vec<Vec<WaveModulatedNeuron>>,
    pub spectrum: BrainwaveSpectrum,
    pub current_state: ConsciousnessState,
    pub time: f64,
    pub thought_log: Vec<String>,
}

impl BrainwaveNetwork {
    pub fn new(layer_sizes: &[usize]) -> Self {
        let mut layers = Vec::new();

        for i in 1..layer_sizes.len() {
            let layer: Vec<WaveModulatedNeuron> = (0..layer_sizes[i])
                .map(|_| WaveModulatedNeuron::new(layer_sizes[i - 1]))
                .collect();
            layers.push(layer);
        }

        let mut spectrum = BrainwaveSpectrum::new();
        spectrum.set_consciousness_state(ConsciousnessState::Awake);

        Self {
            layers,
            spectrum,
            current_state: ConsciousnessState::Awake,
            time: 0.0,
            thought_log: Vec::new(),
        }
    }

    /// Изменить состояние сознания
    pub fn set_consciousness_state(&mut self, state: ConsciousnessState) {
        self.current_state = state;
        self.spectrum.set_consciousness_state(state);

        let thought = format!(
            "🔄 Переход в состояние: {:?} - {}",
            state,
            state.description()
        );
        self.thought_log.push(thought);
    }

    /// Обработка с учетом волн
    pub fn forward(&mut self, inputs: &[f64], dt: f64) -> Vec<bool> {
        self.time += dt;

        // Генерируем волновой сигнал
        let _wave_signal = self.spectrum.generate_signal(dt);

        // Обновляем модуляцию всех нейронов
        for layer in &mut self.layers {
            for neuron in layer {
                neuron.update_modulation(&self.spectrum);
            }
        }

        // Конвертируем входы в спайки
        let mut current_spikes: Vec<bool> = inputs.iter().map(|&x| x > 0.5).collect();

        // Прямое распространение
        for layer in &mut self.layers {
            current_spikes = layer
                .iter_mut()
                .map(|neuron| neuron.process(&current_spikes))
                .collect();
        }

        current_spikes
    }

    /// Анализ влияния волн на активность
    pub fn analyze_wave_influence(&self) {
        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║        АНАЛИЗ ВЛИЯНИЯ МОЗГОВЫХ ВОЛН                    ║");
        println!("╚════════════════════════════════════════════════════════╝\n");

        println!("🧠 Текущее состояние: {:?}", self.current_state);
        println!("   {}", self.current_state.description());
        println!();

        self.spectrum.print_spectrum();

        println!("\n📊 Влияние на нейроны:");

        for (layer_idx, layer) in self.layers.iter().enumerate() {
            let avg_modulation = layer.iter()
                .map(|n| n.current_modulation)
                .sum::<f64>() / layer.len() as f64;

            let avg_spike_rate = layer.iter()
                .map(|n| n.get_spike_rate())
                .sum::<f64>() / layer.len() as f64;

            println!("   Слой {}: модуляция={:.3}, спайк-рейт={:.3}",
                     layer_idx + 1, avg_modulation, avg_spike_rate);

            // Показываем распределение чувствительности к волнам
            if layer_idx == 0 {
                let mut wave_counts = [0; 5];
                for neuron in layer {
                    let dominant = neuron.dominant_wave_sensitivity();
                    let idx = BrainwaveType::all().iter()
                        .position(|&w| w == dominant)
                        .unwrap_or(0);
                    wave_counts[idx] += 1;
                }

                println!("   └─ Чувствительность нейронов:");
                for (wave, count) in BrainwaveType::all().iter().zip(wave_counts.iter()) {
                    if *count > 0 {
                        println!("      {} {:?}: {} нейронов", wave.emoji(), wave, count);
                    }
                }
            }
        }

        println!("\n💭 Последние мысли:");
        for (i, thought) in self.thought_log.iter().rev().take(3).enumerate() {
            println!("   {}. {}", i + 1, thought);
        }
    }

    /// Симуляция перехода между состояниями
    pub fn transition_to(&mut self, target_state: ConsciousnessState, steps: usize, dt: f64) {
        println!("\n🌊 Плавный переход: {:?} → {:?}", self.current_state, target_state);

        let start_powers = self.current_state.get_wave_powers();
        let end_powers = target_state.get_wave_powers();

        for step in 0..=steps {
            let t = step as f64 / steps as f64;

            // Интерполяция между состояниями
            for i in 0..5 {
                let interpolated = start_powers[i] + (end_powers[i] - start_powers[i]) * t;
                match i {
                    0 => self.spectrum.delta_power = interpolated,
                    1 => self.spectrum.theta_power = interpolated,
                    2 => self.spectrum.alpha_power = interpolated,
                    3 => self.spectrum.beta_power = interpolated,
                    4 => self.spectrum.gamma_power = interpolated,
                    _ => {}
                }
            }

            // Обновляем амплитуды осцилляторов
            let powers = [
                self.spectrum.delta_power,
                self.spectrum.theta_power,
                self.spectrum.alpha_power,
                self.spectrum.beta_power,
                self.spectrum.gamma_power,
            ];

            for (osc, &power) in self.spectrum.oscillators.iter_mut().zip(powers.iter()) {
                osc.set_amplitude(power);
            }

            if step % (steps / 4) == 0 {
                let ci = self.spectrum.consciousness_index();
                println!("   Шаг {}/{}: индекс сознания = {:.3}", step, steps, ci);
            }

            self.time += dt;
        }

        self.current_state = target_state;
        println!("   ✅ Переход завершен!\n");
    }

    /// Детектировать текущее состояние по спектру
    pub fn detect_state(&self) -> ConsciousnessState {
        let current = [
            self.spectrum.delta_power,
            self.spectrum.theta_power,
            self.spectrum.alpha_power,
            self.spectrum.beta_power,
            self.spectrum.gamma_power,
        ];

        // Находим наиболее похожее состояние
        ConsciousnessState::all()
            .into_iter()
            .min_by(|a, b| {
                let dist_a = Self::euclidean_distance(&current, &a.get_wave_powers());
                let dist_b = Self::euclidean_distance(&current, &b.get_wave_powers());
                dist_a.partial_cmp(&dist_b).unwrap()
            })
            .unwrap_or(ConsciousnessState::Awake)
    }

    fn euclidean_distance(a: &[f64; 5], b: &[f64; 5]) -> f64 {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f64>()
            .sqrt()
    }
}
