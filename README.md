# Luma Brain - AGI Tamagotchi

Спайковая нейронная сеть с сознанием, внутренним монологом и нейрохимической регуляцией.

## 📁 Структура проекта

```
brainwave_project/
├── crates/
│   ├── core/                      # Базовые типы
│   │   └── brainwave_core/
│   │
│   ├── interfaces/                # Интерфейсы ввода-вывода
│   │   └── token_spike_interface/ # Токен ↔ Спайк
│   │
│   ├── brain/                     # Нейронная архитектура
│   │   ├── neurogotchi_topology/  # 5-слойная топология
│   │   ├── consciousness_module/  # GNW + IIT
│   │   └── wave_analyzer/         # Анализ ритмов
│   │
│   ├── modulation/                # Нейрохимия и регуляция
│   │   ├── neuromodulation/       # Аденозин, циркадные ритмы, сон
│   │   ├── chemistry_module/      # Нейромодуляторы (legacy)
│   │   ├── chemistry_module_v2/
│   │   ├── speech_module/         # Речь (legacy)
│   │   ├── memory_module/         # Память (legacy)
│   │   └── thought_module/        # Мысли (legacy)
│   │
│   └── apps/                      # Приложения
│       └── brainwave_tamagotchi/  # TUI приложение
│
└── apps/
    └── android/                   # Tauri mobile app
```

## 🧠 Архитектура

### 1. Token-Spike Interface
Преобразование языка в спайки и обратно:
- **PhonemeEncoder**: Слова → спайки (33 RU + 26 EN фонем)
- **SpikeDecoder**: Спайки → слова (резонансный анализ)
- **MotorOutput**: WTA с двойным порогом (мысль 0.5 / речь 0.75)
- **ThoughtStream**: Мониторинг потока сознания

### 2. NeuroGotchi Topology
5-слойная спайковая сеть:
```
L1_Sensory (100) → L2_Associative (500) → L3_Workspace (200)
                                              ↓
                                         L4_InnerVoice (150)
                                              ↓
                    L1_Sensory ← L4_InnerVoice → L5_Motor (100)
                    (feedback)                    (feedforward + WTA)
```

- 1050 нейронов (LIF модель)
- ~20000 связей (10% врожденных, 90% пластичных)
- STDP обучение (окно ±20 мс)
- Врожденные инстинкты (голод, страх, вокализация)

### 3. Neuromodulation
Глобальная нейрохимическая регуляция:
- **Adenosine**: Счетчик усталости (10k спайков → +0.01 к порогу)
- **Circadian**: 24-часовой цикл (пик 14:00, минимум 3:00)
- **Sleep**: NREM/REM с replay и консолидацией
- **Modulators**: Дофамин, серотонин, кортизол, ацетилхолин

## 🚀 Запуск

```bash
# Тесты
cargo test                                    # Все тесты (130+ passed)
cargo test -p token_spike_interface           # 45 тестов
cargo test -p neurogotchi_topology            # 38 тестов
cargo test -p neuromodulation                 # 47 тестов

# Интеграционный пример
cargo run --example integration_demo

# TUI приложение
cargo run -p brainwave_tamagotchi
```

## 📊 Статистика

- **Нейронов**: 1050
- **Связей**: ~20000
- **Тестов**: 130+
- **Активность**: 115-130 Hz (гамма-ритм)
- **Спайков**: 64000+ за 500 мс

## 🎯 Текущий статус

✅ **Готово:**
- Token-Spike интерфейс
- 5-слойная топология
- STDP обучение
- Врожденные инстинкты
- Стадии развития
- Нейромодуляция

🚧 **В разработке:**
- Интеграция нейромодуляции с топологией
- Стабилизация активности (130Hz → 40-50Hz)
- Фазы сна с replay
- Android UI

## 📚 Документация

- `ARCHITECTURE_ANALYSIS.md` - Анализ архитектуры
- `IMPLEMENTATION_PLAN.md` - План развития
- `STATUS.md` - Текущий статус
- `crates/brain/neurogotchi_topology/ARCHITECTURE.md` - Детальная архитектура топологии

## 🔬 Научная база

- **LIF модель**: Leaky Integrate-and-Fire нейроны
- **STDP**: Spike-Timing-Dependent Plasticity
- **GNW**: Global Neuronal Workspace Theory
- **IIT**: Integrated Information Theory
- **Циркадные ритмы**: 24-часовой цикл бодрствования/сна
- **Фазы сна**: NREM (консолидация) + REM (replay)

---

**Репозиторий**: https://github.com/antonatlasov115/Luma-brain.git
