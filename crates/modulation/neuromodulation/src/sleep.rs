//! Менеджер сна - фазы NREM/REM с replay
//!
//! Управляет циклами сна, консолидацией памяти и обрезкой слабых связей.

use serde::{Deserialize, Serialize};
use rand::Rng;

/// Фаза сна
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SleepPhase {
    /// Бодрствование
    Awake,

    /// NREM (Non-REM) - медленный сон, консолидация
    NREM,

    /// REM (Rapid Eye Movement) - быстрый сон, replay
    REM,
}

/// Записанный паттерн для replay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryTrace {
    /// Спайки (neuron_id, time)
    pub spikes: Vec<(usize, f64)>,

    /// Важность паттерна (0.0 - 1.0)
    pub importance: f64,

    /// Время записи
    pub timestamp: f64,
}

/// Менеджер сна
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepManager {
    /// Текущая фаза
    pub phase: SleepPhase,

    /// Время начала текущей фазы (мс)
    pub phase_start_time: f64,

    /// Длительность NREM фазы (мс)
    pub nrem_duration: f64,

    /// Длительность REM фазы (мс)
    pub rem_duration: f64,

    /// Записанные паттерны для replay
    pub memory_traces: Vec<MemoryTrace>,

    /// Максимальное количество записей
    pub max_traces: usize,

    /// Порог важности для сохранения паттерна
    pub importance_threshold: f64,

    /// Счетчик циклов сна
    pub sleep_cycles: u32,
}

impl SleepManager {
    /// Создать новый менеджер сна
    pub fn new() -> Self {
        Self {
            phase: SleepPhase::Awake,
            phase_start_time: 0.0,
            nrem_duration: 60_000.0,  // 1 минута NREM
            rem_duration: 30_000.0,   // 30 секунд REM
            memory_traces: Vec::new(),
            max_traces: 100,
            importance_threshold: 0.3,
            sleep_cycles: 0,
        }
    }

    /// Проверить, нужен ли сон
    pub fn should_sleep(&self, activity: f64, adenosine_level: f64) -> bool {
        if self.phase != SleepPhase::Awake {
            return false;
        }

        // Сон нужен если активность низкая И аденозин высокий
        activity < 20.0 && adenosine_level > 0.6
    }

    /// Начать сон
    pub fn start_sleep(&mut self, current_time: f64) {
        if self.phase == SleepPhase::Awake {
            self.phase = SleepPhase::NREM;
            self.phase_start_time = current_time;
            self.sleep_cycles += 1;
        }
    }

    /// Обновить фазу сна
    pub fn update(&mut self, current_time: f64) {
        let elapsed = current_time - self.phase_start_time;

        match self.phase {
            SleepPhase::Awake => {
                // Ничего не делаем
            }
            SleepPhase::NREM => {
                // Переход в REM после NREM
                if elapsed >= self.nrem_duration {
                    self.phase = SleepPhase::REM;
                    self.phase_start_time = current_time;
                }
            }
            SleepPhase::REM => {
                // Пробуждение после REM
                if elapsed >= self.rem_duration {
                    self.phase = SleepPhase::Awake;
                    self.phase_start_time = current_time;
                }
            }
        }
    }

    /// Записать паттерн для последующего replay
    pub fn record_pattern(&mut self, spikes: Vec<(usize, f64)>, importance: f64, timestamp: f64) {
        if importance < self.importance_threshold {
            return;
        }

        let trace = MemoryTrace {
            spikes,
            importance,
            timestamp,
        };

        self.memory_traces.push(trace);

        // Ограничиваем размер буфера
        if self.memory_traces.len() > self.max_traces {
            // Удаляем наименее важные
            self.memory_traces.sort_by(|a, b| b.importance.partial_cmp(&a.importance).unwrap());
            self.memory_traces.truncate(self.max_traces);
        }
    }

    /// Получить паттерн для replay (во время REM)
    pub fn get_replay_pattern(&self) -> Option<&MemoryTrace> {
        if self.phase != SleepPhase::REM || self.memory_traces.is_empty() {
            return None;
        }

        // Выбираем случайный паттерн с весом по важности
        let mut rng = rand::thread_rng();
        let total_importance: f64 = self.memory_traces.iter().map(|t| t.importance).sum();

        if total_importance == 0.0 {
            return self.memory_traces.first();
        }

        let mut target = rng.gen::<f64>() * total_importance;

        for trace in &self.memory_traces {
            target -= trace.importance;
            if target <= 0.0 {
                return Some(trace);
            }
        }

        self.memory_traces.last()
    }

    /// Получить модификатор активности для текущей фазы
    pub fn get_activity_modifier(&self) -> f64 {
        match self.phase {
            SleepPhase::Awake => 1.0,
            SleepPhase::NREM => 0.1,  // Очень низкая активность
            SleepPhase::REM => 0.3,   // Средняя активность (replay)
        }
    }

    /// Получить модификатор обучения для текущей фазы
    pub fn get_learning_modifier(&self) -> f64 {
        match self.phase {
            SleepPhase::Awake => 1.0,
            SleepPhase::NREM => 1.5,  // Усиленная консолидация
            SleepPhase::REM => 0.5,   // Слабое обучение (replay)
        }
    }

    /// Проверить, спит ли сейчас
    pub fn is_sleeping(&self) -> bool {
        self.phase != SleepPhase::Awake
    }

