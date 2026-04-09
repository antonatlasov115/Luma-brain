use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Области мозга
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BrainRegion {
    PrefrontalCortex,  // Префронтальная кора - планирование, решения
    Hippocampus,       // Гиппокамп - память
    Amygdala,          // Амигдала - эмоции, страх
    VTA,               // VTA - дофаминовая система награды
    Thalamus,          // Таламус - релейная станция, интеграция
}

impl BrainRegion {
    pub fn name(&self) -> &str {
        match self {
            BrainRegion::PrefrontalCortex => "Префронтальная кора",
            BrainRegion::Hippocampus => "Гиппокамп",
            BrainRegion::Amygdala => "Амигдала",
            BrainRegion::VTA => "VTA",
            BrainRegion::Thalamus => "Таламус",
        }
    }

    pub fn emoji(&self) -> &str {
        match self {
            BrainRegion::PrefrontalCortex => "🎯",
            BrainRegion::Hippocampus => "📚",
            BrainRegion::Amygdala => "❤️",
            BrainRegion::VTA => "🎁",
            BrainRegion::Thalamus => "🔄",
        }
    }

    pub fn all() -> Vec<BrainRegion> {
        vec![
            BrainRegion::PrefrontalCortex,
            BrainRegion::Hippocampus,
            BrainRegion::Amygdala,
            BrainRegion::VTA,
            BrainRegion::Thalamus,
        ]
    }
}

/// Состояние области мозга
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionState {
    pub activation: f64,                      // Общая активация (0-1)
    pub local_chemistry: HashMap<String, f64>, // Локальные уровни медиаторов
    pub gamma_phase: f64,                     // Фаза гамма-волны (для синхронизации)
    pub theta_phase: f64,                     // Фаза тета-волны
    pub oscillator_power: f64,                // Мощность осцилляций
}

impl RegionState {
    pub fn new() -> Self {
        let mut local_chemistry = HashMap::new();
        local_chemistry.insert("Дофамин".to_string(), 0.5);
        local_chemistry.insert("Серотонин".to_string(), 0.5);
        local_chemistry.insert("Глутамат".to_string(), 0.5);
        local_chemistry.insert("ГАМК".to_string(), 0.5);

        Self {
            activation: 0.5,
            local_chemistry,
            gamma_phase: 0.0,
            theta_phase: 0.0,
            oscillator_power: 0.5,
        }
    }

    /// Обновить фазы волн
    pub fn update_phases(&mut self, dt: f64) {
        // Гамма: ~40 Hz
        self.gamma_phase += 2.0 * std::f64::consts::PI * 40.0 * dt;
        self.gamma_phase %= 2.0 * std::f64::consts::PI;

        // Тета: ~6 Hz
        self.theta_phase += 2.0 * std::f64::consts::PI * 6.0 * dt;
        self.theta_phase %= 2.0 * std::f64::consts::PI;
    }
}

impl Default for RegionState {
    fn default() -> Self {
        Self::new()
    }
}

/// Связь между областями
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub from: BrainRegion,
    pub to: BrainRegion,
    pub strength: f64,           // Сила связи (0-1)
    pub delay: f64,              // Задержка проведения (секунды)
    pub neurotransmitter: String, // Основной медиатор
}

/// Мозг с пространственной структурой
#[derive(Clone, Serialize, Deserialize)]
pub struct SpatialBrain {
    pub regions: HashMap<BrainRegion, RegionState>,
    pub connections: Vec<Connection>,
    pub time: f64,
}

impl SpatialBrain {
    pub fn new() -> Self {
        let mut regions = HashMap::new();

        // Инициализировать все области
        for region in BrainRegion::all() {
            regions.insert(region, RegionState::new());
        }

        // Создать связи между областями
        let connections = vec![
            // PFC ↔ Hippocampus (рабочая память)
            Connection {
                from: BrainRegion::PrefrontalCortex,
                to: BrainRegion::Hippocampus,
                strength: 0.8,
                delay: 0.02,
                neurotransmitter: "Глутамат".to_string(),
            },
            Connection {
                from: BrainRegion::Hippocampus,
                to: BrainRegion::PrefrontalCortex,
                strength: 0.7,
                delay: 0.02,
                neurotransmitter: "Глутамат".to_string(),
            },
            // Amygdala → PFC (эмоциональная регуляция)
            Connection {
                from: BrainRegion::Amygdala,
                to: BrainRegion::PrefrontalCortex,
                strength: 0.6,
                delay: 0.015,
                neurotransmitter: "Глутамат".to_string(),
            },
            // VTA → PFC (дофаминовая модуляция)
            Connection {
                from: BrainRegion::VTA,
                to: BrainRegion::PrefrontalCortex,
                strength: 0.7,
                delay: 0.01,
                neurotransmitter: "Дофамин".to_string(),
            },
            // VTA → Hippocampus (награда и память)
            Connection {
                from: BrainRegion::VTA,
                to: BrainRegion::Hippocampus,
                strength: 0.6,
                delay: 0.015,
                neurotransmitter: "Дофамин".to_string(),
            },
            // Thalamus → все области (релейная станция)
            Connection {
                from: BrainRegion::Thalamus,
                to: BrainRegion::PrefrontalCortex,
                strength: 0.5,
                delay: 0.01,
                neurotransmitter: "Глутамат".to_string(),
            },
            Connection {
                from: BrainRegion::Thalamus,
                to: BrainRegion::Hippocampus,
                strength: 0.5,
                delay: 0.01,
                neurotransmitter: "Глутамат".to_string(),
            },
            Connection {
                from: BrainRegion::Thalamus,
                to: BrainRegion::Amygdala,
                strength: 0.5,
                delay: 0.01,
                neurotransmitter: "Глутамат".to_string(),
            },
        ];

        Self {
            regions,
            connections,
            time: 0.0,
        }
    }

