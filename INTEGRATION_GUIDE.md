# Руководство по интеграции новых возможностей

## Что было создано

Я создал три файла с новыми возможностями для AI ассистента:

1. **INTELLIGENCE_UPGRADES.md** - полный список из 12 новых инструментов
2. **rust/crates/tools/src/web_api.rs** - универсальный HTTP клиент для работы с любыми API
3. **rust/crates/tools/src/ai_inference.rs** - модуль для вызова других AI моделей

## Как интегрировать

### Шаг 1: Обновить `rust/crates/tools/src/lib.rs`

Добавить в начало файла:

```rust
mod web_api;
mod ai_inference;

pub use web_api::{execute_web_api, WebApiInput, WebApiOutput};
pub use ai_inference::{execute_ai_inference, AiInferenceInput, AiInferenceOutput};
```

### Шаг 2: Добавить новые инструменты в `mvp_tool_specs()`

В функцию `mvp_tool_specs()` добавить:

```rust
ToolSpec {
    name: "WebAPI",
    description: "Выполнить HTTP запрос к любому API (GET, POST, PUT, DELETE, PATCH)",
    input_schema: json!({
        "type": "object",
        "properties": {
            "url": { "type": "string", "format": "uri" },
            "method": { 
                "type": "string", 
                "enum": ["GET", "POST", "PUT", "DELETE", "PATCH"]
            },
            "headers": { 
                "type": "object",
                "additionalProperties": { "type": "string" }
            },
            "body": { "type": "string" },
            "auth": {
                "type": "object",
                "properties": {
                    "type": { "type": "string", "enum": ["bearer", "basic", "api_key"] },
                    "token": { "type": "string" },
                    "username": { "type": "string" },
                    "password": { "type": "string" },
                    "api_key_header": { "type": "string" }
                }
            },
            "timeout": { "type": "integer", "minimum": 1 }
        },
        "required": ["url", "method"]
    }),
    required_permission: PermissionMode::ReadOnly,
},
ToolSpec {
    name: "AIInference",
    description: "Вызвать другую AI модель для специализированных задач",
    input_schema: json!({
        "type": "object",
        "properties": {
            "model": { "type": "string" },
            "prompt": { "type": "string" },
            "system_prompt": { "type": "string" },
            "temperature": { "type": "number", "minimum": 0, "maximum": 2 },
            "max_tokens": { "type": "integer", "minimum": 1 },
            "custom_endpoint": { "type": "string" }
        },
        "required": ["model", "prompt"]
    }),
    required_permission: PermissionMode::ReadOnly,
},
```

### Шаг 3: Добавить обработчики в `execute_tool()`

В функцию `execute_tool()` добавить:

```rust
"WebAPI" => from_value::<WebApiInput>(input).and_then(|i| run_web_api(&i)),
"AIInference" => from_value::<AiInferenceInput>(input).and_then(|i| run_ai_inference(&i)),
```

И создать функции-обертки:

```rust
fn run_web_api(input: &WebApiInput) -> Result<String, String> {
    to_pretty_json(execute_web_api(input)?)
}

fn run_ai_inference(input: &AiInferenceInput) -> Result<String, String> {
    to_pretty_json(execute_ai_inference(input)?)
}
```

### Шаг 4: Обновить `Cargo.toml`

Добавить зависимости в `rust/crates/tools/Cargo.toml`:

```toml
[dependencies]
# Существующие зависимости...

# Новые зависимости для WebAPI
reqwest = { version = "0.11", features = ["blocking", "json"] }
base64 = "0.21"

# Для работы с JSON
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### Шаг 5: Собрать проект

```bash
cd rust
cargo build --release
```

## Примеры использования

### WebAPI - Работа с REST API

```bash
# Получить данные пользователя GitHub
./target/release/claw "используй WebAPI чтобы получить информацию о пользователе octocat с GitHub API"

# Отправить POST запрос
./target/release/claw "используй WebAPI чтобы отправить POST запрос на https://httpbin.org/post с JSON данными {\"test\": \"data\"}"

# Работа с аутентификацией
./target/release/claw "используй WebAPI с Bearer токеном чтобы получить мои репозитории с GitHub"
```

### AIInference - Вызов других моделей

```bash
# Использовать специализированную модель для кода
./target/release/claw "используй AIInference с моделью qwen2.5-coder:7b чтобы написать функцию сортировки на Rust"

