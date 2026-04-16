#!/bin/bash

# Тест интеграции Ollama с Claw

echo "=== Тест 1: Проверка Ollama API ==="
curl -s http://localhost:11434/api/tags | head -n 5

echo -e "\n=== Тест 2: Простой промпт ==="
cd rust
./target/release/claw --model qwen2.5-coder:7b "What is 2+2? Answer in one sentence."

echo -e "\n=== Тест 3: Список моделей ==="
./target/release/claw --model llama3.2 "List 3 programming languages"

echo -e "\n=== Тест 4: Автоопределение провайдера ==="
# Эти модели должны автоматически определиться как Ollama
./target/release/claw --model qwen2.5-coder:7b "Hi"
./target/release/claw --model llama3.2 "Hi"

echo -e "\n=== Все тесты завершены ==="
