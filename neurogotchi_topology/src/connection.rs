//! Синаптические связи между нейронами

use serde::{Deserialize, Serialize};

/// Тип связи
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionType {
    /// Прямая связь (feedforward)
    Feedforward,

    /// Обратная связь (feedback)
    Feedback,

    /// Латеральное торможение (lateral inhibition)
    LateralInhibition,

    /// STDP-пластичная связь
    Plastic,
}

/// Синаптический вес
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynapticWeight {
    /// Текущий вес
    pub weight: f64,

    /// Минимальный вес
    pub min_weight: f64,

    /// Максимальный вес
    pub max_weight: f64,

    /// Скорость обучения (для STDP)
    pub learning_rate: f64,

    /// Тип связи
    pub connection_type: ConnectionType,

    /// Врожденная связь (не обучается)
    pub is_innate: bool,
}

impl SynapticWeight {
    /// Создать новый вес
    pub fn new(weight: f64, connection_type: ConnectionType, is_innate: bool) -> Self {
        let (min_weight, max_weight) = match connection_type {
            ConnectionType::Feedforward => (0.0, 2.0),
            ConnectionType::Feedback => (0.0, 0.5),
            ConnectionType::LateralInhibition => (-2.0, 0.0),
            ConnectionType::Plastic => (0.0, 3.0),
        };

        Self {
            weight: weight.clamp(min_weight, max_weight),
            min_weight,
            max_weight,
            learning_rate: if is_innate { 0.0 } else { 0.01 },
            connection_type,
            is_innate,
        }
    }

    /// Применить STDP (Spike-Timing-Dependent Plasticity)
    ///
    /// # Параметры
    /// - `dt`: Разница времени между пре- и пост-синаптическим спайком (мс)
    ///   - dt > 0: пре-спайк раньше пост-спайка → усиление (LTP)
    ///   - dt < 0: пост-спайк раньше пре-спайка → ослабление (LTD)
    pub fn apply_stdp(&mut self, dt: f64) {
        if self.is_innate {
            return;
        }

        // STDP окно: ±20 мс
        let tau = 20.0;

        let delta_w = if dt > 0.0 {
            // LTP (Long-Term Potentiation)
            self.learning_rate * (-dt / tau).exp()
        } else {
            // LTD (Long-Term Depression)
            -self.learning_rate * (dt / tau).exp()
        };

        self.weight = (self.weight + delta_w).clamp(self.min_weight, self.max_weight);
    }

    /// Применить модуляцию (дофамин, серотонин)
    pub fn apply_modulation(&mut self, modulation: f64) {
        if self.is_innate {
            return;
        }

        // Модуляция усиливает или ослабляет обучение
        let modulated_lr = self.learning_rate * modulation;
        self.learning_rate = modulated_lr.clamp(0.0, 0.1);
    }

    /// Получить эффективный вес с учетом типа связи
    pub fn effective_weight(&self) -> f64 {
        match self.connection_type {
            ConnectionType::LateralInhibition => self.weight.min(0.0),
            _ => self.weight.max(0.0),
        }
    }
}

/// Связь между двумя нейронами
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    /// ID пре-синаптического нейрона
    pub pre_neuron_id: usize,

    /// ID пост-синаптического нейрона
    pub post_neuron_id: usize,

    /// Синаптический вес
    pub weight: SynapticWeight,

    /// Задержка проведения (мс)
    pub delay: f64,
}

impl Connection {
    pub fn new(
        pre_neuron_id: usize,
        post_neuron_id: usize,
        weight: f64,
        connection_type: ConnectionType,
        is_innate: bool,
    ) -> Self {
        // Задержка зависит от типа связи
        let delay = match connection_type {
            ConnectionType::Feedforward => 1.0,
            ConnectionType::Feedback => 5.0,
            ConnectionType::LateralInhibition => 0.5,
            ConnectionType::Plastic => 1.0,
        };

        Self {
            pre_neuron_id,
            post_neuron_id,
            weight: SynapticWeight::new(weight, connection_type, is_innate),
            delay,
        }
    }

