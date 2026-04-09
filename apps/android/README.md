# Нейро-Тамагочи - Android версия

## Структура проекта

```
brainwave_project/
├── apps/
│   └── android/
│       └── neuro-tamagotchi-mobile/  # Tauri приложение
│           ├── src/                   # Frontend (TypeScript)
│           ├── src-tauri/             # Backend (Rust)
│           └── gen/                   # Android проект (после init)
├── brainwave_core/                    # Спайковые нейросети
├── speech_module/                     # Модуль речи
├── memory_module/                     # Модуль памяти
├── thought_module/                    # Модуль мыслей
├── chemistry_module/                  # Нейрохимия
├── wave_analyzer/                     # Анализатор волн
└── consciousness_module/              # Модули сознания (GNW, IIT)
```

## Возможности

- **Обучаемый мозг**: Спайковые нейросети с адаптивным обучением
- **Мозговые волны**: Delta, Theta, Alpha, Beta, Gamma
- **Нейрохимия**: Дофамин, серотонин, норадреналин и др.
- **Сознание**: Global Workspace Theory + Integrated Information Theory
- **Собственный язык**: Питомец создает свои слова
- **Эмоциональная память**: Запоминает события с эмоциями
- **Мобильный UI**: Адаптивный интерфейс с тач-управлением

## Сборка APK

### Требования

1. **Java JDK 17+**
   ```bash
   # Установка в Termux
   pkg install openjdk-17
   export JAVA_HOME=$PREFIX/opt/openjdk
   ```

2. **Android SDK & NDK**
   ```bash
   # Установка Android SDK
   pkg install android-tools
   
   # Скачать Android SDK командной строки
   # https://developer.android.com/studio#command-tools
   ```

3. **Rust с Android targets**
   ```bash
   rustup target add aarch64-linux-android
   rustup target add armv7-linux-androideabi
   rustup target add i686-linux-android
   rustup target add x86_64-linux-android
   ```

### Шаги сборки

1. **Инициализация Android проекта**
   ```bash
   cd apps/android/neuro-tamagotchi-mobile
   npm install
   npm run tauri android init
   ```

2. **Сборка APK**
   ```bash
   # Debug версия
   npm run tauri android build
   
   # Release версия
   npm run tauri android build -- --release
   ```

3. **APK будет в**
   ```
   src-tauri/gen/android/app/build/outputs/apk/
   ```

## Альтернатива: Запуск в браузере

Если сборка APK не работает, можно запустить как веб-приложение:

```bash
cd apps/android/neuro-tamagotchi-mobile
npm run tauri dev
```

Откроется окно с приложением, которое можно использовать на любом устройстве.

## Альтернатива 2: Оригинальная TUI версия

Оригинальная версия работает прямо в Termux:

```bash
cd brainwave_project/brainwave_tamagotchi
cargo run --release
```

Управление:
- `F` - Покормить
- `P` - Играть
- `S` - Учиться
- `Z` - Спать/Проснуться
- `T` - Говорить с питомцем
- `Q` - Выход

## Упрощенная сборка для Termux

Если полная сборка не работает, можно собрать только desktop версию:

```bash
cd apps/android/neuro-tamagotchi-mobile
npm run tauri build
```

Это создаст исполняемый файл для Linux, который можно запустить в Termux.

## Troubleshooting

### Java не найдена
```bash
export JAVA_HOME=$PREFIX/opt/openjdk
export PATH=$JAVA_HOME/bin:$PATH
```

### Android SDK не найден
```bash
export ANDROID_HOME=$HOME/android-sdk
export PATH=$ANDROID_HOME/cmdline-tools/latest/bin:$PATH
```

### Ошибки компиляции Rust
```bash
# Очистить кеш
cargo clean
# Пересобрать
cargo build --release
```

## Особенности мобильной версии

- **Тач-управление**: Большие кнопки для удобства
- **Адаптивный дизайн**: Работает на любом размере экрана
- **Автообновление**: Состояние обновляется каждые 500мс
- **Вкладки**: Мозг, Химия, Сознание, События
- **Визуализация**: Прогресс-бары для всех параметров

## API команды (Rust → TypeScript)

```typescript
// Получить состояние
const state = await invoke("get_pet_state");

// Действия
await invoke("feed_pet");
await invoke("play_with_pet");
await invoke("study_with_pet");
await invoke("toggle_sleep");
```

## Производительность

- **Backend**: Rust - нативная скорость
- **Frontend**: TypeScript + Vite - быстрый рендеринг
- **Обновления**: 500мс интервал (можно настроить)
- **Память**: ~50MB RAM

## Roadmap

- [ ] Сохранение состояния в файл
- [ ] Уведомления (голод, энергия)
- [ ] Виджет на главный экран
- [ ] Темная/светлая тема
- [ ] Звуковые эффекты
- [ ] Анимации питомца
