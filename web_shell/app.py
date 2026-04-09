#!/usr/bin/env python3
"""
NeuroGotchi Web Shell - Простой веб-интерфейс для взаимодействия с нейросетью
"""

from flask import Flask, render_template, request, jsonify
import subprocess
import json
import os

app = Flask(__name__)

# HTML шаблон
HTML_TEMPLATE = """
<!DOCTYPE html>
<html>
<head>
    <title>NeuroGotchi Web Shell</title>
    <meta charset="utf-8">
    <style>
        body {
            font-family: 'Courier New', monospace;
            background: #0a0a0a;
            color: #00ff00;
            padding: 20px;
            max-width: 1200px;
            margin: 0 auto;
        }
        h1 {
            color: #00ff00;
            text-shadow: 0 0 10px #00ff00;
        }
        .container {
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 20px;
        }
        .panel {
            background: #1a1a1a;
            border: 2px solid #00ff00;
            border-radius: 10px;
            padding: 20px;
            box-shadow: 0 0 20px rgba(0,255,0,0.3);
        }
        .stats {
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 10px;
            margin: 10px 0;
        }
        .stat {
            background: #0a0a0a;
            padding: 10px;
            border-radius: 5px;
            border: 1px solid #00ff00;
        }
        .stat-label {
            color: #00aa00;
            font-size: 0.8em;
        }
        .stat-value {
            font-size: 1.5em;
            font-weight: bold;
        }
        input, button {
            background: #0a0a0a;
            color: #00ff00;
            border: 2px solid #00ff00;
            padding: 10px;
            font-family: 'Courier New', monospace;
            font-size: 1em;
            border-radius: 5px;
        }
        button {
            cursor: pointer;
            transition: all 0.3s;
        }
        button:hover {
            background: #00ff00;
            color: #0a0a0a;
            box-shadow: 0 0 10px #00ff00;
        }
        #input-box {
            width: 100%;
            margin: 10px 0;
        }
        #output {
            background: #0a0a0a;
            padding: 15px;
            border-radius: 5px;
            min-height: 200px;
            max-height: 400px;
            overflow-y: auto;
            border: 1px solid #00ff00;
        }
        .thought {
            color: #ffaa00;
            margin: 5px 0;
        }
        .speech {
            color: #00ffff;
            margin: 5px 0;
            font-weight: bold;
        }
        .system {
            color: #888;
            margin: 5px 0;
        }
        .controls {
            display: flex;
            gap: 10px;
            margin: 10px 0;
        }
        .progress-bar {
            width: 100%;
            height: 20px;
            background: #0a0a0a;
            border: 1px solid #00ff00;
            border-radius: 5px;
            overflow: hidden;
        }
        .progress-fill {
            height: 100%;
            background: linear-gradient(90deg, #00ff00, #00aa00);
            transition: width 0.3s;
        }
    </style>
</head>
<body>
    <h1>🧠 NeuroGotchi Web Shell</h1>

    <div class="container">
        <div class="panel">
            <h2>💬 Взаимодействие</h2>
            <input type="text" id="input-box" placeholder="Введите слово..." onkeypress="if(event.key==='Enter') sendWord()">
            <div class="controls">
                <button onclick="sendWord()">Отправить</button>
                <button onclick="reward()">🎁 Награда</button>
                <button onclick="stress()">⚡ Стресс</button>
            </div>
            <div id="output"></div>
        </div>

        <div class="panel">
            <h2>📊 Статистика</h2>
            <div class="stats">
                <div class="stat">
                    <div class="stat-label">Нейронов</div>
                    <div class="stat-value" id="neurons">1050</div>
                </div>
                <div class="stat">
                    <div class="stat-label">Связей</div>
                    <div class="stat-value" id="connections">~20000</div>
                </div>
                <div class="stat">
                    <div class="stat-label">Активность</div>
                    <div class="stat-value" id="activity">0 Hz</div>
                </div>
                <div class="stat">
                    <div class="stat-label">Время</div>
                    <div class="stat-value" id="time">0.0 с</div>
                </div>
            </div>

            <h3>🧪 Нейромодуляция</h3>
            <div class="stat">
                <div class="stat-label">Усталость</div>
                <div class="progress-bar">
                    <div class="progress-fill" id="fatigue-bar" style="width: 0%"></div>
                </div>
                <div id="fatigue">0%</div>
            </div>

            <div class="stat">
                <div class="stat-label">Настроение</div>
                <div class="progress-bar">
                    <div class="progress-fill" id="mood-bar" style="width: 50%"></div>
                </div>
                <div id="mood">0.00</div>
            </div>

            <div class="stat">
                <div class="stat-label">Состояние</div>
                <div id="status">Awake</div>
            </div>
        </div>
    </div>

    <script>
        let sessionId = Date.now();

        function addOutput(text, type='system') {
            const output = document.getElementById('output');
            const line = document.createElement('div');
            line.className = type;
            line.textContent = text;
            output.appendChild(line);
            output.scrollTop = output.scrollHeight;
        }

        async function sendWord() {
            const input = document.getElementById('input-box');
            const word = input.value.trim();
            if (!word) return;

            addOutput(`> ${word}`, 'system');
            input.value = '';

            try {
                const response = await fetch('/process', {
                    method: 'POST',
                    headers: {'Content-Type': 'application/json'},
                    body: JSON.stringify({word: word, session: sessionId})
                });
                const data = await response.json();
                updateStats(data);

                if (data.thoughts) {
                    data.thoughts.forEach(t => addOutput(`💭 ${t}`, 'thought'));
                }
                if (data.speech) {
                    addOutput(`🗣️ ${data.speech}`, 'speech');
                }
            } catch (e) {
                addOutput(`Ошибка: ${e}`, 'system');
            }
        }

        async function reward() {
            addOutput('🎁 Награда!', 'system');
            await fetch('/reward', {method: 'POST', body: JSON.stringify({session: sessionId})});
        }

        async function stress() {
            addOutput('⚡ Стресс!', 'system');
            await fetch('/stress', {method: 'POST', body: JSON.stringify({session: sessionId})});
        }

        function updateStats(data) {
            if (data.activity) document.getElementById('activity').textContent = data.activity + ' Hz';
            if (data.time) document.getElementById('time').textContent = data.time + ' с';
            if (data.fatigue !== undefined) {
                document.getElementById('fatigue').textContent = Math.round(data.fatigue * 100) + '%';
                document.getElementById('fatigue-bar').style.width = (data.fatigue * 100) + '%';
            }
            if (data.mood !== undefined) {
                document.getElementById('mood').textContent = data.mood.toFixed(2);
                const moodPercent = ((data.mood + 1) / 2) * 100;
                document.getElementById('mood-bar').style.width = moodPercent + '%';
            }
            if (data.status) document.getElementById('status').textContent = data.status;
        }

        // Обновление статистики каждые 2 секунды
        setInterval(async () => {
            try {
                const response = await fetch('/stats?session=' + sessionId);
                const data = await response.json();
                updateStats(data);
            } catch (e) {}
        }, 2000);

        addOutput('NeuroGotchi готов к взаимодействию!', 'system');
    </script>
</body>
</html>
"""