    /// Вычислить входной ток для пост-синаптического нейрона
    pub fn compute_current(&self, pre_spike: bool) -> f64 {
        if pre_spike {
            self.weight.effective_weight()
        } else {
            0.0
        }
    }
}

/// Матрица связей между слоями
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionMatrix {
    /// Все связи
    pub connections: Vec<Connection>,

    /// Индекс для быстрого поиска связей по пост-синаптическому нейрону
    /// post_neuron_id -> [indices in connections]
    #[serde(skip)]
    pub post_index: Vec<Vec<usize>>,
}

impl ConnectionMatrix {
    pub fn new() -> Self {
        Self {
            connections: Vec::new(),
            post_index: Vec::new(),
        }
    }

    /// Добавить связь
    pub fn add_connection(&mut self, connection: Connection) {
        let post_id = connection.post_neuron_id;
        let conn_idx = self.connections.len();

        self.connections.push(connection);

        // Обновляем индекс
        while self.post_index.len() <= post_id {
            self.post_index.push(Vec::new());
        }
        self.post_index[post_id].push(conn_idx);
    }

    /// Получить все связи для пост-синаптического нейрона
    pub fn get_connections_for_neuron(&self, post_neuron_id: usize) -> Vec<&Connection> {
        if post_neuron_id >= self.post_index.len() {
            return Vec::new();
        }

        self.post_index[post_neuron_id]
            .iter()
            .map(|&idx| &self.connections[idx])
            .collect()
    }

    /// Получить мутабельные связи для пост-синаптического нейрона
    pub fn get_connections_for_neuron_mut(&mut self, post_neuron_id: usize) -> Vec<&mut Connection> {
        if post_neuron_id >= self.post_index.len() {
            return Vec::new();
        }

        let indices: Vec<usize> = self.post_index[post_neuron_id].clone();

        indices
            .into_iter()
            .map(|idx| {
                // SAFETY: Мы гарантируем, что индексы уникальны
                unsafe {
                    let ptr = self.connections.as_mut_ptr();
                    &mut *ptr.add(idx)
                }
            })
            .collect()
    }

    /// Пересоздать индекс (после десериализации)
    pub fn rebuild_index(&mut self) {
        self.post_index.clear();

        for (idx, conn) in self.connections.iter().enumerate() {
            let post_id = conn.post_neuron_id;

            while self.post_index.len() <= post_id {
                self.post_index.push(Vec::new());
            }

            self.post_index[post_id].push(idx);
        }
    }

    /// Получить количество связей
    pub fn len(&self) -> usize {
        self.connections.len()
    }

    /// Проверить, пуста ли матрица
    pub fn is_empty(&self) -> bool {
        self.connections.is_empty()
    }

