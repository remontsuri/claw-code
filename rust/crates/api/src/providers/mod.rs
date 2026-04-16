use std::future::Future;
use std::pin::Pin;

use crate::error::ApiError;
use crate::types::{MessageRequest, MessageResponse};

pub mod claw_provider;
pub mod openai_compat;

pub type ProviderFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T, ApiError>> + Send + 'a>>;

pub trait Provider {
    type Stream;

    fn send_message<'a>(
        &'a self,
        request: &'a MessageRequest,
    ) -> ProviderFuture<'a, MessageResponse>;

    fn stream_message<'a>(
        &'a self,
        request: &'a MessageRequest,
    ) -> ProviderFuture<'a, Self::Stream>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderKind {
    ClawApi,
    Xai,
    OpenAi,
    Ollama,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProviderMetadata {
    pub provider: ProviderKind,
    pub auth_env: &'static str,
    pub base_url_env: &'static str,
    pub default_base_url: &'static str,
}

// Reusable metadata constants
const CLAUDE_METADATA: ProviderMetadata = ProviderMetadata {
    provider: ProviderKind::ClawApi,
    auth_env: "ANTHROPIC_API_KEY",
    base_url_env: "ANTHROPIC_BASE_URL",
    default_base_url: claw_provider::DEFAULT_BASE_URL,
};

const XAI_METADATA: ProviderMetadata = ProviderMetadata {
    provider: ProviderKind::Xai,
    auth_env: "XAI_API_KEY",
    base_url_env: "XAI_BASE_URL",
    default_base_url: openai_compat::DEFAULT_XAI_BASE_URL,
};

const OPENAI_METADATA: ProviderMetadata = ProviderMetadata {
    provider: ProviderKind::OpenAi,
    auth_env: "OPENAI_API_KEY",
    base_url_env: "OPENAI_BASE_URL",
    default_base_url: openai_compat::DEFAULT_OPENAI_BASE_URL,
};

const OLLAMA_METADATA: ProviderMetadata = ProviderMetadata {
    provider: ProviderKind::Ollama,
    auth_env: "OLLAMA_API_KEY",
    base_url_env: "OLLAMA_BASE_URL",
    default_base_url: openai_compat::DEFAULT_OLLAMA_BASE_URL,
};

const MODEL_REGISTRY: &[(&str, ProviderMetadata)] = &[
    // Claude models
    ("opus", CLAUDE_METADATA),
    ("sonnet", CLAUDE_METADATA),
    ("haiku", CLAUDE_METADATA),
    ("claude-opus-4-6", CLAUDE_METADATA),
    ("claude-sonnet-4-6", CLAUDE_METADATA),
    ("claude-haiku-4-5-20251213", CLAUDE_METADATA),
    // Xai/Grok models
    ("grok", XAI_METADATA),
    ("grok-3", XAI_METADATA),
    ("grok-mini", XAI_METADATA),
    ("grok-3-mini", XAI_METADATA),
    ("grok-2", XAI_METADATA),
    // OpenAI models
    ("gpt-4o", OPENAI_METADATA),
    ("gpt-4o-mini", OPENAI_METADATA),
    ("gpt-4-turbo", OPENAI_METADATA),
    ("gpt-4", OPENAI_METADATA),
    ("o1", OPENAI_METADATA),
    ("o1-mini", OPENAI_METADATA),
    ("o1-preview", OPENAI_METADATA),
    ("o3-mini", OPENAI_METADATA),
    // Ollama models
    ("qwen3.5-uncensored:4b", OLLAMA_METADATA),
    ("qwen2.5", OLLAMA_METADATA),
    ("llama3.2", OLLAMA_METADATA),
    ("mistral", OLLAMA_METADATA),
    ("codellama", OLLAMA_METADATA),
];

