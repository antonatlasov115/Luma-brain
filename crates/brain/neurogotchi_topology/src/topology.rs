//! Топология NeuroGotchi - 5-слойная когнитивная архитектура

use crate::layer::{Layer, LayerType, Neuron, NeuronType};
use crate::connection::{Connection, ConnectionMatrix, ConnectionType};
use serde::{Deserialize, Serialize};
use rand::Rng;
use neuromodulation::NeuromodulationState;

/// Главная структура топологии
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuroGotchiTopology {
    /// Слои нейронов
    pub layers: Vec<Layer>,

    /// Матрица связей
    pub connections: ConnectionMatrix,

    /// Глобальный счетчик нейронов
    pub total_neurons: usize,

    /// Текущее время симуляции (мс)
    pub current_time: f64,

    /// Нейрохимическая модуляция (legacy, deprecated)
    #[deprecated(note = "Use neuromodulation instead")]
    pub modulation: f64,

    /// Состояние нейромодуляции
    pub neuromodulation: NeuromodulationState,
}

impl NeuroGotchiTopology {
    /// Создать новую топологию с гибридной инициализацией
    ///
    /// # Архитектура
    ///
    /// ```text
    /// L1_Sensory (100) → L2_Associative (500) → L3_Workspace (200)
    ///                                              ↓
    ///                                         L4_InnerVoice (150)
    ///                                              ↓
    ///                    L1_Sensory ← L4_InnerVoice → L5_Motor (100)
    ///                    (feedback)                    (feedforward)
    ///                                                       ↓
    ///                                                   WTA + Speech
    /// ```
    ///
    /// # Инициализация
    ///
    /// - 10% врожденных связей (hardcoded инстинкты)
    /// - 90% пластичных связей (случайный шум + STDP)
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        // Создаем слои
        let mut layers = Vec::new();
        let mut neuron_id_offset = 0;

        // L1: Sensory (100 нейронов, 80% возбуждающих)
        let mut l1 = Layer::new(LayerType::Sensory, 100, 0.8);
        Self::assign_global_ids(&mut l1, &mut neuron_id_offset);
        layers.push(l1);

        // L2: Associative (500 нейронов, 80% возбуждающих)
        let mut l2 = Layer::new(LayerType::Associative, 500, 0.8);
        Self::assign_global_ids(&mut l2, &mut neuron_id_offset);
        layers.push(l2);

        // L3: Workspace (200 нейронов, 80% возбуждающих)
        let mut l3 = Layer::new(LayerType::Workspace, 200, 0.8);
        Self::assign_global_ids(&mut l3, &mut neuron_id_offset);
        layers.push(l3);

        // L4: InnerVoice (150 нейронов, 80% возбуждающих)
        let mut l4 = Layer::new(LayerType::InnerVoice, 150, 0.8);
        Self::assign_global_ids(&mut l4, &mut neuron_id_offset);
        layers.push(l4);

        // L5: Motor (100 нейронов, 80% возбуждающих)
        let mut l5 = Layer::new(LayerType::Motor, 100, 0.8);
        Self::assign_global_ids(&mut l5, &mut neuron_id_offset);
        layers.push(l5);

        let total_neurons = neuron_id_offset;

        // Создаем матрицу связей
        let mut connections = ConnectionMatrix::new();

        // L1 → L2 (feedforward, разреженные)
        Self::create_sparse_connections(
            &layers[0], &layers[1],
            0.15, // 15% связность
            0.3,  // средний вес
            0.1,  // вариация
            ConnectionType::Feedforward,
            0.1,  // 10% врожденных
            &mut connections,
            &mut rng,
        );

        // L2 → L3 (feedforward, разреженные)
        Self::create_sparse_connections(
            &layers[1], &layers[2],
            0.1,  // 10% связность
            0.4,  // средний вес
            0.15, // вариация
            ConnectionType::Feedforward,
            0.1,  // 10% врожденных
            &mut connections,
            &mut rng,
        );

