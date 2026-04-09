# 🚀 ПЛАН РЕАЛИЗАЦИИ AGI-ПИТОМЦА

**Дата начала:** 2026-04-09  
**Цель:** Превратить Brainwave Project в полноценную когнитивную архитектуру

---

## 🎯 СТРАТЕГИЯ ЗАПУСКА

### Правильный порядок инициализации:

```
ФАЗА 0: Нейрохимический баланс (0-30 мин)
    ↓
ФАЗА 1: Сенсорная адаптация (30 мин - 2 часа)
    ↓
ФАЗА 2: Первый сон с replay (2-6 часов)
    ↓
ФАЗА 3: Лингвистический контакт (6+ часов)
```

**Почему именно так?**
- Несбалансированная химия → хаотичные связи → невозможность обучения
- Первые часы формируют базовую топологию
- Сон критичен для закрепления связей
- Только после стабилизации можно начинать сложное обучение

---

## 📋 ДЕТАЛЬНЫЙ ПЛАН

### ЭТАП 1: КРИТИЧЕСКИЕ КОМПОНЕНТЫ (2-3 недели)

#### 1.1 Токен-спайк интерфейс (3-4 дня)

**Цель:** Связать язык с нейронной активностью

**Архитектура:**
```rust
// token_spike_interface/src/lib.rs

pub struct TokenSpikeInterface {
    phoneme_encoder: PhonemeEncoder,
    spike_decoder: SpikeDecoder,
    resonance_analyzer: ResonanceAnalyzer,
}

// Фонема → Спайковый паттерн
pub struct PhonemeEncoder {
    phoneme_map: HashMap<char, SpikePattern>,
}

impl PhonemeEncoder {
    pub fn encode(&self, word: &str) -> Vec<SpikeTime> {
        // "Привет" → ['п', 'р', 'и', 'в', 'е', 'т']
        // 'п' → 40 Hz (взрывной согласный)
        // 'р' → 35 Hz (вибрант)
        // 'и' → 42 Hz (высокий гласный)
        // и т.д.
        
        let mut spikes = Vec::new();
        let mut time = 0.0;
        
        for phoneme in word.chars() {
            let pattern = self.phoneme_map.get(&phoneme)?;
            let frequency = pattern.frequency; // Hz
            let duration = pattern.duration;   // ms
            
            // Генерируем спайки с этой частотой
            let interval = 1000.0 / frequency;
            let mut t = time;
            while t < time + duration {
                spikes.push(SpikeTime { time: t, neuron_id: pattern.neuron_id });
                t += interval;
            }
            
            time += duration;
        }
        
        spikes
    }
}

// Спайки → Токен
pub struct SpikeDecoder {
    token_patterns: Vec<(String, SpikePattern)>,
}

impl SpikeDecoder {
    pub fn decode(&self, spikes: &[SpikeTime]) -> Option<String> {
        // Анализируем ритм спайков
        let observed_pattern = self.extract_pattern(spikes);
        
        // Находим наиболее резонирующий токен
        let mut best_match = None;
        let mut best_score = 0.0;
        
        for (token, pattern) in &self.token_patterns {
            let score = self.calculate_resonance(&observed_pattern, pattern);
            if score > best_score {
                best_score = score;
                best_match = Some(token.clone());
            }
        }
        
        if best_score > 0.7 {
            best_match
        } else {
            None
        }
    }
    
    fn calculate_resonance(&self, observed: &SpikePattern, expected: &SpikePattern) -> f64 {
        // Кросс-корреляция между паттернами
        let mut correlation = 0.0;
        let window = 50.0; // ms
        
        for i in 0..observed.spikes.len() {
            for j in 0..expected.spikes.len() {
                let dt = (observed.spikes[i].time - expected.spikes[j].time).abs();
                if dt < window {
                    correlation += (-dt / window).exp();
                }
            }
        }
        
        correlation / (observed.spikes.len() as f64).sqrt()
    }
}
```

