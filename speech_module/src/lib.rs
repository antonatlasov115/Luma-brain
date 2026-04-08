use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Эмоция для окраски речи
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Emotion {
    Joy,        // Радость
    Sadness,    // Грусть
    Curiosity,  // Любопытство
    Fear,       // Страх
    Anger,      // Злость
    Neutral,    // Нейтральное
}

impl Emotion {
    pub fn emoji(&self) -> &str {
        match self {
            Emotion::Joy => "😊",
            Emotion::Sadness => "😢",
            Emotion::Curiosity => "🤔",
            Emotion::Fear => "😰",
            Emotion::Anger => "😠",
            Emotion::Neutral => "😐",
        }
    }

    pub fn intensity_modifier(&self) -> f64 {
        match self {
            Emotion::Joy => 1.3,
            Emotion::Sadness => 0.7,
            Emotion::Curiosity => 1.1,
            Emotion::Fear => 1.5,
            Emotion::Anger => 1.4,
            Emotion::Neutral => 1.0,
        }
    }
}

/// Тип речи
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpeechType {
    Response(String),           // Ответ на команду
    SpontaneousThought(String), // Спонтанная мысль
    Question(String),           // Вопрос
    Greeting(String),           // Приветствие
    Request(String),            // Просьба
    Exclamation(String),        // Восклицание
}

/// Модуль речи с эмоциями и спонтанностью
#[derive(Clone, Serialize, Deserialize)]
pub struct SpeechModule {
    pub speech_history: VecDeque<(SpeechType, Emotion, u64)>,
    pub spontaneous_rate: f64,
    pub question_rate: f64,
    pub current_emotion: Emotion,
    pub emotion_duration: u64,
    pub last_speech_time: u64,
}

impl SpeechModule {
    pub fn new() -> Self {
        Self {
            speech_history: VecDeque::with_capacity(50),
            spontaneous_rate: 0.5,  // 50% шанс говорить
            question_rate: 0.2,     // 20% шанс задать вопрос
            current_emotion: Emotion::Neutral,
            emotion_duration: 0,
            last_speech_time: 0,
        }
    }

    /// Обновление эмоции на основе состояния
    pub fn update_emotion(&mut self, hunger: f64, energy: f64, happiness: f64, stimulation: f64) {
        let new_emotion = if happiness > 70.0 && energy > 60.0 {
            Emotion::Joy
        } else if hunger < 30.0 || energy < 20.0 {
            Emotion::Sadness
        } else if stimulation > 70.0 {
            Emotion::Curiosity
        } else if energy < 15.0 {
            Emotion::Fear
        } else if hunger < 20.0 && happiness < 30.0 {
            Emotion::Anger
        } else {
            Emotion::Neutral
        };

        if new_emotion != self.current_emotion {
            self.current_emotion = new_emotion;
            self.emotion_duration = 0;
        } else {
            self.emotion_duration += 1;
        }
    }

    /// Спонтанная речь
    pub fn generate_spontaneous_speech(&mut self,
                                       hunger: f64,
                                       energy: f64,
                                       happiness: f64,
                                       current_time: u64) -> Option<String> {
        let mut rng = rand::thread_rng();

        // Проверка времени с последней речи (2 секунды минимум)
        if current_time - self.last_speech_time < 2 {
            return None;
        }

        if rng.gen::<f64>() > self.spontaneous_rate {
            return None;
        }

        let speech = match self.current_emotion {
            Emotion::Joy => {
                let phrases = vec![
                    "Как хорошо!",
                    "Мне нравится!",
                    "Весело!",
                    "Ура!",
                ];
                phrases[rng.gen_range(0..phrases.len())].to_string()
            }
            Emotion::Sadness => {
                if hunger < 30.0 {
                    "Хочу кушать...".to_string()
                } else if energy < 30.0 {
                    "Устал...".to_string()
                } else {
                    "Грустно...".to_string()
                }
            }
            Emotion::Curiosity => {
                let phrases = vec![
                    "Интересно...",
                    "Что это?",
                    "Хочу узнать больше!",
                ];
                phrases[rng.gen_range(0..phrases.len())].to_string()
            }
            Emotion::Fear => {
                "Страшно...".to_string()
            }
            Emotion::Anger => {
                "Не нравится!".to_string()
            }
            Emotion::Neutral => {
                if rng.gen::<f64>() < 0.5 {
                    return None;
                }
                let phrases = vec![
                    "Хм...",
                    "Думаю...",
                    "...",
                ];
                phrases[rng.gen_range(0..phrases.len())].to_string()
            }
        };

        self.last_speech_time = current_time;
        self.record_speech(SpeechType::SpontaneousThought(speech.clone()), current_time);
        Some(format!("{} {}", self.current_emotion.emoji(), speech))
    }

