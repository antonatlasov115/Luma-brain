use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Тип мысли
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThoughtType {
    Observation(String),    // Наблюдение
    Planning(String),       // Планирование
    Reflection(String),     // Рефлексия
    Association(String),    // Ассоциация
    Question(String),       // Внутренний вопрос
    Memory(String),         // Воспоминание
}

/// Внутренняя мысль
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thought {
    pub thought_type: ThoughtType,
    pub content: String,
    pub intensity: f64,
    pub timestamp: u64,
}

/// Модуль внутренних мыслей
#[derive(Clone, Serialize, Deserialize)]
pub struct ThoughtModule {
    pub thoughts: VecDeque<Thought>,
    pub thinking_rate: f64,
    pub internal_monologue_active: bool,
    pub consciousness_level: f64,
    pub last_thought_time: u64,
}

impl ThoughtModule {
    pub fn new() -> Self {
        Self {
            thoughts: VecDeque::with_capacity(50),
            thinking_rate: 0.4,  // 40% шанс думать
            internal_monologue_active: true,
            consciousness_level: 1.0,  // Полное сознание
            last_thought_time: 0,
        }
    }

    /// Генерация спонтанной мысли
    pub fn generate_thought(&mut self,
                           hunger: f64,
                           energy: f64,
                           happiness: f64,
                           stimulation: f64,
                           vocabulary_size: usize,
                           current_time: u64) -> Option<Thought> {
        let mut rng = rand::thread_rng();

        // Проверка времени (1 секунда минимум)
        if current_time - self.last_thought_time < 1 {
            return None;
        }

        // Вероятность мысли зависит от уровня сознания
        let think_probability = self.thinking_rate * self.consciousness_level;
        if rng.gen::<f64>() > think_probability {
            return None;
        }

        // Выбор типа мысли на основе состояния
        let thought = if hunger < 30.0 {
            Thought {
                thought_type: ThoughtType::Observation("Я голоден".to_string()),
                content: "Чувствую голод... Нужна еда".to_string(),
                intensity: (100.0 - hunger) / 100.0,
                timestamp: current_time,
            }
        } else if energy < 30.0 {
            Thought {
                thought_type: ThoughtType::Observation("Я устал".to_string()),
                content: "Так устал... Хочу отдохнуть".to_string(),
                intensity: (100.0 - energy) / 100.0,
                timestamp: current_time,
            }
        } else if stimulation > 70.0 {
            let reflections = vec![
                "Интересно, что это значит?",
                "Я многому научился",
                "Хочу узнать больше",
                "Это увлекательно!",
            ];
            Thought {
                thought_type: ThoughtType::Reflection("Размышление".to_string()),
                content: reflections[rng.gen_range(0..reflections.len())].to_string(),
                intensity: stimulation / 100.0,
                timestamp: current_time,
            }
        } else if vocabulary_size > 10 && rng.gen::<f64>() < 0.3 {
            Thought {
                thought_type: ThoughtType::Memory("Воспоминание".to_string()),
                content: format!("Я знаю уже {} слов!", vocabulary_size),
                intensity: 0.6,
                timestamp: current_time,
            }
        } else if happiness > 70.0 {
            let happy_thoughts = vec![
                "Мне хорошо",
                "Как приятно!",
                "Я счастлив",
            ];
            Thought {
                thought_type: ThoughtType::Observation("Радость".to_string()),
                content: happy_thoughts[rng.gen_range(0..happy_thoughts.len())].to_string(),
                intensity: happiness / 100.0,
                timestamp: current_time,
            }
        } else if rng.gen::<f64>() < 0.5 {
            let random_thoughts = vec![
                "Что будет дальше?",
                "Интересно...",
                "Хм...",
                "Думаю...",
            ];
            Thought {
                thought_type: ThoughtType::Question("Размышление".to_string()),
                content: random_thoughts[rng.gen_range(0..random_thoughts.len())].to_string(),
                intensity: 0.4,
                timestamp: current_time,
            }
        } else {
            return None;
        };

        self.last_thought_time = current_time;
        self.thoughts.push_back(thought.clone());

        if self.thoughts.len() > 50 {
            self.thoughts.pop_front();
        }

        Some(thought)
    }

    /// Планирование действия
    pub fn plan_action(&mut self, goal: &str, current_time: u64) -> Thought {
        let thought = Thought {
            thought_type: ThoughtType::Planning(goal.to_string()),
            content: format!("Нужно: {}", goal),
            intensity: 0.8,
            timestamp: current_time,
        };

        self.thoughts.push_back(thought.clone());
        thought
    }

    /// Ассоциация с новым словом
    pub fn associate_word(&mut self, word: &str, context: &str, current_time: u64) -> Thought {
        let thought = Thought {
            thought_type: ThoughtType::Association(word.to_string()),
            content: format!("'{}' связано с {}", word, context),
            intensity: 0.7,
            timestamp: current_time,
        };

        self.thoughts.push_back(thought.clone());
        thought
    }

    /// Обновление уровня сознания
    pub fn update_consciousness(&mut self, consciousness_index: f64) {
        self.consciousness_level = consciousness_index;

        // Активность внутреннего монолога зависит от сознания
        self.internal_monologue_active = consciousness_index > 0.3;
        self.thinking_rate = 0.05 + consciousness_index * 0.15;
    }

    /// Получить последние мысли
    pub fn get_recent_thoughts(&self, n: usize) -> Vec<String> {
        self.thoughts
            .iter()
            .rev()
            .take(n)
            .map(|t| {
                let icon = match t.thought_type {
                    ThoughtType::Observation(_) => "👁️",
                    ThoughtType::Planning(_) => "📋",
                    ThoughtType::Reflection(_) => "🤔",
                    ThoughtType::Association(_) => "🔗",
                    ThoughtType::Question(_) => "❓",
                    ThoughtType::Memory(_) => "💭",
                };
                format!("{} {}", icon, t.content)
            })
            .collect()
    }

    /// Выразить мысль вслух (иногда)
    pub fn maybe_express_thought(&self) -> Option<String> {
        let mut rng = rand::thread_rng();

        if !self.internal_monologue_active {
            return None;
        }

        if rng.gen::<f64>() > 0.2 {
            return None;
        }

        self.thoughts.back().map(|t| {
            format!("💭 {}", t.content)
        })
    }

    /// Статистика мыслей
    pub fn get_thought_stats(&self) -> (usize, f64) {
        let total = self.thoughts.len();
        let avg_intensity = if total > 0 {
            self.thoughts.iter().map(|t| t.intensity).sum::<f64>() / total as f64
        } else {
            0.0
        };

        (total, avg_intensity)
    }
}

impl Default for ThoughtModule {
    fn default() -> Self {
        Self::new()
    }
}
