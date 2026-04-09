#!/bin/bash

# Скрипт для сборки Android APK
# Использование: ./build-android.sh [debug|release]

set -e

BUILD_TYPE=${1:-debug}

echo "🧠 Сборка Нейро-Тамагочи для Android..."
echo "Тип сборки: $BUILD_TYPE"

# Проверка Java
if ! command -v java &> /dev/null; then
    echo "❌ Java не найдена!"
    echo "Установите: pkg install openjdk-17"
    exit 1
fi

echo "✅ Java: $(java -version 2>&1 | head -n 1)"

# Проверка Rust targets
echo "📦 Проверка Rust targets..."
TARGETS=("aarch64-linux-android" "armv7-linux-androideabi" "i686-linux-android" "x86_64-linux-android")

for target in "${TARGETS[@]}"; do
    if ! rustup target list | grep -q "$target (installed)"; then
        echo "📥 Установка $target..."
        rustup target add $target
    fi
done

# Переход в директорию проекта
cd "$(dirname "$0")/neuro-tamagotchi-mobile"

# Установка зависимостей
echo "📦 Установка npm зависимостей..."
npm install

# Инициализация Android (если еще не сделано)
if [ ! -d "src-tauri/gen/android" ]; then
    echo "🔧 Инициализация Android проекта..."
    npm run tauri android init
fi

# Сборка
echo "🔨 Сборка APK..."
if [ "$BUILD_TYPE" = "release" ]; then
    npm run tauri android build -- --release
else
    npm run tauri android build
fi

# Поиск APK
APK_DIR="src-tauri/gen/android/app/build/outputs/apk"
if [ -d "$APK_DIR" ]; then
    echo ""
    echo "✅ Сборка завершена!"
    echo "📱 APK файлы:"
    find "$APK_DIR" -name "*.apk" -exec ls -lh {} \;
    echo ""
    echo "Установите APK на устройство:"
    echo "  adb install <путь-к-apk>"
else
    echo "❌ APK не найден в $APK_DIR"
    exit 1
fi
