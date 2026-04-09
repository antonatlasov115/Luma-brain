use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Словарь питомца - связь между словами и действиями
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetVocabulary {
    /// Слова -> действия (сила ассоциации)
    pub word_associations: HashMap<String, HashMap<String, f64>>,

    /// Собственный язык питомца
    pub pet_language: HashMap<String, String>, // действие -> слово питомца

    /// Счетчик использования слов
    pub word_usage_count: HashMap<String, usize>,

    /// Последние услышанные слова
    pub recent_words: Vec<String>,
}

impl PetVocabulary {
    pub fn new() -> Self {
        Self {
            word_associations: HashMap::new(),
            pet_language: HashMap::new(),
            word_usage_count: HashMap::new(),
            recent_words: Vec::new(),
        }
    }

    /// Обучение: связать слова с действием
    pub fn learn_association(&mut self, words: &str, action: &str, strength: f64) {
        let words_lower = words.to_lowercase();
        let tokens: Vec<String> = words_lower
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        for word in tokens {
            // Увеличиваем счетчик
            *self.word_usage_count.entry(word.clone()).or_insert(0) += 1;

            // Создаем/усиливаем ассоциацию
            let associations = self.word_associations.entry(word.clone()).or_insert_with(HashMap::new);
            let current_strength = associations.get(action).unwrap_or(&0.0);
            associations.insert(action.to_string(), (current_strength + strength).min(1.0));

            // Добавляем в недавние слова
            self.recent_words.push(word);
            if self.recent_words.len() > 50 {
                self.recent_words.remove(0);
            }
        }
    }

    /// Распознать действие по словам
    pub fn recognize_action(&self, words: &str) -> Option<(String, f64)> {
        let words_lower = words.to_lowercase();
        let tokens: Vec<&str> = words_lower.split_whitespace().collect();

        let mut action_scores: HashMap<String, f64> = HashMap::new();

        for word in tokens {
            if let Some(associations) = self.word_associations.get(word) {
                for (action, strength) in associations {
                    *action_scores.entry(action.clone()).or_insert(0.0) += strength;
                }
            }
        }

        action_scores
            .into_iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }

    /// Создать собственное слово для действия
    pub fn create_pet_word(&mut self, action: &str) -> String {
        // Если уже есть слово, возвращаем его
        if let Some(word) = self.pet_language.get(action) {
            return word.clone();
        }

        // Генерируем новое слово на основе частых звуков
        let syllables = ["ла", "ма", "па", "ба", "да", "ка", "га", "на", "ра", "та"];
        let mut rng = rand::thread_rng();
        use rand::seq::SliceRandom;

        let length = rand::Rng::gen_range(&mut rng, 2..4);
        let mut word = String::new();

        for _ in 0..length {
            word.push_str(syllables.choose(&mut rng).unwrap());
        }

        self.pet_language.insert(action.to_string(), word.clone());
        word
    }

    /// Получить слово питомца для действия
    pub fn get_pet_word(&self, action: &str) -> Option<String> {
        self.pet_language.get(action).cloned()
    }

    /// Статистика словаря
    pub fn get_vocabulary_size(&self) -> usize {
        self.word_associations.len()
    }

    pub fn get_language_size(&self) -> usize {
        self.pet_language.len()
    }

    /// Получить топ слов
    pub fn get_top_words(&self, n: usize) -> Vec<(String, usize)> {
        let mut words: Vec<_> = self.word_usage_count.iter().collect();
        words.sort_by(|a, b| b.1.cmp(a.1));
        words.into_iter().take(n).map(|(k, v)| (k.clone(), *v)).collect()
    }
}

impl Default for PetVocabulary {
    fn default() -> Self {
        Self::new()
    }
}
