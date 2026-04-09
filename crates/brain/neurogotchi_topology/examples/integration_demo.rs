//! Пример интеграции: полный цикл от слова до мысли
//!
//! Демонстрирует работу всей системы:
//! 1. Слово → спайки (encoder)
//! 2. Спайки → сенсорный слой (sensory_input)
//! 3. Распространение по 5 слоям (topology)
//! 4. Мониторинг префронтальной коры (thought_stream)
//! 5. Декодирование моторного выхода (motor_output)

use neurogotchi_topology::{NeuroGotchiTopology, LayerType, DevelopmentManager};
use token_spike_interface::{
    encoder::PhonemeEncoder,
    decoder::SpikeDecoder,
    sensory_input::SensoryInput,
    motor_output::MotorOutput,
    thought_stream::ThoughtStream,
};

fn main() {
    println!("=== NeuroGotchi Integration Demo ===\n");

    // Создаем топологию
    println!("Создание 5-слойной топологии...");
    let mut topology = NeuroGotchiTopology::new();
    let stats = topology.get_stats();
    println!("✓ Нейронов: {}", stats.total_neurons);
    println!("✓ Связей: {} (врожденных: {}, пластичных: {})",
        stats.total_connections,
        stats.innate_connections,
        stats.plastic_connections
    );
    println!();

    // Создаем менеджер развития
    let mut dev_manager = DevelopmentManager::new(0.0);
    println!("Стадия развития: {}\n", dev_manager.get_status(0.0));

    // Создаем компоненты интерфейса
    let encoder = PhonemeEncoder::new();
    let mut decoder = SpikeDecoder::new();
    decoder.add_words(&["привет", "голод", "страх", "мама", "помощь"]);

    let mut sensory_input = SensoryInput::new();

    let mut decoder2 = SpikeDecoder::new();
    decoder2.add_words(&["привет", "голод", "страх", "мама", "помощь"]);
    let mut motor_output = MotorOutput::new(decoder2);

    let mut decoder3 = SpikeDecoder::new();
    decoder3.add_words(&["привет", "голод", "страх", "мама", "помощь"]);
    let mut thought_stream = ThoughtStream::new(decoder3);

    println!("=== Тест 1: Подача слова 'привет' ===\n");

    // Кодируем слово
    let pattern = encoder.encode("привет");
    println!("Закодировано {} спайков за {:.1} мс", pattern.spikes.len(), pattern.duration);

    // Подаем в сенсорный слой
    sensory_input.inject_pattern(pattern.clone(), 0.0);

    // Симулируем 500 мс
    println!("\nСимуляция 500 мс...");
    let mut total_spikes = 0;

    for step in 0..500 {
        let time = step as f64;

        // Обновляем развитие
        dev_manager.update(time);

        // Получаем внешние токи для сенсорного слоя
        let external_currents_map = sensory_input.get_external_current(time);

        // Конвертируем HashMap в Vec для первых 100 нейронов
        let mut external_currents = vec![0.0; 100];
        for (neuron_id, current) in external_currents_map {
            if neuron_id < 100 {
                external_currents[neuron_id] = current;
            }
        }

        // Шаг симуляции
        let spikes = topology.step(&external_currents);
        total_spikes += spikes.len();

        // Собираем спайки из L4 (префронтальная кора)
        let l4_neurons = topology.get_layer(LayerType::InnerVoice).unwrap();
        for spike in &spikes {
            if l4_neurons.neurons.iter().any(|n| n.id == spike.0) {
                thought_stream.add_pfc_spike(spike.1, spike.0);
            }
        }

        // Обновляем поток мыслей каждые 100 мс
        if step % 100 == 0 {
            let thoughts = thought_stream.update(time);
            if !thoughts.is_empty() {
                println!("  [{}ms] Мысли: {:?}", step, thoughts.iter().map(|t| &t.text).collect::<Vec<_>>());
            }
        }

        // Декодируем моторный выход
        let l5_neurons = topology.get_layer(LayerType::Motor).unwrap();
        for spike in &spikes {
            if l5_neurons.neurons.iter().any(|n| n.id == spike.0) {
                motor_output.add_spike(spike.1, spike.0);
            }
        }

        let output = motor_output.decode_output(time);
        match output {
            token_spike_interface::motor_output::OutputType::Speech(word, confidence) => {
                println!("  [{}ms] Речь: {} ({:.2})", step, word, confidence);
            }
            token_spike_interface::motor_output::OutputType::Thought(word, confidence) => {
                println!("  [{}ms] Мысль: {} ({:.2})", step, word, confidence);
            }
            token_spike_interface::motor_output::OutputType::Silent => {}
        }
    }

    println!("\n✓ Всего спайков: {}", total_spikes);

    // Статистика слоев
    println!("\n=== Активность слоев (последние 100 мс) ===");
    let activities = topology.get_layer_activities(100.0);
    for (layer_type, activity) in activities {
        println!("  {:?}: {:.1} Hz", layer_type, activity);
    }

    // Поток сознания
    println!("\n=== Поток сознания ===");
    let stream_text = thought_stream.get_stream_text(10);
    println!("  \"{}\"", stream_text);
    println!("  Когнитивная нагрузка: {:.1}%", thought_stream.get_cognitive_load(500.0) * 100.0);
    println!("  Эмоциональная окраска: {:.2}", thought_stream.get_average_valence(10));

    println!("\n=== Тест 2: Подача слова 'голод' ===\n");

    // Кодируем второе слово
    let pattern2 = encoder.encode("голод");
    sensory_input.inject_pattern(pattern2, 600.0);

    // Симулируем еще 500 мс
    println!("Симуляция 500 мс...");
    let mut total_spikes2 = 0;

    for step in 500..1000 {
        let time = step as f64;

        dev_manager.update(time);

        let external_currents_map = sensory_input.get_external_current(time);

        // Конвертируем HashMap в Vec для первых 100 нейронов
        let mut external_currents = vec![0.0; 100];
        for (neuron_id, current) in external_currents_map {
            if neuron_id < 100 {
                external_currents[neuron_id] = current;
            }
        }

        let spikes = topology.step(&external_currents);
        total_spikes2 += spikes.len();

        // Собираем спайки из L4
        let l4_neurons = topology.get_layer(LayerType::InnerVoice).unwrap();
        for spike in &spikes {
            if l4_neurons.neurons.iter().any(|n| n.id == spike.0) {
                thought_stream.add_pfc_spike(spike.1, spike.0);
            }
        }

        if step % 100 == 0 {
            let thoughts = thought_stream.update(time);
            if !thoughts.is_empty() {
                println!("  [{}ms] Мысли: {:?}", step, thoughts.iter().map(|t| &t.text).collect::<Vec<_>>());
            }
        }

        // Декодируем моторный выход
        let l5_neurons = topology.get_layer(LayerType::Motor).unwrap();
        for spike in &spikes {
            if l5_neurons.neurons.iter().any(|n| n.id == spike.0) {
                motor_output.add_spike(spike.1, spike.0);
            }
        }

        let output = motor_output.decode_output(time);
        match output {
            token_spike_interface::motor_output::OutputType::Speech(word, confidence) => {
                println!("  [{}ms] Речь: {} ({:.2})", step, word, confidence);
            }
            token_spike_interface::motor_output::OutputType::Thought(word, confidence) => {
                println!("  [{}ms] Мысль: {} ({:.2})", step, word, confidence);
            }
            token_spike_interface::motor_output::OutputType::Silent => {}
        }
    }

    println!("\n✓ Всего спайков: {}", total_spikes2);

    // Финальная статистика
    println!("\n=== Финальная статистика ===");
    let final_stats = topology.get_stats();
    println!("  Время симуляции: {:.1} мс", final_stats.current_time);
    println!("  Средний вес связей: {:.3}", final_stats.avg_weight);
    println!("  Стадия развития: {}", dev_manager.get_status(1000.0));

    let activities = topology.get_layer_activities(200.0);
    println!("\n  Активность слоев (последние 200 мс):");
    for (layer_type, activity) in activities {
        println!("    {:?}: {:.1} Hz", layer_type, activity);
    }

    println!("\n=== Демо завершено ===");
}
