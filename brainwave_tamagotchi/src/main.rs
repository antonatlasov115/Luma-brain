use brainwave_core::{BrainwaveSpectrum, ConsciousnessState, LearnableSpikingNetwork, PetVocabulary};
use speech_module::SpeechModule;
use memory_module::MemoryModule;
use thought_module::ThoughtModule;
use chemistry_module::{ChemistryModule, Neurotransmitter};
use wave_analyzer::WaveAnalyzer;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

// Конвертация эмоций между модулями
fn convert_emotion(emotion: speech_module::Emotion) -> memory_module::Emotion {
    match emotion {
        speech_module::Emotion::Joy => memory_module::Emotion::Joy,
        speech_module::Emotion::Sadness => memory_module::Emotion::Sadness,
        speech_module::Emotion::Curiosity => memory_module::Emotion::Curiosity,
        speech_module::Emotion::Fear => memory_module::Emotion::Fear,
        speech_module::Emotion::Anger => memory_module::Emotion::Anger,
        speech_module::Emotion::Neutral => memory_module::Emotion::Neutral,
    }
}
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};
use std::fs;
use std::io;
use std::path::Path;
use std::time::{Duration, Instant};

const SAVE_FILE: &str = "pet_brain.json";

/// Сохраняемое состояние мозга
#[derive(serde::Serialize, serde::Deserialize)]
struct BrainState {
    vocabulary: PetVocabulary,
    age: u64,
    total_interactions: usize,
}

/// Тамагочи с обучаемым мозгом
struct BrainPet {
    name: String,
    learning_network: LearnableSpikingNetwork,
    spectrum: BrainwaveSpectrum,
    current_state: ConsciousnessState,
    vocabulary: PetVocabulary,

    // Новые модули
    speech: SpeechModule,
    memory: MemoryModule,
    thoughts: ThoughtModule,
    chemistry: ChemistryModule,
    wave_analyzer: WaveAnalyzer,

    // Базовые потребности
    hunger: f64,
    energy: f64,
    happiness: f64,
    stimulation: f64,

    // Состояние
    age: u64,
    is_sleeping: bool,
    total_interactions: usize,

    // Ввод текста
    text_input: String,
    input_mode: bool,

    // События
    events: Vec<String>,
    pet_speech: Vec<String>,

    last_update: Instant,
    last_save: Instant,
}

impl BrainPet {
    fn new(name: String) -> Self {
        let learning_network = LearnableSpikingNetwork::new(&[10, 20, 15, 8], 0.01);
        let mut spectrum = BrainwaveSpectrum::new();
        spectrum.set_consciousness_state(ConsciousnessState::Awake);

        let mut pet = Self {
            name,
            learning_network,
            spectrum,
            current_state: ConsciousnessState::Awake,
            vocabulary: PetVocabulary::new(),
            speech: SpeechModule::new(),
            memory: MemoryModule::new(),
            thoughts: ThoughtModule::new(),
            chemistry: ChemistryModule::new(),
            wave_analyzer: WaveAnalyzer::new(100.0, 256),
            hunger: 80.0,
            energy: 100.0,
            happiness: 80.0,
            stimulation: 50.0,
            age: 0,
            is_sleeping: false,
            total_interactions: 0,
            text_input: String::new(),
            input_mode: false,
            events: vec!["🐣 Питомец родился!".to_string()],
            pet_speech: Vec::new(),
            last_update: Instant::now(),
            last_save: Instant::now(),
        };

        // Загружаем сохраненный мозг
        pet.load_brain();

        // Инициализируем базовые слова если словарь пустой
        let is_new = pet.vocabulary.get_vocabulary_size() == 0;
        if is_new {
            pet.vocabulary.create_pet_word("hello");
            pet.vocabulary.create_pet_word("feed");
            pet.vocabulary.create_pet_word("play");
            pet.vocabulary.create_pet_word("sleep");
            pet.vocabulary.create_pet_word("happy");
            pet.vocabulary.create_pet_word("sad");
            pet.vocabulary.create_pet_word("hungry");
        }

        // Приветствие при каждом запуске
        if let Some(hello_word) = pet.vocabulary.get_pet_word("hello") {
            pet.pet_speech.push(format!("🗣️ {}! 👋", hello_word));
        } else {
            // Если слова нет, создаем и говорим
            let hello_word = pet.vocabulary.create_pet_word("hello");
            pet.pet_speech.push(format!("🗣️ {}! 👋", hello_word));
        }

        // Добавим начальные мысли и речь
        pet.pet_speech.push("💭 Я проснулся!".to_string());
        pet.events.push("💬 Питомец готов общаться!".to_string());

        pet
    }