    /// Обновить все области
    pub fn update(&mut self, elapsed: f64, global_chemistry: &HashMap<String, f64>) {
        self.time += elapsed;

        // 1. Обновить фазы волн в каждой области
        for state in self.regions.values_mut() {
            state.update_phases(elapsed);
        }

        // 2. Применить глобальную химию к локальной
        for state in self.regions.values_mut() {
            for (name, global_level) in global_chemistry.iter() {
                if let Some(local_level) = state.local_chemistry.get_mut(name) {
                    // Локальная химия медленно следует за глобальной
                    let diff = global_level - *local_level;
                    *local_level += diff * elapsed * 0.5;
                }
            }
        }

        // 3. Передать сигналы между областями
        self.propagate_signals(elapsed);

        // 4. Синхронизация волн
        self.synchronize_oscillations();
    }

    /// Передача сигналов между областями
    fn propagate_signals(&mut self, _elapsed: f64) {
        let mut activations = HashMap::new();

        // Вычислить новые активации на основе связей
        for connection in &self.connections {
            if let Some(from_state) = self.regions.get(&connection.from) {
                let signal = from_state.activation * connection.strength;
                *activations.entry(connection.to).or_insert(0.0) += signal;
            }
        }

        // Применить новые активации (с затуханием)
        for (region, state) in self.regions.iter_mut() {
            if let Some(incoming) = activations.get(region) {
                state.activation = (state.activation * 0.7 + incoming * 0.3).clamp(0.0, 1.0);
            } else {
                state.activation *= 0.95; // Затухание
            }
        }
    }

    /// Синхронизация волн между областями
    fn synchronize_oscillations(&mut self) {
        // Гамма-синхронизация между PFC и Hippocampus (рабочая память)
        if let (Some(pfc), Some(hipp)) = (
            self.regions.get(&BrainRegion::PrefrontalCortex),
            self.regions.get(&BrainRegion::Hippocampus),
        ) {
            let phase_diff = (pfc.gamma_phase - hipp.gamma_phase).abs();
            let phase_diff_normalized = (phase_diff % (2.0 * std::f64::consts::PI)) / std::f64::consts::PI;

            // Если фазы близки (< 0.3 радиан) → синхронизированы
            if phase_diff_normalized < 0.3 {
                // Усилить обе области
                if let Some(pfc_mut) = self.regions.get_mut(&BrainRegion::PrefrontalCortex) {
                    pfc_mut.oscillator_power = (pfc_mut.oscillator_power * 1.1).min(1.0);
                }
                if let Some(hipp_mut) = self.regions.get_mut(&BrainRegion::Hippocampus) {
                    hipp_mut.oscillator_power = (hipp_mut.oscillator_power * 1.1).min(1.0);
                }
            }
        }
    }

    /// Установить активацию области
    pub fn set_region_activation(&mut self, region: BrainRegion, activation: f64) {
        if let Some(state) = self.regions.get_mut(&region) {
            state.activation = activation.clamp(0.0, 1.0);
        }
    }

    /// Получить активацию области
    pub fn get_region_activation(&self, region: BrainRegion) -> f64 {
        self.regions.get(&region).map(|s| s.activation).unwrap_or(0.0)
    }

    /// Проверить синхронизацию между областями
    pub fn get_synchronization(&self, region_a: BrainRegion, region_b: BrainRegion) -> f64 {
        if let (Some(state_a), Some(state_b)) = (self.regions.get(&region_a), self.regions.get(&region_b)) {
            let phase_diff = (state_a.gamma_phase - state_b.gamma_phase).abs();
            let phase_diff_normalized = (phase_diff % (2.0 * std::f64::consts::PI)) / std::f64::consts::PI;

            // Синхронизация = 1 - нормализованная разница фаз
            1.0 - phase_diff_normalized
        } else {
            0.0
        }
    }

    /// Получить общую связность мозга
    pub fn get_global_connectivity(&self) -> f64 {
        let mut total_sync = 0.0;
        let mut count = 0;

        let regions: Vec<_> = BrainRegion::all();
        for i in 0..regions.len() {
            for j in (i + 1)..regions.len() {
                total_sync += self.get_synchronization(regions[i], regions[j]);
                count += 1;
            }
        }

        if count > 0 {
            total_sync / count as f64
        } else {
            0.0
        }
    }

    /// Получить самую активную область
    pub fn get_most_active_region(&self) -> Option<(BrainRegion, f64)> {
        self.regions
            .iter()
            .max_by(|a, b| a.1.activation.partial_cmp(&b.1.activation).unwrap())
            .map(|(region, state)| (*region, state.activation))
    }
}

impl Default for SpatialBrain {
    fn default() -> Self {
        Self::new()
    }
}