# Использовать thinking модель для сложной задачи
./target/release/claw "используй AIInference с моделью vaultbox/qwen3.5-uncensored:4b чтобы решить математическую задачу"

# Вызвать GPT-4 (если есть API ключ)
./target/release/claw "используй AIInference с моделью gpt-4 чтобы написать эссе о AI"
```

## Преимущества новых возможностей

### 1. WebAPI
- ✅ Работа с любыми REST API
- ✅ Поддержка всех HTTP методов (GET, POST, PUT, DELETE, PATCH)
- ✅ Гибкая аутентификация (Bearer, Basic, API Key)
- ✅ Настраиваемые заголовки
- ✅ Автоматический парсинг JSON
- ✅ Таймауты и обработка ошибок

### 2. AIInference
- ✅ Вызов локальных Ollama моделей
- ✅ Интеграция с OpenAI API
- ✅ Интеграция с Anthropic/Claude API
- ✅ Поддержка кастомных endpoints
- ✅ Контроль температуры и max_tokens
- ✅ Отслеживание использования токенов

## Практические сценарии

### Сценарий 1: Получение данных из API и анализ

```bash
claw "используй WebAPI чтобы получить погоду для Москвы с api.openweathermap.org, затем проанализируй данные"
```

AI сможет:
1. Вызвать WebAPI для получения данных
2. Получить JSON ответ
3. Проанализировать погоду
4. Дать рекомендации

### Сценарий 2: Специализированная обработка кода

```bash
claw "используй AIInference с qwen2.5-coder:7b чтобы оптимизировать функцию в файле src/main.rs"
```

AI сможет:
1. Прочитать файл
2. Вызвать специализированную модель для кода
3. Получить оптимизированную версию
4. Применить изменения

### Сценарий 3: Работа с GitHub API

```bash
claw "используй WebAPI чтобы создать новый issue в моем репозитории"
```

AI сможет:
1. Использовать GitHub API
2. Аутентифицироваться с токеном
3. Создать issue
4. Подтвердить создание

## Безопасность

### WebAPI
- Требует `PermissionMode::ReadOnly` для GET запросов
- Требует `PermissionMode::WorkspaceWrite` для POST/PUT/DELETE
- Автоматическое обновление HTTP на HTTPS (кроме localhost)
- Ограничение редиректов (максимум 10)
- Настраиваемые таймауты

### AIInference
- Требует `PermissionMode::ReadOnly`
- API ключи берутся из переменных окружения
- Не сохраняет чувствительные данные
- Таймаут 120 секунд для длинных запросов

## Переменные окружения

Для работы с внешними API нужно установить:

```bash
# OpenAI
export OPENAI_API_KEY="sk-..."
export OPENAI_BASE_URL="https://api.openai.com/v1"  # опционально

# Anthropic/Claude
export ANTHROPIC_API_KEY="sk-ant-..."
export ANTHROPIC_BASE_URL="https://api.anthropic.com/v1"  # опционально

# Ollama (локально)
export OLLAMA_BASE_URL="http://localhost:11434"  # опционально

# Для WebSearch (опционально)
export CLAW_WEB_SEARCH_BASE_URL="https://your-search-api.com"
```

## Тестирование

### Тест WebAPI

```bash
cd rust/crates/tools
cargo test web_api::tests --release
```

### Тест AIInference

```bash
cd rust/crates/tools
cargo test ai_inference::tests --release
```

## Следующие шаги

После успешной интеграции WebAPI и AIInference, можно добавить:

1. **DatabaseQuery** - прямые SQL запросы
2. **DataTransform** - конвертация форматов данных
3. **SystemMonitor** - мониторинг системы
4. **DockerManage** - управление контейнерами
5. **CloudStorage** - работа с S3, Azure, GCS

Все спецификации есть в файле `INTELLIGENCE_UPGRADES.md`.

## Поддержка

Если возникнут проблемы:

1. Проверьте, что все зависимости установлены
2. Убедитесь, что переменные окружения установлены
3. Проверьте логи: `RUST_LOG=debug ./target/release/claw`
4. Запустите тесты: `cargo test --release`

## Заключение

Эти улучшения делают AI ассистента значительно "умнее":

- 🌐 Полный доступ к интернету через WebAPI
- 🤖 Возможность вызывать специализированные AI модели
- 🔧 Гибкая настройка и безопасность
- 📊 Отслеживание использования ресурсов
- 🚀 Высокая производительность

Теперь AI может самостоятельно работать с любыми API, получать данные из интернета и использовать специализированные модели для разных задач!
