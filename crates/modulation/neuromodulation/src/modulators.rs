//! Нейромодуляторы - дофамин, серотонин, кортизол
//!
//! Глобальные химические сигналы, которые модулируют обучение,
//! активность и эмоциональное состояние.

use serde::{Deserialize, Serialize};

/// Нейромодуляторы
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Neuromodulators {
    /// Дофамин (0.5 - 2.0)
    /// Награда → усиление STDP
    pub dopamine: f64,

    /// Серотонин (0.5 - 2.0)
    /// Настроение → порог срабатывания
    pub serotonin: f64,

    /// Кортизол (0.5 - 2.0)
    /// Стресс → ослабление STDP
    pub cortisol: f64,

    /// Ацетилхолин (0.5 - 2.0)
    /// Внимание → усиление сигнала
    pub acetylcholine: f64,

    /// Скорость возврата к базовому уровню
    pub decay_rate: f64,
}

impl Neuromodulators {
    /// Создать нейромодуляторы с базовыми уровнями
    pub fn new() -> Self {
        Self {
            dopamine: 1.0,
            serotonin: 1.0,
            cortisol: 1.0,
            acetylcholine: 1.0,
            decay_rate: 0.01, // 1% возврат к базовому уровню за шаг
        }
    }

    /// Обновить уровни (возврат к базовому)
    pub fn update(&mut self) {
        self.dopamine = self.decay_towards(self.dopamine, 1.0);
        self.serotonin = self.decay_towards(self.serotonin, 1.0);
        self.cortisol = self.decay_towards(self.cortisol, 1.0);
        self.acetylcholine = self.decay_towards(self.acetylcholine, 1.0);
    }

    /// Плавный возврат к целевому значению
    fn decay_towards(&self, current: f64, target: f64) -> f64 {
        current + (target - current) * self.decay_rate
    }

    /// Выброс дофамина (награда)
    pub fn reward(&mut self, amount: f64) {
        self.dopamine = (self.dopamine + amount).clamp(0.5, 2.0);
    }

    /// Выброс кортизола (стресс)
    pub fn stress(&mut self, amount: f64) {
        self.cortisol = (self.cortisol + amount).clamp(0.5, 2.0);
    }

    /// Изменение серотонина (настроение)
    pub fn mood_shift(&mut self, amount: f64) {
        self.serotonin = (self.serotonin + amount).clamp(0.5, 2.0);
    }

    /// Изменение ацетилхолина (внимание)
    pub fn attention_shift(&mut self, amount: f64) {
        self.acetylcholine = (self.acetylcholine + amount).clamp(0.5, 2.0);
    }

    /// Получить модификатор STDP обучения
    ///
    /// Дофамин усиливает, кортизол ослабляет
    pub fn get_learning_modifier(&self) -> f64 {
        let dopamine_effect = self.dopamine;
        let cortisol_effect = 2.0 - self.cortisol; // Инверсия: высокий кортизол = низкое обучение

        (dopamine_effect * cortisol_effect * 0.5).clamp(0.1, 3.0)
    }

    /// Получить модификатор порога срабатывания
    ///
    /// Серотонин повышает порог (успокаивает)
    pub fn get_threshold_modifier(&self) -> f64 {
        self.serotonin
    }

    /// Получить модификатор силы сигнала
    ///
    /// Ацетилхолин усиливает входной сигнал
    pub fn get_signal_modifier(&self) -> f64 {
        self.acetylcholine
    }

    /// Получить общее эмоциональное состояние (-1.0 до 1.0)
    ///
    /// Положительное: высокий дофамин + серотонин, низкий кортизол
    /// Отрицательное: низкий дофамин + серотонин, высокий кортизол
    pub fn get_emotional_valence(&self) -> f64 {
        let positive = (self.dopamine + self.serotonin) / 2.0 - 1.0;
        let negative = self.cortisol - 1.0;

        (positive - negative).clamp(-1.0, 1.0)
    }

    /// Получить уровень возбуждения (0.0 - 1.0)
    pub fn get_arousal(&self) -> f64 {
        let avg = (self.dopamine + self.acetylcholine + self.cortisol) / 3.0;
        ((avg - 1.0) * 2.0).clamp(0.0, 1.0)
    }

    /// Сбросить все уровни к базовым
    pub fn reset(&mut self) {
        self.dopamine = 1.0;
        self.serotonin = 1.0;
        self.cortisol = 1.0;
        self.acetylcholine = 1.0;
    }
}

