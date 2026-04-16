# Тест интеграции Ollama

## Команды для тестирования

### 1. Проверить что Ollama работает
```bash
curl http://localhost:11434/api/tags
```

### 2. Запустить Claw с Ollama
```bash
cd rust

# Тест 1: Простой промпт
./target/release/claw --model vaultbox/qwen3.5-uncensored:4b "What is Rust?"

# Тест 2: Интерактивный режим
./target/release/claw --model qwen2.5

# Тест 3: JSON формат
./target/release/claw --model llama3.2 --output-format json "Explain async/await"

# Тест 4: С ограничением инструментов
./target/release/claw --model qwen2.5 --allowedTools read,glob "Summarize this project"
```

### 3. Проверить автоопределение провайдера

Модели которые должны автоматически определиться как Ollama:
- ✅ `vaultbox/qwen3.5-uncensored:4b` (содержит `:`)
- ✅ `qwen2.5` (начинается с `qwen`)
- ✅ `llama3.2` (начинается с `llama`)
- ✅ `mistral` (начинается с `mistral`)
- ✅ `codellama` (начинается с `codellama`)

### 4. Тест с переменными окружения

```bash
# Установить кастомный URL
export OLLAMA_BASE_URL=http://localhost:11434/v1

# Запустить
./target/release/claw --model qwen2.5 "Hello"
```

## Ожидаемое поведение

1. Claw должен подключиться к `http://localhost:11434/v1`
2. Не должно быть ошибок аутентификации (Ollama не требует API ключ)
3. Ответы должны приходить от локальной модели
4. Streaming должен работать в интерактивном режиме

## Проверка логов

Если нужна отладка:
```bash
RUST_LOG=debug ./target/release/claw --model qwen2.5 "test"
```

## Известные ограничения

1. Ollama может не поддерживать все функции Claude API (например, некоторые tool calling возможности)
2. Качество ответов зависит от размера модели
3. Скорость зависит от вашего железа

## Следующие шаги после успешного теста

1. Добавить unit тесты для Ollama провайдера
2. Добавить интеграционные тесты
3. Документировать особенности работы с разными моделями
4. Оптимизировать параметры по умолчанию для Ollama
