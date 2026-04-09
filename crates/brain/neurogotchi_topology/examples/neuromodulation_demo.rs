//! Пример интеграции с нейромодуляцией
//!
//! Демонстрирует:
//! 1. Слово → спайки → 5 слоев → мысли/речь
//! 2. Накопление усталости (аденозин)
//! 3. Циркадный ритм (день/ночь)
//! 4. Переход в сон при усталости
//! 5. Стабилизация активности (130Hz → 40-50Hz)

use neurogotchi_topology::{NeuroGotchiTopology, LayerType};
use token_spike_interface::{
    encoder::PhonemeEncoder,
    decoder::SpikeDecoder,
    sensory_input::SensoryInput,
    motor_output::MotorOutput,
    thought_stream::ThoughtStream,
};

fn main() {
    println!("=== NeuroGotchi с Нейромодуляцией ===\n");

    // Создаем топологию
    println!("Создание 5-слойной топологии с нейромодуляцией...");
    let mut topology = NeuroGotchiTopology::new();
    let stats = topology.get_stats();
    println!("✓ Нейронов: {}", stats.total_neurons);
    println!("✓ Связей: {} (врожденных: {}, пластичных: {})",
        stats.total_connections,
        stats.innate_connections,
        stats.plastic_connections
    );
    println!("✓ Нейромодуляция: {}\n", topology.get_neuromodulation_status());

    // Создаем компоненты интерфейса
    let encoder = PhonemeEncoder::new();

    let mut decoder1 = SpikeDecoder::new();
    decoder1.add_words(&["привет", "голод", "страх", "мама", "помощь", "сон", "устал"]);

    let mut decoder2 = SpikeDecoder::new();
    decoder2.add_words(&["привет", "голод", "страх", "мама", "помощь", "сон", "устал"]);

    let mut decoder3 = SpikeDecoder::new();
    decoder3.add_words(&["привет", "голод", "страх", "мама", "помощь", "сон", "устал"]);

    let mut sensory_input = SensoryInput::new();
    let mut motor_output = MotorOutput::new(decoder1);
    let mut thought_stream = ThoughtStream::new(decoder2);

    println!("=== Симуляция 10 секунд (10000 мс) ===\n");

    let mut total_spikes = 0;
    let mut last_report = 0.0;

    // Подаем слова каждые 2 секунды
    let words = vec!["привет", "голод", "устал", "сон", "мама"];
    let mut word_idx = 0;

    for step in 0..10_000 {
        let time = step as f64;

        // Подаем новое слово каждые 2 секунды
        if step % 2000 == 0 && word_idx < words.len() {
            let word = words[word_idx];
            let pattern = encoder.encode(word);
            sensory_input.inject_pattern(pattern, time);
            println!("[{:.1}s] Подано слово: '{}'", time / 1000.0, word);
            word_idx += 1;
        }

        // Получаем внешние токи
        let external_currents_map = sensory_input.get_external_current(time);
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

        // Обновляем поток мыслей
        if step % 100 == 0 {
            let thoughts = thought_stream.update(time);
            if !thoughts.is_empty() {
                for thought in thoughts {
                    println!("  [💭 {:.1}s] Мысль: {} ({:.2})",
                        time / 1000.0, thought.text, thought.confidence);
                }
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
                println!("  [🗣️  {:.1}s] Речь: {} ({:.2})", time / 1000.0, word, confidence);
            }
            token_spike_interface::motor_output::OutputType::Thought(word, confidence) => {
                println!("  [💬 {:.1}s] Внутренний голос: {} ({:.2})", time / 1000.0, word, confidence);
            }
            token_spike_interface::motor_output::OutputType::Silent => {}
        }

        // Отчет каждую секунду
        if time - last_report >= 1000.0 {
            let activities = topology.get_layer_activities(100.0);
            let avg_activity: f64 = activities.iter().map(|(_, a)| a).sum::<f64>() / 5.0;

            println!("\n[{:.1}s] Статус:", time / 1000.0);
            println!("  Средняя активность: {:.1} Hz", avg_activity);
            println!("  Усталость: {:.0}%", topology.get_fatigue_level() * 100.0);
            println!("  Настроение: {:.2}", topology.get_emotional_valence());
            println!("  Состояние: {}", topology.get_neuromodulation_status());

            if topology.is_sleeping() {
                println!("  💤 СОН");
            }

            println!();
            last_report = time;
        }
    }

    println!("\n=== Финальная статистика ===");
    println!("Всего спайков: {}", total_spikes);
    println!("Средняя частота: {:.1} спайков/мс", total_spikes as f64 / 10_000.0);

    let activities = topology.get_layer_activities(1000.0);
    println!("\nАктивность слоев (последняя секунда):");
    for (layer_type, activity) in activities {
        println!("  {:?}: {:.1} Hz", layer_type, activity);
    }

    println!("\nНейромодуляция:");
    println!("  {}", topology.get_neuromodulation_status());
    println!("  Усталость: {:.0}%", topology.get_fatigue_level() * 100.0);
    println!("  Настроение: {:.2}", topology.get_emotional_valence());

    println!("\n=== Демо завершено ===");
}
