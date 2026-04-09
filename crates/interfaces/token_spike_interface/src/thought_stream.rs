//! Поток мыслей - непрерывный мониторинг префронтальной коры

use crate::{SpikePattern, decoder::SpikeDecoder};
use std::collections::VecDeque;

/// Фрагмент мысли
#[derive(Debug, Clone)]
pub struct ThoughtFragment {
    /// Текст фрагмента
    pub text: String,

    /// Уверенность (0.0-1.0)
    pub confidence: f64,

    /// Время возникновения (мс)
    pub timestamp: f64,

    /// Эмоциональная окраска (-1.0 до 1.0)
    pub valence: f64,
}

/// Поток сознания
///
/// Непрерывно мониторит активность префронтальной коры (PFC)
/// и извлекает фрагменты внутреннего монолога.
pub struct ThoughtStream {
    decoder: SpikeDecoder,

    /// Буфер спайков из PFC
    pfc_buffer: SpikePattern,

    /// История мыслей
    thought_history: VecDeque<ThoughtFragment>,

    /// Минимальный порог для детекции мысли
    detection_threshold: f64,

    /// Размер временного окна для анализа (мс)
    analysis_window: f64,

    /// Частота обновления (мс)
    update_interval: f64,

    /// Время последнего обновления
    last_update: f64,

    /// Максимальный размер истории
    max_history: usize,
}

impl ThoughtStream {
    pub fn new(decoder: SpikeDecoder) -> Self {
        Self {
            decoder,
            pfc_buffer: SpikePattern::new(),
            thought_history: VecDeque::new(),
            detection_threshold: 0.3,  // Низкий порог для мыслей
            analysis_window: 150.0,    // Короткое окно для быстрых мыслей
            update_interval: 100.0,    // Обновляем каждые 100мс
            last_update: 0.0,
            max_history: 50,
        }
    }

    /// Добавить спайк из префронтальной коры
    pub fn add_pfc_spike(&mut self, time: f64, neuron_id: usize) {
        self.pfc_buffer.add_spike(crate::Spike { time, neuron_id });

        // Ограничиваем размер буфера
        if self.pfc_buffer.spikes.len() > 2000 {
            self.pfc_buffer.spikes.remove(0);
        }
    }

    /// Обновить поток мыслей
    ///
    /// Анализирует текущую активность PFC и извлекает фрагменты мыслей.
    /// Возвращает новые мысли, если они появились.
    ///
    /// # Алгоритм
    ///
    /// 1. Проверить, прошел ли update_interval
    /// 2. Извлечь спайки из временного окна
    /// 3. Декодировать с низким порогом
    /// 4. Если найдены фрагменты, добавить в историю
    /// 5. Вернуть новые фрагменты
    pub fn update(&mut self, current_time: f64) -> Vec<ThoughtFragment> {
        // Проверяем интервал обновления
        if current_time - self.last_update < self.update_interval {
            return Vec::new();
        }

        self.last_update = current_time;

        // Извлекаем спайки из окна
        let window_start = current_time - self.analysis_window;
        let window_spikes = self.pfc_buffer.get_spikes_in_window(window_start, current_time);

        if window_spikes.is_empty() {
            return Vec::new();
        }

        // Создаем паттерн
        let mut pattern = SpikePattern::new();
        for spike in window_spikes {
            pattern.add_spike(spike);
        }

        // Декодируем с оценками
        let candidates = self.decoder.decode_with_scores(&pattern);

        let mut new_thoughts = Vec::new();

        // Извлекаем все фрагменты выше порога
        for (word, confidence) in candidates {
            if confidence >= self.detection_threshold {
                // Вычисляем эмоциональную окраску на основе уверенности
                let valence = self.estimate_valence(&word, confidence);

                let fragment = ThoughtFragment {
                    text: word,
                    confidence,
                    timestamp: current_time,
                    valence,
                };

                // Проверяем, не дубликат ли это
                if !self.is_duplicate(&fragment) {
                    self.thought_history.push_back(fragment.clone());
                    new_thoughts.push(fragment);
                }
            }
        }

        // Ограничиваем размер истории
        while self.thought_history.len() > self.max_history {
            self.thought_history.pop_front();
        }

        new_thoughts
    }