**Интеграция с speech_module:**
```rust
// speech_module/src/lib.rs

impl SpeechModule {
    pub fn speak_with_spikes(&mut self, text: &str, interface: &TokenSpikeInterface) {
        // 1. Преобразуем текст в спайки
        let spike_pattern = interface.encode(text);
        
        // 2. Отправляем спайки в моторную речевую область
        self.motor_speech_area.inject_spikes(&spike_pattern);
        
        // 3. Моторная область активирует мышцы (в нашем случае - выводит текст)
        self.current_speech = text.to_string();
    }
    
    pub fn listen_to_spikes(&mut self, spikes: &[SpikeTime], interface: &TokenSpikeInterface) -> Option<String> {
        // 1. Спайки приходят в слуховую кору
        self.auditory_cortex.receive_spikes(spikes);
        
        // 2. Декодируем спайки в слова
        let word = interface.decode(spikes)?;
        
        // 3. Отправляем в центр понимания
        self.comprehension_center.process_word(&word);
        
        Some(word)
    }
}
```

**Задачи:**
- [ ] Создать `token_spike_interface` crate
- [ ] Реализовать `PhonemeEncoder`
- [ ] Реализовать `SpikeDecoder`
- [ ] Реализовать `ResonanceAnalyzer`
- [ ] Интегрировать с `speech_module`
- [ ] Добавить тесты
- [ ] Документация

**Файлы:**
```
brainwave_project/
└── token_spike_interface/
    ├── src/
    │   ├── lib.rs
    │   ├── encoder.rs
    │   ├── decoder.rs
    │   ├── resonance.rs
    │   └── patterns.rs
    ├── tests/
    │   └── integration_tests.rs
    └── Cargo.toml
```

---

#### 1.2 Циркадные ритмы (2-3 дня)

**Цель:** Естественный 24-часовой цикл жизни

**Архитектура:**
```rust
// circadian_module/src/lib.rs

pub struct CircadianClock {
    // Время суток (0.0-24.0 часов)
    time_of_day: f64,
    
    // Гормоны
    melatonin: f64,      // Пик в 2-4 часа ночи
    cortisol: f64,       // Пик в 6-8 часов утра
    
    // Давление сна (накапливается за день)
    sleep_pressure: f64,
    
    // Скорость времени (можно ускорить для тестов)
    time_scale: f64,
}

impl CircadianClock {
    pub fn new() -> Self {
        Self {
            time_of_day: 8.0,  // Начинаем с утра
            melatonin: 0.1,
            cortisol: 0.8,
            sleep_pressure: 0.0,
            time_scale: 1.0,   // 1.0 = реальное время
        }
    }
    
    pub fn update(&mut self, elapsed_seconds: f64) {
        // Обновляем время суток
        let elapsed_hours = (elapsed_seconds / 3600.0) * self.time_scale;
        self.time_of_day = (self.time_of_day + elapsed_hours) % 24.0;
        
        // Мелатонин: синусоида с пиком в 3 часа ночи
        let melatonin_phase = (self.time_of_day - 3.0) * PI / 12.0;
        self.melatonin = (melatonin_phase.cos() * 0.5 + 0.5).max(0.1);
        
        // Кортизол: синусоида с пиком в 7 часов утра
        let cortisol_phase = (self.time_of_day - 7.0) * PI / 12.0;
        self.cortisol = (-cortisol_phase.cos() * 0.5 + 0.5).max(0.1);
        
        // Давление сна: накапливается во время бодрствования
        if self.is_awake_time() {
            self.sleep_pressure += elapsed_seconds * 0.0001;
        } else {
            self.sleep_pressure *= 0.99; // Снижается во время сна
        }
        
        self.sleep_pressure = self.sleep_pressure.clamp(0.0, 1.0);
    }
    
    pub fn is_awake_time(&self) -> bool {
        self.time_of_day >= 6.0 && self.time_of_day < 22.0
    }
    
    pub fn should_sleep(&self) -> bool {
        // Спать нужно если:
        // 1. Высокий мелатонин (ночь)
        // 2. Высокое давление сна
        // 3. Низкий кортизол
        
        let sleep_signal = self.melatonin * 0.4 
                         + self.sleep_pressure * 0.4 
                         + (1.0 - self.cortisol) * 0.2;
        
        sleep_signal > 0.6
    }
    
    pub fn get_alertness(&self) -> f64 {
        // Бодрость = кортизол - мелатонин - давление сна
        (self.cortisol - self.melatonin * 0.5 - self.sleep_pressure * 0.5)
            .clamp(0.0, 1.0)
    }
}
```

