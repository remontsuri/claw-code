# Улучшения интеллекта Kiro AI

## Цель
Расширить возможности AI ассистента для более эффективной работы с интернетом, базами данных и внешними API.

## Новые возможности

### 1. Расширенный веб-доступ

#### WebBrowse - Интерактивный браузер
```rust
ToolSpec {
    name: "WebBrowse",
    description: "Открыть веб-страницу, выполнить JavaScript, взаимодействовать с элементами",
    input_schema: json!({
        "type": "object",
        "properties": {
            "url": { "type": "string", "format": "uri" },
            "action": { 
                "type": "string", 
                "enum": ["navigate", "click", "fill", "screenshot", "execute_js"]
            },
            "selector": { "type": "string" },
            "value": { "type": "string" },
            "javascript": { "type": "string" },
            "wait_for": { "type": "string" }
        },
        "required": ["url", "action"]
    }),
    required_permission: PermissionMode::ReadOnly,
}
```

#### WebAPI - REST API клиент
```rust
ToolSpec {
    name: "WebAPI",
    description: "Выполнить HTTP запрос к любому API (GET, POST, PUT, DELETE)",
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
                    "password": { "type": "string" }
                }
            },
            "timeout": { "type": "integer", "minimum": 1 }
        },
        "required": ["url", "method"]
    }),
    required_permission: PermissionMode::ReadOnly,
}
```

### 2. Работа с базами данных

#### DatabaseQuery - SQL запросы
```rust
ToolSpec {
    name: "DatabaseQuery",
    description: "Выполнить SQL запрос к базе данных (PostgreSQL, MySQL, SQLite)",
    input_schema: json!({
        "type": "object",
        "properties": {
            "connection_string": { "type": "string" },
            "query": { "type": "string" },
            "params": { 
                "type": "array",
                "items": { "type": ["string", "number", "boolean", "null"] }
            },
            "database_type": { 
                "type": "string", 
                "enum": ["postgresql", "mysql", "sqlite", "mssql"]
            }
        },
        "required": ["connection_string", "query", "database_type"]
    }),
    required_permission: PermissionMode::DangerFullAccess,
}
```

### 3. Работа с AI моделями

#### AIInference - Вызов других AI моделей
```rust
ToolSpec {
    name: "AIInference",
    description: "Вызвать другую AI модель для специализированных задач",
    input_schema: json!({
        "type": "object",
        "properties": {
            "model": { 
                "type": "string",
                "enum": ["gpt-4", "claude-3", "llama3", "mistral", "custom"]
            },
            "prompt": { "type": "string" },
            "system_prompt": { "type": "string" },
            "temperature": { "type": "number", "minimum": 0, "maximum": 2 },
            "max_tokens": { "type": "integer", "minimum": 1 },
            "custom_endpoint": { "type": "string" }
        },
        "required": ["model", "prompt"]
    }),
    required_permission: PermissionMode::ReadOnly,
}
```

### 4. Работа с файлами и данными

#### DataTransform - Преобразование данных
```rust
ToolSpec {
    name: "DataTransform",
    description: "Преобразовать данные между форматами (JSON, XML, CSV, YAML, TOML)",
    input_schema: json!({
        "type": "object",
        "properties": {
            "input_data": { "type": "string" },
            "input_format": { 
                "type": "string", 
                "enum": ["json", "xml", "csv", "yaml", "toml", "ini"]
            },
            "output_format": { 
                "type": "string", 
                "enum": ["json", "xml", "csv", "yaml", "toml", "ini"]
            },
            "transform_rules": { "type": "object" }
        },
        "required": ["input_data", "input_format", "output_format"]
    }),
    required_permission: PermissionMode::ReadOnly,
}
```

#### FileAnalyze - Анализ файлов
```rust
ToolSpec {
    name: "FileAnalyze",
    description: "Анализировать файлы: размер, тип, метаданные, содержимое",
    input_schema: json!({
        "type": "object",
        "properties": {
            "path": { "type": "string" },
            "analysis_type": { 
                "type": "string", 
                "enum": ["metadata", "content", "structure", "dependencies", "security"]
            },
            "deep_scan": { "type": "boolean" }
        },
        "required": ["path", "analysis_type"]
    }),
    required_permission: PermissionMode::ReadOnly,
}
```