@app.route('/')
def index():
    return HTML_TEMPLATE

@app.route('/process', methods=['POST'])
def process_word():
    """Обработать слово через нейросеть"""
    data = request.json
    word = data.get('word', '')

    # Здесь будет вызов Rust бинарника
    # Пока возвращаем mock данные
    return jsonify({
        'thoughts': [f'думаю о "{word}"'],
        'speech': word if len(word) > 3 else None,
        'activity': 45.5,
        'time': 1.5,
        'fatigue': 0.15,
        'mood': 0.2,
        'status': 'День | Awake'
    })

@app.route('/reward', methods=['POST'])
def reward():
    """Выброс дофамина"""
    return jsonify({'status': 'ok'})

@app.route('/stress', methods=['POST'])
def stress():
    """Выброс кортизола"""
    return jsonify({'status': 'ok'})

@app.route('/stats', methods=['GET'])
def stats():
    """Получить текущую статистику"""
    return jsonify({
        'activity': 42.3,
        'time': 2.5,
        'fatigue': 0.25,
        'mood': 0.15,
        'status': 'День | Awake'
    })

if __name__ == '__main__':
    print("🧠 NeuroGotchi Web Shell запущен!")
    print("📡 Откройте: http://localhost:5000")
    app.run(host='0.0.0.0', port=5000, debug=True)