#[must_use]
pub fn resolve_model_alias(model: &str) -> String {
    let trimmed = model.trim();
    let lower = trimmed.to_ascii_lowercase();
    MODEL_REGISTRY
        .iter()
        .find_map(|(alias, metadata)| {
            (*alias == lower).then_some(match metadata.provider {
                ProviderKind::ClawApi => match *alias {
                    "opus" => "claude-opus-4-6",
                    "sonnet" => "claude-sonnet-4-6",
                    "haiku" => "claude-haiku-4-5-20251213",
                    _ => trimmed,
                },
                ProviderKind::Xai => match *alias {
                    "grok" | "grok-3" => "grok-3",
                    "grok-mini" | "grok-3-mini" => "grok-3-mini",
                    "grok-2" => "grok-2",
                    _ => trimmed,
                },
                ProviderKind::OpenAi | ProviderKind::Ollama => trimmed,
            })
        })
        .map_or_else(|| trimmed.to_string(), ToOwned::to_owned)
}

#[must_use]
pub fn metadata_for_model(model: &str) -> Option<ProviderMetadata> {
    let lower = model.trim().to_ascii_lowercase();
    
    // Direct lookup in registry
    if let Some((_, metadata)) = MODEL_REGISTRY.iter().find(|(alias, _)| *alias == lower) {
        return Some(*metadata);
    }
    
    // Fallback: unknown grok models default to Xai
    if lower.starts_with("grok") {
        return Some(XAI_METADATA);
    }
    
    // Fallback: check for common OpenAI patterns
    if lower.starts_with("gpt-") || lower.starts_with("o") {
        return Some(OPENAI_METADATA);
    }
    
    // Fallback: Ollama models — colon tag format, known prefixes, or namespace/model format
    if lower.contains(':') || lower.contains('/')
        || lower.starts_with("llama") || lower.starts_with("qwen")
        || lower.starts_with("mistral") || lower.starts_with("codellama")
        || lower.starts_with("gemma") || lower.starts_with("deepseek")
        || lower.starts_with("phi") || lower.starts_with("olmo") {
        return Some(OLLAMA_METADATA);
    }
    
    None
}

#[must_use]
pub fn detect_provider_kind(model: &str) -> ProviderKind {
    // First: check model name in registry
    if let Some(metadata) = metadata_for_model(model) {
        return metadata.provider;
    }
    
    // Second: explicit env vars take priority over saved credentials
    if openai_compat::has_api_key("OPENAI_API_KEY") {
        return ProviderKind::OpenAi;
    }
    if openai_compat::has_api_key("XAI_API_KEY") {
        return ProviderKind::Xai;
    }
    if openai_compat::has_api_key("OLLAMA_API_KEY") {
        return ProviderKind::Ollama;
    }
    if claw_provider::has_auth_from_env_or_saved().unwrap_or(false) {
        return ProviderKind::ClawApi;
    }
    
    // Default fallback: if nothing else matches, assume ClawApi
    ProviderKind::ClawApi
}

#[must_use]
pub fn max_tokens_for_model(model: &str) -> u32 {
    let canonical = resolve_model_alias(model);
    if canonical.contains("opus") {
        32_000
    } else {
        64_000
    }
}

#[cfg(test)]
mod tests {
    use super::{detect_provider_kind, max_tokens_for_model, resolve_model_alias, ProviderKind};

    #[test]
    fn resolves_grok_aliases() {
        assert_eq!(resolve_model_alias("grok"), "grok-3");
        assert_eq!(resolve_model_alias("grok-mini"), "grok-3-mini");
        assert_eq!(resolve_model_alias("grok-2"), "grok-2");
    }

    #[test]
    fn resolves_claude_aliases() {
        assert_eq!(resolve_model_alias("opus"), "claude-opus-4-6");
        assert_eq!(resolve_model_alias("sonnet"), "claude-sonnet-4-6");
        assert_eq!(resolve_model_alias("haiku"), "claude-haiku-4-5-20251213");
    }

    #[test]
    fn detects_provider_from_model_name_first() {
        assert_eq!(detect_provider_kind("grok"), ProviderKind::Xai);
        assert_eq!(
            detect_provider_kind("claude-sonnet-4-6"),
            ProviderKind::ClawApi
        );
        assert_eq!(detect_provider_kind("gpt-4o"), ProviderKind::OpenAi);
        assert_eq!(detect_provider_kind("o1"), ProviderKind::OpenAi);
    }

    #[test]
    fn keeps_existing_max_token_heuristic() {
        assert_eq!(max_tokens_for_model("opus"), 32_000);
        assert_eq!(max_tokens_for_model("grok-3"), 64_000);
    }
}