    fn update(&mut self) {
        let elapsed = self.last_update.elapsed().as_secs_f64();
        self.last_update = Instant::now();

        self.age += elapsed as u64;

        // Естественное снижение параметров (медленнее)
        if self.is_sleeping {
            // Во сне восстанавливается энергия
            self.energy = (self.energy + elapsed * 3.0).min(100.0);
            self.hunger = (self.hunger - elapsed * 0.5).max(0.0);
            self.happiness = (self.happiness - elapsed * 0.2).max(0.0);
            self.stimulation = (self.stimulation - elapsed * 0.3).max(0.0);
        } else {
            // Бодрствование
            self.hunger = (self.hunger - elapsed * 0.8).max(0.0);
            self.energy = (self.energy - elapsed * 0.5).max(0.0);
            self.happiness = (self.happiness - elapsed * 0.4).max(0.0);
            self.stimulation = (self.stimulation - elapsed * 0.3).max(0.0);
        }

        // Автосон при низкой энергии
        if self.energy < 15.0 && !self.is_sleeping {
            self.sleep();
        }

        // Пробуждение при полной энергии
        if self.energy > 90.0 && self.is_sleeping {
            self.wake_up();
        }

        self.update_consciousness_state();

        // Обновление нейросети с обучением
        let inputs = vec![
            self.hunger / 100.0,
            self.energy / 100.0,
            self.happiness / 100.0,
            self.stimulation / 100.0,
            if self.is_sleeping { 1.0 } else { 0.0 },
            (self.age % 60) as f64 / 60.0,
            self.vocabulary.get_vocabulary_size() as f64 / 100.0,
            self.vocabulary.get_language_size() as f64 / 20.0,
            (self.total_interactions % 100) as f64 / 100.0,
            self.spectrum.consciousness_index(),
        ];

        let wave_modulation = match self.spectrum.dominant_wave() {
            brainwave_core::BrainwaveType::Delta => 0.5,
            brainwave_core::BrainwaveType::Theta => 0.7,
            brainwave_core::BrainwaveType::Alpha => 0.8,
            brainwave_core::BrainwaveType::Beta => 1.0,
            brainwave_core::BrainwaveType::Gamma => 1.2,
        };

        self.learning_network.forward_with_learning(&inputs, wave_modulation, elapsed);

        // 1. Обновить химию мозга
        self.chemistry.update(self.hunger, self.energy, self.happiness, self.stimulation, elapsed);

        // 2. Обновить эмоции в речевом модуле
        self.speech.update_emotion(self.hunger, self.energy, self.happiness, self.stimulation);

        // 3. Обновить сознание в модуле мыслей
        let ci = self.spectrum.consciousness_index();
        self.thoughts.update_consciousness(ci);

        // 4. Генерировать спонтанную речь
        if let Some(speech) = self.speech.generate_spontaneous_speech(self.hunger, self.energy, self.happiness, self.age) {
            self.pet_speech.push(speech);
        }

        // 5. Генерировать мысли
        if let Some(_thought) = self.thoughts.generate_thought(self.hunger, self.energy, self.happiness, self.stimulation, self.vocabulary.get_vocabulary_size(), self.age) {
            // Иногда выражать мысли вслух
            if let Some(expressed) = self.thoughts.maybe_express_thought() {
                self.pet_speech.push(expressed);
            }
        }

        // 6. Инициатива в общении
        if let Some(message) = self.speech.initiate_conversation(self.hunger, self.energy, self.happiness, self.age) {
            self.pet_speech.push(message);
        }

        // 7. Вопросы
        if let Some(question) = self.speech.generate_question(self.vocabulary.get_vocabulary_size(), self.age) {
            self.pet_speech.push(question);
        }

        // 8. Модуляция обучения через химию
        let learning_mod = self.chemistry.get_learning_modulation();
        self.learning_network.learning_rate = 0.01 * learning_mod;

        // 9. Анализ волн
        let spike_rate = self.learning_network.get_total_spike_rate();
        self.wave_analyzer.add_sample(spike_rate);

        // 10. Консолидация и затухание памяти
        self.memory.consolidate_memories(self.age);
        self.memory.decay_memories(self.age);

        // Автосохранение каждые 30 секунд
        if self.last_save.elapsed().as_secs() >= 30 {
            self.save_brain();
            self.last_save = Instant::now();
        }
    }

