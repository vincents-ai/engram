//! Ingestion: Parse pi session JSONL files into Trajectory structs.
//!
//! Handles parsing of pi session files stored in ~/.pi/agent/sessions/
//! into structured Trajectory objects for evaluation.
//!
//! Session JSONL format (v3):
//! - Line 1: session header with id, version, cwd, timestamp
//! - Subsequent: entries with type, id, parentId, timestamp + type-specific fields
//! - message entries: { type: "message", message: { role, content, ... } }
//!   - role "user": content is text or array of content blocks
//!   - role "assistant": content has thinking, text, toolCall blocks
//!   - role "toolResult": content has tool results with isError flag

use crate::evo::cli::IngestArgs;
use crate::evo::types::*;
use crate::error::EngramError;
use chrono::{DateTime, Utc};
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Handle the ingest subcommand
pub fn handle_ingest(args: IngestArgs) -> Result<(), EngramError> {
    let sessions_dir = args.sessions_dir.unwrap_or_else(|| {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".pi/agent/sessions")
            .to_string_lossy()
            .to_string()
    });

    let output_path = &args.output;
    let json_mode = args.output == "-" || output_path.ends_with(".json");

    tracing::info!("Ingesting sessions from: {:?}", sessions_dir);

    let session_files = discover_sessions(&sessions_dir, args.filter.as_deref(), args.limit)?;

    if session_files.is_empty() {
        println!("No session files found in {}", sessions_dir);
        return Ok(());
    }

    let mut trajectories = Vec::new();
    let mut errors = 0;

    for path in &session_files {
        match parse_session_file(path) {
            Ok(t) => trajectories.push(t),
            Err(e) => {
                tracing::warn!("Failed to parse {}: {}", path.display(), e);
                errors += 1;
            }
        }
    }

    let report = format!(
        "Ingested {} trajectories from {} files ({} errors)",
        trajectories.len(),
        session_files.len(),
        errors
    );
    tracing::info!("{}", report);

    if json_mode {
        let json = serde_json::to_string_pretty(&trajectories)
            .map_err(|e| EngramError::Serialization(e))?;
        if output_path == "-" {
            println!("{}", json);
        } else {
            fs::write(output_path, json)
                .map_err(|e| EngramError::Io(e))?;
            println!("{}", report);
        }
    } else {
        println!("{}", report);
    }

    Ok(())
}

/// Discover session files in the given directory
pub fn discover_sessions(
    dir: &str,
    filter: Option<&str>,
    limit: usize,
) -> Result<Vec<PathBuf>, EngramError> {
    let path = Path::new(dir);
    if !path.exists() {
        return Err(EngramError::NotFound(format!(
            "Sessions directory not found: {}",
            dir
        )));
    }

    let mut files: Vec<PathBuf> = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file()
                && e.path().extension().map_or(false, |ext| ext == "jsonl")
        })
        .map(|e| e.into_path())
        .collect();

    // Sort by modification time (newest first)
    files.sort_by(|a, b| {
        let ta = fs::metadata(a).and_then(|m| m.modified()).ok();
        let tb = fs::metadata(b).and_then(|m| m.modified()).ok();
        tb.cmp(&ta)
    });

    // Apply filter
    if let Some(pattern) = filter {
        files.retain(|f| {
            let name = f.to_string_lossy().to_lowercase();
            name.contains(&pattern.to_lowercase())
        });
    }

    files.truncate(limit);
    Ok(files)
}