### 5. Работа с Git и версионным контролем

#### GitAdvanced - Расширенные Git операции
```rust
ToolSpec {
    name: "GitAdvanced",
    description: "Выполнить сложные Git операции: diff, blame, log, cherry-pick",
    input_schema: json!({
        "type": "object",
        "properties": {
            "operation": { 
                "type": "string", 
                "enum": ["diff", "blame", "log", "show", "cherry-pick", "rebase", "stash"]
            },
            "path": { "type": "string" },
            "commit": { "type": "string" },
            "branch": { "type": "string" },
            "options": { 
                "type": "array",
                "items": { "type": "string" }
            }
        },
        "required": ["operation"]
    }),
    required_permission: PermissionMode::WorkspaceWrite,
}
```

### 6. Мониторинг и логирование

#### SystemMonitor - Мониторинг системы
```rust
ToolSpec {
    name: "SystemMonitor",
    description: "Получить информацию о системе: CPU, память, диск, сеть, процессы",
    input_schema: json!({
        "type": "object",
        "properties": {
            "metric": { 
                "type": "string", 
                "enum": ["cpu", "memory", "disk", "network", "processes", "all"]
            },
            "interval": { "type": "integer", "minimum": 1 },
            "duration": { "type": "integer", "minimum": 1 }
        },
        "required": ["metric"]
    }),
    required_permission: PermissionMode::ReadOnly,
}
```

### 7. Работа с облачными сервисами

#### CloudStorage - Облачное хранилище
```rust
ToolSpec {
    name: "CloudStorage",
    description: "Работа с облачными хранилищами (S3, Azure Blob, Google Cloud Storage)",
    input_schema: json!({
        "type": "object",
        "properties": {
            "provider": { 
                "type": "string", 
                "enum": ["s3", "azure", "gcs", "dropbox", "onedrive"]
            },
            "operation": { 
                "type": "string", 
                "enum": ["upload", "download", "list", "delete", "copy"]
            },
            "bucket": { "type": "string" },
            "key": { "type": "string" },
            "local_path": { "type": "string" },
            "credentials": { "type": "object" }
        },
        "required": ["provider", "operation"]
    }),
    required_permission: PermissionMode::DangerFullAccess,
}
```

### 8. Работа с Docker и контейнерами

#### DockerManage - Управление Docker
```rust
ToolSpec {
    name: "DockerManage",
    description: "Управление Docker контейнерами и образами",
    input_schema: json!({
        "type": "object",
        "properties": {
            "operation": { 
                "type": "string", 
                "enum": ["list", "start", "stop", "build", "pull", "push", "logs", "exec"]
            },
            "container": { "type": "string" },
            "image": { "type": "string" },
            "command": { "type": "string" },
            "dockerfile_path": { "type": "string" },
            "options": { "type": "object" }
        },
        "required": ["operation"]
    }),
    required_permission: PermissionMode::DangerFullAccess,
}
```

### 9. Криптография и безопасность

#### CryptoTools - Криптографические операции
```rust
ToolSpec {
    name: "CryptoTools",
    description: "Шифрование, хеширование, генерация ключей",
    input_schema: json!({
        "type": "object",
        "properties": {
            "operation": { 
                "type": "string", 
                "enum": ["encrypt", "decrypt", "hash", "sign", "verify", "generate_key"]
            },
            "algorithm": { 
                "type": "string", 
                "enum": ["aes", "rsa", "sha256", "sha512", "md5", "bcrypt"]
            },
            "data": { "type": "string" },
            "key": { "type": "string" },
            "salt": { "type": "string" }
        },
        "required": ["operation", "algorithm"]
    }),
    required_permission: PermissionMode::ReadOnly,
}
```

### 10. Работа с изображениями и медиа