    fn update_consciousness_state(&mut self) {
        let state = if self.is_sleeping {
            if self.energy < 50.0 {
                ConsciousnessState::DeepSleep
            } else {
                ConsciousnessState::Meditation
            }
        } else if self.stimulation > 80.0 && self.happiness > 70.0 {
            ConsciousnessState::FlowState
        } else if self.stimulation > 90.0 {
            ConsciousnessState::Insight
        } else if self.stimulation > 60.0 {
            ConsciousnessState::Focused
        } else if self.energy > 60.0 {
            ConsciousnessState::Awake
        } else {
            ConsciousnessState::Relaxed
        };

        if state != self.current_state {
            self.current_state = state;
            self.spectrum.set_consciousness_state(state);
        }
    }

    fn process_text_input(&mut self, text: &str) {
        if text.is_empty() {
            return;
        }

        self.total_interactions += 1;

        // Пытаемся распознать действие
        if let Some((action, confidence)) = self.vocabulary.recognize_action(text) {
            self.events.push(format!("🧠 Понял: {} ({:.0}%)", action, confidence * 100.0));

            match action.as_str() {
                "feed" => self.feed_with_learning(text),
                "play" => self.play_with_learning(text),
                "study" => self.study_with_learning(text),
                "sleep" => {
                    if self.is_sleeping {
                        self.wake_up();
                    } else {
                        self.sleep();
                    }
                }
                _ => {}
            }

            // Питомец отвечает своим словом
            if let Some(pet_word) = self.vocabulary.get_pet_word(&action) {
                self.pet_speech.push(format!("🗣️ {}", pet_word));
            }
        } else {
            // Не понял - пытаемся угадать по контексту
            self.events.push(format!("❓ Услышал: '{}'", text));

            // Обучаемся на основе текущего состояния
            if self.hunger < 50.0 {
                self.vocabulary.learn_association(text, "feed", 0.1);
            } else if self.energy < 50.0 {
                self.vocabulary.learn_association(text, "sleep", 0.1);
            } else if self.happiness < 50.0 {
                self.vocabulary.learn_association(text, "play", 0.1);
            }
        }

        if self.events.len() > 8 {
            self.events.remove(0);
        }
        if self.pet_speech.len() > 5 {
            self.pet_speech.remove(0);
        }
    }

    fn feed_with_learning(&mut self, text: &str) {
        self.hunger = (self.hunger + 40.0).min(100.0);
        self.happiness = (self.happiness + 10.0).min(100.0);
        self.energy = (self.energy + 5.0).min(100.0);

        // 1. Выброс дофамина (награда!)
        self.chemistry.release(Neurotransmitter::Dopamine, 0.3);

        // 2. Запомнить событие с эмоцией
        let emotion = convert_emotion(self.speech.current_emotion);
        self.memory.remember_event("Покормили".to_string(), emotion, self.hunger, self.energy, self.happiness, self.age);

        // 3. Ассоциация в мыслях
        self.thoughts.associate_word(text, "еда", self.age);

        // Обучаемся связывать эти слова с кормлением
        self.vocabulary.learn_association(text, "feed", 0.3);

        // Создаем свое слово если еще нет
        let pet_word = self.vocabulary.create_pet_word("feed");
        self.events.push(format!("🍎 Покормили"));

        // 4. Эмоциональный ответ
        let response = format!("{} (спасибо!)", pet_word);
        let emotional_response = self.speech.respond_with_emotion(&response, self.age);
        self.pet_speech.push(format!("🗣️ {}", emotional_response));
    }

    fn play_with_learning(&mut self, text: &str) {
        if self.energy > 15.0 {
            self.happiness = (self.happiness + 25.0).min(100.0);
            self.stimulation = (self.stimulation + 20.0).min(100.0);
            self.energy = (self.energy - 8.0).max(0.0);

            // Выброс дофамина и серотонина (радость!)
            self.chemistry.release(Neurotransmitter::Dopamine, 0.4);
            self.chemistry.release(Neurotransmitter::Serotonin, 0.3);

            // Запомнить веселое событие
            let emotion = convert_emotion(self.speech.current_emotion);
            self.memory.remember_event("Поиграли".to_string(), emotion, self.hunger, self.energy, self.happiness, self.age);

            self.vocabulary.learn_association(text, "play", 0.3);
            let pet_word = self.vocabulary.create_pet_word("play");

            self.events.push("🎮 Поиграли".to_string());
            let response = format!("{}! (весело!)", pet_word);
            let emotional_response = self.speech.respond_with_emotion(&response, self.age);
            self.pet_speech.push(format!("🗣️ {}", emotional_response));
        } else {
            self.events.push("😴 Слишком устал".to_string());
        }
    }

