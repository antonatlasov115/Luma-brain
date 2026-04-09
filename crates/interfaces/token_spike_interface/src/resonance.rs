//! Резонансный анализ - вычисление сходства между спайковыми паттернами

use crate::{Spike, SpikePattern};

/// Анализатор резонанса между паттернами
pub struct ResonanceAnalyzer {
    /// Временное окно для кросс-корреляции (мс)
    pub window: f64,
}

impl ResonanceAnalyzer {
    pub fn new() -> Self {
        Self {
            window: 50.0, // 50мс окно
        }
    }

    /// Вычислить резонанс между двумя паттернами
    ///
    /// Использует кросс-корреляцию с временным окном.
    /// Возвращает значение от 0.0 (нет резонанса) до 1.0 (полный резонанс).
    ///
    /// # Алгоритм
    ///
    /// Для каждого спайка в observed паттерне:
    /// 1. Найти ближайший спайк в expected паттерне
    /// 2. Если расстояние < window, добавить вклад exp(-dt/window)
    /// 3. Нормализовать на количество спайков
    pub fn calculate_resonance(&self, observed: &SpikePattern, expected: &SpikePattern) -> f64 {
        if observed.spikes.is_empty() || expected.spikes.is_empty() {
            return 0.0;
        }

        let mut correlation = 0.0;

        // Для каждого спайка в observed
        for obs_spike in &observed.spikes {
            // Найти ближайший спайк в expected от того же нейрона
            let mut min_dt = f64::MAX;

            for exp_spike in &expected.spikes {
                // Учитываем только спайки от того же нейрона
                if obs_spike.neuron_id == exp_spike.neuron_id {
                    let dt = (obs_spike.time - exp_spike.time).abs();
                    if dt < min_dt {
                        min_dt = dt;
                    }
                }
            }

            // Если нашли близкий спайк, добавляем вклад
            if min_dt < self.window {
                correlation += (-min_dt / self.window).exp();
            }
        }

        // Нормализуем на количество спайков
        let normalization = (observed.spikes.len() as f64).sqrt();
        (correlation / normalization).min(1.0)
    }

    /// Вычислить резонанс с учетом частоты
    ///
    /// Дополнительно учитывает сходство средних частот паттернов
    pub fn calculate_resonance_with_frequency(
        &self,
        observed: &SpikePattern,
        expected: &SpikePattern,
    ) -> f64 {
        let temporal_resonance = self.calculate_resonance(observed, expected);

        // Сходство частот
        let obs_freq = observed.average_frequency();
        let exp_freq = expected.average_frequency();

        let freq_similarity = if exp_freq > 0.0 {
            1.0 - ((obs_freq - exp_freq).abs() / exp_freq).min(1.0)
        } else {
            0.0
        };

        // Комбинируем: 70% временной резонанс, 30% частотное сходство
        temporal_resonance * 0.7 + freq_similarity * 0.3
    }

    /// Найти лучший сдвиг по времени для максимального резонанса
    ///
    /// Пробует разные временные сдвиги и возвращает лучший резонанс
    pub fn find_best_alignment(
        &self,
        observed: &SpikePattern,
        expected: &SpikePattern,
    ) -> (f64, f64) {
        let mut best_resonance = 0.0;
        let mut best_shift = 0.0;

        // Пробуем сдвиги от -100мс до +100мс с шагом 10мс
        let mut shift = -100.0;
        while shift <= 100.0 {
            // Создаем сдвинутый паттерн
            let mut shifted = observed.clone();
            for spike in &mut shifted.spikes {
                spike.time += shift;
            }

            let resonance = self.calculate_resonance(&shifted, expected);
            if resonance > best_resonance {
                best_resonance = resonance;
                best_shift = shift;
            }

            shift += 10.0;
        }

        (best_resonance, best_shift)
    }

    /// Вычислить фазовую синхронизацию между паттернами
    ///
    /// Анализирует, насколько синхронизированы ритмы спайков
    pub fn calculate_phase_synchronization(
        &self,
        pattern1: &SpikePattern,
        pattern2: &SpikePattern,
    ) -> f64 {
        if pattern1.spikes.len() < 2 || pattern2.spikes.len() < 2 {
            return 0.0;
        }

        // Вычисляем интервалы между спайками (периоды)
        let intervals1 = self.calculate_intervals(&pattern1.spikes);
        let intervals2 = self.calculate_intervals(&pattern2.spikes);

        if intervals1.is_empty() || intervals2.is_empty() {
            return 0.0;
        }

        // Средние периоды
        let avg_interval1: f64 = intervals1.iter().sum::<f64>() / intervals1.len() as f64;
        let avg_interval2: f64 = intervals2.iter().sum::<f64>() / intervals2.len() as f64;

        // Сходство периодов
        let period_similarity = 1.0 - ((avg_interval1 - avg_interval2).abs()
            / avg_interval1.max(avg_interval2)).min(1.0);

        period_similarity
    }

