# Changelog - Ollama Integration

## [Unreleased] - 2026-04-02

### Added

#### Ollama Provider Support
- Полная интеграция локальных LLM через Ollama
- Автоматическое определение Ollama моделей
- Поддержка OpenAI-совместимого API Ollama
- Работа без API ключа для локальных инстансов

#### Файлы изменены

**rust/crates/api/src/providers/mod.rs**
- Добавлен `ProviderKind::Ollama` enum вариант
- Добавлен `OLLAMA_METADATA` с конфигурацией
- Добавлены Ollama модели в `MODEL_REGISTRY`:
  - `qwen3.5-uncensored:4b`
  - `qwen2.5`
  - `llama3.2`
  - `mistral`
  - `codellama`
- Улучшена функция `metadata_for_model()` для автоопределения:
  - Модели с `:` (формат `model:tag`)
  - Модели начинающиеся с `llama`, `qwen`, `mistral`, `codellama`
- Обновлена функция `resolve_model_alias()` для обработки Ollama

**rust/crates/api/src/providers/openai_compat.rs**
- Добавлена константа `DEFAULT_OLLAMA_BASE_URL = "http://localhost:11434/v1"`
- Добавлена константа `OLLAMA_ENV_VARS`
- Добавлен метод `OpenAiCompatConfig::ollama()`
- Обновлен `from_env()` для работы без API ключа (Ollama не требует)
- Добавлена обработка Ollama в `credential_env_vars()`

**rust/crates/api/src/client.rs**
- Добавлен вариант `ProviderClient::Ollama(OpenAiCompatClient)`
- Обновлен `from_model_with_default_auth()` для создания Ollama клиента
- Обновлен `provider_kind()` для возврата `ProviderKind::Ollama`
- Обновлены `send_message()` и `stream_message()` для обработки Ollama
- Добавлена функция `read_ollama_base_url()`

#### Документация

**Новые файлы:**
- `OLLAMA_QUICKSTART.md` - Быстрый старт с Ollama
- `TEST_OLLAMA.md` - Инструкции по тестированию
- `test_ollama.sh` - Скрипт автоматического тестирования
- `STRUCTURE_IMPROVEMENTS.md` - План улучшения архитектуры проекта
- `CHANGELOG_OLLAMA.md` - Этот файл

**Обновленные файлы:**
- `README.md` - Добавлена секция Ollama Integration

### Technical Details

#### Автоопределение провайдера

Логика в `detect_provider_kind()`:
1. Проверка в `MODEL_REGISTRY`
2. Проверка переменных окружения (`OPENAI_API_KEY`, `XAI_API_KEY`)
3. Fallback на паттерны:
   - Ollama: содержит `:` или начинается с известных префиксов
   - OpenAI: начинается с `gpt-` или `o`
   - Claude: по умолчанию

#### API Endpoint

Ollama использует OpenAI-совместимый endpoint:
```
http://localhost:11434/v1/chat/completions
```

#### Аутентификация

- Локальный Ollama: API ключ не требуется (используется placeholder "ollama")
- Удаленный Ollama: можно установить `OLLAMA_API_KEY`

#### Конфигурация

Переменные окружения:
- `OLLAMA_BASE_URL` - базовый URL (по умолчанию `http://localhost:11434/v1`)
- `OLLAMA_API_KEY` - API ключ (опционально)

### Usage Examples

```bash
# Автоопределение по имени модели
claw --model qwen2.5-coder:7b "Hello"
claw --model llama3.2 "Explain Rust"

# С явной конфигурацией
export OLLAMA_BASE_URL=http://localhost:11434/v1
claw --model mistral "Write a function"

# Интерактивный режим
claw --model codellama
```

### Supported Models

Любые модели из Ollama библиотеки:
- Code models: `codellama`, `deepseek-coder`, `qwen2.5-coder`
- General: `llama3.2`, `qwen2.5`, `mistral`, `gemma3`
- Small: `llama3.2:1b`, `qwen2.5:0.5b`, `phi3:mini`
- Custom: любые модели с форматом `model:tag`

### Performance

Зависит от:
- Размера модели (0.5B - 70B+ параметров)
- Доступной RAM
- GPU (если используется)
- Квантизация (Q4_K_M, Q5_K_M, Q8_0, etc.)

### Compatibility

- ✅ Streaming responses
- ✅ Non-streaming responses
- ✅ System prompts
- ✅ Multi-turn conversations
- ⚠️ Tool calling (зависит от модели)
- ⚠️ Function calling (зависит от модели)

### Testing

```bash
# Сборка
cd rust
cargo build --release

# Тест
./target/release/claw --model qwen2.5 "test"

# Автоматические тесты
bash test_ollama.sh
```

### Known Limitations

1. Не все Ollama модели поддерживают tool calling
2. Качество зависит от размера модели
3. Скорость зависит от железа
4. Некоторые продвинутые функции Claude API могут не работать

### Future Improvements

- [ ] Добавить unit тесты для Ollama провайдера
- [ ] Добавить интеграционные тесты
- [ ] Оптимизировать параметры по умолчанию
- [ ] Добавить автоматический выбор модели по задаче
- [ ] Документировать особенности разных моделей
- [ ] Добавить поддержку Ollama embeddings
- [ ] Добавить мониторинг производительности

### Breaking Changes

Нет breaking changes. Все существующие провайдеры работают как раньше.

### Migration Guide

Не требуется миграция. Ollama работает out-of-the-box:

```bash
# Старый способ (Claude/GPT)
claw --model opus "task"

# Новый способ (Ollama)
claw --model llama3.2 "task"
```

### Contributors

- Initial Ollama integration
- Architecture improvements planning
- Documentation

### References

- [Ollama](https://ollama.com)
- [Ollama API](https://github.com/ollama/ollama/blob/main/docs/api.md)
- [OpenAI API](https://platform.openai.com/docs/api-reference)
- [Claw Code](https://github.com/instructkr/claw-code)