    /// Применить STDP ко всем пластичным связям
    pub fn apply_stdp_batch(&mut self, spike_times: &[(usize, f64)]) {
        // spike_times: [(neuron_id, spike_time), ...]

        for conn in &mut self.connections {
            if conn.weight.is_innate {
                continue;
            }

            // Найти времена спайков пре- и пост-синаптических нейронов
            let pre_times: Vec<f64> = spike_times
                .iter()
                .filter(|(id, _)| *id == conn.pre_neuron_id)
                .map(|(_, t)| *t)
                .collect();

            let post_times: Vec<f64> = spike_times
                .iter()
                .filter(|(id, _)| *id == conn.post_neuron_id)
                .map(|(_, t)| *t)
                .collect();

            // Применяем STDP для всех пар спайков
            for &pre_t in &pre_times {
                for &post_t in &post_times {
                    let dt = post_t - pre_t;

                    // Применяем только если спайки близки по времени
                    if dt.abs() < 50.0 {
                        conn.weight.apply_stdp(dt);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synaptic_weight_creation() {
        let weight = SynapticWeight::new(0.5, ConnectionType::Feedforward, false);
        assert_eq!(weight.weight, 0.5);
        assert_eq!(weight.min_weight, 0.0);
        assert_eq!(weight.max_weight, 2.0);
        assert!(!weight.is_innate);
    }

    #[test]
    fn test_stdp_ltp() {
        let mut weight = SynapticWeight::new(0.5, ConnectionType::Plastic, false);
        let initial = weight.weight;

        // Пре-спайк раньше пост-спайка → усиление
        weight.apply_stdp(10.0);

        assert!(weight.weight > initial);
    }

    #[test]
    fn test_stdp_ltd() {
        let mut weight = SynapticWeight::new(0.5, ConnectionType::Plastic, false);
        let initial = weight.weight;

        // Пост-спайк раньше пре-спайка → ослабление
        weight.apply_stdp(-10.0);

        assert!(weight.weight < initial);
    }

    #[test]
    fn test_innate_no_learning() {
        let mut weight = SynapticWeight::new(0.5, ConnectionType::Feedforward, true);
        let initial = weight.weight;

        weight.apply_stdp(10.0);

        assert_eq!(weight.weight, initial);
    }

    #[test]
    fn test_connection_creation() {
        let conn = Connection::new(0, 1, 0.5, ConnectionType::Feedforward, false);
        assert_eq!(conn.pre_neuron_id, 0);
        assert_eq!(conn.post_neuron_id, 1);
        assert_eq!(conn.delay, 1.0);
    }

    #[test]
    fn test_compute_current() {
        let conn = Connection::new(0, 1, 0.5, ConnectionType::Feedforward, false);

        let current = conn.compute_current(true);
        assert_eq!(current, 0.5);

        let no_current = conn.compute_current(false);
        assert_eq!(no_current, 0.0);
    }

    #[test]
    fn test_lateral_inhibition() {
        let weight = SynapticWeight::new(-0.5, ConnectionType::LateralInhibition, false);
        assert!(weight.effective_weight() <= 0.0);
    }

    #[test]
    fn test_connection_matrix() {
        let mut matrix = ConnectionMatrix::new();

        matrix.add_connection(Connection::new(0, 1, 0.5, ConnectionType::Feedforward, false));
        matrix.add_connection(Connection::new(2, 1, 0.3, ConnectionType::Feedforward, false));

        let conns = matrix.get_connections_for_neuron(1);
        assert_eq!(conns.len(), 2);
    }

    #[test]
    fn test_rebuild_index() {
        let mut matrix = ConnectionMatrix::new();

        matrix.add_connection(Connection::new(0, 1, 0.5, ConnectionType::Feedforward, false));
        matrix.add_connection(Connection::new(2, 1, 0.3, ConnectionType::Feedforward, false));

        // Симулируем десериализацию
        matrix.post_index.clear();
        matrix.rebuild_index();

        let conns = matrix.get_connections_for_neuron(1);
        assert_eq!(conns.len(), 2);
    }

    #[test]
    fn test_stdp_batch() {
        let mut matrix = ConnectionMatrix::new();

        matrix.add_connection(Connection::new(0, 1, 0.5, ConnectionType::Plastic, false));

        let spike_times = vec![
            (0, 10.0),  // Пре-спайк
            (1, 15.0),  // Пост-спайк
        ];

        let initial = matrix.connections[0].weight.weight;
        matrix.apply_stdp_batch(&spike_times);

        // Должно быть усиление (LTP)
        assert!(matrix.connections[0].weight.weight > initial);
    }

    #[test]
    fn test_modulation() {
        let mut weight = SynapticWeight::new(0.5, ConnectionType::Plastic, false);
        let initial_lr = weight.learning_rate;

        // Дофамин усиливает обучение
        weight.apply_modulation(2.0);
        assert!(weight.learning_rate > initial_lr);

        // Серотонин ослабляет обучение
        weight.apply_modulation(0.5);
        assert!(weight.learning_rate < initial_lr * 2.0);
    }
}