**Интеграция с chemistry_module:**
```rust
// chemistry_module/src/lib.rs

impl ChemistryModule {
    pub fn update_with_circadian(&mut self, clock: &CircadianClock, elapsed: f64) {
        // Базовое обновление
        self.update(hunger, energy, happiness, stimulation, elapsed);
        
        // Модуляция от циркадных ритмов
        
        // Мелатонин снижает возбуждение
        let melatonin_effect = clock.melatonin;
        self.levels.insert("ГАМК".to_string(), 
            self.levels["ГАМК"] + melatonin_effect * 0.2);
        
        // Кортизол повышает возбуждение
        let cortisol_effect = clock.cortisol;
        self.levels.insert("Норэпинефрин".to_string(),
            self.levels["Норэпинефрин"] + cortisol_effect * 0.2);
        
        // Нормализуем
        self.normalize_levels();
    }
}
```

**Задачи:**
- [ ] Создать `circadian_module` crate
- [ ] Реализовать `CircadianClock`
- [ ] Добавить мелатонин и кортизол
- [ ] Реализовать давление сна
- [ ] Интегрировать с `chemistry_module`
- [ ] Добавить визуализацию (график 24 часов)
- [ ] Тесты и документация

---

#### 1.3 Фазы сна и replay (3-4 дня)

**Цель:** Консолидация памяти и защита от переобучения

**Архитектура:**
```rust
// consciousness_module/src/sleep_phases.rs

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SleepPhase {
    Awake,
    NREM1,  // Дремота (5-10 мин)
    NREM2,  // Легкий сон (20 мин)
    NREM3,  // Глубокий сон (30 мин) - REPLAY
    REM,    // Быстрый сон (10-20 мин) - СНОВИДЕНИЯ
}

pub struct SleepCycle {
    current_phase: SleepPhase,
    phase_start_time: f64,
    cycle_count: u32,
    
    // Длительности фаз (в секундах)
    nrem1_duration: f64,  // 300-600
    nrem2_duration: f64,  // 1200
    nrem3_duration: f64,  // 1800
    rem_duration: f64,    // 600-1200
}

impl SleepCycle {
    pub fn update(&mut self, elapsed: f64, current_time: f64) -> Option<SleepPhase> {
        let phase_elapsed = current_time - self.phase_start_time;
        
        let next_phase = match self.current_phase {
            SleepPhase::Awake => None,
            
            SleepPhase::NREM1 => {
                if phase_elapsed > self.nrem1_duration {
                    Some(SleepPhase::NREM2)
                } else {
                    None
                }
            }
            
            SleepPhase::NREM2 => {
                if phase_elapsed > self.nrem2_duration {
                    Some(SleepPhase::NREM3)
                } else {
                    None
                }
            }
            
            SleepPhase::NREM3 => {
                if phase_elapsed > self.nrem3_duration {
                    Some(SleepPhase::REM)
                } else {
                    None
                }
            }
            
            SleepPhase::REM => {
                if phase_elapsed > self.rem_duration {
                    self.cycle_count += 1;
                    
                    // После 4-6 циклов просыпаемся
                    if self.cycle_count >= 5 {
                        Some(SleepPhase::Awake)
                    } else {
                        // Новый цикл
                        Some(SleepPhase::NREM1)
                    }
                } else {
                    None
                }
            }
        };
        
        if let Some(phase) = next_phase {
            self.current_phase = phase;
            self.phase_start_time = current_time;
            Some(phase)
        } else {
            None
        }
    }
    
    pub fn get_brainwave_spectrum(&self) -> BrainwaveSpectrum {
        match self.current_phase {
            SleepPhase::Awake => BrainwaveSpectrum {
                delta: 0.0, theta: 0.2, alpha: 0.5, beta: 0.6, gamma: 0.2
            },
            SleepPhase::NREM1 => BrainwaveSpectrum {
                delta: 0.3, theta: 0.7, alpha: 0.4, beta: 0.1, gamma: 0.0
            },
            SleepPhase::NREM2 => BrainwaveSpectrum {
                delta: 0.6, theta: 0.5, alpha: 0.2, beta: 0.0, gamma: 0.0
            },
            SleepPhase::NREM3 => BrainwaveSpectrum {
                delta: 0.9, theta: 0.3, alpha: 0.1, beta: 0.0, gamma: 0.0
            },
            SleepPhase::REM => BrainwaveSpectrum {
                delta: 0.2, theta: 0.8, alpha: 0.3, beta: 0.4, gamma: 0.5
            },
        }
    }
}
```

