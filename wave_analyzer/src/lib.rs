use serde::{Deserialize, Serialize};

/// FFT анализатор волн (упрощенный)
#[derive(Clone, Serialize, Deserialize)]
pub struct WaveAnalyzer {
    pub sample_rate: f64,
    pub window_size: usize,
    pub samples: Vec<f64>,
}

impl WaveAnalyzer {
    pub fn new(sample_rate: f64, window_size: usize) -> Self {
        Self {
            sample_rate,
            window_size,
            samples: Vec::with_capacity(window_size),
        }
    }

    /// Добавить сэмпл
    pub fn add_sample(&mut self, value: f64) {
        self.samples.push(value);
        if self.samples.len() > self.window_size {
            self.samples.remove(0);
        }
    }

    /// Упрощенный анализ частот (без настоящего FFT)
    pub fn analyze_frequencies(&self) -> FrequencyBands {
        if self.samples.len() < 10 {
            return FrequencyBands::default();
        }

        // Упрощенный анализ через автокорреляцию
        let mut delta = 0.0;
        let mut theta = 0.0;
        let mut alpha = 0.0;
        let mut beta = 0.0;
        let mut gamma = 0.0;

        // Анализ вариаций для разных частотных диапазонов
        for window in self.samples.windows(10) {
            let variance = self.calculate_variance(window);
            let mean = window.iter().sum::<f64>() / window.len() as f64;

            // Эвристика: медленные изменения = низкие частоты
            if variance < 0.1 {
                delta += mean;
            } else if variance < 0.3 {
                theta += mean;
            } else if variance < 0.5 {
                alpha += mean;
            } else if variance < 0.7 {
                beta += mean;
            } else {
                gamma += mean;
            }
        }

        let total = delta + theta + alpha + beta + gamma;
        if total > 0.0 {
            delta /= total;
            theta /= total;
            alpha /= total;
            beta /= total;
            gamma /= total;
        }

        FrequencyBands {
            delta,
            theta,
            alpha,
            beta,
            gamma,
        }
    }

    fn calculate_variance(&self, data: &[f64]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        let mean = data.iter().sum::<f64>() / data.len() as f64;
        data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64
    }

    /// Детектировать паттерны
    pub fn detect_patterns(&self) -> Vec<String> {
        let mut patterns = Vec::new();

        if self.samples.len() < 20 {
            return patterns;
        }

        // Детекция ритмичности
        let recent: Vec<f64> = self.samples.iter().rev().take(20).copied().collect();
        let variance = self.calculate_variance(&recent);

        if variance < 0.1 {
            patterns.push("Стабильная активность".to_string());
        } else if variance > 0.7 {
            patterns.push("Высокая вариабельность".to_string());
        }

        // Детекция трендов
        let first_half: f64 = recent[..10].iter().sum::<f64>() / 10.0;
        let second_half: f64 = recent[10..].iter().sum::<f64>() / 10.0;

        if second_half > first_half * 1.2 {
            patterns.push("Возрастающая активность".to_string());
        } else if second_half < first_half * 0.8 {
            patterns.push("Снижающаяся активность".to_string());
        }

        patterns
    }

    /// Очистить буфер
    pub fn clear(&mut self) {
        self.samples.clear();
    }
}

/// Частотные диапазоны
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrequencyBands {
    pub delta: f64,
    pub theta: f64,
    pub alpha: f64,
    pub beta: f64,
    pub gamma: f64,
}

impl Default for FrequencyBands {
    fn default() -> Self {
        Self {
            delta: 0.2,
            theta: 0.2,
            alpha: 0.2,
            beta: 0.2,
            gamma: 0.2,
        }
    }
}
