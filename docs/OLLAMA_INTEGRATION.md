# Интеграция Ollama с Claw

## Обзор

Claw теперь поддерживает локальные LLM через Ollama. Ollama использует OpenAI-совместимый API, что позволяет легко интегрировать локальные модели.

## Установка Ollama

```bash
# Linux/macOS
curl -fsSL https://ollama.com/install.sh | sh

# Windows
# Скачайте установщик с https://ollama.com/download
```

## Запуск моделей

```bash
# Запустить модель Qwen 3.5 (4B параметров)
ollama run vaultbox/qwen3.5-uncensored:4b

# Другие популярные модели
ollama run llama3.2
ollama run qwen2.5
## Ссылки

- [Ollama официальный сайт](https://ollama.com)
- [Ollama GitHub](https://github.com/ollama/ollama)
- [Библиотека моделей Ollama](https://ollama.com/library)
- [OpenAI API документация](https://platform.openai.com/docs/api-reference)
ude/GPT | Grok |
|---------|--------|------------|------|
| Стоимость | Бесплатно | Платно | Платно |
| Приватность | 100% локально | Облако | Облако |
| Скорость | Зависит от железа | Быстро | Быстро |
| Качество | Хорошее | Отличное | Отличное |
| Офлайн | Да | Нет | Нет |

## Дальнейшие шаги

- Экспериментируйте с разными моделями
- Настройте параметры для вашего железа
- Используйте маленькие модели для быстрых задач
- Используйте большие модели для сложных задач
- Комбинируйте локальные и облачные модели
 логи
RUST_LOG=debug cargo run --release -- --model llama3.2
```

### Частые проблемы

1. **Ollama не запущен**
   ```bash
   # Запустить Ollama сервис
   ollama serve
   ```

2. **Модель не найдена**
   ```bash
   # Скачать модель
   ollama pull llama3.2
   ```

3. **Порт занят**
   ```bash
   # Изменить порт Ollama
   OLLAMA_HOST=0.0.0.0:11435 ollama serve
   
   # Обновить base URL в Claw
   export OLLAMA_BASE_URL=http://localhost:11435/v1
   ```

## Сравнение с облачными провайдерами

| Функция | Ollama | Cla 13B - 34B     | 16GB+ | Средне | Отличное | Сложные задачи |
| 70B+          | 32GB+ | Медленно | Превосходное | Продакшн |

### Оптимизация

```bash
# Использовать GPU (если доступно)
ollama run --gpu llama3.2

# Ограничить контекст для скорости
cargo run --release -- --model qwen2.5 --max-tokens 2048
```

## Отладка

### Проверка статуса Ollama

```bash
# Проверить запущен ли Ollama
curl http://localhost:11434/api/tags

# Список установленных моделей
ollama list
```

### Логи

```bash
# Включить подробныельная аутентификация**: API ключ не требуется локально
- **Стандартный endpoint**: `http://localhost:11434/v1` по умолчанию
- **Полная совместимость**: Поддержка streaming, tool calling, и всех функций

## Производительность

### Рекомендации по размеру модели

| Размер модели | RAM | Скорость | Качество | Рекомендация |
|---------------|-----|----------|----------|--------------|
| 0.5B - 1B     | 2GB | Очень быстро | Базовое | Быстрые задачи |
| 3B - 7B       | 8GB | Быстро | Хорошее | Общее использование |
|ama)
```

### Как это работает

1. **Определение провайдера**: `detect_provider_kind()` в `providers/mod.rs` определяет Ollama по имени модели
2. **Конфигурация**: `OpenAiCompatConfig::ollama()` предоставляет настройки по умолчанию
3. **API клиент**: `OpenAiCompatClient` обрабатывает запросы к Ollama API
4. **Без аутентификации**: Ollama не требует API ключ для локального использования

### Ключевые особенности

- **Автоматическое определение**: Модели с `:` или начинающиеся с известных префиксов
- **Опционаmodel qwen2.5
```

### С конкретным промптом

```bash
cargo run --release -- --model llama3.2 --prompt "Explain Rust ownership"
```

### JSON режим

```bash
cargo run --release -- --model codellama --format json
```

## Архитектура интеграции

### Структура кода

```
rust/crates/api/src/
├── client.rs                    # ProviderClient с Ollama вариантом
├── providers/
│   ├── mod.rs                   # Регистрация Ollama моделей
│   └── openai_compat.rs         # OpenAI-совместимый клиент (используется Ollя
- `qwen2.5-coder` - оптимизирована для кода

### Общие модели
- `llama3.2` - универсальная модель Meta
- `qwen2.5` - мощная модель от Alibaba
- `mistral` - эффективная модель от Mistral AI
- `vaultbox/qwen3.5-uncensored:4b` - без цензуры, 4B параметров

### Маленькие модели (для быстрой работы)
- `llama3.2:1b` - 1 миллиард параметров
- `qwen2.5:0.5b` - 500 миллионов параметров
- `phi3:mini` - компактная модель от Microsoft

## Примеры использования

### Базовый чат

```bash
cd rust
cargo run --release -- --# Вариант 2: Явная конфигурация

Установите переменные окружения:

```bash
# Базовый URL Ollama (по умолчанию http://localhost:11434/v1)
export OLLAMA_BASE_URL=http://localhost:11434/v1

# API ключ не требуется для локального Ollama
# Но можно установить для удаленных инстансов
export OLLAMA_API_KEY=your-key-here
```

## Поддерживаемые модели

Claw поддерживает любые модели, доступные через Ollama:

### Модели для кода
- `codellama` - специализирована на коде
- `deepseek-coder` - отличная для программировани
ollama run mistral
ollama run codellama
```

## Использование с Claw

### Вариант 1: Автоматическое определение

Claw автоматически определяет Ollama модели по следующим признакам:
- Имя содержит двоеточие (формат `model:tag`)
- Начинается с `llama`, `qwen`, `mistral`, `codellama`

```bash
# Запустить Claw с Ollama моделью
cd rust
cargo run --release -- --model vaultbox/qwen3.5-uncensored:4b

# Или с другими моделями
cargo run --release -- --model llama3.2
cargo run --release -- --model qwen2.5:latest
```

##