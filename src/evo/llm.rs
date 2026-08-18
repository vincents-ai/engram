//! LLM Client for Engram-Evo evaluation and optimization.
//!
//! Provides structured output via LLM API calls for:
//! - PlanAdherence metric (LLM-as-judge comparing thinking vs execution)
//! - MemoryOptimizer (failure analysis → MemoryPatch generation)
//!
//! Uses the OpenAI-compatible chat completions API format, which works with:
//! - OpenAI / GPT models
//! - Anthropic (via OpenAI-compatible proxy)
//! - OpenRouter
//! - Any compatible endpoint
//!
//! Authentication reads from pi's ~/.pi/agent/auth.json when available,
//! falling back to environment variables.

use crate::error::EngramError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ─── Types ────────────────────────────────────────────────────────────────────

/// LLM client configuration
#[derive(Debug, Clone)]
pub struct LlmClient {
    endpoint: String,
    api_key: String,
    model: String,
    client: reqwest::blocking::Client,
}

/// OpenAI-compatible chat completion request
#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<ResponseFormat>,
}

#[derive(Debug, Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct ResponseFormat {
    r#type: String,
}

/// OpenAI-compatible chat completion response
#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatChoiceMessage,
}

#[derive(Debug, Deserialize)]
struct ChatChoiceMessage {
    content: String,
}

// ─── Config discovery ─────────────────────────────────────────────────────────

/// Discover LLM configuration from pi's auth settings.
///
/// Reads ~/.pi/agent/auth.json to find available API keys,
/// then selects the best available model.
fn discover_config() -> Result<(String, String, String), EngramError> {
    let auth_path = dirs::home_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join(".pi/agent/auth.json");

    if auth_path.exists() {
        let content = std::fs::read_to_string(&auth_path).map_err(EngramError::Io)?;
        let auth: serde_json::Value =
            serde_json::from_str(&content).map_err(EngramError::Serialization)?;

        // Try providers in preference order
        let providers = [
            (
                "anthropic",
                "https://api.anthropic.com/v1/chat/completions",
                "claude-sonnet-4-20250514",
            ),
            (
                "openai",
                "https://api.openai.com/v1/chat/completions",
                "gpt-4o",
            ),
            (
                "openrouter",
                "https://openrouter.ai/api/v1/chat/completions",
                "anthropic/claude-sonnet-4-20250514",
            ),
        ];

        for (provider, endpoint, model) in &providers {
            if let Some(key) = auth.get(provider).and_then(|v| v["apiKey"].as_str()) {
                if !key.is_empty() {
                    return Ok((endpoint.to_string(), key.to_string(), model.to_string()));
                }
            }
        }
    }

    // Fall back to environment variables
    if let Ok(key) = std::env::var("OPENAI_API_KEY") {
        return Ok((
            "https://api.openai.com/v1/chat/completions".to_string(),
            key,
            "gpt-4o".to_string(),
        ));
    }
    if let Ok(key) = std::env::var("ANTHROPIC_API_KEY") {
        return Ok((
            "https://api.anthropic.com/v1/chat/completions".to_string(),
            key,
            "claude-sonnet-4-20250514".to_string(),
        ));
    }
    if let Ok(key) = std::env::var("OPENROUTER_API_KEY") {
        return Ok((
            "https://openrouter.ai/api/v1/chat/completions".to_string(),
            key,
            "anthropic/claude-sonnet-4-20250514".to_string(),
        ));
    }

    Err(EngramError::Config(
        crate::error::ConfigError::ValidationFailed(
            "No LLM API key found. Set OPENAI_API_KEY, ANTHROPIC_API_KEY, or configure ~/.pi/agent/auth.json".into()
        )
    ))
}

// ─── Implementation ───────────────────────────────────────────────────────────

impl LlmClient {
    /// Create a new LLM client with auto-discovered configuration
    pub fn new() -> Result<Self, EngramError> {
        let (endpoint, api_key, model) = discover_config()?;
        Ok(Self {
            endpoint,
            api_key,
            model,
            client: reqwest::blocking::Client::new(),
        })
    }

    /// Create a client with explicit configuration
    pub fn with_config(endpoint: String, api_key: String, model: String) -> Self {
        Self {
            endpoint,
            api_key,
            model,
            client: reqwest::blocking::Client::new(),
        }
    }

    /// Get the model name being used
    pub fn model(&self) -> &str {
        &self.model
    }

