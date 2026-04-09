//! Циркадный ритм - внутренние часы
//!
//! Моделирует 24-часовой цикл бодрствования/сна с пиком активности
//! в середине дня и минимумом ночью.

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Циркадные часы
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircadianClock {
    /// Время суток (0.0 - 24.0 часов)
    pub time_of_day: f64,

    /// Скорость течения времени (1.0 = реальное время)
    pub time_scale: f64,

    /// Время рождения (мс)
    pub birth_time: f64,

    /// Фаза циркадного ритма (0.0 - 2π)
    pub phase: f64,
}

impl CircadianClock {
    /// Создать новые циркадные часы
    ///
    /// # Параметры
    /// - `birth_time`: Время рождения в мс
    /// - `initial_time_of_day`: Начальное время суток (0-24)
    pub fn new(birth_time: f64, initial_time_of_day: f64) -> Self {
        let phase = (initial_time_of_day / 24.0) * 2.0 * PI;

        Self {
            time_of_day: initial_time_of_day,
            time_scale: 1.0,
            birth_time,
            phase,
        }
    }

    /// Обновить время
    pub fn update(&mut self, current_time: f64) {
        // Вычисляем прошедшее время в часах
        let elapsed_ms = current_time - self.birth_time;
        let elapsed_hours = (elapsed_ms / 3600_000.0) * self.time_scale;

        self.time_of_day = elapsed_hours % 24.0;
        self.phase = (self.time_of_day / 24.0) * 2.0 * PI;
    }

    /// Получить уровень возбуждения (arousal) 0.0 - 1.0
    ///
    /// Пик в 14:00, минимум в 3:00
    pub fn get_arousal(&self) -> f64 {
        // Косинусоида со сдвигом: пик в 14:00
        let shifted_phase = self.phase - (14.0 / 24.0) * 2.0 * PI;
        0.5 + 0.5 * shifted_phase.cos()
    }

    /// Получить склонность ко сну (0.0 - 1.0)
    pub fn get_sleep_pressure(&self) -> f64 {
        1.0 - self.get_arousal()
    }

    /// Проверить, ночное ли время (22:00 - 6:00)
    pub fn is_night_time(&self) -> bool {
        self.time_of_day >= 22.0 || self.time_of_day < 6.0
    }

    /// Проверить, дневное ли время (10:00 - 18:00)
    pub fn is_day_time(&self) -> bool {
        self.time_of_day >= 10.0 && self.time_of_day < 18.0
    }

    /// Получить модификатор активности для нейронов
    ///
    /// Днем: 1.0-1.2, ночью: 0.5-0.8
    pub fn get_activity_modifier(&self) -> f64 {
        0.5 + 0.7 * self.get_arousal()
    }

    /// Установить ускорение времени (для тестов)
    pub fn set_time_scale(&mut self, scale: f64) {
        self.time_scale = scale.max(0.1).min(1000.0);
    }

    /// Получить текстовое описание времени суток
    pub fn get_time_description(&self) -> &str {
        let hour = self.time_of_day as u32;
        match hour {
            0..=5 => "Глубокая ночь",
            6..=8 => "Раннее утро",
            9..=11 => "Утро",
            12..=14 => "День",
            15..=17 => "Вечер",
            18..=21 => "Поздний вечер",
            _ => "Ночь",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circadian_creation() {
        let clock = CircadianClock::new(0.0, 12.0);
        assert_eq!(clock.time_of_day, 12.0);
        assert_eq!(clock.birth_time, 0.0);
    }

    #[test]
    fn test_update() {
        let mut clock = CircadianClock::new(0.0, 0.0);

        // 1 час = 3600000 мс
        clock.update(3600_000.0);
        assert!((clock.time_of_day - 1.0).abs() < 0.01);

        // 12 часов
        clock.update(12.0 * 3600_000.0);
        assert!((clock.time_of_day - 12.0).abs() < 0.01);

        // 25 часов → 1 час (цикл)
        clock.update(25.0 * 3600_000.0);
        assert!((clock.time_of_day - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_arousal_peak() {
        let clock = CircadianClock::new(0.0, 14.0);

        // В 14:00 должен быть пик возбуждения
        let arousal = clock.get_arousal();
        assert!(arousal > 0.95, "arousal = {}", arousal);
    }

    #[test]
    fn test_arousal_minimum() {
        let mut clock = CircadianClock::new(0.0, 3.0);
        clock.update(0.0);

        // В 3:00 должен быть минимум возбуждения
        let arousal = clock.get_arousal();
        assert!(arousal < 0.2);
    }

    #[test]
    fn test_sleep_pressure() {
        let mut clock = CircadianClock::new(0.0, 3.0);
        clock.update(0.0);

        let pressure = clock.get_sleep_pressure();
        assert!(pressure > 0.8); // Ночью высокое давление сна
    }

    #[test]
    fn test_night_time() {
        let clock1 = CircadianClock::new(0.0, 23.0);
        assert!(clock1.is_night_time());

        let clock2 = CircadianClock::new(0.0, 3.0);
        assert!(clock2.is_night_time());

        let clock3 = CircadianClock::new(0.0, 12.0);
        assert!(!clock3.is_night_time());
    }

    #[test]
    fn test_day_time() {
        let clock1 = CircadianClock::new(0.0, 14.0);
        assert!(clock1.is_day_time());

        let clock2 = CircadianClock::new(0.0, 3.0);
        assert!(!clock2.is_day_time());
    }

    #[test]
    fn test_activity_modifier() {
        let clock_day = CircadianClock::new(0.0, 14.0);
        let mod_day = clock_day.get_activity_modifier();
        assert!(mod_day > 1.0);

        let clock_night = CircadianClock::new(0.0, 3.0);
        let mod_night = clock_night.get_activity_modifier();
        assert!(mod_night < 0.7);
    }

    #[test]
    fn test_time_scale() {
        let mut clock = CircadianClock::new(0.0, 0.0);
        clock.set_time_scale(10.0); // 10x ускорение

        // 1 час реального времени = 10 часов симуляции
        clock.update(3600_000.0);
        assert!((clock.time_of_day - 10.0).abs() < 0.1);
    }

    #[test]
    fn test_time_description() {
        let clock1 = CircadianClock::new(0.0, 3.0);
        assert_eq!(clock1.get_time_description(), "Глубокая ночь");

        let clock2 = CircadianClock::new(0.0, 12.0);
        assert_eq!(clock2.get_time_description(), "День");

        let clock3 = CircadianClock::new(0.0, 20.0);
        assert_eq!(clock3.get_time_description(), "Поздний вечер");
    }
}
