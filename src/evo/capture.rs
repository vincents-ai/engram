//! Capture: Live capture of pi --mode json event stream.
//!
//! Spawns pi as a subprocess with `--mode json` and captures
//! the JSON event stream, parsing it into Trajectory structs.

use crate::error::EngramError;
use crate::evo::types::*;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};

/// Configuration for a pi capture session
#[derive(Debug, Clone)]
pub struct CaptureConfig {
    /// The prompt to send to pi
    pub prompt: String,
    /// Working directory for pi
    pub cwd: Option<String>,
    /// Model to use (passed as --model argument if set)
    pub model: Option<String>,
    /// Session directory for the replay (passed as --session-dir)
    pub session_dir: Option<String>,
    /// Timeout in seconds (default: 600)
    pub timeout_secs: u64,
}

/// Result of capturing a pi session
#[derive(Debug)]
pub struct CaptureResult {
    /// Parsed trajectory
    pub trajectory: Trajectory,
    /// Raw JSON event lines
    pub raw_events: Vec<String>,
    /// Exit code of the pi process
    pub exit_code: Option<i32>,
}

/// Capture events from pi --mode json and build a Trajectory.
///
/// Spawns pi as a child process, reads stdout line by line,
/// parses JSON events, and assembles a Trajectory.
pub fn capture_pi_session(config: &CaptureConfig) -> Result<CaptureResult, EngramError> {
    let mut cmd = Command::new("pi");
    cmd.arg("--mode")
        .arg("json")
        .arg("--no-session")
        .arg(&config.prompt)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    if let Some(ref cwd) = config.cwd {
        cmd.current_dir(cwd);
    }

    if let Some(ref session_dir) = config.session_dir {
        cmd.arg("--session-dir").arg(session_dir);
    }

    let mut child = cmd
        .spawn()
        .map_err(|e| EngramError::InvalidOperation(format!("Failed to spawn pi: {}", e)))?;

    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| EngramError::InvalidOperation("Failed to capture pi stdout".to_string()))?;

    let reader = BufReader::new(stdout);
    let mut raw_events = Vec::new();
    let mut events: Vec<serde_json::Value> = Vec::new();

    for line in reader.lines() {
        let line = line.map_err(|e| EngramError::Io(e))?;
        if line.trim().is_empty() {
            continue;
        }
        raw_events.push(line.clone());
        if let Ok(event) = serde_json::from_str::<serde_json::Value>(&line) {
            events.push(event);
        }
    }

    // Wait for process to finish
    let status = child.wait().map_err(|e| EngramError::Io(e))?;
    let exit_code = status.code();

    // Parse events into Trajectory
    let trajectory = parse_events(&events)?;

    Ok(CaptureResult {
        trajectory,
        raw_events,
        exit_code,
    })
}