    fn study_with_learning(&mut self, text: &str) {
        if self.energy > 20.0 {
            self.stimulation = (self.stimulation + 35.0).min(100.0);
            self.happiness = (self.happiness + 15.0).min(100.0);
            self.energy = (self.energy - 12.0).max(0.0);

            self.vocabulary.learn_association(text, "study", 0.3);
            let pet_word = self.vocabulary.create_pet_word("study");

            self.events.push("📚 Позанимались".to_string());
            self.pet_speech.push(format!("🗣️ {} (интересно!)", pet_word));
        } else {
            self.events.push("😴 Слишком устал".to_string());
        }
    }

    fn feed(&mut self) {
        self.feed_with_learning("покормить еда");
    }

    fn play(&mut self) {
        self.play_with_learning("играть игра");
    }

    fn study(&mut self) {
        self.study_with_learning("учиться занятие");
    }

    fn sleep(&mut self) {
        if !self.is_sleeping {
            self.is_sleeping = true;
            self.current_state = ConsciousnessState::DeepSleep;
            self.spectrum.set_consciousness_state(ConsciousnessState::DeepSleep);

            // Выброс серотонина (сон)
            self.chemistry.release(Neurotransmitter::Serotonin, 0.5);

            self.events.push("😴 Заснул".to_string());

            if let Some(word) = self.vocabulary.get_pet_word("sleep") {
                self.pet_speech.push(format!("🗣️ {}...", word));
            }
        }
    }

    fn wake_up(&mut self) {
        if self.is_sleeping {
            self.is_sleeping = false;
            self.current_state = ConsciousnessState::Awake;
            self.spectrum.set_consciousness_state(ConsciousnessState::Awake);
            self.events.push("☀️ Проснулся".to_string());

            if let Some(word) = self.vocabulary.get_pet_word("wake") {
                self.pet_speech.push(format!("🗣️ {}!", word));
            }
        }
    }

    fn save_brain(&self) {
        let state = BrainState {
            vocabulary: self.vocabulary.clone(),
            age: self.age,
            total_interactions: self.total_interactions,
        };

        if let Ok(json) = serde_json::to_string_pretty(&state) {
            let _ = fs::write(SAVE_FILE, json);
        }
    }

    fn load_brain(&mut self) {
        if Path::new(SAVE_FILE).exists() {
            if let Ok(json) = fs::read_to_string(SAVE_FILE) {
                if let Ok(state) = serde_json::from_str::<BrainState>(&json) {
                    self.vocabulary = state.vocabulary;
                    self.age = state.age;
                    self.total_interactions = state.total_interactions;
                    self.events.push(format!("💾 Загружен мозг (возраст: {}с, взаимодействий: {})",
                                            self.age, self.total_interactions));
                }
            }
        }
    }

    fn get_mood(&self) -> &str {
        if self.is_sleeping {
            return "😴 Спит";
        }

        match self.current_state {
            ConsciousnessState::Insight => "💡 Озарение!",
            ConsciousnessState::FlowState => "🌟 В потоке",
            ConsciousnessState::Focused => "🎯 Сосредоточен",
            ConsciousnessState::Awake => {
                if self.happiness > 70.0 {
                    "😊 Счастлив"
                } else if self.happiness > 40.0 {
                    "😐 Нормально"
                } else {
                    "😢 Грустит"
                }
            }
            ConsciousnessState::Relaxed => "😌 Расслаблен",
            ConsciousnessState::Meditation => "🧘 Медитирует",
            _ => "😐 Обычное",
        }
    }

    fn get_ascii_art(&self) -> Vec<&str> {
        if self.is_sleeping {
            vec!["     zzZ", "    zzZ", "   (-.-)  ", "   />  <\\"]
        } else if self.happiness > 70.0 {
            vec!["   (^_^)  ", "   />  <\\", "    | |   "]
        } else if self.happiness > 40.0 {
            vec!["   (o_o)  ", "   />  <\\", "    | |   "]
        } else {
            vec!["   (T_T)  ", "   />  <\\", "    | |   "]
        }
    }
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut pet = BrainPet::new("Нейро".to_string());
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(100);

