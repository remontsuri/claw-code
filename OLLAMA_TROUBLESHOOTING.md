# Troubleshooting Ollama Integration

## Проблема

Claw пытается подключиться к `http://localhost:11434/v1/messages` (Claude API endpoint) вместо `http://localhost:11434/v1/chat/completions` (OpenAI-compatible endpoint).

## Диагностика

### 1. Ollama работает
```bash
curl http://127.0.0.1:11434/api/version
# ✅ Возвращает {"version":""}
```

### 2. Модели доступны
```bash
ollama list
# ✅ Показывает 10 моделей
```

### 3. Модель отвечает через ollama CLI
```bash
ollama run llama3.2 "hi"
# ✅ Работает
```

### 4. Claw падает с ошибкой
```bash
./target/release/claw --model llama3.2 "hi"
# ❌ error: api failed after 3 attempts: http error: error sending request for url (http://localhost:11434/v1/messages)
```

## Причина

Проблема в логике определения провайдера в `detect_provider_kind()`:

1. `metadata_for_model("llama3.2")` возвращает `Some(OLLAMA_METADATA)` ✅
2. НО если установлен `ANTHROPIC_API_KEY` или есть сохраненные credentials, функция возвращает `ProviderKind::ClawApi` ❌
3. Это происходит из-за fallback логики в конце функции

## Решение

### Вариант 1: Явно указать провайдера через env var
```bash
export OLLAMA_BASE_URL=http://127.0.0.1:11434/v1
unset ANTHROPIC_API_KEY  # Временно убрать Claude credentials
./target/release/claw --model llama3.2 "hi"
```

### Вариант 2: Исправить логику detect_provider_kind()

Проблема в этом коде:
```rust
pub fn detect_provider_kind(model: &str) -> ProviderKind {
    // First: check model name in registry
    if let Some(metadata) = metadata_for_model(model) {
        return metadata.provider;  // ✅ Возвращает Ollama
    }
    
    // Second: explicit env vars take priority
    if openai_compat::has_api_key("OPENAI_API_KEY") {
        return ProviderKind::OpenAi;
    }
    if openai_compat::has_api_key("XAI_API_KEY") {
        return ProviderKind::Xai;
    }
    if claw_provider::has_auth_from_env_or_saved().unwrap_or(false) {
        return ProviderKind::ClawApi;  // ❌ Перезаписывает Ollama!
    }
    
    ProviderKind::ClawApi  // Default fallback
}
```

**Проблема**: Функция возвращает правильный провайдер на первом шаге, но потом где-то он перезаписывается.

### Вариант 3: Проверить что происходит в from_model_with_default_auth()

```rust
pub fn from_model_with_default_auth(
    model: &str,
    default_auth: Option<AuthSource>,
) -> Result<Self, ApiError> {
    let resolved_model = providers::resolve_model_alias(model);
    match providers::detect_provider_kind(&resolved_model) {
        ProviderKind::Ollama => Ok(Self::Ollama(OpenAiCompatClient::from_env(
            OpenAiCompatConfig::ollama(),
        )?)),
        // ...
    }
}
```

Нужно добавить debug логирование чтобы увидеть что возвращает `detect_provider_kind()`.

## Временное решение

Пока интеграция дорабатывается, можно использовать Ollama напрямую:

```bash
# Через ollama CLI
ollama run llama3.2

# Через curl
curl -X POST http://127.0.0.1:11434/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "llama3.2",
    "messages": [{"role": "user", "content": "Hello"}],
    "max_tokens": 100
  }'
```

## Следующие шаги

1. Добавить debug логирование в `detect_provider_kind()`
2. Добавить debug логирование в `from_model_with_default_auth()`
3. Проверить что `OpenAiCompatClient` использует правильный endpoint
4. Добавить unit тесты для Ollama провайдера
5. Добавить интеграционные тесты

## Проверка endpoint в OpenAiCompatClient

Файл: `rust/crates/api/src/providers/openai_compat.rs`

```rust
async fn send_raw_request(
    &self,
    request: &MessageRequest,
) -> Result<reqwest::Response, ApiError> {
    let request_url = chat_completions_endpoint(&self.base_url);
    // ...
}

fn chat_completions_endpoint(base_url: &str) -> String {
    let trimmed = base_url.trim_end_matches('/');
    if trimmed.ends_with("/chat/completions") {
        trimmed.to_string()
    } else {
        format!("{trimmed}/chat/completions")
    }
}
```

Это выглядит правильно! Значит проблема точно в том, что создается `ClawApiClient` вместо `OpenAiCompatClient`.

## Финальная диагностика

Нужно добавить в код:

```rust
pub fn from_model_with_default_auth(
    model: &str,
    default_auth: Option<AuthSource>,
) -> Result<Self, ApiError> {
    let resolved_model = providers::resolve_model_alias(model);
    let provider_kind = providers::detect_provider_kind(&resolved_model);
    
    eprintln!("DEBUG: model={}, resolved={}, provider={:?}", 
              model, resolved_model, provider_kind);
    
    match provider_kind {
        // ...
    }
}
```

Это покажет что именно происходит при определении провайдера.
