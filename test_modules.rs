use brainwave_core::{BrainwaveSpectrum, ConsciousnessState, LearnableSpikingNetwork, PetVocabulary};
use speech_module::SpeechModule;
use memory_module::MemoryModule;
use thought_module::ThoughtModule;
use chemistry_module::{ChemistryModule, Neurotransmitter};
use wave_analyzer::WaveAnalyzer;
use std::thread;
use std::time::Duration;

fn main() {
    println!("🧠 Тест модулей тамагочи\n");

    // Создаем модули
    let mut vocabulary = PetVocabulary::new();
    let mut speech = SpeechModule::new();
    let mut thoughts = ThoughtModule::new();
    let mut chemistry = ChemistryModule::new();

    // Создаем базовые слова
    println!("📝 Создаем язык питомца...");
    let hello = vocabulary.create_pet_word("hello");
    let feed = vocabulary.create_pet_word("feed");
    let play = vocabulary.create_pet_word("play");

    // Учим ассоциации чтобы vocabulary_size был > 0
    vocabulary.learn_association("привет здравствуй", "hello", 0.5);
    vocabulary.learn_association("еда кушать", "feed", 0.5);
    vocabulary.learn_association("играть игра", "play", 0.5);

    println!("   hello -> {}", hello);
    println!("   feed -> {}", feed);
    println!("   play -> {}", play);
    println!("   Словарь: {} слов", vocabulary.get_vocabulary_size());
    println!("   Язык: {} слов\n", vocabulary.get_language_size());

    // Параметры
    let mut hunger = 80.0;
    let mut energy = 100.0;
    let mut happiness = 80.0;
    let stimulation = 50.0;

    println!("🎬 Начинаем симуляцию (10 секунд)...");
    println!("   Настройки: речь={:.0}%, мысли={:.0}%, вопросы={:.0}%\n",
             speech.spontaneous_rate * 100.0,
             thoughts.thinking_rate * 100.0,
             speech.question_rate * 100.0);

    for age in 0..10 {
        println!("⏰ Секунда {} (hunger={:.0}, energy={:.0}, happiness={:.0})", age, hunger, energy, happiness);

        // Обновляем эмоции
        speech.update_emotion(hunger, energy, happiness, stimulation);

        // Обновляем химию
        chemistry.update(hunger, energy, happiness, stimulation, 1.0);

        // Обновляем сознание
        thoughts.update_consciousness(0.8);

        // Генерируем спонтанную речь
        if let Some(s) = speech.generate_spontaneous_speech(hunger, energy, happiness, age) {
            println!("   🗣️  {}", s);
        }

        // Генерируем мысли
        if let Some(t) = thoughts.generate_thought(hunger, energy, happiness, stimulation, vocabulary.get_vocabulary_size(), age) {
            println!("   💭 {}", t.content);
        }

        // Генерируем вопросы
        if let Some(q) = speech.generate_question(vocabulary.get_vocabulary_size(), age) {
            println!("   {}", q);
        }

        // Инициатива
        if let Some(m) = speech.initiate_conversation(hunger, energy, happiness, age) {
            println!("   {}", m);
        }

        // Показываем химию
        let report = chemistry.get_chemistry_report();
        let dopamine = report.iter().find(|(n, _)| n == "Дофамин").map(|(_, v)| v).unwrap_or(&0.0);
        if *dopamine > 0.3 {
            println!("   🧪 Дофамин: {:.0}%", dopamine * 100.0);
        }

        // Показываем последние мысли
        let recent = thoughts.get_recent_thoughts(3);
        if !recent.is_empty() && age % 3 == 0 {
            println!("   📋 Мысли: {}", recent.len());
        }

        println!();

        // Снижаем параметры
        hunger -= 2.0;
        energy -= 1.0;
        happiness -= 1.0;

        thread::sleep(Duration::from_millis(500));
    }

    println!("\n✅ Тест завершен!");
    println!("\n📊 Итоги:");
    println!("   Речь: {} записей", speech.speech_history.len());
    println!("   Мысли: {} записей", thoughts.thoughts.len());
    println!("   Словарь: {} слов", vocabulary.get_vocabulary_size());
}
