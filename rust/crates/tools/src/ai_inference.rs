use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::{Duration, Instant};

#[derive(Debug, Deserialize)]
pub struct AiInferenceInput {
    pub model: String,
    pub prompt: String,
    pub system_prompt: Option<String>,
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
    pub custom_endpoint: Option<String>,
}

fn default_temperature() -> f32 { 0.7 }
fn default_max_tokens() -> u32 { 2048 }

#[derive(Debug, Serialize)]
pub struct AiInferenceOutput {
    pub model: String,
    pub response: String,
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub duration_ms: u128,
    pub provider: String,
}

pub fn execute_ai_inference(input: &AiInferenceInput) -> Result<AiInferenceOutput, String> {
    let started = Instant::now();

    let (provider, endpoint, api_key) = resolve_provider(&input.model, &input.custom_endpoint)?;

    let client = Client::builder()
        .timeout(Duration::from_secs(120))
        .build()
        .map_err(|e| format!("HTTP client error: {e}"))?;

    let raw = match provider.as_str() {
        "ollama" => call_ollama(&client, &endpoint, input)?,
        "openai" | "xai" => call_openai_compat(&client, &endpoint, &api_key, input)?,
        "anthropic" => call_anthropic(&client, &endpoint, &api_key, input)?,
        _ => return Err(format!("Unknown provider: {provider}")),
    };

    let parsed: Value = serde_json::from_str(&raw)
        .map_err(|e| format!("Failed to parse response: {e}\nRaw: {raw}"))?;

    let (response, in_tok, out_tok) = extract_response(&parsed, &provider)?;

    Ok(AiInferenceOutput {
        model: input.model.clone(),
        response,
        input_tokens: in_tok,
        output_tokens: out_tok,
        duration_ms: started.elapsed().as_millis(),
        provider,
    })
}

fn resolve_provider(
    model: &str,
    custom_endpoint: &Option<String>,
) -> Result<(String, String, String), String> {
    if let Some(ep) = custom_endpoint {
        return Ok(("openai".to_string(), ep.clone(), String::new()));
    }

    let lower = model.to_ascii_lowercase();

    // Ollama: local models
    if lower.contains(':') || lower.contains('/')
        || lower.starts_with("llama")
        || lower.starts_with("qwen")
        || lower.starts_with("mistral")
        || lower.starts_with("gemma")
        || lower.starts_with("phi")
        || lower.starts_with("deepseek")
    {
        let ep = std::env::var("OLLAMA_BASE_URL")
            .unwrap_or_else(|_| "http://localhost:11434".to_string());
        return Ok(("ollama".to_string(), ep, String::new()));
    }

    // OpenAI
    if lower.starts_with("gpt-") || lower.starts_with("o1") || lower.starts_with("o3") {
        let ep = std::env::var("OPENAI_BASE_URL")
            .unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
        let key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| "OPENAI_API_KEY not set".to_string())?;
        return Ok(("openai".to_string(), ep, key));
    }

    // xAI / Grok
    if lower.starts_with("grok") {
        let ep = std::env::var("XAI_BASE_URL")
            .unwrap_or_else(|_| "https://api.x.ai/v1".to_string());
        let key = std::env::var("XAI_API_KEY")
            .map_err(|_| "XAI_API_KEY not set".to_string())?;
        return Ok(("xai".to_string(), ep, key));
    }

    // Anthropic
    if lower.starts_with("claude") {
        let ep = std::env::var("ANTHROPIC_BASE_URL")
            .unwrap_or_else(|_| "https://api.anthropic.com/v1".to_string());
        let key = std::env::var("ANTHROPIC_API_KEY")
            .map_err(|_| "ANTHROPIC_API_KEY not set".to_string())?;
        return Ok(("anthropic".to_string(), ep, key));
    }

    Err(format!("Cannot determine provider for model: {model}"))
}