#### ImageProcess - Обработка изображений
```rust
ToolSpec {
    name: "ImageProcess",
    description: "Обработка изображений: изменение размера, конвертация, анализ",
    input_schema: json!({
        "type": "object",
        "properties": {
            "input_path": { "type": "string" },
            "output_path": { "type": "string" },
            "operation": { 
                "type": "string", 
                "enum": ["resize", "crop", "rotate", "convert", "analyze", "ocr"]
            },
            "width": { "type": "integer" },
            "height": { "type": "integer" },
            "format": { "type": "string" },
            "quality": { "type": "integer", "minimum": 1, "maximum": 100 }
        },
        "required": ["input_path", "operation"]
    }),
    required_permission: PermissionMode::WorkspaceWrite,
}
```

## Реализация

### Шаг 1: Добавить новые инструменты в `rust/crates/tools/src/lib.rs`

Добавить все новые `ToolSpec` в функцию `mvp_tool_specs()`.

### Шаг 2: Реализовать функции выполнения

Создать функции `execute_*` для каждого нового инструмента:
- `execute_web_browse()`
- `execute_web_api()`
- `execute_database_query()`
- `execute_ai_inference()`
- `execute_data_transform()`
- `execute_file_analyze()`
- `execute_git_advanced()`
- `execute_system_monitor()`
- `execute_cloud_storage()`
- `execute_docker_manage()`
- `execute_crypto_tools()`
- `execute_image_process()`

### Шаг 3: Добавить зависимости в `Cargo.toml`

```toml
[dependencies]
# Веб и HTTP
reqwest = { version = "0.11", features = ["blocking", "json"] }
headless_chrome = "1.0"  # для WebBrowse

# Базы данных
sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "postgres", "mysql", "sqlite"] }

# Криптография
ring = "0.17"
sha2 = "0.10"
bcrypt = "0.15"

# Обработка данных
serde_yaml = "0.9"
toml = "0.8"
csv = "1.3"
quick-xml = "0.31"

# Обработка изображений
image = "0.24"
tesseract = "0.14"  # для OCR

# Docker
bollard = "0.16"  # Docker API

# Системный мониторинг
sysinfo = "0.30"

# AWS/Cloud
aws-sdk-s3 = "1.0"
azure_storage = "0.19"
```

### Шаг 4: Обновить Ollama интеграцию

Добавить поддержку специализированных моделей для разных задач:

```rust
pub fn select_best_model_for_task(task_type: &str) -> &str {
    match task_type {
        "code" => "qwen2.5-coder:7b",
        "thinking" => "vaultbox/qwen3.5-uncensored:4b",
        "fast" => "llama3.2",
        "vision" => "llava",
        "embedding" => "nomic-embed-text",
        _ => "llama3.2"
    }
}
```

## Преимущества

1. **Полный доступ к интернету** - можно искать информацию, вызывать API, скачивать данные
2. **Работа с базами данных** - прямые SQL запросы без промежуточных инструментов
3. **Интеграция с AI** - можно вызывать специализированные модели для разных задач
4. **Облачные сервисы** - работа с S3, Azure, GCS
5. **DevOps инструменты** - Docker, Git, мониторинг
6. **Безопасность** - криптография и шифрование
7. **Обработка медиа** - изображения, OCR, конвертация

## Безопасность

Все опасные операции требуют `PermissionMode::DangerFullAccess`:
- Выполнение SQL запросов
- Работа с облачными хранилищами
- Управление Docker контейнерами
- Выполнение системных команд

Безопасные операции (чтение, анализ) требуют только `PermissionMode::ReadOnly`.

## Использование

После реализации, AI сможет:

```bash
# Поиск в интернете и анализ
claw "найди последнюю версию React и создай новый проект"

# Работа с API
claw "получи данные из API https://api.github.com/users/octocat"

# Работа с базой данных
claw "подключись к PostgreSQL и покажи все таблицы"

# Обработка изображений
claw "измени размер всех PNG файлов в папке images до 800x600"

# Docker
claw "запусти PostgreSQL в Docker контейнере"

# Мониторинг
claw "покажи использование CPU и памяти"
```

## Следующие шаги

1. Реализовать базовые инструменты (WebAPI, DataTransform)
2. Добавить поддержку баз данных
3. Интегрировать с облачными сервисами
4. Добавить Docker управление
5. Реализовать обработку изображений
6. Тестирование и оптимизация
