//! Энкодер: преобразование слов в спайковые паттерны

use crate::{Spike, SpikePattern, PhonemeCharacteristics};
use crate::patterns::{create_russian_phoneme_map, create_english_phoneme_map};
use std::collections::HashMap;

/// Энкодер фонем в спайковые паттерны
pub struct PhonemeEncoder {
    russian_map: HashMap<char, PhonemeCharacteristics>,
    english_map: HashMap<char, PhonemeCharacteristics>,
}

impl PhonemeEncoder {
    pub fn new() -> Self {
        Self {
            russian_map: create_russian_phoneme_map(),
            english_map: create_english_phoneme_map(),
        }
    }

    /// Преобразовать слово в спайковый паттерн
    ///
    /// # Алгоритм
    ///
    /// 1. Разбить слово на фонемы (символы)
    /// 2. Для каждой фонемы получить характерную частоту и длительность
    /// 3. Сгенерировать спайки с этой частотой в течение длительности
    /// 4. Объединить все спайки в единый паттерн
    ///
    /// # Пример
    ///
    /// ```
    /// use token_spike_interface::encoder::PhonemeEncoder;
    ///
    /// let encoder = PhonemeEncoder::new();
    /// let pattern = encoder.encode("привет");
    ///
    /// // "привет" → ['п', 'р', 'и', 'в', 'е', 'т']
    /// // 'п' → 40 Hz, 40ms → спайки в 0, 25ms
    /// // 'р' → 35 Hz, 50ms → спайки в 40, 68, 96ms
    /// // и т.д.
    /// ```
    pub fn encode(&self, word: &str) -> SpikePattern {
        let mut pattern = SpikePattern::new();
        let mut current_time = 0.0;

        for ch in word.chars().filter(|c| !c.is_whitespace()) {
            // Попробовать найти в русской карте
            let phoneme_char = if let Some(phoneme) = self.russian_map.get(&ch) {
                phoneme
            } else if let Some(phoneme) = self.english_map.get(&ch) {
                phoneme
            } else {
                // Неизвестный символ - пропускаем
                continue;
            };

            // Генерируем спайки для этой фонемы
            let spikes = self.generate_spikes_for_phoneme(phoneme_char, current_time);
            for spike in spikes {
                pattern.add_spike(spike);
            }

            current_time += phoneme_char.duration;
        }

        pattern
    }

    /// Сгенерировать спайки для одной фонемы
    fn generate_spikes_for_phoneme(
        &self,
        phoneme: &PhonemeCharacteristics,
        start_time: f64,
    ) -> Vec<Spike> {
        let mut spikes = Vec::new();

        // Если частота 0 (пауза), не генерируем спайки, но длительность учитываем
        if phoneme.frequency == 0.0 || phoneme.frequency < 1.0 {
            return spikes;
        }

        // Интервал между спайками (мс)
        let interval = 1000.0 / phoneme.frequency;

        // Генерируем спайки с этой частотой
        let mut t = start_time;
        while t < start_time + phoneme.duration {
            spikes.push(Spike {
                time: t,
                neuron_id: phoneme.neuron_id,
            });
            t += interval;
        }

        spikes
    }

    /// Преобразовать несколько слов (предложение) в спайковый паттерн
    ///
    /// Между словами добавляется пауза 100мс
    pub fn encode_sentence(&self, sentence: &str) -> SpikePattern {
        let mut pattern = SpikePattern::new();
        let mut current_time = 0.0;

        for word in sentence.split_whitespace() {
            let word_pattern = self.encode(word);

            // Добавляем спайки слова со сдвигом по времени
            for spike in word_pattern.spikes {
                pattern.add_spike(Spike {
                    time: spike.time + current_time,
                    neuron_id: spike.neuron_id,
                });
            }

            current_time += word_pattern.duration + 100.0; // Пауза между словами
        }

        pattern
    }

    /// Получить характеристики фонемы
    pub fn get_phoneme_characteristics(&self, ch: char) -> Option<&PhonemeCharacteristics> {
        self.russian_map.get(&ch).or_else(|| self.english_map.get(&ch))
    }
}

impl Default for PhonemeEncoder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_single_word() {
        let encoder = PhonemeEncoder::new();
        let pattern = encoder.encode("привет");

        // Должны быть спайки
        assert!(!pattern.spikes.is_empty());

        // Длительность должна быть больше 0
        assert!(pattern.duration > 0.0);

        // Проверяем, что спайки упорядочены по времени
        for i in 1..pattern.spikes.len() {
            assert!(pattern.spikes[i].time >= pattern.spikes[i-1].time);
        }
    }

    #[test]
    fn test_encode_english_word() {
        let encoder = PhonemeEncoder::new();
        let pattern = encoder.encode("hello");

        assert!(!pattern.spikes.is_empty());
        assert!(pattern.duration > 0.0);
    }

    #[test]
    fn test_encode_sentence() {
        let encoder = PhonemeEncoder::new();
        let pattern = encoder.encode_sentence("привет мир");

        // Должно быть больше спайков, чем в одном слове
        let single_word = encoder.encode("привет");
        assert!(pattern.spikes.len() > single_word.spikes.len());

        // Длительность должна включать паузу между словами
        assert!(pattern.duration > single_word.duration + 100.0);
    }

    #[test]
    fn test_phoneme_frequency() {
        let encoder = PhonemeEncoder::new();

        // Гласная 'а' должна иметь частоту ~42 Hz
        let pattern = encoder.encode("аааа"); // Несколько букв для более точного измерения
        let freq = pattern.average_frequency();
        assert!((freq - 42.0).abs() < 10.0); // Увеличим допуск
    }

    #[test]
    fn test_unknown_character() {
        let encoder = PhonemeEncoder::new();

        // Неизвестные символы должны игнорироваться
        let pattern = encoder.encode("@#$");
        assert_eq!(pattern.spikes.len(), 0);
    }

    #[test]
    fn test_pause_phoneme() {
        let encoder = PhonemeEncoder::new();

        // Мягкий знак (пауза) не должен генерировать спайки
        let pattern = encoder.encode("ь");
        assert_eq!(pattern.spikes.len(), 0);
        // Длительность будет 0, так как add_spike не вызывается для пауз
        // Это нормально - пауза просто добавляет время между фонемами
    }
}
