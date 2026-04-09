//! Аденозин - счетчик усталости
//!
//! Моделирует накопление аденозина в мозге, который повышает порог
//! срабатывания нейронов и вызывает потребность во сне.

use serde::{Deserialize, Serialize};

/// Счетчик аденозина
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdenosineClock {
    /// Общее количество спайков с момента последнего сна
    pub spike_counter: u64,

    /// Сдвиг порога срабатывания (0.0 - 0.5)
    pub threshold_shift: f64,

    /// Скорость накопления (спайков на единицу аденозина)
    pub accumulation_rate: u64,

    /// Максимальный сдвиг порога
    pub max_threshold_shift: f64,

    /// Время последнего сна (мс)
    pub last_sleep_time: f64,
}

impl AdenosineClock {
    /// Создать новый счетчик аденозина
    pub fn new() -> Self {
        Self {
            spike_counter: 0,
            threshold_shift: 0.0,
            accumulation_rate: 10_000, // Каждые 10k спайков
            max_threshold_shift: 0.5,
            last_sleep_time: 0.0,
        }
    }

    /// Накопить аденозин от спайков
    pub fn accumulate(&mut self, spike_count: usize) {
        self.spike_counter += spike_count as u64;

        // Каждые accumulation_rate спайков → +0.01 к порогу
        let units = (self.spike_counter / self.accumulation_rate) as f64;
        self.threshold_shift = (units * 0.01).min(self.max_threshold_shift);
    }

    /// Получить текущий уровень усталости (0.0 - 1.0)
    pub fn get_fatigue_level(&self) -> f64 {
        self.threshold_shift / self.max_threshold_shift
    }

    /// Проверить, нужен ли сон
    pub fn needs_sleep(&self) -> bool {
        self.get_fatigue_level() > 0.6 // 60% усталости
    }

    /// Сбросить счетчик (после сна)
    pub fn reset(&mut self, current_time: f64) {
        self.spike_counter = 0;
        self.threshold_shift = 0.0;
        self.last_sleep_time = current_time;
    }

    /// Получить время бодрствования (мс)
    pub fn get_awake_time(&self, current_time: f64) -> f64 {
        current_time - self.last_sleep_time
    }

    /// Получить модификатор порога для нейронов
    pub fn get_threshold_modifier(&self) -> f64 {
        1.0 + self.threshold_shift
    }
}

impl Default for AdenosineClock {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adenosine_creation() {
        let clock = AdenosineClock::new();
        assert_eq!(clock.spike_counter, 0);
        assert_eq!(clock.threshold_shift, 0.0);
        assert_eq!(clock.get_fatigue_level(), 0.0);
    }

    #[test]
    fn test_accumulation() {
        let mut clock = AdenosineClock::new();

        // 10k спайков → +0.01 к порогу
        clock.accumulate(10_000);
        assert_eq!(clock.threshold_shift, 0.01);
        assert_eq!(clock.get_fatigue_level(), 0.02);

        // Еще 10k → +0.02
        clock.accumulate(10_000);
        assert_eq!(clock.threshold_shift, 0.02);
    }

    #[test]
    fn test_max_threshold() {
        let mut clock = AdenosineClock::new();

        // Накапливаем много спайков
        clock.accumulate(1_000_000);

        // Порог не должен превышать max
        assert!(clock.threshold_shift <= clock.max_threshold_shift);
        assert_eq!(clock.get_fatigue_level(), 1.0);
    }

    #[test]
    fn test_needs_sleep() {
        let mut clock = AdenosineClock::new();

        assert!(!clock.needs_sleep());

        // 310k спайков → 62% усталости
        clock.accumulate(310_000);
        assert!(clock.needs_sleep());
    }

    #[test]
    fn test_reset() {
        let mut clock = AdenosineClock::new();

        clock.accumulate(100_000);
        assert!(clock.threshold_shift > 0.0);

        clock.reset(1000.0);
        assert_eq!(clock.spike_counter, 0);
        assert_eq!(clock.threshold_shift, 0.0);
        assert_eq!(clock.last_sleep_time, 1000.0);
    }

    #[test]
    fn test_threshold_modifier() {
        let mut clock = AdenosineClock::new();

        assert_eq!(clock.get_threshold_modifier(), 1.0);

        clock.accumulate(50_000);
        assert!(clock.get_threshold_modifier() > 1.0);
        assert!(clock.get_threshold_modifier() < 1.5);
    }

    #[test]
    fn test_awake_time() {
        let mut clock = AdenosineClock::new();
        clock.reset(0.0);

        let awake = clock.get_awake_time(5000.0);
        assert_eq!(awake, 5000.0);
    }
}
