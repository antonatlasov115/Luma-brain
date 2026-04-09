//! # Neuromodulation - Нейрохимическая регуляция
//!
//! Модуль для управления глобальным состоянием нейронной сети через
//! нейромодуляторы, циркадные ритмы, аденозин и фазы сна.
//!
//! ## Компоненты
//!
//! - **Adenosine**: Счетчик усталости, повышает порог срабатывания
//! - **Circadian**: 24-часовой цикл бодрствования/сна
//! - **Sleep**: Фазы NREM/REM с replay и консолидацией памяти
//! - **Modulators**: Дофамин, серотонин, кортизол, ацетилхолин
//!
//! ## Использование
//!
//! ```rust
//! use neuromodulation::{AdenosineClock, CircadianClock, SleepManager, Neuromodulators};
//!
//! let mut adenosine = AdenosineClock::new();
//! let mut circadian = CircadianClock::new(0.0, 12.0);
//! let mut sleep = SleepManager::new();
//! let mut modulators = Neuromodulators::new();
//!
//! // Накапливаем усталость
//! adenosine.accumulate(10000);
//!
//! // Обновляем время суток
//! circadian.update(current_time);
//!
//! // Проверяем необходимость сна
//! if sleep.should_sleep(activity, adenosine.get_fatigue_level()) {
//!     sleep.start_sleep(current_time);
//! }
//!
//! // Модулируем обучение
//! modulators.reward(0.5);
//! let learning_mod = modulators.get_learning_modifier();
//! ```

pub mod adenosine;
pub mod circadian;
pub mod sleep;
pub mod modulators;

pub use adenosine::AdenosineClock;
pub use circadian::CircadianClock;
pub use sleep::{SleepManager, SleepPhase, MemoryTrace, SleepStats};
pub use modulators::Neuromodulators;

use serde::{Deserialize, Serialize};

/// Глобальное состояние нейромодуляции
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuromodulationState {
    pub adenosine: AdenosineClock,
    pub circadian: CircadianClock,
    pub sleep: SleepManager,
    pub modulators: Neuromodulators,
}

impl NeuromodulationState {
    /// Создать новое состояние
    pub fn new(birth_time: f64, initial_time_of_day: f64) -> Self {
        Self {
            adenosine: AdenosineClock::new(),
            circadian: CircadianClock::new(birth_time, initial_time_of_day),
            sleep: SleepManager::new(),
            modulators: Neuromodulators::new(),
        }
    }

    /// Обновить все компоненты
    pub fn update(&mut self, current_time: f64, spike_count: usize, activity: f64) {
        // Накапливаем аденозин
        self.adenosine.accumulate(spike_count);

        // Обновляем циркадный ритм
        self.circadian.update(current_time);

        // Обновляем фазу сна
        self.sleep.update(current_time);

        // Проверяем необходимость сна
        if self.sleep.should_sleep(activity, self.adenosine.get_fatigue_level()) {
            self.sleep.start_sleep(current_time);

            // После засыпания сбрасываем аденозин
            if self.sleep.phase == SleepPhase::NREM {
                self.adenosine.reset(current_time);
            }
        }

        // Обновляем нейромодуляторы (decay)
        self.modulators.update();
    }

    /// Получить общий модификатор активности
    pub fn get_activity_modifier(&self) -> f64 {
        let circadian_mod = self.circadian.get_activity_modifier();
        let sleep_mod = self.sleep.get_activity_modifier();

        circadian_mod * sleep_mod
    }

    /// Получить общий модификатор обучения
    pub fn get_learning_modifier(&self) -> f64 {
        let sleep_mod = self.sleep.get_learning_modifier();
        let neuro_mod = self.modulators.get_learning_modifier();

        sleep_mod * neuro_mod
    }

    /// Получить модификатор порога срабатывания
    pub fn get_threshold_modifier(&self) -> f64 {
        let adenosine_mod = self.adenosine.get_threshold_modifier();
        let neuro_mod = self.modulators.get_threshold_modifier();

        adenosine_mod * neuro_mod
    }

    /// Получить модификатор входного сигнала
    pub fn get_signal_modifier(&self) -> f64 {
        self.modulators.get_signal_modifier()
    }

    /// Проверить, спит ли сейчас
    pub fn is_sleeping(&self) -> bool {
        self.sleep.is_sleeping()
    }

    /// Получить текстовое описание состояния
    pub fn get_status(&self, current_time: f64) -> String {
        let time_desc = self.circadian.get_time_description();
        let fatigue = self.adenosine.get_fatigue_level();
        let phase = self.sleep.phase;
        let valence = self.modulators.get_emotional_valence();

        format!(
            "{} | {:?} | Усталость: {:.0}% | Настроение: {:.2}",
            time_desc,
            phase,
            fatigue * 100.0,
            valence
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neuromodulation_state_creation() {
        let state = NeuromodulationState::new(0.0, 12.0);

        assert_eq!(state.circadian.time_of_day, 12.0);
        assert_eq!(state.sleep.phase, SleepPhase::Awake);
        assert_eq!(state.adenosine.spike_counter, 0);
    }

    #[test]
    fn test_update() {
        let mut state = NeuromodulationState::new(0.0, 12.0);

        state.update(1000.0, 1000, 50.0);

        assert!(state.adenosine.spike_counter > 0);
    }

    #[test]
    fn test_activity_modifier() {
        let state = NeuromodulationState::new(0.0, 14.0); // День

        let mod_day = state.get_activity_modifier();
        assert!(mod_day > 0.8);
    }

    #[test]
    fn test_learning_modifier() {
        let mut state = NeuromodulationState::new(0.0, 12.0);

        state.modulators.reward(0.5);
        let learning_mod = state.get_learning_modifier();
        assert!(learning_mod > 0.5, "learning_mod = {}", learning_mod);
    }

    #[test]
    fn test_threshold_modifier() {
        let mut state = NeuromodulationState::new(0.0, 12.0);

        // Накапливаем усталость
        state.adenosine.accumulate(100_000);

        let threshold_mod = state.get_threshold_modifier();
        assert!(threshold_mod > 1.0);
    }

    #[test]
    fn test_sleep_trigger() {
        let mut state = NeuromodulationState::new(0.0, 3.0); // Ночь

        // Накапливаем много усталости
        state.adenosine.accumulate(500_000);

        // Низкая активность должна вызвать сон
        state.update(1000.0, 0, 10.0);

        assert!(state.is_sleeping());
    }

    #[test]
    fn test_get_status() {
        let state = NeuromodulationState::new(0.0, 14.0);

        let status = state.get_status(0.0);
        assert!(status.contains("День"));
        assert!(status.contains("Awake"));
    }
}
