/// Типы мозговых волн с их частотными характеристиками
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BrainwaveType {
    /// Дельта (0.5-4 Hz) - глубокий сон, бессознательное
    Delta,
    /// Тета (4-8 Hz) - медитация, творчество, REM-сон
    Theta,
    /// Альфа (8-13 Hz) - расслабление, спокойное бодрствование
    Alpha,
    /// Бета (13-30 Hz) - активное мышление, концентрация
    Beta,
    /// Гамма (30-100 Hz) - высшие когнитивные функции, сознание
    Gamma,
}

impl BrainwaveType {
    /// Получить частотный диапазон в Hz
    pub fn frequency_range(&self) -> (f64, f64) {
        match self {
            BrainwaveType::Delta => (0.5, 4.0),
            BrainwaveType::Theta => (4.0, 8.0),
            BrainwaveType::Alpha => (8.0, 13.0),
            BrainwaveType::Beta => (13.0, 30.0),
            BrainwaveType::Gamma => (30.0, 100.0),
        }
    }

    /// Получить центральную частоту
    pub fn center_frequency(&self) -> f64 {
        let (min, max) = self.frequency_range();
        (min + max) / 2.0
    }

    /// Описание состояния сознания
    pub fn description(&self) -> &str {
        match self {
            BrainwaveType::Delta => "Глубокий сон, восстановление, бессознательное",
            BrainwaveType::Theta => "Медитация, творчество, интуиция, REM-сон",
            BrainwaveType::Alpha => "Расслабленное бодрствование, спокойствие",
            BrainwaveType::Beta => "Активное мышление, концентрация, решение задач",
            BrainwaveType::Gamma => "Высшее сознание, инсайты, интеграция информации",
        }
    }

    /// Эмодзи для визуализации
    pub fn emoji(&self) -> &str {
        match self {
            BrainwaveType::Delta => "😴",
            BrainwaveType::Theta => "🧘",
            BrainwaveType::Alpha => "😌",
            BrainwaveType::Beta => "🤔",
            BrainwaveType::Gamma => "💡",
        }
    }

    /// Все типы волн
    pub fn all() -> Vec<BrainwaveType> {
        vec![
            BrainwaveType::Delta,
            BrainwaveType::Theta,
            BrainwaveType::Alpha,
            BrainwaveType::Beta,
            BrainwaveType::Gamma,
        ]
    }
}

/// Генератор мозговых волн
#[derive(Debug, Clone)]
pub struct BrainwaveOscillator {
    wave_type: BrainwaveType,
    frequency: f64,
    phase: f64,
    amplitude: f64,
    time: f64,
}

impl BrainwaveOscillator {
    pub fn new(wave_type: BrainwaveType) -> Self {
        Self {
            wave_type,
            frequency: wave_type.center_frequency(),
            phase: 0.0,
            amplitude: 1.0,
            time: 0.0,
        }
    }

    /// Генерировать следующее значение волны
    pub fn tick(&mut self, dt: f64) -> f64 {
        self.time += dt;
        let value = self.amplitude * (2.0 * std::f64::consts::PI * self.frequency * self.time + self.phase).sin();
        value
    }

    /// Установить амплитуду (мощность волны)
    pub fn set_amplitude(&mut self, amplitude: f64) {
        self.amplitude = amplitude.max(0.0).min(1.0);
    }

    /// Сбросить время
    pub fn reset(&mut self) {
        self.time = 0.0;
    }

    pub fn wave_type(&self) -> BrainwaveType {
        self.wave_type
    }

    pub fn frequency(&self) -> f64 {
        self.frequency
    }

    pub fn amplitude(&self) -> f64 {
        self.amplitude
    }
}

/// Спектр мозговых волн - комбинация всех частот
#[derive(Debug, Clone)]
pub struct BrainwaveSpectrum {
    pub oscillators: Vec<BrainwaveOscillator>,
    pub delta_power: f64,
    pub theta_power: f64,
    pub alpha_power: f64,
    pub beta_power: f64,
    pub gamma_power: f64,
}

impl BrainwaveSpectrum {
    pub fn new() -> Self {
        let oscillators = BrainwaveType::all()
            .into_iter()
            .map(BrainwaveOscillator::new)
            .collect();

        Self {
            oscillators,
            delta_power: 0.0,
            theta_power: 0.0,
            alpha_power: 0.0,
            beta_power: 0.0,
            gamma_power: 0.0,
        }
    }

    /// Установить мощности волн для определенного состояния сознания
    pub fn set_consciousness_state(&mut self, state: ConsciousnessState) {
        let powers = state.get_wave_powers();
        self.delta_power = powers[0];
        self.theta_power = powers[1];
        self.alpha_power = powers[2];
        self.beta_power = powers[3];
        self.gamma_power = powers[4];

        self.update_amplitudes();
    }

    fn update_amplitudes(&mut self) {
        let powers = [
            self.delta_power,
            self.theta_power,
            self.alpha_power,
            self.beta_power,
            self.gamma_power,
        ];

        for (osc, &power) in self.oscillators.iter_mut().zip(powers.iter()) {
            osc.set_amplitude(power);
        }
    }