        // L3 → L4 (feedforward, разреженные)
        Self::create_sparse_connections(
            &layers[2], &layers[3],
            0.12, // 12% связность
            0.35, // средний вес
            0.1,  // вариация
            ConnectionType::Feedforward,
            0.1,  // 10% врожденных
            &mut connections,
            &mut rng,
        );

        // L4 → L5 (feedforward, высокий порог)
        Self::create_sparse_connections(
            &layers[3], &layers[4],
            0.2,  // 20% связность
            0.5,  // средний вес
            0.15, // вариация
            ConnectionType::Feedforward,
            0.15, // 15% врожденных (речевые инстинкты)
            &mut connections,
            &mut rng,
        );

        // L4 → L1 (feedback, эхо мыслей)
        Self::create_sparse_connections(
            &layers[3], &layers[0],
            0.05, // 5% связность (слабая обратная связь)
            0.15, // слабый вес
            0.05, // малая вариация
            ConnectionType::Feedback,
            0.0,  // 0% врожденных (развивается через опыт)
            &mut connections,
            &mut rng,
        );

        // L5: Lateral inhibition (WTA)
        Self::create_lateral_inhibition(
            &layers[4],
            0.3,  // 30% связность
            -0.4, // тормозящий вес
            &mut connections,
            &mut rng,
        );

