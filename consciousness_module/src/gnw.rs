use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Источник информации (модуль мозга)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InformationSource {
    Perception,    // Восприятие (голод, энергия)
    Memory,        // Память
    Emotion,       // Эмоции
    Thought,       // Мысли
    Speech,        // Речь
    Planning,      // Планирование
}

impl InformationSource {
    pub fn name(&self) -> &str {
        match self {
            InformationSource::Perception => "Восприятие",
            InformationSource::Memory => "Память",
            InformationSource::Emotion => "Эмоции",
            InformationSource::Thought => "Мысли",
            InformationSource::Speech => "Речь",
            InformationSource::Planning => "Планирование",
        }
    }

    pub fn emoji(&self) -> &str {
        match self {
            InformationSource::Perception => "👁️",
            InformationSource::Memory => "📚",
            InformationSource::Emotion => "❤️",
            InformationSource::Thought => "💭",
            InformationSource::Speech => "🗣️",
            InformationSource::Planning => "🎯",
        }
    }
}

/// Информация, конкурирующая за доступ к сознанию
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Information {
    pub source: InformationSource,
    pub content: String,
    pub activation: f64,      // Уровень активации (0-1)
    pub salience: f64,        // Важность/яркость (0-1)
    pub timestamp: f64,
}

impl Information {
    pub fn new(source: InformationSource, content: String, salience: f64) -> Self {
        Self {
            source,
            content,
            activation: salience,
            salience,
            timestamp: 0.0,
        }
    }

    /// Вычислить приоритет для конкуренции
    pub fn priority(&self) -> f64 {
        // Приоритет = активация × важность × свежесть
        let freshness = (-self.timestamp * 0.05).exp(); // Медленнее спад (было 0.1)
        self.activation * self.salience * freshness
    }
}

/// Глобальное рабочее пространство (Global Neuronal Workspace)
#[derive(Clone, Serialize, Deserialize)]
pub struct GlobalWorkspace {
    /// Порог для глобальной трансляции
    pub broadcast_threshold: f64,

    /// Текущее содержимое сознания
    pub current_content: Option<Information>,

    /// Очередь конкурирующей информации
    pub competing_info: VecDeque<Information>,

    /// История осознанной информации
    pub consciousness_history: VecDeque<Information>,

    /// Время с последней трансляции
    pub time_since_broadcast: f64,

    /// Минимальный интервал между трансляциями
    pub refractory_period: f64,

    /// Текущее время
    pub time: f64,
}

impl GlobalWorkspace {
    pub fn new() -> Self {
        Self {
            broadcast_threshold: 0.4, // Понижен с 0.6 до 0.4 - легче осознавать
            current_content: None,
            competing_info: VecDeque::new(),
            consciousness_history: VecDeque::new(),
            time_since_broadcast: 0.0,
            refractory_period: 0.3, // Понижен с 0.5 до 0.3 - чаще обновляется
            time: 0.0,
        }
    }

    /// Добавить информацию в конкуренцию
    pub fn submit_information(&mut self, mut info: Information) {
        info.timestamp = self.time;
        self.competing_info.push_back(info);

        // Ограничить размер очереди
        if self.competing_info.len() > 20 {
            self.competing_info.pop_front();
        }
    }

    /// Обновить рабочее пространство
    pub fn update(&mut self, elapsed: f64) -> Option<Information> {
        self.time += elapsed;
        self.time_since_broadcast += elapsed;

        // Обновить временные метки
        for info in self.competing_info.iter_mut() {
            info.timestamp += elapsed;
        }

        // Проверить рефрактерный период
        if self.time_since_broadcast < self.refractory_period {
            return None;
        }

        // Конкуренция за доступ к сознанию
        if let Some(winner) = self.compete_for_access() {
            // Если активация превышает порог → глобальная трансляция
            if winner.priority() > self.broadcast_threshold {
                self.global_broadcast(winner.clone());
                return Some(winner);
            }
        }

        None
    }

    /// Конкуренция между информацией
    fn compete_for_access(&mut self) -> Option<Information> {
        if self.competing_info.is_empty() {
            return None;
        }

        // Найти информацию с максимальным приоритетом
        let mut max_priority = 0.0;
        let mut winner_idx = 0;

        for (idx, info) in self.competing_info.iter().enumerate() {
            let priority = info.priority();
            if priority > max_priority {
                max_priority = priority;
                winner_idx = idx;
            }
        }

        // Удалить победителя из очереди
        self.competing_info.remove(winner_idx)
    }

    /// Глобальная трансляция (broadcast)
    fn global_broadcast(&mut self, info: Information) {
        self.current_content = Some(info.clone());
        self.time_since_broadcast = 0.0;

        // Добавить в историю сознания
        self.consciousness_history.push_back(info);
        if self.consciousness_history.len() > 50 {
            self.consciousness_history.pop_front();
        }

        // Снизить активацию конкурирующей информации (латеральное торможение)
        for other in self.competing_info.iter_mut() {
            other.activation *= 0.7;
        }
    }

    /// Получить текущее содержимое сознания
    pub fn get_conscious_content(&self) -> Option<&Information> {
        self.current_content.as_ref()
    }

    /// Проверить, осознана ли информация из источника
    pub fn is_source_conscious(&self, source: InformationSource) -> bool {
        if let Some(content) = &self.current_content {
            content.source == source
        } else {
            false
        }
    }

    /// Получить индекс осознанности (0-1)
    pub fn get_awareness_index(&self) -> f64 {
        if let Some(content) = &self.current_content {
            // Осознанность зависит от активации и свежести
            let freshness = (-self.time_since_broadcast * 2.0).exp();
            content.activation * freshness
        } else {
            0.0
        }
    }

    /// Получить статистику сознания
    pub fn get_consciousness_stats(&self) -> Vec<(InformationSource, usize)> {
        let mut stats = std::collections::HashMap::new();

        for info in self.consciousness_history.iter() {
            *stats.entry(info.source).or_insert(0) += 1;
        }

        let mut result: Vec<_> = stats.into_iter().collect();
        result.sort_by(|a, b| b.1.cmp(&a.1));
        result
    }

    /// Очистить устаревшую информацию
    pub fn cleanup(&mut self) {
        self.competing_info.retain(|info| info.timestamp < 5.0);
    }
}

impl Default for GlobalWorkspace {
    fn default() -> Self {
        Self::new()
    }
}