/// Parse JSON events from pi --mode json into a Trajectory.
///
/// Event types from pi JSON mode:
/// - session: header with id, version, cwd, timestamp
/// - agent_start / agent_end: lifecycle
/// - turn_start / turn_end: per-turn
/// - message_start / message_update / message_end: messages
/// - tool_execution_start / tool_execution_end: tool calls
pub fn parse_events(events: &[serde_json::Value]) -> Result<Trajectory, EngramError> {
    use chrono::{DateTime, Utc};

    let mut session_id = String::new();
    let mut cwd = String::new();
    let mut model = String::new();
    let mut provider = String::new();
    let mut turns: Vec<Turn> = Vec::new();
    let mut task_description: Option<String> = None;
    let mut total_tokens: u64 = 0;
    let mut created_at = Utc::now();

    // Accumulate current turn state
    let mut current_thinking: Option<String> = None;
    let mut current_text: Option<String> = None;
    let mut current_tool_calls: Vec<ToolCall> = Vec::new();
    let mut current_tool_results: Vec<ToolResult> = Vec::new();
    let mut current_stop_reason = StopReason::Unknown;
    let mut current_timestamp: i64 = 0;
    let mut turn_index: usize = 0;
    let mut _pending_tool_call_id: Option<String> = None;
    let mut _pending_tool_name: Option<String> = None;

    for event in events {
        let event_type = event["type"].as_str().unwrap_or("");

        match event_type {
            "session" => {
                session_id = event["id"].as_str().unwrap_or("unknown").to_string();
                cwd = event["cwd"].as_str().unwrap_or("").to_string();
                if let Some(ts) = event["timestamp"].as_str() {
                    created_at = DateTime::parse_from_rfc3339(ts)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now());
                }
            }
            "message_start" => {
                let msg = &event["message"];
                let role = msg["role"].as_str().unwrap_or("");

                if role == "assistant" {
                    current_thinking = None;
                    current_text = None;
                    current_tool_calls.clear();
                    current_stop_reason = StopReason::Unknown;
                    current_timestamp = msg["timestamp"].as_i64().unwrap_or(0);

                    // Parse content blocks
                    if let Some(blocks) = msg["content"].as_array() {
                        for block in blocks {
                            match block["type"].as_str().unwrap_or("") {
                                "thinking" => {
                                    current_thinking = block["thinking"].as_str().map(String::from);
                                }
                                "text" => {
                                    current_text = block["text"].as_str().map(String::from);
                                }
                                "toolCall" => {
                                    current_tool_calls.push(ToolCall {
                                        id: block["id"].as_str().unwrap_or("").to_string(),
                                        name: block["name"].as_str().unwrap_or("").to_string(),
                                        arguments: block["arguments"].clone(),
                                    });
                                }
                                _ => {}
                            }
                        }
                    }

                    if let Some(reason) = msg["stopReason"].as_str() {
                        current_stop_reason = match reason {
                            "stop" => StopReason::Stop,
                            "length" => StopReason::Length,
                            "toolUse" => StopReason::ToolUse,
                            "error" => StopReason::Error,
                            "aborted" => StopReason::Aborted,
                            _ => StopReason::Unknown,
                        };
                    }

                    // Track model
                    if let Some(m) = msg["model"].as_str() {
                        model = m.to_string();
                    }
                    if let Some(p) = msg["provider"].as_str() {
                        provider = p.to_string();
                    }
                    if let Some(usage) = msg.get("usage") {
                        total_tokens += usage["totalTokens"].as_u64().unwrap_or(0);
                    }
                } else if role == "user" {
                    let text = extract_text(msg["content"].clone());
                    if task_description.is_none() {
                        task_description = Some(text);
                    }
                }
            }
            "message_update" => {
                // Accumulate streaming text/thinking updates
                let _msg = &event["message"];
                let update = &event["assistantMessageEvent"];
                let update_type = update["type"].as_str().unwrap_or("");

                match update_type {
                    "text_delta" => {
                        let delta = update["delta"].as_str().unwrap_or("");
                        if let Some(ref mut text) = current_text {
                            text.push_str(delta);
                        } else {
                            current_text = Some(delta.to_string());
                        }
                    }
                    "thinking_delta" => {
                        let delta = update["thinking"].as_str().unwrap_or("");
                        if let Some(ref mut thinking) = current_thinking {
                            thinking.push_str(delta);
                        } else {
                            current_thinking = Some(delta.to_string());
                        }
                    }
                    _ => {}
                }
            }
            "tool_execution_start" => {
                _pending_tool_call_id = event["toolCallId"].as_str().map(String::from);
                _pending_tool_name = event["toolName"].as_str().map(String::from);
            }
            "tool_execution_end" => {
                let tool_call_id = event["toolCallId"].as_str().unwrap_or("").to_string();
                let tool_name = event["toolName"].as_str().unwrap_or("").to_string();
                let is_error = event["isError"].as_bool().unwrap_or(false);

                let content = if let Some(result) = event.get("result") {
                    if let Some(s) = result.as_str() {
                        s.to_string()
                    } else {
                        result.to_string()
                    }
                } else {
                    String::new()
                };

                current_tool_results.push(ToolResult {
                    tool_call_id,
                    tool_name,
                    content: if content.len() > 10000 {
                        format!("{}...", &content[..10000])
                    } else {
                        content
                    },
                    is_error,
                    exit_code: None,
                    truncated: false,
                });

                _pending_tool_call_id = None;
                _pending_tool_name = None;
            }
            "turn_end" => {
                // Flush current turn
                let tool_results = std::mem::take(&mut current_tool_results);
                let tool_calls = if current_tool_calls.is_empty() {
                    Vec::new()
                } else {
                    std::mem::take(&mut current_tool_calls)
                };

                turns.push(Turn {
                    index: turn_index,
                    user_message: None,
                    assistant_thinking: current_thinking.take(),
                    assistant_text: current_text.take(),
                    tool_calls,
                    tool_results,
                    timestamp: current_timestamp,
                    stopped_reason: current_stop_reason.clone(),
                });
                turn_index += 1;
            }
            "agent_end" => {
                // Flush any remaining state
                if current_thinking.is_some()
                    || current_text.is_some()
                    || !current_tool_results.is_empty()
                {
                    turns.push(Turn {
                        index: turn_index,
                        user_message: None,
                        assistant_thinking: current_thinking.take(),
                        assistant_text: current_text.take(),
                        tool_calls: std::mem::take(&mut current_tool_calls),
                        tool_results: std::mem::take(&mut current_tool_results),
                        timestamp: current_timestamp,
                        stopped_reason: current_stop_reason.clone(),
                    });
                }
            }
            _ => {}
        }
    }

    Ok(Trajectory {
        session_id,
        cwd,
        model,
        provider,
        turns,
        task_description,
        created_at,
        total_tokens: if total_tokens > 0 {
            Some(total_tokens)
        } else {
            None
        },
    })
}

/// Extract text from message content (string or array of blocks)
fn extract_text(content: serde_json::Value) -> String {
    match content {
        serde_json::Value::String(s) => s,
        serde_json::Value::Array(blocks) => {
            let mut texts = Vec::new();
            for block in blocks {
                if block["type"].as_str() == Some("text") {
                    if let Some(text) = block["text"].as_str() {
                        texts.push(text.to_string());
                    }
                }
            }
            texts.join("\n")
        }
        _ => String::new(),
    }
}