        Self {
            layers,
            connections,
            total_neurons,
            current_time: 0.0,
            modulation: 1.0,
            neuromodulation: NeuromodulationState::new(0.0, 12.0), // Рождение в полдень
        }
    }

    /// Назначить глобальные ID нейронам в слое
    fn assign_global_ids(layer: &mut Layer, offset: &mut usize) {
        for neuron in &mut layer.neurons {
            neuron.id = *offset;
            *offset += 1;
        }
    }

    /// Создать разреженные связи между слоями
    fn create_sparse_connections<R: Rng>(
        pre_layer: &Layer,
        post_layer: &Layer,
        connectivity: f64,
        mean_weight: f64,
        weight_std: f64,
        conn_type: ConnectionType,
        innate_ratio: f64,
        connections: &mut ConnectionMatrix,
        rng: &mut R,
    ) {
        for post_neuron in &post_layer.neurons {
            for pre_neuron in &pre_layer.neurons {
                // Пропускаем с вероятностью (1 - connectivity)
                if rng.gen::<f64>() > connectivity {
                    continue;
                }

                // Только возбуждающие нейроны создают feedforward связи
                if conn_type == ConnectionType::Feedforward &&
                   pre_neuron.neuron_type == NeuronType::Inhibitory {
                    continue;
                }

                // Генерируем вес
                let weight = mean_weight + rng.gen::<f64>() * weight_std * 2.0 - weight_std;
                let weight = weight.max(0.01); // Минимальный вес

                // Определяем, врожденная ли связь
                let is_innate = rng.gen::<f64>() < innate_ratio;

                connections.add_connection(Connection::new(
                    pre_neuron.id,
                    post_neuron.id,
                    weight,
                    conn_type,
                    is_innate,
                ));
            }
        }
    }

    /// Создать латеральное торможение внутри слоя (для WTA)
    fn create_lateral_inhibition<R: Rng>(
        layer: &Layer,
        connectivity: f64,
        inhibition_weight: f64,
        connections: &mut ConnectionMatrix,
        rng: &mut R,
    ) {
        // Только тормозящие нейроны создают латеральное торможение
        let inhibitory_neurons: Vec<_> = layer.neurons
            .iter()
            .filter(|n| n.neuron_type == NeuronType::Inhibitory)
            .collect();

        for inhibitory in &inhibitory_neurons {
            for target in &layer.neurons {
                // Не тормозим сами себя
                if inhibitory.id == target.id {
                    continue;
                }

                // Пропускаем с вероятностью (1 - connectivity)
                if rng.gen::<f64>() > connectivity {
                    continue;
                }

                connections.add_connection(Connection::new(
                    inhibitory.id,
                    target.id,
                    inhibition_weight,
                    ConnectionType::LateralInhibition,
                    true, // Врожденное торможение
                ));
            }
        }
    }

    /// Получить слой по типу
    pub fn get_layer(&self, layer_type: LayerType) -> Option<&Layer> {
        self.layers.iter().find(|l| l.layer_type == layer_type)
    }

    /// Получить мутабельный слой по типу
    pub fn get_layer_mut(&mut self, layer_type: LayerType) -> Option<&mut Layer> {
        self.layers.iter_mut().find(|l| l.layer_type == layer_type)
    }

    /// Получить нейрон по глобальному ID
    pub fn get_neuron(&self, neuron_id: usize) -> Option<&Neuron> {
        for layer in &self.layers {
            if let Some(neuron) = layer.neurons.iter().find(|n| n.id == neuron_id) {
                return Some(neuron);
            }
        }
        None
    }

    /// Получить мутабельный нейрон по глобальному ID
    pub fn get_neuron_mut(&mut self, neuron_id: usize) -> Option<&mut Neuron> {
        for layer in &mut self.layers {
            if let Some(neuron) = layer.neurons.iter_mut().find(|n| n.id == neuron_id) {
                return Some(neuron);
            }
        }
        None
    }

    /// Выполнить один шаг симуляции (1 мс)
    ///
    /// # Алгоритм
    ///
    /// 1. Обновить нейромодуляцию
    /// 2. Вычислить входные токи для всех нейронов
    /// 3. Обновить состояние нейронов с модуляцией
    /// 4. Собрать спайки
    /// 5. Применить STDP с модуляцией обучения
    /// 6. Записать спайки в историю
    pub fn step(&mut self, external_currents: &[f64]) -> Vec<(usize, f64)> {
        // Шаг 0: Получить текущую активность для нейромодуляции
        let current_activity = self.get_layer_activities(100.0)
            .iter()
            .map(|(_, a)| a)
            .sum::<f64>() / 5.0; // Средняя активность по всем слоям

        // Шаг 1: Вычислить синаптические токи для всех нейронов
        let mut synaptic_currents = vec![0.0; self.total_neurons];

        for neuron_id in 0..self.total_neurons {
            synaptic_currents[neuron_id] = self.compute_synaptic_current(neuron_id);
        }

        // Получить модификаторы от нейромодуляции
        let activity_mod = self.neuromodulation.get_activity_modifier();
        let threshold_mod = self.neuromodulation.get_threshold_modifier();
        let signal_mod = self.neuromodulation.get_signal_modifier();

        // Шаг 2: Обработать все нейроны
        let mut spikes = Vec::new();

        for (layer_idx, layer) in self.layers.iter_mut().enumerate() {
            for neuron in &mut layer.neurons {
                // Пропускаем сенсорный слой во время сна
                if layer_idx == 0 && self.neuromodulation.is_sleeping() {
                    continue;
                }

                // Получить внешний ток (для сенсорного слоя)
                let external_current = if layer_idx == 0 && neuron.id < external_currents.len() {
                    external_currents[neuron.id] * signal_mod
                } else {
                    0.0
                };

                // Получить синаптический ток
                let synaptic_current = synaptic_currents[neuron.id];

                // Применить модуляцию активности
                let total_current = (external_current + synaptic_current) * activity_mod;

                // Применить модуляцию порога
                let modulated_threshold = neuron.threshold * threshold_mod;
                let original_threshold = neuron.threshold;
                neuron.threshold = modulated_threshold;

                // Обработать нейрон
                let fired = neuron.process(total_current, self.modulation);

                // Восстановить порог
                neuron.threshold = original_threshold;

                if fired {
                    neuron.record_spike(self.current_time);
                    spikes.push((neuron.id, self.current_time));
                }
            }
        }

        // Шаг 3: Применить STDP с модуляцией обучения
        if !spikes.is_empty() {
            let learning_mod = self.neuromodulation.get_learning_modifier();

            // Применяем модуляцию к скорости обучения всех связей
            for conn in &mut self.connections.connections {
                if !conn.weight.is_innate {
                    let original_lr = conn.weight.learning_rate;
                    conn.weight.learning_rate = original_lr * learning_mod;
                }
            }

            self.connections.apply_stdp_batch(&spikes);

            // Восстанавливаем оригинальные скорости обучения
            for conn in &mut self.connections.connections {
                if !conn.weight.is_innate {
                    conn.weight.learning_rate /= learning_mod;
                }
            }
        }

        // Шаг 4: Обновить нейромодуляцию
        self.neuromodulation.update(self.current_time, spikes.len(), current_activity);

        // Шаг 5: Обновить время
        self.current_time += 1.0;

        spikes
    }

    /// Вычислить синаптический ток для нейрона
    fn compute_synaptic_current(&self, neuron_id: usize) -> f64 {
        let connections = self.connections.get_connections_for_neuron(neuron_id);

        let mut total_current = 0.0;

        for conn in connections {
            // Проверяем, был ли спайк у пре-синаптического нейрона
            if let Some(pre_neuron) = self.get_neuron(conn.pre_neuron_id) {
                // Проверяем последний спайк с учетом задержки
                if let Some(&last_spike) = pre_neuron.spike_history.last() {
                    let time_since_spike = self.current_time - last_spike;

                    // Спайк доходит с задержкой
                    if time_since_spike >= conn.delay && time_since_spike < conn.delay + 2.0 {
                        total_current += conn.compute_current(true);
                    }
                }
            }
        }

        total_current
    }

    /// Получить активность всех слоев
    pub fn get_layer_activities(&self, window: f64) -> Vec<(LayerType, f64)> {
        self.layers
            .iter()
            .map(|layer| {
                let activity = layer.get_activity(self.current_time, window);
                (layer.layer_type, activity)
            })
            .collect()
    }

    /// Получить средний потенциал всех слоев
    pub fn get_layer_potentials(&self) -> Vec<(LayerType, f64)> {
        self.layers
            .iter()
            .map(|layer| {
                let potential = layer.get_average_potential();
                (layer.layer_type, potential)
            })
            .collect()
    }

    /// Установить нейрохимическую модуляцию
    pub fn set_modulation(&mut self, modulation: f64) {
        self.modulation = modulation.clamp(0.1, 3.0);
    }

    /// Получить статистику топологии
    pub fn get_stats(&self) -> TopologyStats {
        let total_connections = self.connections.len();
        let innate_connections = self.connections.connections
            .iter()
            .filter(|c| c.weight.is_innate)
            .count();

        let plastic_connections = total_connections - innate_connections;

        let avg_weight: f64 = self.connections.connections
            .iter()
            .map(|c| c.weight.weight.abs())
            .sum::<f64>() / total_connections as f64;

        TopologyStats {
            total_neurons: self.total_neurons,
            total_connections,
            innate_connections,
            plastic_connections,
            avg_weight,
            current_time: self.current_time,
        }
    }

    /// Получить статус нейромодуляции
    pub fn get_neuromodulation_status(&self) -> String {
        self.neuromodulation.get_status(self.current_time)
    }

    /// Выброс дофамина (награда)
    pub fn reward(&mut self, amount: f64) {
        self.neuromodulation.modulators.reward(amount);
    }

    /// Выброс кортизола (стресс)
    pub fn stress(&mut self, amount: f64) {
        self.neuromodulation.modulators.stress(amount);
    }

    /// Изменение настроения
    pub fn mood_shift(&mut self, amount: f64) {
        self.neuromodulation.modulators.mood_shift(amount);
    }

    /// Проверить, спит ли сейчас
    pub fn is_sleeping(&self) -> bool {
        self.neuromodulation.is_sleeping()
    }

    /// Получить уровень усталости (0.0 - 1.0)
    pub fn get_fatigue_level(&self) -> f64 {
        self.neuromodulation.adenosine.get_fatigue_level()
    }

    /// Получить эмоциональное состояние (-1.0 до 1.0)
    pub fn get_emotional_valence(&self) -> f64 {
        self.neuromodulation.modulators.get_emotional_valence()
    }
}