    let result = run_app(&mut terminal, &mut pet, tick_rate, &mut last_tick);

    // Сохраняем перед выходом
    pet.save_brain();

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    result
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    pet: &mut BrainPet,
    tick_rate: Duration,
    last_tick: &mut Instant,
) -> Result<(), io::Error> {
    loop {
        terminal.draw(|f| ui(f, pet))?;

        let timeout = tick_rate.checked_sub(last_tick.elapsed()).unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if pet.input_mode {
                    match key.code {
                        KeyCode::Enter => {
                            let text = pet.text_input.clone();
                            pet.text_input.clear();
                            pet.input_mode = false;
                            pet.process_text_input(&text);
                        }
                        KeyCode::Char(c) => {
                            pet.text_input.push(c);
                        }
                        KeyCode::Backspace => {
                            pet.text_input.pop();
                        }
                        KeyCode::Esc => {
                            pet.input_mode = false;
                            pet.text_input.clear();
                        }
                        _ => {}
                    }
                } else {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('f') => pet.feed(),
                        KeyCode::Char('p') => pet.play(),
                        KeyCode::Char('s') => pet.study(),
                        KeyCode::Char('z') => {
                            if pet.is_sleeping {
                                pet.wake_up();
                            } else {
                                pet.sleep();
                            }
                        }
                        KeyCode::Char('t') => {
                            pet.input_mode = true;
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            pet.update();
            *last_tick = Instant::now();
        }
    }

    Ok(())
}

fn ui(f: &mut Frame, pet: &BrainPet) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),   // Заголовок
            Constraint::Length(8),   // Питомец
            Constraint::Length(12),  // Параметры
            Constraint::Length(12),  // Волны
            Constraint::Length(8),   // Химия (НОВОЕ)
            Constraint::Length(8),   // Мысли (НОВОЕ)
            Constraint::Min(0),      // События
        ])
        .split(f.area());

    // Заголовок
    let vocab_size = pet.vocabulary.get_vocabulary_size();
    let lang_size = pet.vocabulary.get_language_size();
    let title = Paragraph::new(format!(
        "🧠 {} - Возраст: {}с | Словарь: {} слов | Язык: {} слов | Взаимодействий: {}",
        pet.name, pet.age, vocab_size, lang_size, pet.total_interactions
    ))
    .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
    .block(Block::default().borders(Borders::ALL).title("Обучаемый Нейро-Тамагочи"));
    f.render_widget(title, chunks[0]);

    // ASCII арт + речь питомца
    let mut art_lines: Vec<Line> = pet
        .get_ascii_art()
        .iter()
        .map(|line| Line::from(Span::styled(*line, Style::default().fg(Color::Yellow))))
        .collect();

    art_lines.push(Line::from(""));
    art_lines.push(Line::from(Span::styled(
        pet.get_mood(),
        Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
    )));

    // Речь питомца
    if !pet.pet_speech.is_empty() {
        art_lines.push(Line::from(""));
        for speech in pet.pet_speech.iter().rev().take(2) {
            art_lines.push(Line::from(Span::styled(speech, Style::default().fg(Color::Green))));
        }
    }

    let pet_display = Paragraph::new(art_lines)
        .block(Block::default().borders(Borders::ALL).title("Питомец"))
        .style(Style::default().fg(Color::White));
    f.render_widget(pet_display, chunks[1]);

    // Параметры
    let params_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(chunks[2]);

    render_gauge(f, params_chunks[0], "🍎 Голод", pet.hunger, Color::Green);
    render_gauge(f, params_chunks[1], "⚡ Энергия", pet.energy, Color::Yellow);
    render_gauge(f, params_chunks[2], "😊 Счастье", pet.happiness, Color::Magenta);
    render_gauge(f, params_chunks[3], "🧠 Стимуляция", pet.stimulation, Color::Cyan);

    // Мозговые волны
    render_brainwaves(f, chunks[3], pet);

    // Химия мозга (НОВОЕ)
    render_chemistry(f, chunks[4], pet);

    // Мысли (НОВОЕ)
    render_thoughts(f, chunks[5], pet);

    // События и управление
    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[6]);

    let events: Vec<ListItem> = pet.events.iter().rev().map(|e| ListItem::new(e.as_str())).collect();
    let events_list = List::new(events).block(Block::default().borders(Borders::ALL).title("События"));
    f.render_widget(events_list, bottom_chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(bottom_chunks[1]);

    // Управление
    let mut controls_text = vec![
        Line::from("Управление:"),
        Line::from(""),
        Line::from("F - Покормить 🍎"),
        Line::from("P - Поиграть 🎮"),
        Line::from("S - Позаниматься 📚"),
        Line::from("Z - Спать/Проснуться 😴"),
        Line::from("T - Говорить с питомцем 💬"),
        Line::from("Q - Выход (автосохранение)"),
    ];

    if pet.input_mode {
        controls_text.push(Line::from(""));
        controls_text.push(Line::from(Span::styled(
            format!("Ввод: {}_", pet.text_input),
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        )));
        controls_text.push(Line::from("Enter - отправить, Esc - отмена"));
    }

    let controls = Paragraph::new(controls_text).block(Block::default().borders(Borders::ALL).title("Помощь"));
    f.render_widget(controls, right_chunks[0]);

    // Топ слов
    let top_words = pet.vocabulary.get_top_words(5);
    let words_text: Vec<Line> = if top_words.is_empty() {
        vec![Line::from("Пока не выучил слов...")]
    } else {
        top_words
            .iter()
            .map(|(word, count)| Line::from(format!("{}: {} раз", word, count)))
            .collect()
    };

    let words_display = Paragraph::new(words_text)
        .block(Block::default().borders(Borders::ALL).title("📖 Топ слов"))
        .wrap(Wrap { trim: true });
    f.render_widget(words_display, right_chunks[1]);
}