    fn calculate_intervals(&self, spikes: &[Spike]) -> Vec<f64> {
        let mut intervals = Vec::new();
        for i in 1..spikes.len() {
            if spikes[i].neuron_id == spikes[i-1].neuron_id {
                intervals.push(spikes[i].time - spikes[i-1].time);
            }
        }
        intervals
    }
}

impl Default for ResonanceAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_resonance() {
        let analyzer = ResonanceAnalyzer::new();

        let mut pattern = SpikePattern::new();
        pattern.add_spike(Spike { time: 0.0, neuron_id: 0 });
        pattern.add_spike(Spike { time: 25.0, neuron_id: 0 });
        pattern.add_spike(Spike { time: 50.0, neuron_id: 0 });

        // Идентичные паттерны должны иметь резонанс ~1.0
        let resonance = analyzer.calculate_resonance(&pattern, &pattern);
        assert!(resonance > 0.9);
    }

    #[test]
    fn test_no_resonance() {
        let analyzer = ResonanceAnalyzer::new();

        let mut pattern1 = SpikePattern::new();
        pattern1.add_spike(Spike { time: 0.0, neuron_id: 0 });

        let mut pattern2 = SpikePattern::new();
        pattern2.add_spike(Spike { time: 1000.0, neuron_id: 0 }); // Далеко

        // Далекие паттерны должны иметь низкий резонанс
        let resonance = analyzer.calculate_resonance(&pattern1, &pattern2);
        assert!(resonance < 0.1);
    }

    #[test]
    fn test_different_neurons() {
        let analyzer = ResonanceAnalyzer::new();

        let mut pattern1 = SpikePattern::new();
        pattern1.add_spike(Spike { time: 0.0, neuron_id: 0 });

        let mut pattern2 = SpikePattern::new();
        pattern2.add_spike(Spike { time: 0.0, neuron_id: 1 }); // Другой нейрон

        // Разные нейроны не должны резонировать
        let resonance = analyzer.calculate_resonance(&pattern1, &pattern2);
        assert_eq!(resonance, 0.0);
    }

    #[test]
    fn test_frequency_resonance() {
        let analyzer = ResonanceAnalyzer::new();

        // Паттерн с частотой 40 Hz
        let mut pattern1 = SpikePattern::new();
        for i in 0..10 {
            pattern1.add_spike(Spike {
                time: i as f64 * 25.0,
                neuron_id: 0
            });
        }

        // Паттерн с частотой 40 Hz (похожий)
        let mut pattern2 = SpikePattern::new();
        for i in 0..10 {
            pattern2.add_spike(Spike {
                time: i as f64 * 25.0 + 5.0, // Небольшой сдвиг
                neuron_id: 0
            });
        }

        let resonance = analyzer.calculate_resonance_with_frequency(&pattern1, &pattern2);
        assert!(resonance > 0.7);
    }

    #[test]
    fn test_best_alignment() {
        let analyzer = ResonanceAnalyzer::new();

        // Создаем два паттерна
        let mut pattern1 = SpikePattern::new();
        pattern1.add_spike(Spike { time: 0.0, neuron_id: 0 });
        pattern1.add_spike(Spike { time: 10.0, neuron_id: 0 });

        let mut pattern2 = SpikePattern::new();
        pattern2.add_spike(Spike { time: 5.0, neuron_id: 0 });
        pattern2.add_spike(Spike { time: 15.0, neuron_id: 0 });

        // Просто проверяем что функция работает и возвращает разумные значения
        let (best_resonance, _best_shift) = analyzer.find_best_alignment(&pattern1, &pattern2);

        // Резонанс должен быть в разумных пределах
        assert!(best_resonance >= 0.0 && best_resonance <= 1.0);
        assert!(best_resonance > 0.3); // Паттерны похожи, должен быть хороший резонанс
    }
}