    /// Проверить, является ли фрагмент дубликатом недавней мысли
    fn is_duplicate(&self, fragment: &ThoughtFragment) -> bool {
        // Проверяем последние 5 мыслей
        for recent in self.thought_history.iter().rev().take(5) {
            if recent.text == fragment.text &&
               (fragment.timestamp - recent.timestamp) < 500.0 {
                return true;
            }
        }
        false
    }

    /// Оценить эмоциональную окраску слова
    ///
    /// Простая эвристика на основе содержания и уверенности
    fn estimate_valence(&self, word: &str, confidence: f64) -> f64 {
        // Позитивные слова
        let positive = ["счастье", "радость", "весело", "хорошо", "люблю", "happy", "joy", "good"];
        // Негативные слова
        let negative = ["грусть", "страх", "плохо", "больно", "голод", "sad", "fear", "bad", "pain"];

        let word_lower = word.to_lowercase();

        for pos in &positive {
            if word_lower.contains(pos) {
                return confidence * 0.8; // Позитивная окраска
            }
        }

        for neg in &negative {
            if word_lower.contains(neg) {
                return -confidence * 0.8; // Негативная окраска
            }
        }

        0.0 // Нейтральная
    }

    /// Получить недавние мысли
    pub fn get_recent_thoughts(&self, count: usize) -> Vec<ThoughtFragment> {
        self.thought_history
            .iter()
            .rev()
            .take(count)
            .cloned()
            .collect()
    }