/// Статистика топологии
#[derive(Debug, Clone)]
pub struct TopologyStats {
    pub total_neurons: usize,
    pub total_connections: usize,
    pub innate_connections: usize,
    pub plastic_connections: usize,
    pub avg_weight: f64,
    pub current_time: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topology_creation() {
        let topology = NeuroGotchiTopology::new();

        assert_eq!(topology.layers.len(), 5);
        assert_eq!(topology.total_neurons, 100 + 500 + 200 + 150 + 100);
        assert!(topology.connections.len() > 0);
    }

    #[test]
    fn test_layer_sizes() {
        let topology = NeuroGotchiTopology::new();

        let l1 = topology.get_layer(LayerType::Sensory).unwrap();
        assert_eq!(l1.size, 100);

        let l2 = topology.get_layer(LayerType::Associative).unwrap();
        assert_eq!(l2.size, 500);

        let l3 = topology.get_layer(LayerType::Workspace).unwrap();
        assert_eq!(l3.size, 200);

        let l4 = topology.get_layer(LayerType::InnerVoice).unwrap();
        assert_eq!(l4.size, 150);

        let l5 = topology.get_layer(LayerType::Motor).unwrap();
        assert_eq!(l5.size, 100);
    }

    #[test]
    fn test_global_ids() {
        let topology = NeuroGotchiTopology::new();

        // Проверяем, что ID уникальны и последовательны
        let mut all_ids: Vec<usize> = Vec::new();

        for layer in &topology.layers {
            for neuron in &layer.neurons {
                all_ids.push(neuron.id);
            }
        }

        all_ids.sort();

        for (i, &id) in all_ids.iter().enumerate() {
            assert_eq!(id, i);
        }
    }