    /// Очистить старые паттерны
    pub fn prune_old_traces(&mut self, current_time: f64, max_age: f64) {
        self.memory_traces.retain(|trace| {
            current_time - trace.timestamp < max_age
        });
    }

    /// Получить статистику сна
    pub fn get_stats(&self) -> SleepStats {
        SleepStats {
            phase: self.phase,
            sleep_cycles: self.sleep_cycles,
            memory_traces: self.memory_traces.len(),
            avg_importance: if self.memory_traces.is_empty() {
                0.0
            } else {
                self.memory_traces.iter().map(|t| t.importance).sum::<f64>()
                    / self.memory_traces.len() as f64
            },
        }
    }
}

impl Default for SleepManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Статистика сна
#[derive(Debug, Clone)]
pub struct SleepStats {
    pub phase: SleepPhase,
    pub sleep_cycles: u32,
    pub memory_traces: usize,
    pub avg_importance: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sleep_manager_creation() {
        let manager = SleepManager::new();
        assert_eq!(manager.phase, SleepPhase::Awake);
        assert_eq!(manager.sleep_cycles, 0);
    }

    #[test]
    fn test_should_sleep() {
        let manager = SleepManager::new();

        // Низкая активность + высокий аденозин = сон
        assert!(manager.should_sleep(15.0, 0.7));

        // Высокая активность = не спать
        assert!(!manager.should_sleep(100.0, 0.7));

        // Низкий аденозин = не спать
        assert!(!manager.should_sleep(15.0, 0.3));
    }

    #[test]
    fn test_start_sleep() {
        let mut manager = SleepManager::new();

        manager.start_sleep(0.0);
        assert_eq!(manager.phase, SleepPhase::NREM);
        assert_eq!(manager.sleep_cycles, 1);
    }

    #[test]
    fn test_phase_transitions() {
        let mut manager = SleepManager::new();
        manager.nrem_duration = 100.0;
        manager.rem_duration = 50.0;

        // Начинаем сон
        manager.start_sleep(0.0);
        assert_eq!(manager.phase, SleepPhase::NREM);

        // Переход в REM
        manager.update(150.0);
        assert_eq!(manager.phase, SleepPhase::REM);

        // Пробуждение
        manager.update(250.0);
        assert_eq!(manager.phase, SleepPhase::Awake);
    }

    #[test]
    fn test_record_pattern() {
        let mut manager = SleepManager::new();

        let spikes = vec![(0, 0.0), (1, 10.0), (2, 20.0)];
        manager.record_pattern(spikes.clone(), 0.8, 0.0);

        assert_eq!(manager.memory_traces.len(), 1);
        assert_eq!(manager.memory_traces[0].importance, 0.8);
    }

    #[test]
    fn test_importance_threshold() {
        let mut manager = SleepManager::new();

        // Низкая важность - не записывается
        manager.record_pattern(vec![], 0.1, 0.0);
        assert_eq!(manager.memory_traces.len(), 0);

        // Высокая важность - записывается
        manager.record_pattern(vec![], 0.5, 0.0);
        assert_eq!(manager.memory_traces.len(), 1);
    }

    #[test]
    fn test_max_traces() {
        let mut manager = SleepManager::new();
        manager.max_traces = 5;

        // Записываем 10 паттернов
        for i in 0..10 {
            manager.record_pattern(vec![], 0.5 + i as f64 * 0.05, 0.0);
        }

        // Должно остаться только 5 самых важных
        assert_eq!(manager.memory_traces.len(), 5);
        assert!(manager.memory_traces[0].importance >= 0.7);
    }

    #[test]
    fn test_get_replay_pattern() {
        let mut manager = SleepManager::new();

        // В бодрствовании нет replay
        assert!(manager.get_replay_pattern().is_none());

        // Записываем паттерн
        manager.record_pattern(vec![(0, 0.0)], 0.8, 0.0);

        // Переходим в REM
        manager.phase = SleepPhase::REM;
        let pattern = manager.get_replay_pattern();
        assert!(pattern.is_some());
    }

    #[test]
    fn test_activity_modifier() {
        let mut manager = SleepManager::new();

        assert_eq!(manager.get_activity_modifier(), 1.0);

        manager.phase = SleepPhase::NREM;
        assert_eq!(manager.get_activity_modifier(), 0.1);

        manager.phase = SleepPhase::REM;
        assert_eq!(manager.get_activity_modifier(), 0.3);
    }

    #[test]
    fn test_learning_modifier() {
        let mut manager = SleepManager::new();

        assert_eq!(manager.get_learning_modifier(), 1.0);

        manager.phase = SleepPhase::NREM;
        assert_eq!(manager.get_learning_modifier(), 1.5);
    }

    #[test]
    fn test_prune_old_traces() {
        let mut manager = SleepManager::new();

        manager.record_pattern(vec![], 0.5, 0.0);
        manager.record_pattern(vec![], 0.5, 1000.0);
        manager.record_pattern(vec![], 0.5, 2000.0);

        assert_eq!(manager.memory_traces.len(), 3);

        // Удаляем паттерны старше 1500 мс
        manager.prune_old_traces(2000.0, 1500.0);
        assert_eq!(manager.memory_traces.len(), 2);
    }

    #[test]
    fn test_stats() {
        let mut manager = SleepManager::new();

        manager.record_pattern(vec![], 0.6, 0.0);
        manager.record_pattern(vec![], 0.8, 0.0);

        let stats = manager.get_stats();
        assert_eq!(stats.memory_traces, 2);
        assert_eq!(stats.avg_importance, 0.7);
    }
}