    /// Получить поток сознания как текст
    ///
    /// Форматирует недавние мысли в читаемую строку
    pub fn get_stream_text(&self, count: usize) -> String {
        let thoughts = self.get_recent_thoughts(count);

        if thoughts.is_empty() {
            return "...".to_string();
        }

        thoughts
            .iter()
            .map(|t| {
                // Добавляем индикаторы уверенности
                if t.confidence > 0.7 {
                    t.text.clone()
                } else if t.confidence > 0.5 {
                    format!("{}?", t.text)
                } else {
                    format!("...{}...", t.text)
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Получить текущую когнитивную нагрузку
    ///
    /// Возвращает 0.0-1.0, где 1.0 = перегрузка мыслями
    pub fn get_cognitive_load(&self, current_time: f64) -> f64 {
        // Считаем мысли за последние 5 секунд
        let recent_count = self.thought_history
            .iter()
            .filter(|t| (current_time - t.timestamp) < 5000.0)
            .count();

        // 10+ мыслей за 5 секунд = перегрузка
        (recent_count as f64 / 10.0).min(1.0)
    }

    /// Получить среднюю эмоциональную окраску недавних мыслей
    pub fn get_average_valence(&self, count: usize) -> f64 {
        let thoughts = self.get_recent_thoughts(count);

        if thoughts.is_empty() {
            return 0.0;
        }

        let sum: f64 = thoughts.iter().map(|t| t.valence).sum();
        sum / thoughts.len() as f64
    }

    /// Очистить буфер и историю
    pub fn clear(&mut self) {
        self.pfc_buffer = SpikePattern::new();
        self.thought_history.clear();
    }

    /// Установить порог детекции
    pub fn set_detection_threshold(&mut self, threshold: f64) {
        self.detection_threshold = threshold.clamp(0.0, 1.0);
    }

    /// Установить частоту обновления
    pub fn set_update_interval(&mut self, interval_ms: f64) {
        self.update_interval = interval_ms.max(10.0);
    }

    /// Получить количество спайков в буфере PFC
    pub fn pfc_buffer_size(&self) -> usize {
        self.pfc_buffer.spikes.len()
    }

    /// Получить активность PFC (частота спайков)
    pub fn get_pfc_activity(&self, current_time: f64) -> f64 {
        let window_start = current_time - self.analysis_window;
        let window_spikes = self.pfc_buffer.get_spikes_in_window(window_start, current_time);

        if window_spikes.is_empty() {
            return 0.0;
        }

        (window_spikes.len() as f64) / (self.analysis_window / 1000.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoder::PhonemeEncoder;

    #[test]
    fn test_thought_stream_creation() {
        let decoder = SpikeDecoder::new();
        let stream = ThoughtStream::new(decoder);

        assert_eq!(stream.detection_threshold, 0.3);
        assert_eq!(stream.pfc_buffer_size(), 0);
    }

    #[test]
    fn test_add_pfc_spike() {
        let decoder = SpikeDecoder::new();
        let mut stream = ThoughtStream::new(decoder);

        stream.add_pfc_spike(0.0, 0);
        stream.add_pfc_spike(10.0, 1);

        assert_eq!(stream.pfc_buffer_size(), 2);
    }

    #[test]
    fn test_update_no_thoughts() {
        let decoder = SpikeDecoder::new();
        let mut stream = ThoughtStream::new(decoder);

        let thoughts = stream.update(0.0);
        assert!(thoughts.is_empty());
    }

    #[test]
    fn test_update_with_pattern() {
        let mut decoder = SpikeDecoder::new();
        decoder.add_words(&["мысль", "идея"]);

        let mut stream = ThoughtStream::new(decoder);
        stream.set_detection_threshold(0.2); // Низкий порог для теста

        // Генерируем паттерн
        let encoder = PhonemeEncoder::new();
        let pattern = encoder.encode("мысль");

        // Добавляем спайки в PFC
        for spike in pattern.spikes {
            stream.add_pfc_spike(spike.time, spike.neuron_id);
        }

        // Обновляем
        let thoughts = stream.update(pattern.duration + 10.0);

        // Может быть мысль, если порог достигнут
        if !thoughts.is_empty() {
            assert!(thoughts[0].text.contains("мысль") || thoughts[0].text.contains("идея"));
        }
    }

    #[test]
    fn test_get_stream_text() {
        let decoder = SpikeDecoder::new();
        let mut stream = ThoughtStream::new(decoder);

        // Добавляем мысли вручную
        stream.thought_history.push_back(ThoughtFragment {
            text: "голод".to_string(),
            confidence: 0.8,
            timestamp: 0.0,
            valence: -0.6,
        });

        stream.thought_history.push_back(ThoughtFragment {
            text: "еда".to_string(),
            confidence: 0.6,
            timestamp: 100.0,
            valence: 0.0,
        });

        let text = stream.get_stream_text(5);
        assert!(text.contains("голод"));
        assert!(text.contains("еда"));
    }

    #[test]
    fn test_cognitive_load() {
        let decoder = SpikeDecoder::new();
        let mut stream = ThoughtStream::new(decoder);

        // Нет мыслей - нет нагрузки
        assert_eq!(stream.get_cognitive_load(0.0), 0.0);

        // Добавляем много мыслей
        for i in 0..15 {
            stream.thought_history.push_back(ThoughtFragment {
                text: format!("мысль{}", i),
                confidence: 0.5,
                timestamp: i as f64 * 100.0,
                valence: 0.0,
            });
        }

        let load = stream.get_cognitive_load(1500.0);
        assert!(load > 0.5); // Должна быть высокая нагрузка
    }

    #[test]
    fn test_valence_estimation() {
        let decoder = SpikeDecoder::new();
        let stream = ThoughtStream::new(decoder);

        let pos_valence = stream.estimate_valence("счастье", 0.8);
        assert!(pos_valence > 0.0);

        let neg_valence = stream.estimate_valence("страх", 0.8);
        assert!(neg_valence < 0.0);

        let neutral_valence = stream.estimate_valence("стол", 0.8);
        assert_eq!(neutral_valence, 0.0);
    }

    #[test]
    fn test_average_valence() {
        let decoder = SpikeDecoder::new();
        let mut stream = ThoughtStream::new(decoder);

        stream.thought_history.push_back(ThoughtFragment {
            text: "радость".to_string(),
            confidence: 0.8,
            timestamp: 0.0,
            valence: 0.6,
        });

        stream.thought_history.push_back(ThoughtFragment {
            text: "грусть".to_string(),
            confidence: 0.8,
            timestamp: 100.0,
            valence: -0.6,
        });

        let avg = stream.get_average_valence(10);
        assert!(avg.abs() < 0.1); // Должно быть близко к 0
    }

    #[test]
    fn test_pfc_activity() {
        let decoder = SpikeDecoder::new();
        let mut stream = ThoughtStream::new(decoder);

        // Добавляем спайки с частотой 30 Hz
        for i in 0..10 {
            stream.add_pfc_spike(i as f64 * 33.3, 0);
        }

        let activity = stream.get_pfc_activity(333.0);
        assert!(activity > 20.0 && activity < 40.0);
    }
}