    /// Генерировать комбинированный сигнал всех волн
    pub fn generate_signal(&mut self, dt: f64) -> f64 {
        self.oscillators
            .iter_mut()
            .map(|osc| osc.tick(dt))
            .sum::<f64>() / self.oscillators.len() as f64
    }

    /// Получить текущую доминирующую волну
    pub fn dominant_wave(&self) -> BrainwaveType {
        let powers = [
            (BrainwaveType::Delta, self.delta_power),
            (BrainwaveType::Theta, self.theta_power),
            (BrainwaveType::Alpha, self.alpha_power),
            (BrainwaveType::Beta, self.beta_power),
            (BrainwaveType::Gamma, self.gamma_power),
        ];

        powers
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(wave, _)| *wave)
            .unwrap_or(BrainwaveType::Alpha)
    }

    /// Вычислить индекс сознания на основе спектра
    pub fn consciousness_index(&self) -> f64 {
        // Высокое сознание = больше гамма и бета, меньше дельта
        let awareness = (self.gamma_power * 2.0 + self.beta_power + self.alpha_power * 0.5) / 3.5;
        let drowsiness = (self.delta_power + self.theta_power * 0.5) / 1.5;

        (awareness - drowsiness * 0.5).max(0.0).min(1.0)
    }

    pub fn print_spectrum(&self) {
        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║           СПЕКТР МОЗГОВЫХ ВОЛН                         ║");
        println!("╚════════════════════════════════════════════════════════╝\n");

        let waves = [
            (BrainwaveType::Delta, self.delta_power),
            (BrainwaveType::Theta, self.theta_power),
            (BrainwaveType::Alpha, self.alpha_power),
            (BrainwaveType::Beta, self.beta_power),
            (BrainwaveType::Gamma, self.gamma_power),
        ];

        for (wave_type, power) in waves {
            let bar = self.create_bar(power);
            let (min, max) = wave_type.frequency_range();
            println!(
                "{} {:6} ({:5.1}-{:5.1} Hz) │ {:.2} {} │",
                wave_type.emoji(),
                format!("{:?}", wave_type),
                min,
                max,
                power,
                bar
            );
        }

        println!("\n🧠 Доминирующая волна: {} {:?}",
                 self.dominant_wave().emoji(),
                 self.dominant_wave());
        println!("💫 Индекс сознания: {:.3}", self.consciousness_index());
    }

    fn create_bar(&self, value: f64) -> String {
        let filled = (value * 20.0) as usize;
        let empty = 20 - filled;
        format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
    }
}

/// Состояния сознания с характерными паттернами волн
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConsciousnessState {
    /// Глубокий сон
    DeepSleep,
    /// Легкий сон / дремота
    LightSleep,
    /// Медитация
    Meditation,
    /// Расслабленное бодрствование
    Relaxed,
    /// Нормальное бодрствование
    Awake,
    /// Активная концентрация
    Focused,
    /// Пиковое состояние (flow state)
    FlowState,
    /// Инсайт / озарение
    Insight,
}

impl ConsciousnessState {
    /// Получить характерные мощности волн для состояния
    /// Возвращает [delta, theta, alpha, beta, gamma]
    pub fn get_wave_powers(&self) -> [f64; 5] {
        match self {
            ConsciousnessState::DeepSleep => [0.9, 0.3, 0.1, 0.0, 0.0],
            ConsciousnessState::LightSleep => [0.6, 0.7, 0.2, 0.1, 0.0],
            ConsciousnessState::Meditation => [0.2, 0.8, 0.6, 0.1, 0.3],
            ConsciousnessState::Relaxed => [0.1, 0.3, 0.8, 0.3, 0.1],
            ConsciousnessState::Awake => [0.0, 0.2, 0.5, 0.6, 0.2],
            ConsciousnessState::Focused => [0.0, 0.1, 0.2, 0.9, 0.4],
            ConsciousnessState::FlowState => [0.0, 0.3, 0.4, 0.7, 0.8],
            ConsciousnessState::Insight => [0.0, 0.4, 0.3, 0.5, 1.0],
        }
    }

    pub fn description(&self) -> &str {
        match self {
            ConsciousnessState::DeepSleep => "Глубокий восстановительный сон",
            ConsciousnessState::LightSleep => "Легкий сон, сновидения",
            ConsciousnessState::Meditation => "Глубокая медитация, измененное сознание",
            ConsciousnessState::Relaxed => "Спокойное расслабленное состояние",
            ConsciousnessState::Awake => "Обычное бодрствование",
            ConsciousnessState::Focused => "Интенсивная концентрация на задаче",
            ConsciousnessState::FlowState => "Состояние потока, пиковая производительность",
            ConsciousnessState::Insight => "Момент озарения, инсайт",
        }
    }

    pub fn all() -> Vec<ConsciousnessState> {
        vec![
            ConsciousnessState::DeepSleep,
            ConsciousnessState::LightSleep,
            ConsciousnessState::Meditation,
            ConsciousnessState::Relaxed,
            ConsciousnessState::Awake,
            ConsciousnessState::Focused,
            ConsciousnessState::FlowState,
            ConsciousnessState::Insight,
        ]
    }
}

impl Default for BrainwaveSpectrum {
    fn default() -> Self {
        Self::new()
    }
}