    #[test]
    fn test_get_neuron() {
        let topology = NeuroGotchiTopology::new();

        let neuron = topology.get_neuron(0);
        assert!(neuron.is_some());
        assert_eq!(neuron.unwrap().id, 0);

        let no_neuron = topology.get_neuron(10000);
        assert!(no_neuron.is_none());
    }

    #[test]
    fn test_step_simulation() {
        let mut topology = NeuroGotchiTopology::new();

        // Подаем сильный ток на сенсорный слой
        let mut external_currents = vec![0.0; 100];
        external_currents[0] = 2.0;

        let spikes = topology.step(&external_currents);

        // Должен быть хотя бы один спайк
        assert!(!spikes.is_empty());
        assert_eq!(topology.current_time, 1.0);
    }

    #[test]
    fn test_layer_activities() {
        let mut topology = NeuroGotchiTopology::new();

        // Симулируем несколько шагов
        let external_currents = vec![1.5; 100];

        for _ in 0..100 {
            topology.step(&external_currents);
        }

        let activities = topology.get_layer_activities(100.0);
        assert_eq!(activities.len(), 5);

        // L1 должен быть активен
        let l1_activity = activities.iter()
            .find(|(t, _)| *t == LayerType::Sensory)
            .unwrap()
            .1;

        assert!(l1_activity > 0.0);
    }

    #[test]
    fn test_modulation() {
        let mut topology = NeuroGotchiTopology::new();

        topology.set_modulation(2.0);
        assert_eq!(topology.modulation, 2.0);

        // Проверяем границы
        topology.set_modulation(10.0);
        assert_eq!(topology.modulation, 3.0);

        topology.set_modulation(0.01);
        assert_eq!(topology.modulation, 0.1);
    }

    #[test]
    fn test_stats() {
        let topology = NeuroGotchiTopology::new();
        let stats = topology.get_stats();

        assert_eq!(stats.total_neurons, 1050);
        assert!(stats.total_connections > 0);
        assert!(stats.innate_connections > 0);
        assert!(stats.plastic_connections > 0);
        assert!(stats.avg_weight > 0.0);
    }

    #[test]
    fn test_innate_ratio() {
        let topology = NeuroGotchiTopology::new();
        let stats = topology.get_stats();

        let innate_ratio = stats.innate_connections as f64 / stats.total_connections as f64;

        // Должно быть примерно 10% врожденных связей
        assert!(innate_ratio > 0.05 && innate_ratio < 0.20);
    }
}