    /// Генерация вопроса
    pub fn generate_question(&mut self, vocabulary_size: usize, current_time: u64) -> Option<String> {
        let mut rng = rand::thread_rng();

        if rng.gen::<f64>() > self.question_rate {
            return None;
        }

        let question = if vocabulary_size < 5 {
            "Как это называется?".to_string()
        } else if vocabulary_size < 15 {
            let questions = vec![
                "Что это значит?",
                "Расскажи еще!",
                "Можем поиграть?",
            ];
            questions[rng.gen_range(0..questions.len())].to_string()
        } else {
            let questions = vec![
                "Что нового?",
                "Как дела?",
                "Что будем делать?",
                "Расскажи что-нибудь!",
            ];
            questions[rng.gen_range(0..questions.len())].to_string()
        };

        self.last_speech_time = current_time;
        self.record_speech(SpeechType::Question(question.clone()), current_time);
        Some(format!("❓ {}", question))
    }

    /// Инициатива в общении
    pub fn initiate_conversation(&mut self,
                                  hunger: f64,
                                  energy: f64,
                                  happiness: f64,
                                  current_time: u64) -> Option<String> {
        let mut rng = rand::thread_rng();

        if current_time - self.last_speech_time < 10 {
            return None;
        }

        if rng.gen::<f64>() > 0.1 {
            return None;
        }

        let message = if hunger < 30.0 {
            "Покормишь меня?".to_string()
        } else if energy < 30.0 {
            "Хочу спать...".to_string()
        } else if happiness > 70.0 {
            let greetings = vec![
                "Привет!",
                "Как дела?",
                "Давай поиграем!",
            ];
            greetings[rng.gen_range(0..greetings.len())].to_string()
        } else {
            return None;
        };

        self.last_speech_time = current_time;
        self.record_speech(SpeechType::Greeting(message.clone()), current_time);
        Some(format!("💬 {}", message))
    }

    /// Ответ с эмоцией
    pub fn respond_with_emotion(&mut self, base_response: &str, current_time: u64) -> String {
        self.record_speech(SpeechType::Response(base_response.to_string()), current_time);
        format!("{} {}", self.current_emotion.emoji(), base_response)
    }

    fn record_speech(&mut self, speech_type: SpeechType, time: u64) {
        self.speech_history.push_back((speech_type, self.current_emotion, time));
        if self.speech_history.len() > 50 {
            self.speech_history.pop_front();
        }
    }

    pub fn get_recent_speeches(&self, n: usize) -> Vec<String> {
        self.speech_history
            .iter()
            .rev()
            .take(n)
            .map(|(speech_type, emotion, _)| {
                match speech_type {
                    SpeechType::Response(s) => format!("{} {}", emotion.emoji(), s),
                    SpeechType::SpontaneousThought(s) => format!("💭 {}", s),
                    SpeechType::Question(s) => format!("❓ {}", s),
                    SpeechType::Greeting(s) => format!("💬 {}", s),
                    SpeechType::Request(s) => format!("🙏 {}", s),
                    SpeechType::Exclamation(s) => format!("❗ {}", s),
                }
            })
            .collect()
    }
}

impl Default for SpeechModule {
    fn default() -> Self {
        Self::new()
    }
}
