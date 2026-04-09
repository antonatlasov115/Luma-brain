use serde::{Deserialize, Serialize};

/// Состояние нейронной сети для IIT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkState {
    pub nodes: Vec<f64>,           // Состояние каждого узла (0-1)
    pub connections: Vec<Vec<f64>>, // Матрица связей
}

impl NetworkState {
    pub fn new(size: usize) -> Self {
        Self {
            nodes: vec![0.5; size],
            connections: vec![vec![0.0; size]; size],
        }
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    /// Установить случайные связи
    pub fn randomize_connections(&mut self, density: f64) {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        for i in 0..self.size() {
            for j in 0..self.size() {
                if i != j && rng.gen::<f64>() < density {
                    self.connections[i][j] = rng.gen_range(0.1..1.0);
                }
            }
        }
    }

    /// Обновить состояние узлов на основе связей
    pub fn propagate(&mut self) {
        let mut new_nodes = vec![0.0; self.size()];

        for i in 0..self.size() {
            let mut input = 0.0;
            for j in 0..self.size() {
                input += self.nodes[j] * self.connections[j][i];
            }
            // Сигмоида
            new_nodes[i] = 1.0 / (1.0 + (-input).exp());
        }

        self.nodes = new_nodes;
    }
}

/// Разбиение сети (partition)
#[derive(Debug, Clone)]
pub struct Partition {
    pub part_a: Vec<usize>,
    pub part_b: Vec<usize>,
}

/// Integrated Information Theory - мера сознания
#[derive(Clone, Serialize, Deserialize)]
pub struct IITConsciousness {
    /// Φ (phi) - интегрированная информация
    pub phi: f64,

    /// Состояние сети
    pub network: NetworkState,

    /// История Φ
    pub phi_history: Vec<f64>,

    /// Порог для "сознательного" состояния
    pub consciousness_threshold: f64,
}

impl IITConsciousness {
    pub fn new(network_size: usize) -> Self {
        let mut network = NetworkState::new(network_size);
        network.randomize_connections(0.3); // 30% связность

        Self {
            phi: 0.0,
            network,
            phi_history: Vec::new(),
            consciousness_threshold: 0.3,
        }
    }

    /// Обновить состояние сети
    pub fn update_network(&mut self, inputs: &[f64]) {
        // Установить входные узлы
        for (i, &input) in inputs.iter().enumerate() {
            if i < self.network.size() {
                self.network.nodes[i] = input;
            }
        }

        // Распространить активацию
        self.network.propagate();
    }

    /// Вычислить Φ (упрощенная версия)
    pub fn calculate_phi(&mut self) -> f64 {
        // Полный расчет IIT очень сложен, используем упрощение:
        // Φ ≈ связность × разнообразие × интеграция

        let connectivity = self.calculate_connectivity();
        let diversity = self.calculate_diversity();
        let integration = self.calculate_integration();

        self.phi = connectivity * diversity * integration;

        // Добавить в историю
        self.phi_history.push(self.phi);
        if self.phi_history.len() > 100 {
            self.phi_history.remove(0);
        }

        self.phi
    }

    /// Связность сети
    fn calculate_connectivity(&self) -> f64 {
        let mut total_connections = 0.0;
        let mut count = 0;

        for i in 0..self.network.size() {
            for j in 0..self.network.size() {
                if i != j {
                    total_connections += self.network.connections[i][j];
                    count += 1;
                }
            }
        }

        if count > 0 {
            total_connections / count as f64
        } else {
            0.0
        }
    }

    /// Разнообразие состояний
    fn calculate_diversity(&self) -> f64 {
        // Энтропия состояний узлов
        let mut entropy = 0.0;

        for &node in &self.network.nodes {
            if node > 0.0 && node < 1.0 {
                entropy -= node * node.ln() + (1.0 - node) * (1.0 - node).ln();
            }
        }

        (entropy / self.network.size() as f64).min(1.0)
    }

    /// Интеграция (насколько система работает как целое)
    fn calculate_integration(&self) -> f64 {
        // Упрощение: корреляция между узлами
        let mean: f64 = self.network.nodes.iter().sum::<f64>() / self.network.size() as f64;

        let mut variance = 0.0;
        for &node in &self.network.nodes {
            variance += (node - mean).powi(2);
        }
        variance /= self.network.size() as f64;

        // Высокая интеграция = низкая вариация (узлы работают согласованно)
        let integration = 1.0 - variance.sqrt();
        integration.max(0.0).min(1.0)
    }

    /// Проверить, находится ли система в сознательном состоянии
    pub fn is_conscious(&self) -> bool {
        self.phi > self.consciousness_threshold
    }

    /// Получить уровень сознания (0-1)
    pub fn consciousness_level(&self) -> f64 {
        (self.phi / self.consciousness_threshold).min(1.0)
    }

    /// Получить среднее Φ за последние N измерений
    pub fn get_average_phi(&self, n: usize) -> f64 {
        if self.phi_history.is_empty() {
            return 0.0;
        }

        let start = self.phi_history.len().saturating_sub(n);
        let slice = &self.phi_history[start..];
        slice.iter().sum::<f64>() / slice.len() as f64
    }

    /// Получить тренд Φ (растет/падает)
    pub fn get_phi_trend(&self) -> f64 {
        if self.phi_history.len() < 2 {
            return 0.0;
        }

        let recent = self.phi_history[self.phi_history.len() - 1];
        let previous = self.phi_history[self.phi_history.len() - 2];

        recent - previous
    }
}

impl Default for IITConsciousness {
    fn default() -> Self {
        Self::new(20)
    }
}
