use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Нейромедиатор (химический сигнал)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Neurotransmitter {
    Dopamine,      // Дофамин - награда, мотивация
    Serotonin,     // Серотонин - настроение, счастье
    Norepinephrine, // Норэпинефрин - внимание, возбуждение
    GABA,          // ГАМК - торможение, спокойствие
    Glutamate,     // Глутамат - возбуждение, обучение
    Acetylcholine, // Ацетилхолин - память, внимание
}

impl Neurotransmitter {
    pub fn name(&self) -> &str {
        match self {
            Neurotransmitter::Dopamine => "Дофамин",
            Neurotransmitter::Serotonin => "Серотонин",
            Neurotransmitter::Norepinephrine => "Норэпинефрин",
            Neurotransmitter::GABA => "ГАМК",
            Neurotransmitter::Glutamate => "Глутамат",
            Neurotransmitter::Acetylcholine => "Ацетилхолин",
        }
    }

    pub fn emoji(&self) -> &str {
        match self {
            Neurotransmitter::Dopamine => "🎯",
            Neurotransmitter::Serotonin => "😊",
            Neurotransmitter::Norepinephrine => "⚡",
            Neurotransmitter::GABA => "😌",
            Neurotransmitter::Glutamate => "🔥",
            Neurotransmitter::Acetylcholine => "🧠",
        }
    }
}

/// Химическая система мозга
#[derive(Clone, Serialize, Deserialize)]
pub struct ChemistryModule {
    pub levels: HashMap<String, f64>,
    pub baseline: HashMap<String, f64>,
    pub decay_rate: f64,
}

impl ChemistryModule {
    pub fn new() -> Self {
        let mut levels = HashMap::new();
        let mut baseline = HashMap::new();

        // Базовые уровни
        for nt in [
            Neurotransmitter::Dopamine,
            Neurotransmitter::Serotonin,
            Neurotransmitter::Norepinephrine,
            Neurotransmitter::GABA,
            Neurotransmitter::Glutamate,
            Neurotransmitter::Acetylcholine,
        ] {
            let name = nt.name().to_string();
            levels.insert(name.clone(), 0.5);
            baseline.insert(name, 0.5);
        }

        Self {
            levels,
            baseline,
            decay_rate: 0.95,
        }
    }

    /// Обновление химии на основе состояния
    pub fn update(&mut self, hunger: f64, energy: f64, happiness: f64, stimulation: f64, elapsed: f64) {
        // Дофамин - награда и мотивация
        let dopamine_target = if happiness > 70.0 {
            0.8
        } else if hunger < 30.0 {
            0.3 // Низкий при голоде
        } else {
            0.5
        };
        self.adjust_level("Дофамин", dopamine_target, elapsed);

        // Серотонин - настроение
        let serotonin_target = happiness / 100.0;
        self.adjust_level("Серотонин", serotonin_target, elapsed);

        // Норэпинефрин - возбуждение
        let norepinephrine_target = if stimulation > 60.0 {
            0.7
        } else if energy < 30.0 {
            0.2
        } else {
            0.4
        };
        self.adjust_level("Норэпинефрин", norepinephrine_target, elapsed);

        // ГАМК - торможение (обратно возбуждению)
        let gaba_target = if energy < 30.0 {
            0.8 // Высокий при усталости
        } else if stimulation > 70.0 {
            0.3
        } else {
            0.5
        };
        self.adjust_level("ГАМК", gaba_target, elapsed);

        // Глутамат - обучение
        let glutamate_target = if stimulation > 50.0 {
            0.7
        } else {
            0.4
        };
        self.adjust_level("Глутамат", glutamate_target, elapsed);

        // Ацетилхолин - внимание
        let acetylcholine_target = if stimulation > 60.0 && energy > 40.0 {
            0.7
        } else {
            0.4
        };
        self.adjust_level("Ацетилхолин", acetylcholine_target, elapsed);
    }

    fn adjust_level(&mut self, name: &str, target: f64, elapsed: f64) {
        if let Some(current) = self.levels.get_mut(name) {
            // Плавное движение к целевому уровню
            let diff = target - *current;
            *current += diff * elapsed * 0.1;
            *current = current.clamp(0.0, 1.0);
        }
    }

    /// Выброс нейромедиатора (награда, стресс и т.д.)
    pub fn release(&mut self, neurotransmitter: Neurotransmitter, amount: f64) {
        let name = neurotransmitter.name();
        if let Some(level) = self.levels.get_mut(name) {
            *level = (*level + amount).min(1.0);
        }
    }

    /// Получить модуляцию для обучения
    pub fn get_learning_modulation(&self) -> f64 {
        let dopamine = self.levels.get("Дофамин").unwrap_or(&0.5);
        let glutamate = self.levels.get("Глутамат").unwrap_or(&0.5);
        let acetylcholine = self.levels.get("Ацетилхолин").unwrap_or(&0.5);

        // Обучение лучше при высоком дофамине, глутамате и ацетилхолине
        (dopamine + glutamate + acetylcholine) / 3.0
    }

    /// Получить модуляцию настроения
    pub fn get_mood_modulation(&self) -> f64 {
        let serotonin = self.levels.get("Серотонин").unwrap_or(&0.5);
        let dopamine = self.levels.get("Дофамин").unwrap_or(&0.5);

        (serotonin + dopamine) / 2.0
    }

    /// Получить модуляцию возбуждения
    pub fn get_arousal_modulation(&self) -> f64 {
        let norepinephrine = self.levels.get("Норэпинефрин").unwrap_or(&0.5);
        let gaba = self.levels.get("ГАМК").unwrap_or(&0.5);

        norepinephrine - gaba * 0.5
    }

    /// Статистика химии
    pub fn get_chemistry_report(&self) -> Vec<(String, f64)> {
        let mut report: Vec<_> = self.levels.iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        report.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        report
    }

    /// Получить доминирующий нейромедиатор
    pub fn get_dominant_neurotransmitter(&self) -> Option<(String, f64)> {
        self.levels.iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(k, v)| (k.clone(), *v))
    }
}

impl Default for ChemistryModule {
    fn default() -> Self {
        Self::new()
    }
}
