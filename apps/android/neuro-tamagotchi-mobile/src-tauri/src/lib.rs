use brainwave_core::{BrainwaveSpectrum, ConsciousnessState, LearnableSpikingNetwork, PetVocabulary};
use speech_module::SpeechModule;
use memory_module::MemoryModule;
use thought_module::ThoughtModule;
use chemistry_module::{ChemistryModule, Neurotransmitter};
use wave_analyzer::WaveAnalyzer;
use consciousness_module::{GlobalWorkspace, IITConsciousness, SpatialBrain, Information, InformationSource, BrainRegion};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time::{Duration, Instant};

// Конвертация эмоций
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

#[derive(Serialize, Deserialize, Clone)]
pub struct PetState {
    pub name: String,
    pub hunger: f64,
    pub energy: f64,
    pub happiness: f64,
    pub stimulation: f64,
    pub age: u64,
    pub is_sleeping: bool,
    pub mood: String,
    pub consciousness_index: f64,
    pub phi: f64,
    pub awareness: f64,
    pub vocabulary_size: usize,
    pub total_interactions: usize,
    pub events: Vec<String>,
    pub pet_speech: Vec<String>,
    pub brainwaves: BrainwaveData,
    pub chemistry: Vec<ChemistryLevel>,
    pub thoughts: Vec<String>,
    pub brain_regions: Vec<BrainRegionData>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BrainwaveData {
    pub delta: f64,
    pub theta: f64,
    pub alpha: f64,
    pub beta: f64,
    pub gamma: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChemistryLevel {
    pub name: String,
    pub level: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BrainRegionData {
    pub name: String,
    pub emoji: String,
    pub activation: f64,
}

struct BrainPet {
    name: String,
    learning_network: LearnableSpikingNetwork,
    spectrum: BrainwaveSpectrum,
    current_state: ConsciousnessState,
    vocabulary: PetVocabulary,
    speech: SpeechModule,
    memory: MemoryModule,
    thoughts: ThoughtModule,
    chemistry: ChemistryModule,
    wave_analyzer: WaveAnalyzer,
    global_workspace: GlobalWorkspace,
    iit_consciousness: IITConsciousness,
    spatial_brain: SpatialBrain,
    hunger: f64,
    energy: f64,
    happiness: f64,
    stimulation: f64,
    age: u64,
    is_sleeping: bool,
    total_interactions: usize,
    events: Vec<String>,
    pet_speech: Vec<String>,
    last_update: Instant,
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
            global_workspace: GlobalWorkspace::new(),
            iit_consciousness: IITConsciousness::new(20),
            spatial_brain: SpatialBrain::new(),
            hunger: 80.0,
            energy: 100.0,
            happiness: 80.0,
            stimulation: 50.0,
            age: 0,
            is_sleeping: false,
            total_interactions: 0,
            events: vec!["🐣 Питомец родился!".to_string()],
            pet_speech: Vec::new(),
            last_update: Instant::now(),
        };

        // Инициализация базовых слов
        pet.vocabulary.create_pet_word("hello");
        pet.vocabulary.create_pet_word("feed");
        pet.vocabulary.create_pet_word("play");
        pet.vocabulary.create_pet_word("sleep");
        pet.vocabulary.create_pet_word("happy");

        if let Some(hello_word) = pet.vocabulary.get_pet_word("hello") {
            pet.pet_speech.push(format!("🗣️ {}! 👋", hello_word));
        }

        pet
    }

    fn update(&mut self) {
        let elapsed = self.last_update.elapsed().as_secs_f64();
        self.last_update = Instant::now();

        self.age += elapsed as u64;

        if self.is_sleeping {
            self.energy = (self.energy + elapsed * 3.0).min(100.0);
            self.hunger = (self.hunger - elapsed * 0.5).max(0.0);
            self.happiness = (self.happiness - elapsed * 0.2).max(0.0);
            self.stimulation = (self.stimulation - elapsed * 0.3).max(0.0);
        } else {
            self.hunger = (self.hunger - elapsed * 0.8).max(0.0);
            self.energy = (self.energy - elapsed * 0.5).max(0.0);
            self.happiness = (self.happiness - elapsed * 0.4).max(0.0);
            self.stimulation = (self.stimulation - elapsed * 0.3).max(0.0);
        }

        if self.energy < 15.0 && !self.is_sleeping {
            self.sleep();
        }
        if self.energy > 90.0 && self.is_sleeping {
            self.wake_up();
        }

        self.update_consciousness_state();

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

        self.chemistry.update(self.hunger, self.energy, self.happiness, self.stimulation, elapsed);
        self.speech.update_emotion(self.hunger, self.energy, self.happiness, self.stimulation);
        self.thoughts.update_consciousness(self.spectrum.consciousness_index());

        if let Some(speech) = self.speech.generate_spontaneous_speech(self.hunger, self.energy, self.happiness, self.age) {
            self.pet_speech.push(speech);
            if self.pet_speech.len() > 5 {
                self.pet_speech.remove(0);
            }
        }

        if let Some(_thought) = self.thoughts.generate_thought(self.hunger, self.energy, self.happiness, self.stimulation, self.vocabulary.get_vocabulary_size(), self.age) {
            if let Some(expressed) = self.thoughts.maybe_express_thought() {
                self.pet_speech.push(expressed);
                if self.pet_speech.len() > 5 {
                    self.pet_speech.remove(0);
                }
            }
        }

        let learning_mod = self.chemistry.get_learning_modulation();
        self.learning_network.learning_rate = 0.01 * learning_mod;

        let spike_rate = self.learning_network.get_total_spike_rate();
        self.wave_analyzer.add_sample(spike_rate);

        self.memory.consolidate_memories(self.age);
        self.memory.decay_memories(self.age);

        let chemistry_map: std::collections::HashMap<String, f64> = self.chemistry.levels.clone();
        self.spatial_brain.update(elapsed, &chemistry_map);

        self.submit_to_consciousness();

        if let Some(conscious_info) = self.global_workspace.update(elapsed) {
            self.events.push(format!("💫 Осознал: {} {}", conscious_info.source.emoji(), conscious_info.content));
            if self.events.len() > 10 {
                self.events.remove(0);
            }
        }

        let iit_inputs = vec![
            self.hunger / 100.0,
            self.energy / 100.0,
            self.happiness / 100.0,
            self.global_workspace.get_awareness_index(),
            self.spatial_brain.get_global_connectivity(),
        ];
        self.iit_consciousness.update_network(&iit_inputs);
        self.iit_consciousness.calculate_phi();

        self.update_brain_regions();
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

    fn feed(&mut self) {
        self.hunger = (self.hunger + 40.0).min(100.0);
        self.happiness = (self.happiness + 10.0).min(100.0);
        self.energy = (self.energy + 5.0).min(100.0);

        self.chemistry.release(Neurotransmitter::Dopamine, 0.3);
        let emotion = convert_emotion(self.speech.current_emotion);
        self.memory.remember_event("Покормили".to_string(), emotion, self.hunger, self.energy, self.happiness, self.age);

        let pet_word = self.vocabulary.create_pet_word("feed");
        self.events.push("🍎 Покормили".to_string());
        let response = format!("{} (спасибо!)", pet_word);
        let emotional_response = self.speech.respond_with_emotion(&response, self.age);
        self.pet_speech.push(format!("🗣️ {}", emotional_response));

        if self.events.len() > 10 {
            self.events.remove(0);
        }
        if self.pet_speech.len() > 5 {
            self.pet_speech.remove(0);
        }
    }

    fn play(&mut self) {
        if self.energy > 15.0 {
            self.happiness = (self.happiness + 25.0).min(100.0);
            self.stimulation = (self.stimulation + 20.0).min(100.0);
            self.energy = (self.energy - 8.0).max(0.0);

            self.chemistry.release(Neurotransmitter::Dopamine, 0.4);
            self.chemistry.release(Neurotransmitter::Serotonin, 0.3);

            let emotion = convert_emotion(self.speech.current_emotion);
            self.memory.remember_event("Поиграли".to_string(), emotion, self.hunger, self.energy, self.happiness, self.age);

            let pet_word = self.vocabulary.create_pet_word("play");
            self.events.push("🎮 Поиграли".to_string());
            let response = format!("{}! (весело!)", pet_word);
            let emotional_response = self.speech.respond_with_emotion(&response, self.age);
            self.pet_speech.push(format!("🗣️ {}", emotional_response));
        } else {
            self.events.push("😴 Слишком устал".to_string());
        }

        if self.events.len() > 10 {
            self.events.remove(0);
        }
        if self.pet_speech.len() > 5 {
            self.pet_speech.remove(0);
        }
    }

    fn study(&mut self) {
        if self.energy > 20.0 {
            self.stimulation = (self.stimulation + 35.0).min(100.0);
            self.happiness = (self.happiness + 15.0).min(100.0);
            self.energy = (self.energy - 12.0).max(0.0);

            let pet_word = self.vocabulary.create_pet_word("study");
            self.events.push("📚 Позанимались".to_string());
            self.pet_speech.push(format!("🗣️ {} (интересно!)", pet_word));
        } else {
            self.events.push("😴 Слишком устал".to_string());
        }

        if self.events.len() > 10 {
            self.events.remove(0);
        }
        if self.pet_speech.len() > 5 {
            self.pet_speech.remove(0);
        }
    }

    fn sleep(&mut self) {
        if !self.is_sleeping {
            self.is_sleeping = true;
            self.current_state = ConsciousnessState::DeepSleep;
            self.spectrum.set_consciousness_state(ConsciousnessState::DeepSleep);
            self.chemistry.release(Neurotransmitter::Serotonin, 0.5);
            self.events.push("😴 Заснул".to_string());

            if let Some(word) = self.vocabulary.get_pet_word("sleep") {
                self.pet_speech.push(format!("🗣️ {}...", word));
            }

            if self.events.len() > 10 {
                self.events.remove(0);
            }
            if self.pet_speech.len() > 5 {
                self.pet_speech.remove(0);
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

            if self.events.len() > 10 {
                self.events.remove(0);
            }
            if self.pet_speech.len() > 5 {
                self.pet_speech.remove(0);
            }
        }
    }

    fn get_mood(&self) -> String {
        if self.is_sleeping {
            return "😴 Спит".to_string();
        }

        match self.current_state {
            ConsciousnessState::Insight => "💡 Озарение!".to_string(),
            ConsciousnessState::FlowState => "🌟 В потоке".to_string(),
            ConsciousnessState::Focused => "🎯 Сосредоточен".to_string(),
            ConsciousnessState::Awake => {
                if self.happiness > 70.0 {
                    "😊 Счастлив".to_string()
                } else if self.happiness > 40.0 {
                    "😐 Нормально".to_string()
                } else {
                    "😢 Грустит".to_string()
                }
            }
            ConsciousnessState::Relaxed => "😌 Расслаблен".to_string(),
            ConsciousnessState::Meditation => "🧘 Медитирует".to_string(),
            _ => "😐 Обычное".to_string(),
        }
    }

    fn submit_to_consciousness(&mut self) {
        if self.hunger < 50.0 {
            let salience = if self.hunger < 30.0 { 0.9 } else { 0.6 };
            let info = Information::new(
                InformationSource::Perception,
                format!("Голод: {:.0}%", self.hunger),
                salience,
            );
            self.global_workspace.submit_information(info);
        }

        if self.energy < 50.0 {
            let salience = if self.energy < 20.0 { 0.9 } else { 0.6 };
            let info = Information::new(
                InformationSource::Perception,
                format!("Энергия: {:.0}%", self.energy),
                salience,
            );
            self.global_workspace.submit_information(info);
        }

        if self.happiness > 80.0 {
            let info = Information::new(
                InformationSource::Emotion,
                "Счастлив!".to_string(),
                0.7,
            );
            self.global_workspace.submit_information(info);
        } else if self.happiness < 30.0 {
            let info = Information::new(
                InformationSource::Emotion,
                "Грустно...".to_string(),
                0.6,
            );
            self.global_workspace.submit_information(info);
        }
    }

    fn update_brain_regions(&mut self) {
        let pfc_activation = self.stimulation / 100.0;
        self.spatial_brain.set_region_activation(BrainRegion::PrefrontalCortex, pfc_activation);

        let (episodic_count, _, _) = self.memory.get_memory_stats();
        let memory_activation = (episodic_count as f64 / 20.0).min(1.0);
        self.spatial_brain.set_region_activation(BrainRegion::Hippocampus, memory_activation);

        let emotion_activation = if self.happiness > 70.0 || self.happiness < 30.0 { 0.8 } else { 0.4 };
        self.spatial_brain.set_region_activation(BrainRegion::Amygdala, emotion_activation);

        let dopamine = self.chemistry.levels.get("Дофамин").unwrap_or(&0.5);
        self.spatial_brain.set_region_activation(BrainRegion::VTA, *dopamine);

        let thalamus_activation = self.spectrum.consciousness_index();
        self.spatial_brain.set_region_activation(BrainRegion::Thalamus, thalamus_activation);
    }

    fn get_state(&self) -> PetState {
        let chemistry_report = self.chemistry.get_chemistry_report();
        let chemistry: Vec<ChemistryLevel> = chemistry_report
            .iter()
            .map(|(name, level)| ChemistryLevel {
                name: name.clone(),
                level: *level,
            })
            .collect();

        let brain_regions: Vec<BrainRegionData> = BrainRegion::all()
            .iter()
            .filter_map(|region| {
                let activation = self.spatial_brain.get_region_activation(*region);
                if activation > 0.2 {
                    Some(BrainRegionData {
                        name: region.name().to_string(),
                        emoji: region.emoji().to_string(),
                        activation,
                    })
                } else {
                    None
                }
            })
            .collect();

        PetState {
            name: self.name.clone(),
            hunger: self.hunger,
            energy: self.energy,
            happiness: self.happiness,
            stimulation: self.stimulation,
            age: self.age,
            is_sleeping: self.is_sleeping,
            mood: self.get_mood(),
            consciousness_index: self.spectrum.consciousness_index(),
            phi: self.iit_consciousness.phi,
            awareness: self.global_workspace.get_awareness_index(),
            vocabulary_size: self.vocabulary.get_vocabulary_size(),
            total_interactions: self.total_interactions,
            events: self.events.clone(),
            pet_speech: self.pet_speech.clone(),
            brainwaves: BrainwaveData {
                delta: self.spectrum.delta_power,
                theta: self.spectrum.theta_power,
                alpha: self.spectrum.alpha_power,
                beta: self.spectrum.beta_power,
                gamma: self.spectrum.gamma_power,
            },
            chemistry,
            thoughts: self.thoughts.get_recent_thoughts(5),
            brain_regions,
        }
    }
}

// Глобальное состояние питомца
struct AppState {
    pet: Mutex<BrainPet>,
}

impl AppState {
    fn new() -> Self {
        Self {
            pet: Mutex::new(BrainPet::new("Нейро".to_string())),
        }
    }
}

// Tauri команды
#[tauri::command]
fn get_pet_state(state: tauri::State<AppState>) -> Result<PetState, String> {
    let mut pet = state.pet.lock().map_err(|e| e.to_string())?;
    pet.update();
    Ok(pet.get_state())
}

#[tauri::command]
fn feed_pet(state: tauri::State<AppState>) -> Result<(), String> {
    let mut pet = state.pet.lock().map_err(|e| e.to_string())?;
    pet.feed();
    Ok(())
}

#[tauri::command]
fn play_with_pet(state: tauri::State<AppState>) -> Result<(), String> {
    let mut pet = state.pet.lock().map_err(|e| e.to_string())?;
    pet.play();
    Ok(())
}

#[tauri::command]
fn study_with_pet(state: tauri::State<AppState>) -> Result<(), String> {
    let mut pet = state.pet.lock().map_err(|e| e.to_string())?;
    pet.study();
    Ok(())
}

#[tauri::command]
fn toggle_sleep(state: tauri::State<AppState>) -> Result<(), String> {
    let mut pet = state.pet.lock().map_err(|e| e.to_string())?;
    if pet.is_sleeping {
        pet.wake_up();
    } else {
        pet.sleep();
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            get_pet_state,
            feed_pet,
            play_with_pet,
            study_with_pet,
            toggle_sleep
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
