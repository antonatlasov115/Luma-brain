//! Стадии развития и импринтинг
//!
//! Моделирует критические периоды развития, когда нейронная сеть
//! особенно чувствительна к определенным типам стимулов.

use serde::{Deserialize, Serialize};

/// Стадия развития
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DevelopmentalStage {
    /// Новорожденный (0-30 мин): Калибровка нейрохимии
    Newborn,

    /// Младенец (30 мин - 2 часа): Сенсорная адаптация
    Infant,

    /// Ребенок (2-6 часов): Первый сон с replay
    Child,

    /// Подросток (6-24 часа): Лингвистический контакт
    Adolescent,

    /// Взрослый (24+ часа): Полная когнитивная зрелость
    Adult,
}

impl DevelopmentalStage {
    /// Получить стадию по возрасту (в миллисекундах)
    pub fn from_age(age_ms: f64) -> Self {
        let age_min = age_ms / 60_000.0;

        if age_min < 30.0 {
            Self::Newborn
        } else if age_min < 120.0 {
            Self::Infant
        } else if age_min < 360.0 {
            Self::Child
        } else if age_min < 1440.0 {
            Self::Adolescent
        } else {
            Self::Adult
        }
    }

    /// Получить модификатор скорости обучения для этой стадии
    pub fn learning_rate_modifier(&self) -> f64 {
        match self {
            Self::Newborn => 0.5,    // Медленное обучение (калибровка)
            Self::Infant => 2.0,     // Быстрое обучение (критический период)
            Self::Child => 1.5,      // Ускоренное обучение
            Self::Adolescent => 1.2, // Нормальное обучение
            Self::Adult => 1.0,      // Базовое обучение
        }
    }

    /// Получить порог пластичности (насколько легко меняются связи)
    pub fn plasticity_threshold(&self) -> f64 {
        match self {
            Self::Newborn => 0.3,    // Низкий порог (высокая пластичность)
            Self::Infant => 0.2,     // Очень низкий порог (импринтинг)
            Self::Child => 0.4,      // Средний порог
            Self::Adolescent => 0.5, // Повышенный порог
            Self::Adult => 0.6,      // Высокий порог (стабильность)
        }
    }

    /// Проверить, находится ли в критическом периоде
    pub fn is_critical_period(&self) -> bool {
        matches!(self, Self::Infant | Self::Child)
    }

    /// Получить название стадии
    pub fn name(&self) -> &str {
        match self {
            Self::Newborn => "Новорожденный",
            Self::Infant => "Младенец",
            Self::Child => "Ребенок",
            Self::Adolescent => "Подросток",
            Self::Adult => "Взрослый",
        }
    }
}

/// Окно импринтинга
///
/// Критический период, когда определенные стимулы оставляют
/// особенно сильный след в нейронной сети.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprintingWindow {
    /// Начало окна (мс)
    pub start_time: f64,

    /// Конец окна (мс)
    pub end_time: f64,

    /// Тип импринтинга
    pub imprint_type: ImprintType,

    /// Усилитель обучения в этом окне
    pub learning_boost: f64,

    /// Активно ли окно
    pub is_active: bool,
}

/// Тип импринтинга
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImprintType {
    /// Лингвистический (первые слова)
    Linguistic,

    /// Социальный (первый контакт)
    Social,

    /// Эмоциональный (первые эмоции)
    Emotional,

    /// Сенсорный (первые ощущения)
    Sensory,
}

impl ImprintingWindow {
    /// Создать новое окно импринтинга
    pub fn new(start_time: f64, duration: f64, imprint_type: ImprintType) -> Self {
        let learning_boost = match imprint_type {
            ImprintType::Linguistic => 3.0,  // Очень сильный буст для языка
            ImprintType::Social => 2.5,      // Сильный буст для социализации
            ImprintType::Emotional => 2.0,   // Средний буст для эмоций
            ImprintType::Sensory => 1.5,     // Слабый буст для сенсорики
        };

        Self {
            start_time,
            end_time: start_time + duration,
            imprint_type,
            learning_boost,
            is_active: false,
        }
    }

    /// Обновить состояние окна
    pub fn update(&mut self, current_time: f64) {
        self.is_active = current_time >= self.start_time && current_time < self.end_time;
    }