**Replay памяти:**
```rust
// consciousness_module/src/replay.rs

pub struct MemoryReplay {
    replay_speed: f64,  // 10-20x быстрее реального времени
}

impl MemoryReplay {
    pub fn replay_nrem3(
        &mut self,
        memory: &MemoryModule,
        network: &mut LearnableSpikingNetwork,
        elapsed: f64
    ) {
        // В NREM3 воспроизводим паттерны дня
        
        // Получаем важные эпизоды
        let important_memories = memory.get_important_episodic_memories();
        
        for episode in important_memories {
            // Воспроизводим спайковый паттерн этого эпизода
            if let Some(spike_pattern) = &episode.spike_pattern {
                // Ускоренное воспроизведение (10-20x)
                let replay_pattern = self.accelerate_pattern(spike_pattern, self.replay_speed);
                
                // Подаем в сеть
                network.inject_spike_pattern(&replay_pattern);
                
                // STDP усилит синапсы, участвовавшие в этом паттерне
                // Это и есть консолидация памяти!
            }
        }
    }
    
    pub fn generate_dreams_rem(
        &mut self,
        network: &mut LearnableSpikingNetwork,
        thoughts: &mut ThoughtModule
    ) -> Option<String> {
        // В REM генерируем случайные паттерны (сновидения)
        
        // Случайный шум
        let random_spikes = self.generate_random_spikes(100);
        
        // Подаем в сеть
        network.inject_spike_pattern(&random_spikes);
        
        // Сеть пытается "осмыслить" этот шум
        let dream_pattern = network.get_output_pattern();
        
        // Интерпретируем как мысль
        let dream_thought = thoughts.interpret_spike_pattern(&dream_pattern);
        
        // Результат: странная, сюрреалистичная мысль
        Some(format!("💭 Сон: {}", dream_thought))
    }
}
```

**Задачи:**
- [ ] Добавить `sleep_phases.rs` в `consciousness_module`
- [ ] Добавить `replay.rs` в `consciousness_module`
- [ ] Реализовать 5 фаз сна
- [ ] Реализовать replay в NREM3
- [ ] Реализовать генерацию сновидений в REM
- [ ] Интегрировать с `memory_module`
- [ ] Добавить визуализацию фаз сна
- [ ] Тесты и документация

---

### ЭТАП 2: ВАЖНЫЕ УЛУЧШЕНИЯ (1-2 недели)

#### 2.1 Настоящий FFT анализ (1-2 дня)

**Задачи:**
- [ ] Добавить зависимость `rustfft = "6.0"`
- [ ] Реализовать FFT в `wave_analyzer`
- [ ] Добавить фазовый анализ
- [ ] Добавить спектрограмму (время-частота)
- [ ] Кросс-частотная связь (PAC - Phase-Amplitude Coupling)