impl Default for Neuromodulators {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neuromodulators_creation() {
        let nm = Neuromodulators::new();
        assert_eq!(nm.dopamine, 1.0);
        assert_eq!(nm.serotonin, 1.0);
        assert_eq!(nm.cortisol, 1.0);
        assert_eq!(nm.acetylcholine, 1.0);
    }

    #[test]
    fn test_reward() {
        let mut nm = Neuromodulators::new();

        nm.reward(0.5);
        assert_eq!(nm.dopamine, 1.5);

        // Проверяем границы
        nm.reward(1.0);
        assert_eq!(nm.dopamine, 2.0); // Максимум
    }

    #[test]
    fn test_stress() {
        let mut nm = Neuromodulators::new();

        nm.stress(0.3);
        assert_eq!(nm.cortisol, 1.3);

        nm.stress(1.0);
        assert_eq!(nm.cortisol, 2.0); // Максимум
    }

    #[test]
    fn test_mood_shift() {
        let mut nm = Neuromodulators::new();

        nm.mood_shift(0.4);
        assert_eq!(nm.serotonin, 1.4);

        nm.mood_shift(-1.0);
        assert_eq!(nm.serotonin, 0.5); // Минимум
    }

    #[test]
    fn test_learning_modifier() {
        let mut nm = Neuromodulators::new();

        // Базовый уровень
        let base = nm.get_learning_modifier();
        assert!((base - 0.5).abs() < 0.1, "base = {}", base);

        // Высокий дофамин → усиленное обучение
        nm.reward(0.5);
        let boosted = nm.get_learning_modifier();
        assert!(boosted > base);

        // Высокий кортизол → ослабленное обучение
        nm.reset();
        nm.stress(0.5);
        let reduced = nm.get_learning_modifier();
        assert!(reduced < base);
    }

    #[test]
    fn test_threshold_modifier() {
        let mut nm = Neuromodulators::new();

        assert_eq!(nm.get_threshold_modifier(), 1.0);

        nm.mood_shift(0.5);
        assert_eq!(nm.get_threshold_modifier(), 1.5);
    }

    #[test]
    fn test_signal_modifier() {
        let mut nm = Neuromodulators::new();

        assert_eq!(nm.get_signal_modifier(), 1.0);

        nm.attention_shift(0.3);
        assert_eq!(nm.get_signal_modifier(), 1.3);
    }

    #[test]
    fn test_emotional_valence() {
        let mut nm = Neuromodulators::new();

        // Нейтральное состояние
        let neutral = nm.get_emotional_valence();
        assert!(neutral.abs() < 0.1);

        // Позитивное состояние
        nm.reward(0.5);
        nm.mood_shift(0.3);
        let positive = nm.get_emotional_valence();
        assert!(positive > 0.3);

        // Негативное состояние
        nm.reset();
        nm.stress(0.5);
        nm.mood_shift(-0.3);
        let negative = nm.get_emotional_valence();
        assert!(negative < -0.3);
    }

    #[test]
    fn test_arousal() {
        let mut nm = Neuromodulators::new();

        // Базовый уровень возбуждения
        let base = nm.get_arousal();
        assert!(base.abs() < 0.5, "base = {}", base);

        // Высокое возбуждение
        nm.reward(0.5);
        nm.attention_shift(0.5);
        let high = nm.get_arousal();
        assert!(high > 0.5);
    }

    #[test]
    fn test_decay() {
        let mut nm = Neuromodulators::new();
        nm.decay_rate = 0.1; // Ускоряем для теста

        nm.reward(0.5);
        assert_eq!(nm.dopamine, 1.5);

        // Несколько шагов decay
        for _ in 0..50 {
            nm.update();
        }

        // Должен вернуться близко к 1.0
        assert!((nm.dopamine - 1.0).abs() < 0.1, "dopamine = {}", nm.dopamine);
    }

    #[test]
    fn test_reset() {
        let mut nm = Neuromodulators::new();

        nm.reward(0.5);
        nm.stress(0.3);
        nm.mood_shift(-0.2);

        nm.reset();

        assert_eq!(nm.dopamine, 1.0);
        assert_eq!(nm.serotonin, 1.0);
        assert_eq!(nm.cortisol, 1.0);
    }
}