    /// Получить текущий буст обучения
    pub fn get_learning_boost(&self) -> f64 {
        if self.is_active {
            self.learning_boost
        } else {
            1.0
        }
    }

    /// Проверить, закончилось ли окно
    pub fn is_closed(&self, current_time: f64) -> bool {
        current_time >= self.end_time
    }
}

/// Менеджер развития
///
/// Управляет стадиями развития и окнами импринтинга.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopmentManager {
    /// Время рождения (мс)
    pub birth_time: f64,

    /// Текущая стадия
    pub current_stage: DevelopmentalStage,

    /// Окна импринтинга
    pub imprinting_windows: Vec<ImprintingWindow>,

    /// История стадий
    pub stage_history: Vec<(DevelopmentalStage, f64)>,
}

impl DevelopmentManager {
    /// Создать новый менеджер развития
    pub fn new(birth_time: f64) -> Self {
        let mut windows = Vec::new();

        // Сенсорное окно: 30 мин - 2 часа (1.5 часа)
        windows.push(ImprintingWindow::new(
            birth_time + 30.0 * 60_000.0,
            90.0 * 60_000.0,
            ImprintType::Sensory,
        ));

        // Эмоциональное окно: 2-4 часа (2 часа)
        windows.push(ImprintingWindow::new(
            birth_time + 120.0 * 60_000.0,
            120.0 * 60_000.0,
            ImprintType::Emotional,
        ));

        // Лингвистическое окно: 6-12 часов (6 часов)
        windows.push(ImprintingWindow::new(
            birth_time + 360.0 * 60_000.0,
            360.0 * 60_000.0,
            ImprintType::Linguistic,
        ));

        // Социальное окно: 12-24 часа (12 часов)
        windows.push(ImprintingWindow::new(
            birth_time + 720.0 * 60_000.0,
            720.0 * 60_000.0,
            ImprintType::Social,
        ));

        Self {
            birth_time,
            current_stage: DevelopmentalStage::Newborn,
            imprinting_windows: windows,
            stage_history: vec![(DevelopmentalStage::Newborn, birth_time)],
        }
    }

    /// Обновить состояние развития
    pub fn update(&mut self, current_time: f64) {
        // Обновляем стадию
        let age = current_time - self.birth_time;
        let new_stage = DevelopmentalStage::from_age(age);

        if new_stage != self.current_stage {
            self.current_stage = new_stage;
            self.stage_history.push((new_stage, current_time));
        }

        // Обновляем окна импринтинга
        for window in &mut self.imprinting_windows {
            window.update(current_time);
        }
    }

    /// Получить текущий возраст (в миллисекундах)
    pub fn get_age(&self, current_time: f64) -> f64 {
        current_time - self.birth_time
    }

    /// Получить текущий модификатор обучения
    ///
    /// Учитывает стадию развития и активные окна импринтинга
    pub fn get_learning_modifier(&self) -> f64 {
        let stage_modifier = self.current_stage.learning_rate_modifier();

        let imprint_boost: f64 = self.imprinting_windows
            .iter()
            .map(|w| w.get_learning_boost())
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(1.0);

        stage_modifier * imprint_boost
    }

    /// Получить активные окна импринтинга
    pub fn get_active_windows(&self) -> Vec<&ImprintingWindow> {
        self.imprinting_windows
            .iter()
            .filter(|w| w.is_active)
            .collect()
    }

    /// Проверить, находится ли в критическом периоде
    pub fn is_in_critical_period(&self) -> bool {
        self.current_stage.is_critical_period() || !self.get_active_windows().is_empty()
    }

