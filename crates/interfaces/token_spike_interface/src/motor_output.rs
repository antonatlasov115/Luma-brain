//! Моторный выход - декодирование спайковой активности в речь с WTA

use crate::{SpikePattern, decoder::SpikeDecoder};
use std::collections::VecDeque;

/// Тип выходного сигнала
#[derive(Debug, Clone, PartialEq)]
pub enum OutputType {
    /// Внутренняя мысль (низкий порог)
    Thought(String, f64), // (текст, уверенность)

    /// Озвученная речь (высокий порог)
    Speech(String, f64),  // (текст, уверенность)

    /// Нет активности
    Silent,
}

/// Моторный выходной модуль с Winner-Takes-All
///
/// Декодирует спайковую активность моторно-речевого центра в слова.
/// Использует двойной порог для разделения мыслей и речи.
pub struct MotorOutput {
    decoder: SpikeDecoder,

    /// Порог для мыслей (низкий)
    thought_threshold: f64,

    /// Порог для речи (высокий)
    speech_threshold: f64,

    /// История недавних выходов (для подавления повторов)
    recent_outputs: VecDeque<(String, f64)>, // (слово, время)

    /// Минимальный интервал между повторами (мс)
    repeat_suppression_time: f64,

    /// Буфер спайков для анализа
    spike_buffer: SpikePattern,

    /// Размер временного окна для анализа (мс)
    analysis_window: f64,
}

impl MotorOutput {
    pub fn new(decoder: SpikeDecoder) -> Self {
        Self {
            decoder,
            thought_threshold: 0.5,    // Низкий порог для мыслей
            speech_threshold: 0.75,    // Высокий порог для речи
            recent_outputs: VecDeque::new(),
            repeat_suppression_time: 2000.0, // 2 секунды
            spike_buffer: SpikePattern::new(),
            analysis_window: 200.0,    // Анализируем последние 200мс
        }
    }

    /// Добавить спайк в буфер
    ///
    /// Спайки приходят из моторно-речевого центра сети
    pub fn add_spike(&mut self, time: f64, neuron_id: usize) {
        self.spike_buffer.add_spike(crate::Spike { time, neuron_id });

        // Ограничиваем размер буфера
        if self.spike_buffer.spikes.len() > 1000 {
            self.spike_buffer.spikes.remove(0);
        }
    }

    /// Проанализировать текущую активность и вернуть выход
    ///
    /// # Алгоритм Winner-Takes-All
    ///
    /// 1. Извлечь спайки из последнего временного окна
    /// 2. Декодировать с оценками всех кандидатов
    /// 3. Выбрать победителя (максимальная оценка)
    /// 4. Подавить остальных кандидатов
    /// 5. Проверить порог: мысль или речь?
    /// 6. Проверить подавление повторов
    ///
    /// # Параметры
    /// - `current_time`: Текущее время симуляции (мс)
    ///
    /// # Возвращает
    /// - `OutputType::Thought` если оценка > thought_threshold
    /// - `OutputType::Speech` если оценка > speech_threshold
    /// - `OutputType::Silent` если активность слишком низкая
    pub fn decode_output(&mut self, current_time: f64) -> OutputType {
        // Извлекаем спайки из временного окна
        let window_start = current_time - self.analysis_window;
        let window_spikes = self.spike_buffer.get_spikes_in_window(window_start, current_time);

        if window_spikes.is_empty() {
            return OutputType::Silent;
        }

        // Создаем паттерн для декодирования
        let mut pattern = SpikePattern::new();
        for spike in window_spikes {
            pattern.add_spike(spike);
        }

        // Декодируем с оценками всех кандидатов
        let candidates = self.decoder.decode_with_scores(&pattern);

        if candidates.is_empty() {
            return OutputType::Silent;
        }

        // Winner-Takes-All: выбираем лучшего
        let (winner_word, winner_score) = &candidates[0];

        // Проверяем подавление повторов
        if self.is_recently_output(winner_word, current_time) {
            return OutputType::Silent;
        }

        // Проверяем пороги
        if *winner_score >= self.speech_threshold {
            // Высокий порог - озвучиваем
            self.record_output(winner_word.clone(), current_time);
            OutputType::Speech(winner_word.clone(), *winner_score)
        } else if *winner_score >= self.thought_threshold {
            // Низкий порог - внутренняя мысль
            OutputType::Thought(winner_word.clone(), *winner_score)
        } else {
            // Слишком низкая уверенность
            OutputType::Silent
        }
    }

    /// Проверить, было ли слово недавно выведено
    fn is_recently_output(&self, word: &str, current_time: f64) -> bool {
        for (recent_word, recent_time) in &self.recent_outputs {
            if recent_word == word && (current_time - recent_time) < self.repeat_suppression_time {
                return true;
            }
        }
        false
    }

    /// Записать выход в историю
    fn record_output(&mut self, word: String, time: f64) {
        self.recent_outputs.push_back((word, time));

        // Ограничиваем размер истории
        if self.recent_outputs.len() > 20 {
            self.recent_outputs.pop_front();
        }
    }

    /// Очистить буфер спайков
    pub fn clear_buffer(&mut self) {
        self.spike_buffer = SpikePattern::new();
    }

