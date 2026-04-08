use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Нейромедиатор (химический сигнал)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
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

/// Улучшенная химическая система мозга с взаимодействиями
#[derive(Clone, Serialize, Deserialize)]
pub struct ChemistryModule {
    pub levels: HashMap<String, f64>,
    pub baseline: HashMap<String, f64>,

    // НОВОЕ: Рецепторы
    pub receptor_sensitivity: HashMap<String, f64>,

    // НОВОЕ: Обратный захват (транспортеры)
    pub reuptake_rates: HashMap<String, f64>,

    // НОВОЕ: Ферменты деградации
    pub mao_activity: f64,  // Моноаминоксидаза
    pub comt_activity: f64, // COMT
    pub ache_activity: f64, // Ацетилхолинэстераза

    pub decay_rate: f64,
}

impl ChemistryModule {
    pub fn new() -> Self {
        let mut levels = HashMap::new();
        let mut baseline = HashMap::new();
        let mut receptor_sensitivity = HashMap::new();
        let mut reuptake_rates = HashMap::new();

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
            baseline.insert(name.clone(), 0.5);
            receptor_sensitivity.insert(name.clone(), 1.0);

            // Разные скорости обратного захвата
            let rate = match nt {
                Neurotransmitter::Dopamine => 0.3,
                Neurotransmitter::Serotonin => 0.25,
                Neurotransmitter::Norepinephrine => 0.35,
                Neurotransmitter::GABA => 0.4,
                Neurotransmitter::Glutamate => 0.5,
                Neurotransmitter::Acetylcholine => 0.8, // Очень быстрый!
            };
            reuptake_rates.insert(name, rate);
        }