fn render_gauge(f: &mut Frame, area: Rect, label: &str, value: f64, color: Color) {
    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(label))
        .gauge_style(Style::default().fg(color))
        .percent(value as u16);
    f.render_widget(gauge, area);
}

fn render_brainwaves(f: &mut Frame, area: Rect, pet: &BrainPet) {
    let spike_rate = pet.learning_network.get_total_spike_rate();

    let waves = vec![
        format!("😴 Delta:  {:.0}%  {}", pet.spectrum.delta_power * 100.0, create_bar(pet.spectrum.delta_power)),
        format!("🧘 Theta:  {:.0}%  {}", pet.spectrum.theta_power * 100.0, create_bar(pet.spectrum.theta_power)),
        format!("😌 Alpha:  {:.0}%  {}", pet.spectrum.alpha_power * 100.0, create_bar(pet.spectrum.alpha_power)),
        format!("🤔 Beta:   {:.0}%  {}", pet.spectrum.beta_power * 100.0, create_bar(pet.spectrum.beta_power)),
        format!("💡 Gamma:  {:.0}%  {}", pet.spectrum.gamma_power * 100.0, create_bar(pet.spectrum.gamma_power)),
        format!(""),
        format!("💫 Индекс сознания: {:.2}", pet.spectrum.consciousness_index()),
        format!("⚡ Спайк-рейт: {:.3}", spike_rate),
        format!("🎓 Скорость обучения: {:.3}", pet.learning_network.learning_rate),
    ];

    let lines: Vec<Line> = waves.iter().map(|w| Line::from(w.as_str())).collect();
    let waves_display = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("🌊 Мозг + Обучение"))
        .style(Style::default().fg(Color::White));
    f.render_widget(waves_display, area);
}

fn create_bar(value: f64) -> String {
    let filled = (value * 10.0) as usize;
    let empty = 10 - filled;
    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
}

fn render_chemistry(f: &mut Frame, area: Rect, pet: &BrainPet) {
    let report = pet.chemistry.get_chemistry_report();
    let lines: Vec<Line> = report
        .iter()
        .map(|(name, level)| {
            Line::from(format!("{}: {:.0}% {}", name, level * 100.0, create_bar(*level)))
        })
        .collect();

    let widget = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("🧪 Нейрохимия"))
        .style(Style::default().fg(Color::White));
    f.render_widget(widget, area);
}

fn render_thoughts(f: &mut Frame, area: Rect, pet: &BrainPet) {
    let recent = pet.thoughts.get_recent_thoughts(5);
    let items: Vec<ListItem> = recent
        .iter()
        .map(|t| ListItem::new(t.as_str()))
        .collect();

    let widget = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("💭 Мысли"))
        .style(Style::default().fg(Color::White));
    f.render_widget(widget, area);
}