/// Parse a single session JSONL file into a Trajectory
pub fn parse_session_file(path: &Path) -> Result<Trajectory, EngramError> {
    let content = fs::read_to_string(path)
        .map_err(|e| EngramError::Io(e))?;

    let lines: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();
    if lines.is_empty() {
        return Err(EngramError::Deserialization("Empty session file".into()));
    }

    // Parse header
    let header: serde_json::Value = serde_json::from_str(lines[0])
        .map_err(|e| EngramError::Serialization(e))?;

    if header["type"].as_str() != Some("session") {
        return Err(EngramError::Deserialization(
            "First line is not a session header".into(),
        ));
    }

    let session_id = header["id"].as_str().unwrap_or("unknown").to_string();
    let cwd = header["cwd"].as_str().unwrap_or("").to_string();
    let created_at_str = header["timestamp"].as_str().unwrap_or("");
    let created_at = DateTime::parse_from_rfc3339(created_at_str)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now());

    let mut model = String::new();
    let mut provider = String::new();
    let mut turns: Vec<Turn> = Vec::new();
    let mut current_turn_tool_calls: Vec<ToolCall> = Vec::new();
    let mut current_turn_tool_results: Vec<ToolResult> = Vec::new();
    let mut turn_index = 0usize;
    let mut task_description: Option<String> = None;
    let mut total_tokens: u64 = 0;

    // Walk entries, grouping into turns
    for line in &lines[1..] {
        let entry: serde_json::Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(_) => continue, // Skip malformed lines
        };

        let entry_type = entry["type"].as_str().unwrap_or("");

        match entry_type {
            "model_change" => {
                provider = entry["provider"].as_str().unwrap_or("").to_string();
                model = entry["modelId"].as_str().unwrap_or("").to_string();
            }
            "message" => {
                let msg = &entry["message"];
                let role = msg["role"].as_str().unwrap_or("");

                match role {
                    "user" => {
                        // Flush any pending turn
                        if !current_turn_tool_calls.is_empty()
                            || !current_turn_tool_results.is_empty()
                        {
                            turns.push(Turn {
                                index: turn_index,
                                user_message: None,
                                assistant_thinking: None,
                                assistant_text: None,
                                tool_calls: std::mem::take(&mut current_turn_tool_calls),
                                tool_results: std::mem::take(&mut current_turn_tool_results),
                                timestamp: msg["timestamp"].as_i64().unwrap_or(0),
                                stopped_reason: StopReason::Unknown,
                            });
                            turn_index += 1;
                        }

                        let text = extract_text_content(msg["content"].clone());
                        if task_description.is_none() {
                            task_description = Some(text.clone());
                        }
                        // Don't create a turn for user message alone - will be paired with assistant
                    }
                    "assistant" => {
                        // Parse assistant message content blocks
                        let mut thinking = None;
                        let mut text = None;
                        let mut tool_calls = Vec::new();
                        let mut stopped_reason = StopReason::Unknown;

                        if let Some(reason) = msg["stopReason"].as_str() {
                            stopped_reason = match reason {
                                "stop" => StopReason::Stop,
                                "length" => StopReason::Length,
                                "toolUse" => StopReason::ToolUse,
                                "error" => StopReason::Error,
                                "aborted" => StopReason::Aborted,
                                _ => StopReason::Unknown,
                            };
                        }

                        if let Some(blocks) = msg["content"].as_array() {
                            for block in blocks {
                                let block_type = block["type"].as_str().unwrap_or("");
                                match block_type {
                                    "thinking" => {
                                        thinking = block["thinking"].as_str().map(String::from);
                                    }
                                    "text" => {
                                        text = block["text"].as_str().map(String::from);
                                    }
                                    "toolCall" => {
                                        tool_calls.push(ToolCall {
                                            id: block["id"]
                                                .as_str()
                                                .unwrap_or("")
                                                .to_string(),
                                            name: block["name"]
                                                .as_str()
                                                .unwrap_or("")
                                                .to_string(),
                                            arguments: block["arguments"].clone(),
                                        });
                                    }
                                    _ => {}
                                }
                            }
                        }

                        // Accumulate token usage
                        if let Some(usage) = msg.get("usage") {
                            total_tokens += usage["totalTokens"].as_u64().unwrap_or(0);
                        }

                        current_turn_tool_calls = tool_calls;

                        // If stopped for a reason other than tool use, flush the turn
                        if !matches!(stopped_reason, StopReason::ToolUse) {
                            turns.push(Turn {
                                index: turn_index,
                                user_message: None,
                                assistant_thinking: thinking,
                                assistant_text: text,
                                tool_calls: std::mem::take(&mut current_turn_tool_calls),
                                tool_results: std::mem::take(&mut current_turn_tool_results),
                                timestamp: msg["timestamp"].as_i64().unwrap_or(0),
                                stopped_reason,
                            });
                            turn_index += 1;
                        } else {
                            // Store thinking/text for when tool results come back
                            // We'll add them when flushing after tool results
                            turns.push(Turn {
                                index: turn_index,
                                user_message: None,
                                assistant_thinking: thinking,
                                assistant_text: text,
                                tool_calls: std::mem::take(&mut current_turn_tool_calls),
                                tool_results: Vec::new(), // Will be filled by subsequent toolResult messages
                                timestamp: msg["timestamp"].as_i64().unwrap_or(0),
                                stopped_reason,
                            });
                            turn_index += 1;
                        }
                    }
                    "toolResult" => {
                        let result = ToolResult {
                            tool_call_id: msg["toolCallId"]
                                .as_str()
                                .unwrap_or("")
                                .to_string(),
                            tool_name: msg["toolName"]
                                .as_str()
                                .unwrap_or("")
                                .to_string(),
                            content: extract_text_content(msg["content"].clone()),
                            is_error: msg["isError"].as_bool().unwrap_or(false),
                            exit_code: None, // Will be extracted from bashExecution messages
                            truncated: false,
                        };
                        current_turn_tool_results.push(result);

                        // If this is the last tool result for a toolUse turn, attach them
                        // We detect this by checking if the next message is not a toolResult
                        // For simplicity, we'll attach all accumulated results to the last turn
                        if let Some(last_turn) = turns.last_mut() {
                            if matches!(last_turn.stopped_reason, StopReason::ToolUse) {
                                last_turn.tool_results =
                                    std::mem::take(&mut current_turn_tool_results);
                            }
                        }
                    }
                    _ => {} // Skip bashExecution, custom, etc. for now
                }
            }
            _ => {} // Skip other entry types
        }
    }

    // Flush any remaining
    if !current_turn_tool_results.is_empty() {
        if let Some(last_turn) = turns.last_mut() {
            if last_turn.tool_results.is_empty() {
                last_turn.tool_results = current_turn_tool_results;
            }
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

/// Extract text content from a message content field (string or array of blocks)
fn extract_text_content(content: serde_json::Value) -> String {
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