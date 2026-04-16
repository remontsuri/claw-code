# Быстрый старт с Ollama

## Что сделано

Добавлена полная поддержка локальных LLM через Ollama в Claw Code.

## Как использовать

### 1. Убедитесь что Ollama запущен

```bash
# Проверить статус
curl http://localhost:11434/api/tags

# Если не запущен, запустить
ollama serve
```

### 2. Запустить Claw с Ollama моделью

```bash
cd rust

# С моделью Qwen 3.5 (которую вы уже запустили)
cargo run --release -- --model vaultbox/qwen3.5-uncensored:4b

# Или с другими моделями
cargo run --release -- --model llama3.2
cargo run --release -- --model qwen2.5
cargo run --release -- --model mistral
```

### 3. Начать работу

Claw автоматически определит что это Ollama модель и подключится к `http://localhost:11434/v1`

## Примеры команд

```bash
# Базовый чат
./target/release/claw --model qwen2.5

# С промптом
./target/release/claw --model llama3.2 --prompt "Explain Rust"

# JSON режим
./target/release/claw --model codellama --format json
```

## Что изменилось в коде

### Новые файлы
- `STRUCTURE_IMPROVEMENTS.md` - план улучшения архитектуры
- `OLLAMA_QUICKSTART.md` - этот файл

### Измененные файлы

1. **rust/crates/api/src/providers/mod.rs**
   - Добавлен `ProviderKind::Ollama`
   - Добавлен `OLLAMA_METADATA`
   - Автоопределение Ollama моделей по паттернам

2. **rust/crates/api/src/providers/openai_compat.rs**
   - Добавлен `DEFAULT_OLLAMA_BASE_URL`
   - Добавлен `OpenAiCompatConfig::ollama()`
   - Ollama не требует API ключ

3. **rust/crates/api/src/client.rs**
   - Добавлен вариант `ProviderClient::Ollama`
   - Обработка Ollama в send/stream методах
   - Добавлена функция `read_ollama_base_url()`

## Поддерживаемые модели

Любые модели из Ollama библиотеки:
- `llama3.2`, `llama3.2:1b`, `llama3.2:3b`
- `qwen2.5`, `qwen2.5:0.5b`, `qwen2.5:7b`
- `mistral`, `mistral:7b`
- `codellama`, `codellama:7b`, `codellama:13b`
- `deepseek-coder`
- `phi3:mini`
- И любые другие с форматом `model:tag`

## Автоопределение

Claw автоматически определяет Ollama если:
- Имя модели содержит `:` (например `qwen2.5:7b`)
- Начинается с `llama`, `qwen`, `mistral`, `codellama`

## Конфигурация (опционально)

```bash
# Изменить базовый URL (если Ollama на другом порту/хосте)
export OLLAMA_BASE_URL=http://localhost:11435/v1

# API ключ (только для удаленных инстансов)
export OLLAMA_API_KEY=your-key
```

## Тестирование

```bash
# Собрать проект
cd rust
cargo build --release

# Запустить с вашей моделью
./target/release/claw --model vaultbox/qwen3.5-uncensored:4b

# Проверить что модель определилась как Ollama
# В логах должно быть: "Using provider: Ollama"
```

## Производительность

| Модель | Размер | RAM | Скорость | Качество |
|--------|--------|-----|----------|----------|
| qwen2.5:0.5b | 500MB | 2GB | ⚡⚡⚡ | ⭐⭐ |
| llama3.2:1b | 1GB | 2GB | ⚡⚡⚡ | ⭐⭐⭐ |
| qwen3.5:4b | 4GB | 8GB | ⚡⚡ | ⭐⭐⭐⭐ |
| llama3.2:3b | 3GB | 8GB | ⚡⚡ | ⭐⭐⭐⭐ |
| mistral:7b | 7GB | 16GB | ⚡ | ⭐⭐⭐⭐⭐ |
| codellama:13b | 13GB | 32GB | ⚡ | ⭐⭐⭐⭐⭐ |

## Отладка

### Проблема: "Connection refused"
```bash
# Запустить Ollama
ollama serve
```

### Проблема: "Model not found"
```bash
# Скачать модель
ollama pull qwen2.5
```

### Проблема: Медленная работа
```bash
# Использовать меньшую модель
cargo run --release -- --model qwen2.5:0.5b

# Или ограничить токены
cargo run --release -- --model qwen2.5 --max-tokens 2048
```

## Следующие шаги

1. ✅ Ollama интегрирован
2. ⏳ Добавить тесты для Ollama провайдера
3. ⏳ Добавить примеры в документацию
4. ⏳ Оптимизировать параметры для разных размеров моделей
5. ⏳ Добавить автоматический выбор модели по задаче

## Ссылки

- [Ollama](https://ollama.com)
- [Модели Ollama](https://ollama.com/library)
- [OpenAI API](https://platform.openai.com/docs/api-reference)