fn call_ollama(client: &Client, endpoint: &str, input: &AiInferenceInput) -> Result<String, String> {
    let url = format!("{}/api/generate", endpoint.trim_end_matches('/'));
    let mut payload = json!({
        "model": input.model,
        "prompt": input.prompt,
        "stream": false,
        "options": {
            "temperature": input.temperature,
            "num_predict": input.max_tokens,
        }
    });
    if let Some(sys) = &input.system_prompt {
        payload["system"] = json!(sys);
    }
    let resp = client.post(&url).json(&payload).send()
        .map_err(|e| format!("Ollama request failed: {e}"))?;
    if !resp.status().is_success() {
        return Err(format!("Ollama error: {}", resp.status()));
    }
    resp.text().map_err(|e| format!("Read error: {e}"))
}

fn call_openai_compat(client: &Client, endpoint: &str, api_key: &str, input: &AiInferenceInput) -> Result<String, String> {
    let url = format!("{}/chat/completions", endpoint.trim_end_matches('/'));
    let mut messages = Vec::new();
    if let Some(sys) = &input.system_prompt {
        messages.push(json!({"role": "system", "content": sys}));
    }
    messages.push(json!({"role": "user", "content": input.prompt}));
    let payload = json!({
        "model": input.model,
        "messages": messages,
        "temperature": input.temperature,
        "max_tokens": input.max_tokens,
    });
    let mut req = client.post(&url).json(&payload);
    if !api_key.is_empty() {
        req = req.header("Authorization", format!("Bearer {api_key}"));
    }
    let resp = req.send().map_err(|e| format!("Request failed: {e}"))?;
    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().unwrap_or_default();
        return Err(format!("API error {status}: {body}"));
    }
    resp.text().map_err(|e| format!("Read error: {e}"))
}

fn call_anthropic(client: &Client, endpoint: &str, api_key: &str, input: &AiInferenceInput) -> Result<String, String> {
    let url = format!("{}/messages", endpoint.trim_end_matches('/'));
    let mut payload = json!({
        "model": input.model,
        "max_tokens": input.max_tokens,
        "messages": [{"role": "user", "content": input.prompt}]
    });
    if let Some(sys) = &input.system_prompt {
        payload["system"] = json!(sys);
    }
    let resp = client.post(&url)
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&payload)
        .send()
        .map_err(|e| format!("Anthropic request failed: {e}"))?;
    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().unwrap_or_default();
        return Err(format!("Anthropic error {status}: {body}"));
    }
    resp.text().map_err(|e| format!("Read error: {e}"))
}

fn extract_response(parsed: &Value, provider: &str) -> Result<(String, u32, u32), String> {
    match provider {
        "ollama" => {
            let text = parsed["response"].as_str()
                .ok_or("Missing 'response' in Ollama output")?
                .to_string();
            let in_tok = parsed["prompt_eval_count"].as_u64().unwrap_or(0) as u32;
            let out_tok = parsed["eval_count"].as_u64().unwrap_or(0) as u32;
            Ok((text, in_tok, out_tok))
        }
        "openai" | "xai" => {
            let text = parsed["choices"][0]["message"]["content"].as_str()
                .ok_or("Missing content in OpenAI response")?
                .to_string();
            let in_tok = parsed["usage"]["prompt_tokens"].as_u64().unwrap_or(0) as u32;
            let out_tok = parsed["usage"]["completion_tokens"].as_u64().unwrap_or(0) as u32;
            Ok((text, in_tok, out_tok))
        }
        "anthropic" => {
            let text = parsed["content"][0]["text"].as_str()
                .ok_or("Missing text in Anthropic response")?
                .to_string();
            let in_tok = parsed["usage"]["input_tokens"].as_u64().unwrap_or(0) as u32;
            let out_tok = parsed["usage"]["output_tokens"].as_u64().unwrap_or(0) as u32;
            Ok((text, in_tok, out_tok))
        }
        _ => Err(format!("Unknown provider: {provider}")),
    }
}
