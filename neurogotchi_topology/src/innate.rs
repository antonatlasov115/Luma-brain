//! Врожденные связи и инстинкты
//!
//! Hardcoded паттерны, которые обеспечивают базовое выживание
//! и коммуникацию до начала обучения.

use crate::connection::{Connection, ConnectionType};
use serde::{Deserialize, Serialize};

/// Врожденные связи
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnateConnections {
    /// Инстинкт голода (сенсорный паттерн → моторный ответ)
    pub hunger_circuit: Vec<Connection>,

    /// Инстинкт страха (угроза → избегание)
    pub fear_circuit: Vec<Connection>,

    /// Базовые речевые паттерны (крик, плач)
    pub vocalization_circuit: Vec<Connection>,

    /// Циркадный ритм (внутренние часы)
    pub circadian_circuit: Vec<Connection>,
}

impl InnateConnections {
    /// Создать врожденные связи для топологии
    ///
    /// # Инстинкты
    ///
    /// 1. **Голод**: Низкая энергия → активация моторных нейронов "голод", "еда"
    /// 2. **Страх**: Резкий стимул → активация моторных нейронов "страх", "помощь"
    /// 3. **Вокализация**: Дискомфорт → крик (базовая коммуникация)
    /// 4. **Циркадный ритм**: Внутренний таймер → модуляция активности
    pub fn new(
        sensory_neurons: &[usize],
        motor_neurons: &[usize],
    ) -> Self {
        let mut hunger_circuit = Vec::new();
        let mut fear_circuit = Vec::new();
        let mut vocalization_circuit = Vec::new();
        let circadian_circuit = Vec::new();

        // Инстинкт голода: первые 10 сенсорных нейронов → первые 10 моторных
        for i in 0..10.min(sensory_neurons.len()).min(motor_neurons.len()) {
            hunger_circuit.push(Connection::new(
                sensory_neurons[i],
                motor_neurons[i],
                0.8, // Сильная врожденная связь
                ConnectionType::Feedforward,
                true, // Врожденная
            ));
        }

        // Инстинкт страха: средние 10 сенсорных → средние 10 моторных
        let sensory_mid = sensory_neurons.len() / 2;
        let motor_mid = motor_neurons.len() / 2;

        for i in 0..10 {
            if sensory_mid + i < sensory_neurons.len() && motor_mid + i < motor_neurons.len() {
                fear_circuit.push(Connection::new(
                    sensory_neurons[sensory_mid + i],
                    motor_neurons[motor_mid + i],
                    0.9, // Очень сильная связь (выживание)
                    ConnectionType::Feedforward,
                    true,
                ));
            }
        }

        // Вокализация: последние 10 сенсорных → последние 10 моторных
        let sensory_end = sensory_neurons.len().saturating_sub(10);
        let motor_end = motor_neurons.len().saturating_sub(10);

        for i in 0..10 {
            if sensory_end + i < sensory_neurons.len() && motor_end + i < motor_neurons.len() {
                vocalization_circuit.push(Connection::new(
                    sensory_neurons[sensory_end + i],
                    motor_neurons[motor_end + i],
                    0.7, // Средняя связь
                    ConnectionType::Feedforward,
                    true,
                ));
            }
        }

        // Циркадный ритм: связи между всеми слоями (модуляция)
        // Реализуется через внешнюю модуляцию, а не прямые связи

        Self {
            hunger_circuit,
            fear_circuit,
            vocalization_circuit,
            circadian_circuit,
        }
    }

    /// Получить все врожденные связи
    pub fn get_all_connections(&self) -> Vec<Connection> {
        let mut all = Vec::new();
        all.extend(self.hunger_circuit.clone());
        all.extend(self.fear_circuit.clone());
        all.extend(self.vocalization_circuit.clone());
        all.extend(self.circadian_circuit.clone());
        all
    }

    /// Получить количество врожденных связей
    pub fn count(&self) -> usize {
        self.hunger_circuit.len() +
        self.fear_circuit.len() +
        self.vocalization_circuit.len() +
        self.circadian_circuit.len()
    }
}

/// Врожденные словарные паттерны
///
/// Базовые слова, которые "зашиты" в моторную кору
/// и могут быть активированы до обучения.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnateVocabulary {
    /// Слова выживания
    pub survival_words: Vec<String>,

    /// Эмоциональные слова
    pub emotion_words: Vec<String>,

    /// Базовые потребности
    pub need_words: Vec<String>,
}

impl InnateVocabulary {
    pub fn new() -> Self {
        Self {
            survival_words: vec![
                "голод".to_string(),
                "страх".to_string(),
                "боль".to_string(),
                "опасность".to_string(),
            ],
            emotion_words: vec![
                "плохо".to_string(),
                "хорошо".to_string(),
                "страшно".to_string(),
                "больно".to_string(),
            ],
            need_words: vec![
                "еда".to_string(),
                "вода".to_string(),
                "помощь".to_string(),
                "мама".to_string(),
            ],
        }
    }

    /// Получить все врожденные слова
    pub fn get_all_words(&self) -> Vec<String> {
        let mut all = Vec::new();
        all.extend(self.survival_words.clone());
        all.extend(self.emotion_words.clone());
        all.extend(self.need_words.clone());
        all
    }

    /// Проверить, является ли слово врожденным
    pub fn is_innate(&self, word: &str) -> bool {
        self.get_all_words().iter().any(|w| w == word)
    }
}

impl Default for InnateVocabulary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_innate_connections_creation() {
        let sensory_neurons: Vec<usize> = (0..100).collect();
        let motor_neurons: Vec<usize> = (100..200).collect();

        let innate = InnateConnections::new(&sensory_neurons, &motor_neurons);

        assert_eq!(innate.hunger_circuit.len(), 10);
        assert_eq!(innate.fear_circuit.len(), 10);
        assert_eq!(innate.vocalization_circuit.len(), 10);
    }

    #[test]
    fn test_innate_connections_are_innate() {
        let sensory_neurons: Vec<usize> = (0..100).collect();
        let motor_neurons: Vec<usize> = (100..200).collect();

        let innate = InnateConnections::new(&sensory_neurons, &motor_neurons);

        for conn in innate.get_all_connections() {
            assert!(conn.weight.is_innate);
        }
    }

    #[test]
    fn test_innate_vocabulary() {
        let vocab = InnateVocabulary::new();

        assert!(vocab.is_innate("голод"));
        assert!(vocab.is_innate("страх"));
        assert!(vocab.is_innate("помощь"));
        assert!(!vocab.is_innate("компьютер"));
    }

    #[test]
    fn test_get_all_words() {
        let vocab = InnateVocabulary::new();
        let all_words = vocab.get_all_words();

        assert!(all_words.len() >= 10);
        assert!(all_words.contains(&"голод".to_string()));
        assert!(all_words.contains(&"мама".to_string()));
    }

    #[test]
    fn test_hunger_circuit_strength() {
        let sensory_neurons: Vec<usize> = (0..100).collect();
        let motor_neurons: Vec<usize> = (100..200).collect();

        let innate = InnateConnections::new(&sensory_neurons, &motor_neurons);

        for conn in &innate.hunger_circuit {
            assert_eq!(conn.weight.weight, 0.8);
        }
    }

    #[test]
    fn test_fear_circuit_strength() {
        let sensory_neurons: Vec<usize> = (0..100).collect();
        let motor_neurons: Vec<usize> = (100..200).collect();

        let innate = InnateConnections::new(&sensory_neurons, &motor_neurons);

        for conn in &innate.fear_circuit {
            assert_eq!(conn.weight.weight, 0.9);
        }
    }
}