        Self {
            levels,
            baseline,
            receptor_sensitivity,
            reuptake_rates,
            mao_activity: 0.15,
            comt_activity: 0.1,
            ache_activity: 0.6,
            decay_rate: 0.95,
        }
    }

    /// Обновление химии с взаимодействиями
    pub fn update(&mut self, hunger: f64, energy: f64, happiness: f64, stimulation: f64, elapsed: f64) {
        // 1. Базовое обновление целевых уровней
        self.update_targets(hunger, energy, happiness, stimulation, elapsed);

        // 2. НОВОЕ: Обратный захват
        self.reuptake(elapsed);

        // 3. НОВОЕ: Ферментативная деградация
        self.enzymatic_degradation(elapsed);

        // 4. НОВОЕ: Взаимодействия между медиаторами
        self.neurotransmitter_interactions();

        // 5. НОВОЕ: Десенситизация рецепторов
        self.receptor_desensitization(elapsed);

        // 6. НОВОЕ: Гомеостатическая регуляция
        self.homeostatic_regulation();
    }

    fn update_targets(&mut self, hunger: f64, energy: f64, happiness: f64, stimulation: f64, elapsed: f64) {
        // Дофамин
        let dopamine_target = if happiness > 70.0 {
            0.8
        } else if hunger < 30.0 {
            0.3
        } else {
            0.5
        };
        self.adjust_level("Дофамин", dopamine_target, elapsed);

        // Серотонин
        let serotonin_target = happiness / 100.0;
        self.adjust_level("Серотонин", serotonin_target, elapsed);

        // Норэпинефрин
        let norepinephrine_target = if stimulation > 60.0 {
            0.7
        } else if energy < 30.0 {
            0.2
        } else {
            0.4
        };
        self.adjust_level("Норэпинефрин", norepinephrine_target, elapsed);

        // ГАМК
        let gaba_target = if energy < 30.0 {
            0.8
        } else if stimulation > 70.0 {
            0.3
        } else {
            0.5
        };
        self.adjust_level("ГАМК", gaba_target, elapsed);

        // Глутамат
        let glutamate_target = if stimulation > 50.0 {
            0.7
        } else {
            0.4
        };
        self.adjust_level("Глутамат", glutamate_target, elapsed);

        // Ацетилхолин
        let acetylcholine_target = if stimulation > 60.0 && energy > 40.0 {
            0.7
        } else {
            0.4
        };
        self.adjust_level("Ацетилхолин", acetylcholine_target, elapsed);
    }

    /// НОВОЕ: Обратный захват (транспортеры очищают синапс)
    fn reuptake(&mut self, elapsed: f64) {
        for (name, level) in self.levels.iter_mut() {
            if let Some(rate) = self.reuptake_rates.get(name) {
                // Экспоненциальный спад
                *level *= (-rate * elapsed * 0.1).exp();
            }
        }
    }

    /// НОВОЕ: Ферментативная деградация
    fn enzymatic_degradation(&mut self, elapsed: f64) {
        // MAO разрушает моноамины (дофамин, серотонин, норэпинефрин)
        let mao_targets = ["Дофамин", "Серотонин", "Норэпинефрин"];
        for target in mao_targets {
            if let Some(level) = self.levels.get_mut(target) {
                *level *= 1.0 - (self.mao_activity * elapsed * 0.05);
            }
        }

        // COMT разрушает дофамин (дополнительно)
        if let Some(dopamine) = self.levels.get_mut("Дофамин") {
            *dopamine *= 1.0 - (self.comt_activity * elapsed * 0.03);
        }

        // AChE разрушает ацетилхолин (очень быстро!)
        if let Some(ach) = self.levels.get_mut("Ацетилхолин") {
            *ach *= 1.0 - (self.ache_activity * elapsed * 0.2);
        }
    }

    /// НОВОЕ: Взаимодействия между медиаторами
    fn neurotransmitter_interactions(&mut self) {
        let dopamine = *self.levels.get("Дофамин").unwrap_or(&0.5);
        let serotonin = *self.levels.get("Серотонин").unwrap_or(&0.5);
        let norepinephrine = *self.levels.get("Норэпинефрин").unwrap_or(&0.5);
        let gaba = *self.levels.get("ГАМК").unwrap_or(&0.5);
        let glutamate = *self.levels.get("Глутамат").unwrap_or(&0.5);

        // Дофамин-серотонин антагонизм
        if dopamine > 0.7 {
            let inhibition = (dopamine - 0.7) * 0.2;
            self.modulate_level("Серотонин", -inhibition);
        }

        if serotonin > 0.7 {
            let inhibition = (serotonin - 0.7) * 0.15;
            self.modulate_level("Дофамин", -inhibition);
        }

        // Норэпинефрин усиливает дофамин (синергия)
        if norepinephrine > 0.6 {
            let boost = (norepinephrine - 0.6) * 0.1;
            self.modulate_level("Дофамин", boost);
        }

        // ГАМК-глутамат баланс (возбуждение/торможение)
        let ei_balance = glutamate - gaba;
        if ei_balance > 0.4 {
            // Слишком много возбуждения → усилить ГАМК
            self.modulate_level("ГАМК", 0.05);
        } else if ei_balance < -0.4 {
            // Слишком много торможения → усилить глутамат
            self.modulate_level("Глутамат", 0.05);
        }
    }

    /// НОВОЕ: Десенситизация рецепторов
    fn receptor_desensitization(&mut self, elapsed: f64) {
        for (name, level) in self.levels.iter() {
            if let Some(sensitivity) = self.receptor_sensitivity.get_mut(name) {
                if *level > 0.8 {
                    // Хронически высокий → снижение чувствительности
                    *sensitivity *= 1.0 - (elapsed * 0.005);
                    *sensitivity = sensitivity.max(0.3);
                } else if *level < 0.3 {
                    // Хронически низкий → повышение чувствительности (upregulation)
                    *sensitivity += elapsed * 0.005;
                    *sensitivity = sensitivity.min(1.5);
                } else {
                    // Нормализация к 1.0
                    if *sensitivity > 1.0 {
                        *sensitivity -= elapsed * 0.002;
                    } else if *sensitivity < 1.0 {
                        *sensitivity += elapsed * 0.002;
                    }
                }
            }
        }
    }

    /// НОВОЕ: Гомеостатическая регуляция
    fn homeostatic_regulation(&mut self) {
        for (name, level) in self.levels.iter_mut() {
            if let Some(baseline) = self.baseline.get(name) {
                // Медленное возвращение к базовому уровню
                let diff = baseline - *level;
                *level += diff * 0.01;
            }
        }
    }

    fn adjust_level(&mut self, name: &str, target: f64, elapsed: f64) {
        if let Some(current) = self.levels.get_mut(name) {
            let diff = target - *current;
            *current += diff * elapsed * 0.1;
            *current = current.clamp(0.0, 1.0);
        }
    }

    fn modulate_level(&mut self, name: &str, amount: f64) {
        if let Some(level) = self.levels.get_mut(name) {
            *level = (*level + amount).clamp(0.0, 1.0);
        }
    }

    /// Выброс нейромедиатора
    pub fn release(&mut self, neurotransmitter: Neurotransmitter, amount: f64) {
        let name = neurotransmitter.name();
        if let Some(level) = self.levels.get_mut(name) {
            *level = (*level + amount).min(1.0);
        }
    }

    /// НОВОЕ: Эффективный уровень (с учетом рецепторов)
    pub fn get_effective_level(&self, name: &str) -> f64 {
        let level = self.levels.get(name).unwrap_or(&0.5);
        let sensitivity = self.receptor_sensitivity.get(name).unwrap_or(&1.0);
        level * sensitivity
    }

    /// Модуляция обучения
    pub fn get_learning_modulation(&self) -> f64 {
        let dopamine = self.get_effective_level("Дофамин");
        let glutamate = self.get_effective_level("Глутамат");
        let acetylcholine = self.get_effective_level("Ацетилхолин");

        (dopamine + glutamate + acetylcholine) / 3.0
    }

    /// Модуляция настроения
    pub fn get_mood_modulation(&self) -> f64 {
        let serotonin = self.get_effective_level("Серотонин");
        let dopamine = self.get_effective_level("Дофамин");

        (serotonin + dopamine) / 2.0
    }

    /// Модуляция возбуждения
    pub fn get_arousal_modulation(&self) -> f64 {
        let norepinephrine = self.get_effective_level("Норэпинефрин");
        let gaba = self.get_effective_level("ГАМК");

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

    /// НОВОЕ: Расширенный отчет с рецепторами
    pub fn get_detailed_report(&self) -> Vec<(String, f64, f64)> {
        let mut report = Vec::new();
        for (name, level) in self.levels.iter() {
            let sensitivity = self.receptor_sensitivity.get(name).unwrap_or(&1.0);
            report.push((name.clone(), *level, *sensitivity));
        }
        report.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        report
    }

    /// Доминирующий нейромедиатор
    pub fn get_dominant_neurotransmitter(&self) -> Option<(String, f64)> {
        self.levels.iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(k, v)| (k.clone(), *v))
    }
}
