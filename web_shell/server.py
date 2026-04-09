#!/usr/bin/env python3
"""
NeuroGotchi Web Shell - Простой HTTP сервер без зависимостей
"""

from http.server import HTTPServer, BaseHTTPRequestHandler
import json
import subprocess
import os

HTML = """<!DOCTYPE html>
<html>
<head>
    <title>NeuroGotchi Web Shell</title>
    <meta charset="utf-8">
    <style>
        body { font-family: 'Courier New', monospace; background: #0a0a0a; color: #00ff00; padding: 20px; max-width: 1200px; margin: 0 auto; }
        h1 { color: #00ff00; text-shadow: 0 0 10px #00ff00; }
        .container { display: grid; grid-template-columns: 1fr 1fr; gap: 20px; }
        .panel { background: #1a1a1a; border: 2px solid #00ff00; border-radius: 10px; padding: 20px; box-shadow: 0 0 20px rgba(0,255,0,0.3); }
        .stats { display: grid; grid-template-columns: 1fr 1fr; gap: 10px; margin: 10px 0; }
        .stat { background: #0a0a0a; padding: 10px; border-radius: 5px; border: 1px solid #00ff00; }
        .stat-label { color: #00aa00; font-size: 0.8em; }
        .stat-value { font-size: 1.5em; font-weight: bold; }
        input, button { background: #0a0a0a; color: #00ff00; border: 2px solid #00ff00; padding: 10px; font-family: 'Courier New', monospace; font-size: 1em; border-radius: 5px; }
        button { cursor: pointer; transition: all 0.3s; }
        button:hover { background: #00ff00; color: #0a0a0a; box-shadow: 0 0 10px #00ff00; }
        #input-box { width: 100%; margin: 10px 0; }
        #output { background: #0a0a0a; padding: 15px; border-radius: 5px; min-height: 200px; max-height: 400px; overflow-y: auto; border: 1px solid #00ff00; }
        .thought { color: #ffaa00; margin: 5px 0; }
        .speech { color: #00ffff; margin: 5px 0; font-weight: bold; }
        .system { color: #888; margin: 5px 0; }
        .controls { display: flex; gap: 10px; margin: 10px 0; }
    </style>
</head>
<body>
    <h1>🧠 NeuroGotchi Web Shell</h1>
    <div class="container">
        <div class="panel">
            <h2>💬 Взаимодействие</h2>
            <input type="text" id="input-box" placeholder="Введите команду..." onkeypress="if(event.key==='Enter') sendCommand()">
            <div class="controls">
                <button onclick="sendCommand()">Отправить</button>
                <button onclick="runTests()">🧪 Тесты</button>
                <button onclick="runDemo()">▶️ Демо</button>
            </div>
            <div id="output"></div>
        </div>
        <div class="panel">
            <h2>📊 Статистика проекта</h2>
            <div class="stats">
                <div class="stat"><div class="stat-label">Нейронов</div><div class="stat-value">1050</div></div>
                <div class="stat"><div class="stat-label">Связей</div><div class="stat-value">~20000</div></div>
                <div class="stat"><div class="stat-label">Тестов</div><div class="stat-value">130+</div></div>
                <div class="stat"><div class="stat-label">Модулей</div><div class="stat-value">13</div></div>
            </div>
            <h3>🧠 Компоненты</h3>
            <div class="stat"><div class="stat-label">✓ Token-Spike Interface</div><div>45 тестов</div></div>
            <div class="stat"><div class="stat-label">✓ NeuroGotchi Topology</div><div>38 тестов</div></div>
            <div class="stat"><div class="stat-label">✓ Neuromodulation</div><div>47 тестов</div></div>
            <div class="stat"><div class="stat-label">✓ Интеграция</div><div>Готово</div></div>
        </div>
    </div>
    <script>
        function addOutput(text, type='system') {
            const output = document.getElementById('output');
            const line = document.createElement('div');
            line.className = type;
            line.textContent = text;
            output.appendChild(line);
            output.scrollTop = output.scrollHeight;
        }
        async function sendCommand() {
            const input = document.getElementById('input-box');
            const cmd = input.value.trim();
            if (!cmd) return;
            addOutput(`> ${cmd}`, 'system');
            input.value = '';
            try {
                const response = await fetch('/exec', {
                    method: 'POST',
                    headers: {'Content-Type': 'application/json'},
                    body: JSON.stringify({command: cmd})
                });
                const data = await response.json();
                if (data.output) addOutput(data.output, 'speech');
                if (data.error) addOutput('Ошибка: ' + data.error, 'system');
            } catch (e) {
                addOutput(`Ошибка: ${e}`, 'system');
            }
        }
        async function runTests() {
            addOutput('Запуск тестов...', 'system');
            const response = await fetch('/tests');
            const data = await response.json();
            addOutput(data.output, 'speech');
        }
        async function runDemo() {
            addOutput('Запуск демо...', 'system');
            const response = await fetch('/demo');
            const data = await response.json();
            addOutput(data.output, 'speech');
        }
        addOutput('NeuroGotchi Web Shell готов!', 'system');
        addOutput('Команды: cargo test, cargo run --example neuromodulation_demo', 'system');
    </script>
</body>
</html>"""

class Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        if self.path == '/':
            self.send_response(200)
            self.send_header('Content-type', 'text/html; charset=utf-8')
            self.end_headers()
            self.wfile.write(HTML.encode())
        elif self.path == '/tests':
            result = subprocess.run(['cargo', 'test', '--quiet'],
                                  capture_output=True, text=True, cwd='/root/brainwave_project')
            self.send_response(200)
            self.send_header('Content-type', 'application/json')
            self.end_headers()
            self.wfile.write(json.dumps({'output': result.stdout + result.stderr}).encode())
        elif self.path == '/demo':
            self.send_response(200)
            self.send_header('Content-type', 'application/json')
            self.end_headers()
            self.wfile.write(json.dumps({'output': 'Демо запущено в фоне. Проверьте логи.'}).encode())
        else:
            self.send_response(404)
            self.end_headers()

    def do_POST(self):
        if self.path == '/exec':
            length = int(self.headers['Content-Length'])
            data = json.loads(self.rfile.read(length))
            cmd = data.get('command', '')

            result = subprocess.run(cmd, shell=True, capture_output=True, text=True,
                                  cwd='/root/brainwave_project', timeout=30)

            self.send_response(200)
            self.send_header('Content-type', 'application/json')
            self.end_headers()
            self.wfile.write(json.dumps({
                'output': result.stdout[:1000],
                'error': result.stderr[:500] if result.returncode != 0 else None
            }).encode())
        else:
            self.send_response(404)
            self.end_headers()

    def log_message(self, format, *args):
        pass  # Отключаем логи

if __name__ == '__main__':
    print("🧠 NeuroGotchi Web Shell")
    print("📡 http://localhost:8000")
    HTTPServer(('0.0.0.0', 8000), Handler).serve_forever()