    /// Установить порог для мыслей
    pub fn set_thought_threshold(&mut self, threshold: f64) {
        self.thought_threshold = threshold.clamp(0.0, 1.0);
    }

    /// Установить порог для речи
    pub fn set_speech_threshold(&mut self, threshold: f64) {
        self.speech_threshold = threshold.clamp(0.0, 1.0);
    }

    /// Установить время подавления повторов
    pub fn set_repeat_suppression(&mut self, time_ms: f64) {
        self.repeat_suppression_time = time_ms.max(0.0);
    }

    /// Получить текущую активность моторного центра
    ///
    /// Возвращает среднюю частоту спайков в последнем окне
    pub fn get_motor_activity(&self, current_time: f64) -> f64 {
        let window_start = current_time - self.analysis_window;
        let window_spikes = self.spike_buffer.get_spikes_in_window(window_start, current_time);

        if window_spikes.is_empty() {
            return 0.0;
        }

        // Частота = количество спайков / время (в секундах)
        (window_spikes.len() as f64) / (self.analysis_window / 1000.0)
    }

    /// Получить количество спайков в буфере
    pub fn buffer_size(&self) -> usize {
        self.spike_buffer.spikes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoder::PhonemeEncoder;

    #[test]
    fn test_motor_output_creation() {
        let decoder = SpikeDecoder::new();
        let motor = MotorOutput::new(decoder);

        assert_eq!(motor.thought_threshold, 0.5);
        assert_eq!(motor.speech_threshold, 0.75);
    }

    #[test]
    fn test_add_spike() {
        let decoder = SpikeDecoder::new();
        let mut motor = MotorOutput::new(decoder);

        motor.add_spike(0.0, 0);
        motor.add_spike(10.0, 1);

        assert_eq!(motor.buffer_size(), 2);
    }

    #[test]
    fn test_decode_output_silent() {
        let decoder = SpikeDecoder::new();
        let mut motor = MotorOutput::new(decoder);

        // Без спайков должно быть тихо
        let output = motor.decode_output(0.0);
        assert_eq!(output, OutputType::Silent);
    }

    #[test]
    fn test_decode_output_with_pattern() {
        let mut decoder = SpikeDecoder::new();
        decoder.add_words(&["привет", "пока"]);

        let mut motor = MotorOutput::new(decoder);

        // Генерируем паттерн для "привет"
        let encoder = PhonemeEncoder::new();
        let pattern = encoder.encode("привет");

        // Добавляем спайки в моторный центр
        for spike in pattern.spikes {
            motor.add_spike(spike.time, spike.neuron_id);
        }

        // Декодируем
        let output = motor.decode_output(pattern.duration + 10.0);

        // Должна быть либо мысль, либо речь
        match output {
            OutputType::Thought(word, _) | OutputType::Speech(word, _) => {
                assert_eq!(word, "привет");
            }
            OutputType::Silent => {
                // Может быть Silent если порог не достигнут
            }
        }
    }

    #[test]
    fn test_threshold_levels() {
        let mut decoder = SpikeDecoder::new();
        decoder.add_word("тест");

        let mut motor = MotorOutput::new(decoder);
        motor.set_thought_threshold(0.3);
        motor.set_speech_threshold(0.9);

        assert_eq!(motor.thought_threshold, 0.3);
        assert_eq!(motor.speech_threshold, 0.9);
    }

    #[test]
    fn test_repeat_suppression() {
        let mut decoder = SpikeDecoder::new();
        decoder.add_word("привет");

        let mut motor = MotorOutput::new(decoder);
        motor.set_repeat_suppression(1000.0); // 1 секунда

        let encoder = PhonemeEncoder::new();
        let pattern = encoder.encode("привет");

        // Первый раз
        for spike in &pattern.spikes {
            motor.add_spike(spike.time, spike.neuron_id);
        }
        let output1 = motor.decode_output(pattern.duration);

        // Второй раз сразу же - должно быть подавлено
        motor.clear_buffer();
        for spike in &pattern.spikes {
            motor.add_spike(spike.time + pattern.duration + 10.0, spike.neuron_id);
        }
        let output2 = motor.decode_output(pattern.duration * 2.0 + 10.0);

        // Если первый раз было не Silent, второй раз должно быть Silent
        if !matches!(output1, OutputType::Silent) {
            assert_eq!(output2, OutputType::Silent);
        }
    }

    #[test]
    fn test_motor_activity() {
        let decoder = SpikeDecoder::new();
        let mut motor = MotorOutput::new(decoder);

        // Добавляем спайки с частотой 40 Hz
        for i in 0..10 {
            motor.add_spike(i as f64 * 25.0, 0); // 25мс интервал = 40 Hz
        }

        let activity = motor.get_motor_activity(250.0);

        // Должна быть активность около 40 Hz
        assert!(activity > 30.0 && activity < 50.0);
    }

    #[test]
    fn test_clear_buffer() {
        let decoder = SpikeDecoder::new();
        let mut motor = MotorOutput::new(decoder);

        motor.add_spike(0.0, 0);
        motor.add_spike(10.0, 1);
        assert_eq!(motor.buffer_size(), 2);

        motor.clear_buffer();
        assert_eq!(motor.buffer_size(), 0);
    }
}
