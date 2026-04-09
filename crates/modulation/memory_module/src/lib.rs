use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

/// Эмоция для памяти
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Emotion {
    Joy,
    Sadness,
    Curiosity,
    Fear,
    Anger,
    Neutral,
}

/// Эпизодическая память (конкретные события)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodicMemory {
    pub event: String,
    pub emotion: Emotion,
    pub context: HashMap<String, f64>, // hunger, energy, etc
    pub timestamp: u64,
    pub importance: f64,
}

/// Семантическая память (факты и знания)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticMemory {
    pub fact: String,
    pub associations: Vec<String>,
    pub strength: f64,
    pub last_accessed: u64,
}

/// Рабочая память (текущий контекст)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingMemory {
    pub current_topic: Option<String>,
    pub recent_words: VecDeque<String>,
    pub active_goals: Vec<String>,
    pub attention_focus: f64,
}

/// Модуль памяти
#[derive(Clone, Serialize, Deserialize)]
pub struct MemoryModule {
    pub episodic: VecDeque<EpisodicMemory>,
    pub semantic: HashMap<String, SemanticMemory>,
    pub working: WorkingMemory,
    pub consolidation_threshold: f64,
}

impl MemoryModule {
    pub fn new() -> Self {
        Self {
            episodic: VecDeque::with_capacity(100),
            semantic: HashMap::new(),
            working: WorkingMemory {
                current_topic: None,
                recent_words: VecDeque::with_capacity(10),
                active_goals: Vec::new(),
                attention_focus: 1.0,
            },
            consolidation_threshold: 0.7,
        }
    }

    /// Запомнить событие
    pub fn remember_event(&mut self,
                          event: String,
                          emotion: Emotion,
                          hunger: f64,
                          energy: f64,
                          happiness: f64,
                          timestamp: u64) {
        let mut context = HashMap::new();
        context.insert("hunger".to_string(), hunger);
        context.insert("energy".to_string(), energy);
        context.insert("happiness".to_string(), happiness);

        // Важность зависит от эмоции
        let importance = match emotion {
            Emotion::Joy => 0.8,
            Emotion::Fear => 0.9,
            Emotion::Anger => 0.85,
            Emotion::Sadness => 0.7,
            Emotion::Curiosity => 0.6,
            Emotion::Neutral => 0.3,
        };

        let memory = EpisodicMemory {
            event,
            emotion,
            context,
            timestamp,
            importance,
        };

        self.episodic.push_back(memory);

        // Ограничиваем размер
        if self.episodic.len() > 100 {
            self.episodic.pop_front();
        }
    }

    /// Консолидация: перенос важных эпизодов в семантическую память
    pub fn consolidate_memories(&mut self, current_time: u64) {
        let important_memories: Vec<_> = self.episodic
            .iter()
            .filter(|m| m.importance > self.consolidation_threshold)
            .filter(|m| current_time - m.timestamp > 30) // Прошло время
            .cloned()
            .collect();

        for memory in important_memories {
            let fact = format!("{} ({})", memory.event, format!("{:?}", memory.emotion));

            let semantic = SemanticMemory {
                fact: fact.clone(),
                associations: vec![format!("{:?}", memory.emotion)],
                strength: memory.importance,
                last_accessed: current_time,
            };

            self.semantic.insert(fact, semantic);
        }
    }

    /// Вспомнить похожее событие
    pub fn recall_similar(&self, query: &str) -> Option<&EpisodicMemory> {
        self.episodic
            .iter()
            .rev()
            .find(|m| m.event.to_lowercase().contains(&query.to_lowercase()))
    }

    /// Получить эмоциональный контекст слова
    pub fn get_emotional_context(&self, word: &str) -> Option<Emotion> {
        self.episodic
            .iter()
            .rev()
            .find(|m| m.event.contains(word))
            .map(|m| m.emotion)
    }

    /// Обновить рабочую память
    pub fn update_working_memory(&mut self, word: &str) {
        self.working.recent_words.push_back(word.to_string());
        if self.working.recent_words.len() > 10 {
            self.working.recent_words.pop_front();
        }

        // Определяем текущую тему
        if let Some(last_word) = self.working.recent_words.back() {
            self.working.current_topic = Some(last_word.clone());
        }
    }

    /// Забывание старых воспоминаний
    pub fn decay_memories(&mut self, current_time: u64) {
        // Ослабляем семантические воспоминания
        for memory in self.semantic.values_mut() {
            let time_since_access = current_time - memory.last_accessed;
            if time_since_access > 100 {
                memory.strength *= 0.95;
            }
        }

        // Удаляем очень слабые
        self.semantic.retain(|_, m| m.strength > 0.1);
    }

    /// Статистика памяти
    pub fn get_memory_stats(&self) -> (usize, usize, usize) {
        (
            self.episodic.len(),
            self.semantic.len(),
            self.working.recent_words.len(),
        )
    }

    /// Получить важные воспоминания
    pub fn get_important_memories(&self, n: usize) -> Vec<String> {
        let mut memories: Vec<_> = self.episodic.iter().collect();
        memories.sort_by(|a, b| b.importance.partial_cmp(&a.importance).unwrap());

        memories
            .into_iter()
            .take(n)
            .map(|m| format!("{} {:?}", m.event, m.emotion))
            .collect()
    }
}

impl Default for MemoryModule {
    fn default() -> Self {
        Self::new()
    }
}