    /// Send a simple chat completion request
    pub fn complete(&self, system_prompt: &str, user_prompt: &str) -> Result<String, EngramError> {
        let request = ChatRequest {
            model: self.model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: user_prompt.to_string(),
                },
            ],
            temperature: 0.3,
            max_tokens: Some(4096),
            response_format: None,
        };

        let response = self
            .client
            .post(&self.endpoint)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .map_err(|e| EngramError::InvalidOperation(format!("LLM request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            return Err(EngramError::InvalidOperation(format!(
                "LLM API error ({}): {}",
                status, body
            )));
        }

        let chat_response: ChatResponse = response.json().map_err(|e| {
            EngramError::Deserialization(format!("Failed to parse LLM response: {}", e))
        })?;

        chat_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| EngramError::InvalidOperation("No response from LLM".to_string()))
    }

    /// Send a chat completion request and parse the response as structured JSON
    pub fn complete_json<T: serde::de::DeserializeOwned>(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<T, EngramError> {
        let request = ChatRequest {
            model: self.model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: user_prompt.to_string(),
                },
            ],
            temperature: 0.2,
            max_tokens: Some(4096),
            response_format: Some(ResponseFormat {
                r#type: "json_object".to_string(),
            }),
        };

        let response = self
            .client
            .post(&self.endpoint)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .map_err(|e| EngramError::InvalidOperation(format!("LLM request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            return Err(EngramError::InvalidOperation(format!(
                "LLM API error ({}): {}",
                status, body
            )));
        }

        let chat_response: ChatResponse = response.json().map_err(|e| {
            EngramError::Deserialization(format!("Failed to parse LLM response: {}", e))
        })?;

        let content = chat_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| EngramError::InvalidOperation("No response from LLM".to_string()))?;

        // Extract JSON from possible markdown code blocks
        let json_str = extract_json(&content);

        serde_json::from_str::<T>(json_str).map_err(|e| {
            EngramError::Deserialization(format!(
                "Failed to parse structured LLM output: {}. Content: {}",
                e,
                &content[..content.len().min(200)]
            ))
        })
    }
}

/// Extract JSON from possible markdown code block wrapping
fn extract_json(content: &str) -> &str {
    let trimmed = content.trim();

    // If wrapped in ```json ... ```, strip it
    if trimmed.starts_with("```json") && trimmed.ends_with("```") {
        return trimmed
            .strip_prefix("```json")
            .unwrap()
            .strip_suffix("```")
            .unwrap()
            .trim();
    }
    if trimmed.starts_with("```") && trimmed.ends_with("```") {
        return trimmed
            .strip_prefix("```")
            .unwrap()
            .strip_suffix("```")
            .unwrap()
            .trim();
    }

    trimmed
}

// ─── Plan Adherence LLM Judge ─────────────────────────────────────────────────

/// Result from LLM-as-judge plan adherence evaluation
#[derive(Debug, Deserialize)]
pub struct PlanAdherenceResult {
    pub score: f64,
    pub reasoning: String,
    pub deviations: Vec<String>,
}

/// Evaluate plan adherence using LLM-as-judge
pub fn evaluate_plan_adherence(
    client: &LlmClient,
    trajectory: &crate::evo::types::Trajectory,
) -> Result<PlanAdherenceResult, EngramError> {
    let system_prompt = r#"You are an expert evaluator for AI coding agents. Your task is to evaluate how well an agent's actual execution adhered to its stated plan and reasoning.

Score on a scale of 0.0 to 1.0:
- 1.0: Perfect adherence — every action followed the agent's reasoning
- 0.7-0.9: Minor deviations — agent adapted sensibly
- 0.4-0.6: Moderate deviations — some unplanned actions or missed steps
- 0.0-0.3: Poor adherence — actions barely related to reasoning

Respond in JSON format:
{
  "score": <float 0.0-1.0>,
  "reasoning": "<brief explanation>",
  "deviations": ["<list of specific deviations>"]
}"#;

    // Build a summary of the trajectory for the LLM
    let mut trajectory_summary = String::new();
    trajectory_summary.push_str(&format!(
        "Task: {}\n",
        trajectory.task_description.as_deref().unwrap_or("Unknown")
    ));
    trajectory_summary.push_str(&format!(
        "Model: {} | Turns: {}\n\n",
        trajectory.model,
        trajectory.turns.len()
    ));

    for turn in &trajectory.turns {
        trajectory_summary.push_str(&format!("--- Turn {} ---\n", turn.index));
        if let Some(thinking) = &turn.assistant_thinking {
            let truncated = if thinking.len() > 300 {
                format!("{}...", &thinking[..300])
            } else {
                thinking.clone()
            };
            trajectory_summary.push_str(&format!("Thinking: {}\n", truncated));
        }
        if let Some(text) = &turn.assistant_text {
            let truncated = if text.len() > 200 {
                format!("{}...", &text[..200])
            } else {
                text.clone()
            };
            trajectory_summary.push_str(&format!("Said: {}\n", truncated));
        }
        for tc in &turn.tool_calls {
            trajectory_summary.push_str(&format!(
                "Tool call: {}({})\n",
                tc.name,
                truncate_value(&tc.arguments, 100)
            ));
        }
        for tr in &turn.tool_results {
            let status = if tr.is_error { "ERROR" } else { "OK" };
            trajectory_summary.push_str(&format!(
                "Tool result ({}): {} - {} chars\n",
                status,
                tr.tool_name,
                tr.content.len()
            ));
        }
        trajectory_summary.push('\n');
    }

    client.complete_json(system_prompt, &trajectory_summary)
}

/// Truncate a JSON value for display
fn truncate_value(value: &serde_json::Value, max_len: usize) -> String {
    let s = value.to_string();
    if s.len() <= max_len {
        s
    } else {
        format!("{}...", &s[..max_len])
    }
}