    /// Получить описание текущего состояния
    pub fn get_status(&self, current_time: f64) -> String {
        let age_min = self.get_age(current_time) / 60_000.0;
        let stage_name = self.current_stage.name();

        let active_windows: Vec<String> = self.get_active_windows()
            .iter()
            .map(|w| format!("{:?}", w.imprint_type))
            .collect();

        if active_windows.is_empty() {
            format!("{} ({:.1} мин)", stage_name, age_min)
        } else {
            format!("{} ({:.1} мин) [Импринтинг: {}]",
                stage_name, age_min, active_windows.join(", "))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage_from_age() {
        assert_eq!(DevelopmentalStage::from_age(0.0), DevelopmentalStage::Newborn);
        assert_eq!(DevelopmentalStage::from_age(15.0 * 60_000.0), DevelopmentalStage::Newborn);
        assert_eq!(DevelopmentalStage::from_age(60.0 * 60_000.0), DevelopmentalStage::Infant);
        assert_eq!(DevelopmentalStage::from_age(180.0 * 60_000.0), DevelopmentalStage::Child);
        assert_eq!(DevelopmentalStage::from_age(720.0 * 60_000.0), DevelopmentalStage::Adolescent);
        assert_eq!(DevelopmentalStage::from_age(1500.0 * 60_000.0), DevelopmentalStage::Adult);
    }

    #[test]
    fn test_learning_rate_modifier() {
        assert_eq!(DevelopmentalStage::Newborn.learning_rate_modifier(), 0.5);
        assert_eq!(DevelopmentalStage::Infant.learning_rate_modifier(), 2.0);
        assert_eq!(DevelopmentalStage::Adult.learning_rate_modifier(), 1.0);
    }

    #[test]
    fn test_critical_period() {
        assert!(!DevelopmentalStage::Newborn.is_critical_period());
        assert!(DevelopmentalStage::Infant.is_critical_period());
        assert!(DevelopmentalStage::Child.is_critical_period());
        assert!(!DevelopmentalStage::Adolescent.is_critical_period());
        assert!(!DevelopmentalStage::Adult.is_critical_period());
    }

    #[test]
    fn test_imprinting_window() {
        let mut window = ImprintingWindow::new(0.0, 1000.0, ImprintType::Linguistic);

        window.update(500.0);
        assert!(window.is_active);
        assert_eq!(window.get_learning_boost(), 3.0);

        window.update(1500.0);
        assert!(!window.is_active);
        assert_eq!(window.get_learning_boost(), 1.0);
    }

    #[test]
    fn test_development_manager() {
        let mut manager = DevelopmentManager::new(0.0);

        assert_eq!(manager.current_stage, DevelopmentalStage::Newborn);
        assert_eq!(manager.imprinting_windows.len(), 4);

        // Обновляем до стадии младенца
        manager.update(60.0 * 60_000.0);
        assert_eq!(manager.current_stage, DevelopmentalStage::Infant);

        // Проверяем активные окна
        let active = manager.get_active_windows();
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].imprint_type, ImprintType::Sensory);
    }

    #[test]
    fn test_learning_modifier() {
        let mut manager = DevelopmentManager::new(0.0);

        // Новорожденный: 0.5x
        let modifier1 = manager.get_learning_modifier();
        assert_eq!(modifier1, 0.5);

        // Младенец с сенсорным окном: 2.0 * 1.5 = 3.0x
        manager.update(60.0 * 60_000.0);
        let modifier2 = manager.get_learning_modifier();
        assert_eq!(modifier2, 3.0);

        // Взрослый без окон: 1.0x
        manager.update(2000.0 * 60_000.0);
        let modifier3 = manager.get_learning_modifier();
        assert_eq!(modifier3, 1.0);
    }

    #[test]
    fn test_stage_history() {
        let mut manager = DevelopmentManager::new(0.0);

        manager.update(60.0 * 60_000.0);
        manager.update(180.0 * 60_000.0);

        assert_eq!(manager.stage_history.len(), 3);
        assert_eq!(manager.stage_history[0].0, DevelopmentalStage::Newborn);
        assert_eq!(manager.stage_history[1].0, DevelopmentalStage::Infant);
        assert_eq!(manager.stage_history[2].0, DevelopmentalStage::Child);
    }

    #[test]
    fn test_get_status() {
        let mut manager = DevelopmentManager::new(0.0);

        let status1 = manager.get_status(15.0 * 60_000.0);
        assert!(status1.contains("Новорожденный"));

        manager.update(60.0 * 60_000.0);
        let status2 = manager.get_status(60.0 * 60_000.0);
        assert!(status2.contains("Младенец"));
        assert!(status2.contains("Sensory"));
    }
}
