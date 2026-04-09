//! Сенсорный вход - трансляция спайковых паттернов в нейронную сеть

use crate::{Spike, SpikePattern};
use std::collections::VecDeque;

/// Сенсорный входной модуль
///
/// Транслирует спайковые паттерны как внешний ток в сенсорные нейроны.
/// Включает систему затухания для предотвращения перегрузки сети.
pub struct SensoryInput {
    /// Буфер активных паттернов
    active_patterns: VecDeque<ActivePattern>,

    /// Коэффициент затухания (0.0-1.0)
    decay_rate: f64,

    /// Максимальная длительность паттерна (мс)
    max_duration: f64,

    /// Усиление входного сигнала
    input_gain: f64,
}

/// Активный паттерн с временем начала
struct ActivePattern {
    pattern: SpikePattern,
    start_time: f64,
    current_time: f64,
    intensity: f64, // Текущая интенсивность (затухает со временем)
}

impl SensoryInput {
    pub fn new() -> Self {
        Self {
            active_patterns: VecDeque::new(),
            decay_rate: 0.95,      // Затухание 5% за мс
            max_duration: 500.0,   // Максимум 500мс
            input_gain: 1.5,       // Усиление входа
        }
    }

    /// Подать спайковый паттерн в сенсорные нейроны
    ///
    /// # Параметры
    /// - `pattern`: Спайковый паттерн (из энкодера)
    /// - `current_time`: Текущее время симуляции (мс)
    ///
    /// # Пример
    /// ```
    /// use token_spike_interface::sensory_input::SensoryInput;
    /// use token_spike_interface::encoder::PhonemeEncoder;
    ///
    /// let mut sensory = SensoryInput::new();
    /// let encoder = PhonemeEncoder::new();
    ///
    /// // Пользователь написал "привет"
    /// let pattern = encoder.encode("привет");
    /// sensory.inject_pattern(pattern, 0.0);
    ///
    /// // Получаем внешний ток для нейронов
    /// let external_current = sensory.get_external_current(10.0);
    /// ```
    pub fn inject_pattern(&mut self, pattern: SpikePattern, current_time: f64) {
        let active = ActivePattern {
            pattern,
            start_time: current_time,
            current_time,
            intensity: 1.0,
        };

        self.active_patterns.push_back(active);

        // Ограничиваем количество одновременных паттернов
        if self.active_patterns.len() > 5 {
            self.active_patterns.pop_front();
        }
    }

    /// Получить внешний ток для каждого нейрона
    ///
    /// Возвращает HashMap: neuron_id -> external_current
    ///
    /// # Алгоритм
    /// 1. Для каждого активного паттерна
    /// 2. Найти спайки в текущем временном окне
    /// 3. Применить затухание: intensity *= decay_rate^(elapsed_time)
    /// 4. Суммировать токи от всех паттернов
    pub fn get_external_current(&mut self, current_time: f64) -> std::collections::HashMap<usize, f64> {
        let mut currents = std::collections::HashMap::new();

        // Обновляем активные паттерны
        self.active_patterns.retain_mut(|active| {
            let elapsed = current_time - active.start_time;

            // Удаляем паттерны, которые слишком долго активны
            if elapsed > self.max_duration {
                return false;
            }

            // Применяем затухание
            active.intensity *= self.decay_rate.powf(current_time - active.current_time);
            active.current_time = current_time;

            // Если интенсивность слишком мала, удаляем
            if active.intensity < 0.01 {
                return false;
            }

            // Находим спайки в текущем временном окне (±5мс)
            let window_start = elapsed - 5.0;
            let window_end = elapsed + 5.0;

            for spike in &active.pattern.spikes {
                if spike.time >= window_start && spike.time < window_end {
                    // Добавляем ток к этому нейрону
                    let current = active.intensity * self.input_gain;
                    *currents.entry(spike.neuron_id).or_insert(0.0) += current;
                }
            }

            true
        });

        currents
    }

    /// Получить количество активных паттернов
    pub fn active_pattern_count(&self) -> usize {
        self.active_patterns.len()
    }

    /// Очистить все активные паттерны
    pub fn clear(&mut self) {
        self.active_patterns.clear();
    }

