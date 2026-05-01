//! Replay Harness: Write patches to engram, spawn pi, capture new Trajectory.
//!
//! The replay flow:
//! 1. Write MemoryPatch entities to engram storage (in-process)
//! 2. Spawn pi --mode json with the original task
//! 3. Capture the event stream into a new Trajectory
//! 4. Return the new Trajectory for re-evaluation
//!
//! Key insight: the existing engram-session.ts pi extension automatically
//! loads engram entities into agent context at session start. Writing patches
//! to engram storage is all that's needed — no custom injection code required.

use crate::error::EngramError;
use crate::evo::capture::{self, CaptureConfig};
use crate::evo::cli::ReplayArgs;
use crate::evo::types::*;
use std::fs;
use std::io::{self, Read as IoRead};

/// Handle the replay subcommand
pub fn handle_replay(args: ReplayArgs) -> Result<(), EngramError> {
    // Read patch
    let patch_json = if args.patch == "-" {
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .map_err(|e| EngramError::Io(e))?;
        buf
    } else {
        fs::read_to_string(&args.patch).map_err(|e| EngramError::Io(e))?
    };

    let patches: Vec<MemoryPatch> = if patch_json.trim_start().starts_with('[') {
        serde_json::from_str(&patch_json).map_err(|e| EngramError::Serialization(e))?
    } else {
        vec![serde_json::from_str(&patch_json).map_err(|e| EngramError::Serialization(e))?]
    };

    let mut all_trajectories = Vec::new();

    for patch in &patches {
        let trajectory = replay_with_patch(patch, &args.task, args.model.as_deref())?;
        all_trajectories.push(trajectory);
    }

    // Output
    let output_json = serde_json::to_string_pretty(&all_trajectories)
        .map_err(|e| EngramError::Serialization(e))?;

    if args.output == "-" {
        println!("{}", output_json);
    } else {
        fs::write(&args.output, &output_json).map_err(|e| EngramError::Io(e))?;
        eprintln!(
            "Wrote {} replay trajectory(ies) to {}",
            all_trajectories.len(),
            args.output
        );
    }

    Ok(())
}

/// Replay a task with a patch injected via engram.
///
/// 1. Writes patch entities to engram storage
/// 2. Creates temp directory for replay session
/// 3. Spawns pi --mode json with the task
/// 4. Captures the resulting Trajectory
/// 5. Returns the new Trajectory
pub fn replay_with_patch(
    patch: &MemoryPatch,
    task: &str,
    model: Option<&str>,
) -> Result<Trajectory, EngramError> {
    // Step 1: Write patch entities to engram
    let entity_ids = write_patch_to_engram(patch)?;
    tracing::info!("Wrote {} entities to engram for replay", entity_ids.len());

    // Step 2: Create temp session directory
    let session_dir = create_temp_dir()?;
    tracing::info!("Replay session dir: {:?}", session_dir);

    // Step 3: Capture pi session
    let config = CaptureConfig {
        prompt: task.to_string(),
        cwd: None,
        model: model.map(String::from),
        session_dir: Some(session_dir.to_string_lossy().to_string()),
        timeout_secs: 600,
    };

    let result = capture_pi_session_with_retry(&config)?;

    tracing::info!(
        "Replay complete: {} turns, exit_code={:?}",
        result.trajectory.turns.len(),
        result.exit_code
    );

    Ok(result.trajectory)
}

