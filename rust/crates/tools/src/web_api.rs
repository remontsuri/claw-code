use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Deserialize)]
pub struct WebApiInput {
    pub url: String,
    pub method: HttpMethod,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub auth: Option<AuthConfig>,
    #[serde(default = "default_timeout")]
    pub timeout: u64,
}

fn default_timeout() -> u64 {
    30
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

#[derive(Debug, Deserialize)]
pub struct AuthConfig {
    #[serde(rename = "type")]
    pub auth_type: AuthType,
    pub token: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub api_key_header: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AuthType {
    Bearer,
    Basic,
    ApiKey,
}

#[derive(Debug, Serialize)]
pub struct WebApiOutput {
    pub status: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub parsed_json: Option<Value>,
    pub duration_ms: u128,
    pub url: String,
}

pub fn execute_web_api(input: &WebApiInput) -> Result<WebApiOutput, String> {
    let started = Instant::now();

    let client = Client::builder()
        .timeout(Duration::from_secs(input.timeout))
        .redirect(reqwest::redirect::Policy::limited(10))
        .user_agent("Claw-AI-Agent/1.0")
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {e}"))?;

    let mut headers = HeaderMap::new();
    for (key, value) in &input.headers {
        let name = HeaderName::from_bytes(key.as_bytes())
            .map_err(|e| format!("Invalid header name '{key}': {e}"))?;
        let val = HeaderValue::from_str(value)
            .map_err(|e| format!("Invalid header value for '{key}': {e}"))?;
        headers.insert(name, val);
    }

    // Auth
    if let Some(auth) = &input.auth {
        match auth.auth_type {
            AuthType::Bearer => {
                if let Some(token) = &auth.token {
                    headers.insert(
                        reqwest::header::AUTHORIZATION,
                        HeaderValue::from_str(&format!("Bearer {token}"))
                            .map_err(|e| format!("Invalid bearer token: {e}"))?,
                    );
                }
            }
            AuthType::Basic => {
                if let (Some(user), Some(pass)) = (&auth.username, &auth.password) {
                    // simple base64 without external crate
                    let creds = encode_base64(format!("{user}:{pass}").as_bytes());
                    headers.insert(
                        reqwest::header::AUTHORIZATION,
                        HeaderValue::from_str(&format!("Basic {creds}"))
                            .map_err(|e| format!("Invalid basic auth: {e}"))?,
                    );
                }
            }
            AuthType::ApiKey => {
                if let (Some(header_name), Some(token)) = (&auth.api_key_header, &auth.token) {
                    let name = HeaderName::from_bytes(header_name.as_bytes())
                        .map_err(|e| format!("Invalid API key header: {e}"))?;
                    headers.insert(
                        name,
                        HeaderValue::from_str(token)
                            .map_err(|e| format!("Invalid API key value: {e}"))?,
                    );
                }
            }
        }
    }

    let request = match input.method {
        HttpMethod::Get => client.get(&input.url),
        HttpMethod::Post => client.post(&input.url),
        HttpMethod::Put => client.put(&input.url),
        HttpMethod::Delete => client.delete(&input.url),
        HttpMethod::Patch => client.patch(&input.url),
    };

    let request = request.headers(headers);
    let request = if let Some(body) = &input.body {
        request.body(body.clone())
    } else {
        request
    };

    let response = request
        .send()
        .map_err(|e| format!("HTTP request failed: {e}"))?;

    let status = response.status();
    let final_url = response.url().to_string();

    let response_headers: HashMap<String, String> = response
        .headers()
        .iter()
        .filter_map(|(k, v)| v.to_str().ok().map(|v| (k.to_string(), v.to_string())))
        .collect();

    let body = response
        .text()
        .map_err(|e| format!("Failed to read response body: {e}"))?;

    let parsed_json = serde_json::from_str::<Value>(&body).ok();

    Ok(WebApiOutput {
        status: status.as_u16(),
        status_text: status.canonical_reason().unwrap_or("Unknown").to_string(),
        headers: response_headers,
        body,
        parsed_json,
        duration_ms: started.elapsed().as_millis(),
        url: final_url,
    })
}

/// Minimal base64 encoder (no external crate needed)
fn encode_base64(input: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in input.chunks(3) {
        let b0 = chunk[0] as usize;
        let b1 = if chunk.len() > 1 { chunk[1] as usize } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as usize } else { 0 };
        out.push(CHARS[b0 >> 2] as char);
        out.push(CHARS[((b0 & 3) << 4) | (b1 >> 4)] as char);
        out.push(if chunk.len() > 1 { CHARS[((b1 & 0xf) << 2) | (b2 >> 6)] as char } else { '=' });
        out.push(if chunk.len() > 2 { CHARS[b2 & 0x3f] as char } else { '=' });
    }
    out
}