    /// Установить коэффициент затухания
    pub fn set_decay_rate(&mut self, rate: f64) {
        self.decay_rate = rate.clamp(0.0, 1.0);
    }

    /// Установить усиление входа
    pub fn set_input_gain(&mut self, gain: f64) {
        self.input_gain = gain.max(0.0);
    }

    /// Получить общую нагрузку на сенсорную систему
    ///
    /// Возвращает значение 0.0-1.0, где 1.0 = перегрузка
    pub fn get_sensory_load(&self) -> f64 {
        let total_intensity: f64 = self.active_patterns
            .iter()
            .map(|p| p.intensity)
            .sum();

        (total_intensity / 3.0).min(1.0) // 3+ паттерна = перегрузка
    }
}

impl Default for SensoryInput {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Spike;

    #[test]
    fn test_inject_pattern() {
        let mut sensory = SensoryInput::new();

        let mut pattern = SpikePattern::new();
        pattern.add_spike(Spike { time: 0.0, neuron_id: 0 });
        pattern.add_spike(Spike { time: 10.0, neuron_id: 1 });

        sensory.inject_pattern(pattern, 0.0);
        assert_eq!(sensory.active_pattern_count(), 1);
    }

    #[test]
    fn test_external_current() {
        let mut sensory = SensoryInput::new();

        let mut pattern = SpikePattern::new();
        pattern.add_spike(Spike { time: 0.0, neuron_id: 0 });

        sensory.inject_pattern(pattern, 0.0);

        // В момент времени 0, должен быть ток к нейрону 0
        let currents = sensory.get_external_current(0.0);
        assert!(currents.contains_key(&0));
        assert!(currents[&0] > 0.0);
    }

    #[test]
    fn test_decay() {
        let mut sensory = SensoryInput::new();
        sensory.set_decay_rate(0.9); // Быстрое затухание для теста

        let mut pattern = SpikePattern::new();
        pattern.add_spike(Spike { time: 0.0, neuron_id: 0 });

        sensory.inject_pattern(pattern, 0.0);

        let current_t0 = sensory.get_external_current(0.0);
        let current_t100 = sensory.get_external_current(100.0);

        // Ток должен уменьшиться со временем
        if let (Some(&i0), Some(&i100)) = (current_t0.get(&0), current_t100.get(&0)) {
            assert!(i100 < i0);
        }
    }

    #[test]
    fn test_max_duration() {
        let mut sensory = SensoryInput::new();

        let mut pattern = SpikePattern::new();
        pattern.add_spike(Spike { time: 0.0, neuron_id: 0 });

        sensory.inject_pattern(pattern, 0.0);
        assert_eq!(sensory.active_pattern_count(), 1);

        // После max_duration паттерн должен удалиться
        sensory.get_external_current(600.0);
        assert_eq!(sensory.active_pattern_count(), 0);
    }

    #[test]
    fn test_sensory_load() {
        let mut sensory = SensoryInput::new();

        assert_eq!(sensory.get_sensory_load(), 0.0);

        // Добавляем несколько паттернов
        for _ in 0..3 {
            let mut pattern = SpikePattern::new();
            pattern.add_spike(Spike { time: 0.0, neuron_id: 0 });
            sensory.inject_pattern(pattern, 0.0);
        }

        let load = sensory.get_sensory_load();
        assert!(load > 0.5); // Должна быть высокая нагрузка
    }

    #[test]
    fn test_multiple_patterns() {
        let mut sensory = SensoryInput::new();

        // Два паттерна на разных нейронах
        let mut pattern1 = SpikePattern::new();
        pattern1.add_spike(Spike { time: 0.0, neuron_id: 0 });

        let mut pattern2 = SpikePattern::new();
        pattern2.add_spike(Spike { time: 0.0, neuron_id: 1 });

        sensory.inject_pattern(pattern1, 0.0);
        sensory.inject_pattern(pattern2, 0.0);

        let currents = sensory.get_external_current(0.0);

        // Должны быть токи к обоим нейронам
        assert!(currents.contains_key(&0));
        assert!(currents.contains_key(&1));
    }
}