/// Write patch entities to engram storage.
///
/// Uses the engram CLI to create entities. This is a deliberate choice
/// over direct storage access because:
/// - The CLI handles all validation and relationship setup
/// - The engram-session.ts extension will auto-load these at replay time
/// - No need to understand the internal storage API
fn write_patch_to_engram(patch: &MemoryPatch) -> Result<Vec<String>, EngramError> {
    let mut ids = Vec::new();

    for entity in &patch.entities {
        let id = match entity.entity_type {
            PatchEntityType::Knowledge => {
                let knowledge_type = entity
                    .knowledge_type
                    .as_ref()
                    .map(|kt| format!("{:?}", kt).to_lowercase())
                    .unwrap_or_else(|| "heuristic".to_string());

                create_knowledge(
                    &entity.title,
                    &entity.content,
                    &knowledge_type,
                    entity.confidence,
                    &entity.tags,
                )?
            }
            PatchEntityType::Lesson => create_lesson(&entity.title, &entity.content, &entity.tags)?,
            PatchEntityType::Context => {
                create_context(&entity.title, &entity.content, &entity.tags)?
            }
            PatchEntityType::Reasoning => {
                // Create as context since standalone reasoning needs a task
                create_context(&entity.title, &entity.content, &entity.tags)?
            }
        };
        ids.push(id);
    }

    Ok(ids)
}

/// Create a knowledge entity via engram CLI
fn create_knowledge(
    title: &str,
    content: &str,
    knowledge_type: &str,
    confidence: Option<f64>,
    tags: &[String],
) -> Result<String, EngramError> {
    let tags_str = tags.join(",");
    let conf = confidence.unwrap_or(0.85);

    let output = run_engram_cli(&[
        "knowledge",
        "create",
        "--title",
        title,
        "--content",
        content,
        "--knowledge-type",
        knowledge_type,
        "--confidence",
        &conf.to_string(),
        "--tags",
        &tags_str,
        "--json",
    ])?;

    // Parse UUID from output
    parse_uuid_from_output(&output)
}

/// Create a lesson entity via engram CLI
fn create_lesson(title: &str, content: &str, tags: &[String]) -> Result<String, EngramError> {
    let tags_str = tags.join(",");

    let output = run_engram_cli(&[
        "lesson",
        "create",
        "--title",
        title,
        "--content",
        content,
        "--tags",
        &tags_str,
        "--json",
    ])?;

    parse_uuid_from_output(&output)
}

/// Create a context entity via engram CLI
fn create_context(title: &str, content: &str, tags: &[String]) -> Result<String, EngramError> {
    let tags_str = tags.join(",");

    let output = run_engram_cli(&[
        "context",
        "create",
        "--title",
        title,
        "--content",
        content,
        "--tags",
        &tags_str,
        "--json",
    ])?;

    parse_uuid_from_output(&output)
}

/// Run engram CLI command
fn run_engram_cli(args: &[&str]) -> Result<String, EngramError> {
    use std::process::Command;

    let output = Command::new("engram")
        .args(args)
        .output()
        .map_err(|e| EngramError::InvalidOperation(format!("Failed to run engram CLI: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(EngramError::InvalidOperation(format!(
            "engram CLI failed: {}",
            stderr
        )));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Parse a UUID from engram CLI output
fn parse_uuid_from_output(output: &str) -> Result<String, EngramError> {
    // Engram outputs the UUID in various formats depending on --json flag
    // Try JSON first
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(output) {
        if let Some(id) = json["id"].as_str() {
            return Ok(id.to_string());
        }
    }

    // Try finding UUID pattern in output
    let re = regex::Regex::new(r"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}")
        .map_err(|e| EngramError::InvalidOperation(format!("Regex error: {}", e)))?;

    if let Some(caps) = re.find(output) {
        return Ok(caps.as_str().to_string());
    }

    // Return the raw output as fallback
    tracing::warn!("Could not parse UUID from engram output: {}", output);
    Ok(output.to_string())
}

/// Capture pi session with retry logic
fn capture_pi_session_with_retry(
    config: &CaptureConfig,
) -> Result<capture::CaptureResult, EngramError> {
    capture::capture_pi_session(config)
}

/// Create a temporary directory for replay sessions
fn create_temp_dir() -> Result<std::path::PathBuf, EngramError> {
    let base = std::env::temp_dir().join("engram-evo-replay");
    let unique = format!("{}", chrono::Utc::now().timestamp_millis());
    let dir = base.join(&unique);
    fs::create_dir_all(&dir).map_err(|e| EngramError::Io(e))?;
    Ok(dir)
}