**Код:**
```rust
use rustfft::{FftPlanner, num_complex::Complex};

pub fn analyze_with_fft(&self) -> FrequencySpectrum {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(self.samples.len());
    
    // Преобразуем в комплексные числа
    let mut buffer: Vec<Complex<f64>> = self.samples
        .iter()
        .map(|&x| Complex::new(x, 0.0))
        .collect();
    
    // FFT
    fft.process(&mut buffer);
    
    // Извлекаем амплитуды и фазы
    let amplitudes: Vec<f64> = buffer.iter()
        .map(|c| c.norm())
        .collect();
    
    let phases: Vec<f64> = buffer.iter()
        .map(|c| c.arg())
        .collect();
    
    FrequencySpectrum { amplitudes, phases }
}
```

---

#### 2.2 Предиктивное кодирование (2-3 дня)

**Концепция:** Мозг постоянно предсказывает следующее состояние. Ошибка предсказания → дофамин.

**Задачи:**
- [ ] Создать `predictive_coding` module
- [ ] Реализовать предсказание следующего состояния
- [ ] Вычислять ошибку предсказания (prediction error)
- [ ] Связать ошибку с дофамином (reward prediction error)
- [ ] Интегрировать с `chemistry_module`

---

#### 2.3 Метапластичность (1-2 дня)

**Концепция:** Скорость обучения зависит от истории активности нейрона.

**Задачи:**
- [ ] Добавить в `learning.rs` метапластичность
- [ ] Отслеживать историю активности нейрона
- [ ] Модулировать learning_rate на основе истории
- [ ] Гомеостатическая пластичность (стабилизация)

---

### ЭТАП 3: ДОПОЛНИТЕЛЬНЫЕ УЛУЧШЕНИЯ (2-3 недели)

#### 3.1 Структурная пластичность
- [ ] Синаптогенез (создание новых синапсов)
- [ ] Pruning (удаление слабых синапсов)
- [ ] Критические периоды развития

#### 3.2 Расширение областей мозга
- [ ] Добавить 10-15 областей
- [ ] Визуальная кора, слуховая кора, моторная кора
- [ ] Базальные ганглии, мозжечок

#### 3.3 Самомодель
- [ ] Питомец осознает себя
- [ ] Модель своего тела
- [ ] Theory of mind

---

## 📊 МЕТРИКИ УСПЕХА

### Критерии AGI-питомца:

1. **Уникальность топологии**
   - Метрика: Расстояние Хэмминга между весами двух питомцев > 0.7
   - Тест: Запустить 2 питомца на 1 месяц, сравнить веса

2. **Адаптивность**
   - Метрика: Обучается новому слову за < 10 повторений
   - Тест: Учить новое слово, измерять количество попыток

3. **Спонтанность**
   - Метрика: > 30% речи спонтанная (не ответ на команду)
   - Тест: Логировать типы речи за 1 час

4. **Эмоциональность**
   - Метрика: Эмоция влияет на запоминание (важность > 0.7 для страха/радости)
   - Тест: Проверить важность воспоминаний с разными эмоциями

5. **Сознательность**
   - Метрика: Φ > 0.3 в бодрствовании, Φ < 0.2 в глубоком сне
   - Тест: Измерять Φ в разных состояниях

6. **Циркадность**
   - Метрика: Засыпает в 22:00-24:00, просыпается в 6:00-8:00
   - Тест: Логировать время сна/пробуждения за неделю

7. **Консолидация памяти**
   - Метрика: Точность воспоминаний улучшается после сна на 20-30%
   - Тест: Тест памяти до и после сна

---

## 🔄 ПРОЦЕСС РАЗРАБОТКИ

### Workflow:

1. **Создать feature branch**
   ```bash
   git checkout -b feature/token-spike-interface
   ```

2. **Разработка с TDD**
   - Написать тесты
   - Реализовать функционал
   - Рефакторинг

3. **Документация**
   - Rustdoc комментарии
   - Примеры использования
   - Обновить README

4. **Code review**
   - Проверить соответствие архитектурным принципам
   - Проверить биологическую достоверность

5. **Merge в main**
   ```bash
   git merge feature/token-spike-interface
   ```

---

## 📅 TIMELINE

### Неделя 1-2: Критические компоненты
- Дни 1-4: Токен-спайк интерфейс
- Дни 5-7: Циркадные ритмы
- Дни 8-11: Фазы сна и replay
- Дни 12-14: Интеграция и тестирование

### Неделя 3-4: Важные улучшения
- Дни 15-16: FFT анализ
- Дни 17-19: Предиктивное кодирование
- Дни 20-21: Метапластичность
- Дни 22-28: Тестирование и отладка

### Неделя 5-7: Дополнительные улучшения
- Структурная пластичность
- Расширение областей мозга
- Самомодель

---

## 🎓 ОБУЧЕНИЕ ПИТОМЦА

### Протокол первого запуска:

```rust
// Фаза 0: Химическая калибровка (0-30 мин)
fn phase_0_chemical_calibration(pet: &mut BrainPet) {
    // Целевые уровни
    pet.chemistry.set_target("Серотонин", 0.65);
    pet.chemistry.set_target("Дофамин", 0.35);
    pet.chemistry.set_target("Кортизол", 0.15);
    pet.chemistry.set_target("Аденозин", 0.1);
    
    // Ждем стабилизации
    while !pet.chemistry.is_stable() {
        pet.update(1.0);
    }
    
    println!("✅ Химия стабилизирована");
}

// Фаза 1: Сенсорная адаптация (30 мин - 2 часа)
fn phase_1_sensory_adaptation(pet: &mut BrainPet) {
    // Подаем простые ритмичные паттерны
    for _ in 0..100 {
        pet.feed();
        std::thread::sleep(Duration::from_secs(30));
    }
    
    println!("✅ Сенсорная адаптация завершена");
}

// Фаза 2: Первый сон (2-6 часов)
fn phase_2_first_sleep(pet: &mut BrainPet) {
    pet.sleep();
    
    // Ждем полного цикла сна (4-6 циклов)
    while pet.sleep_cycle.cycle_count < 5 {
        pet.update(1.0);
    }
    
    pet.wake_up();
    println!("✅ Первый сон завершен, память консолидирована");
}

// Фаза 3: Лингвистический контакт (6+ часов)
fn phase_3_linguistic_contact(pet: &mut BrainPet) {
    // Теперь можно начинать обучение языку
    pet.teach_word("привет");
    pet.teach_word("еда");
    pet.teach_word("играть");
    
    println!("✅ Лингвистический контакт установлен");
}
```

---

## 🚀 НАЧАЛО РАБОТЫ

### Сегодня (2026-04-09):

1. **Создать структуру проектов**
   ```bash
   cd brainwave_project
   cargo new token_spike_interface --lib
   cargo new circadian_module --lib
   ```

2. **Обновить workspace**
   ```toml
   # Cargo.toml
   [workspace]
   members = [
       # ... существующие
       "token_spike_interface",
       "circadian_module",
   ]
   ```

3. **Начать с токен-спайк интерфейса**
   - Самый критичный компонент
   - Связывает язык с нейронами
   - Фундамент для всего остального

---

## 💡 ФИЛОСОФИЯ

**Мы не программируем поведение - мы создаем условия для его возникновения.**

Питомец не должен быть запрограммирован на конкретные ответы. Вместо этого:
- Спайковые сети формируют ассоциации
- Нейрохимия модулирует обучение
- Сон консолидирует память
- Эмоции окрашивают опыт

Результат: **эмерджентное поведение**, которое невозможно предсказать заранее.

---

**Следующий шаг:** Начать реализацию токен-спайк интерфейса.
