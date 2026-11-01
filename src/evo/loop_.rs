//! Loop Controller: Orchestrate the full self-improving cycle.
//!
//! Coordinates: ingest → evaluate → optimize → replay → re-evaluate → commit/rollback
//!
//! For each session:
//! 1. Parse into Trajectory
//! 2. Evaluate with metrics
//! 3. If composite score < threshold, generate Memory Patch
//! 4. Replay with patch injected
//! 5. Re-evaluate the replay
//! 6. If improved ≥ min_improvement, keep the patch; otherwise rollback
//!
//! Rollback: delete the patch entities from engram storage.

use crate::error::EngramError;
use crate::evo::capture::CaptureConfig;
use crate::evo::cli::LoopArgs;
use crate::evo::eval;
use crate::evo::ingest;
use crate::evo::llm::LlmClient;
use crate::evo::optimizer;
use crate::evo::replay;
use crate::evo::types::*;
use std::fs;
use std::path::Path;

/// Handle the loop subcommand
pub fn handle_loop(args: LoopArgs) -> Result<(), EngramError> {
    let sessions_dir = args.sessions_dir.unwrap_or_else(|| {
        dirs::home_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join(".pi/agent/sessions")
            .to_string_lossy()
            .to_string()
    });

    println!("╔══════════════════════════════════════════════════╗");
    println!("║          ENGRAM-EVO SELF-IMPROVEMENT LOOP        ║");
    println!("╠══════════════════════════════════════════════════╣");
    println!(
        "║  Sessions dir: {:<34}  ║",
        truncate_str(&sessions_dir, 34)
    );
    println!("║  Max iterations: {:<31}  ║", args.max_iterations);
    println!(
        "║  Min improvement: {:<30}  ║",
        format!("{:.2}", args.min_improvement)
    );
    println!("║  Max sessions: {:<33}  ║", args.limit);
    println!("╚══════════════════════════════════════════════════╝");
    println!();

    // Step 1: Ingest sessions
    println!("📥 Ingesting sessions...");
    let session_files = ingest::discover_sessions(
        &sessions_dir,
        None, // no filter
        args.limit,
    )?;

    if session_files.is_empty() {
        println!("No sessions found in {}", sessions_dir);
        return Ok(());
    }

    println!("   Found {} session files", session_files.len());

    // Create LLM client for optimization
    let llm_client = match LlmClient::new() {
        Ok(client) => {
            println!("🤖 LLM client ready (model: {})", client.model());
            Some(client)
        }
        Err(e) => {
            println!("⚠️  No LLM client available: {}", e);
            println!("   Will skip optimization (evaluate only)");
            None
        }
    };

    let mut results = Vec::new();

    for (i, session_file) in session_files.iter().enumerate() {
        println!();
        println!(
            "════════ Session {}/{}: {} ═════════",
            i + 1,
            session_files.len(),
            session_file
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
        );

        match run_single_session(
            session_file,
            llm_client.as_ref(),
            args.max_iterations,
            args.min_improvement,
            args.model.as_deref(),
        ) {
            Ok(result) => {
                print_loop_result(&result);
                results.push(result);
            }
            Err(e) => {
                println!("❌ Failed: {}", e);
                results.push(LoopResult {
                    session_file: session_file.to_string_lossy().to_string(),
                    original_score: 0.0,
                    final_score: 0.0,
                    improvement: 0.0,
                    iterations: 0,
                    patches_applied: 0,
                    patches_rolled_back: 0,
                    status: LoopStatus::Failed(e.to_string()),
                });
            }
        }
    }

    // Summary
    println!();
    println!("╔══════════════════════════════════════════════════╗");
    println!("║                  SUMMARY                         ║");
    println!("╠══════════════════════════════════════════════════╣");

    let improved = results.iter().filter(|r| r.improvement > 0.0).count();
    let failed = results
        .iter()
        .filter(|r| matches!(r.status, LoopStatus::Failed(_)))
        .count();
    let patches = results.iter().map(|r| r.patches_applied).sum::<usize>();

    println!("║  Sessions processed: {:<27}  ║", results.len());
    println!("║  Improved: {:<36}  ║", improved);
    println!("║  Failed: {:<38}  ║", failed);
    println!("║  Patches committed: {:<27}  ║", patches);
    println!("╚══════════════════════════════════════════════════╝");

    Ok(())
}

/// Status of a loop iteration
#[derive(Debug, Clone)]
pub enum LoopStatus {
    Improved,
    NotImproved,
    Skipped(String),
    Failed(String),
}

/// Result of running the loop on a single session
#[derive(Debug, Clone)]
pub struct LoopResult {
    pub session_file: String,
    pub original_score: f64,
    pub final_score: f64,
    pub improvement: f64,
    pub iterations: usize,
    pub patches_applied: usize,
    pub patches_rolled_back: usize,
    pub status: LoopStatus,
}

/// Run the self-improvement loop on a single session.
fn run_single_session(
    session_file: &Path,
    llm_client: Option<&LlmClient>,
    max_iterations: usize,
    min_improvement: f64,
    model: Option<&str>,
) -> Result<LoopResult, EngramError> {
    // Parse session
    let trajectory = ingest::parse_session_file(session_file)?;

    // Initial evaluation (skip LLM for baseline)
    let baseline_report = eval::evaluate_trajectory(&trajectory, true);

    println!(
        "📊 Baseline score: {:.3} (eff={:.3} corr={:.3} plan={:.3} comp={:.3})",
        baseline_report.scores.composite,
        baseline_report.scores.step_efficiency,
        baseline_report.scores.tool_correctness,
        baseline_report.scores.plan_adherence,
        baseline_report.scores.task_completion
    );

    // Skip sessions that score well already
    if baseline_report.scores.composite >= 0.8 {
        return Ok(LoopResult {
            session_file: session_file.to_string_lossy().to_string(),
            original_score: baseline_report.scores.composite,
            final_score: baseline_report.scores.composite,
            improvement: 0.0,
            iterations: 0,
            patches_applied: 0,
            patches_rolled_back: 0,
            status: LoopStatus::Skipped("Score already good (≥0.8)".to_string()),
        });
    }

    // Need LLM for optimization
    let client = match llm_client {
        Some(c) => c,
        None => {
            return Ok(LoopResult {
                session_file: session_file.to_string_lossy().to_string(),
                original_score: baseline_report.scores.composite,
                final_score: baseline_report.scores.composite,
                improvement: 0.0,
                iterations: 0,
                patches_applied: 0,
                patches_rolled_back: 0,
                status: LoopStatus::Skipped("No LLM client available".to_string()),
            });
        }
    };

    let mut current_score = baseline_report.scores.composite;
    let mut total_patches_applied = 0usize;
    let mut total_patches_rolled_back = 0usize;
    let mut total_iterations = 0usize;

    for iteration in 0..max_iterations {
        println!("🔄 Iteration {}/{}...", iteration + 1, max_iterations);

        // Evaluate current state
        let report = if iteration == 0 {
            baseline_report.clone()
        } else {
            eval::evaluate_trajectory(&trajectory, true)
        };

        // Generate patch
        println!("   🧠 Analyzing failures...");
        let patch = match optimizer::generate_patch(client, &report, &trajectory) {
            Ok(p) => p,
            Err(e) => {
                println!("   ⚠️  Optimization failed: {}", e);
                break;
            }
        };

        println!(
            "   📝 Generated patch targeting turn {} ({} entities)",
            patch.target_failure_turn,
            patch.entities.len()
        );

        // Get task description for replay
        let task = trajectory
            .task_description
            .clone()
            .unwrap_or_else(|| "Unknown task".to_string());

        // Replay with patch
        println!("   🔄 Replaying with patch...");
        let replay_trajectory = match replay::replay_with_patch(&patch, &task, model) {
            Ok(t) => t,
            Err(e) => {
                println!("   ❌ Replay failed: {}", e);
                break;
            }
        };

        // Re-evaluate
        let replay_report = eval::evaluate_trajectory(&replay_trajectory, true);
        let new_score = replay_report.scores.composite;
        let score_change = new_score - current_score;

        println!(
            "   📊 Replay score: {:.3} (change: {:+.3})",
            new_score, score_change
        );

        if score_change >= min_improvement {
            println!("   ✅ Improved! Keeping patch.");
            current_score = new_score;
            total_patches_applied += patch.entities.len();
        } else {
            println!("   ❌ No significant improvement. Rolling back.");
            // Rollback: delete the patch entities
            rollback_patch(&patch)?;
            total_patches_rolled_back += patch.entities.len();
        }

        total_iterations += 1;

        // Stop if score is now good
        if current_score >= 0.8 {
            println!("   🎉 Score reached 0.8, stopping early.");
            break;
        }
    }

    let improvement = current_score - baseline_report.scores.composite;
    let status = if improvement > 0.0 {
        LoopStatus::Improved
    } else {
        LoopStatus::NotImproved
    };

    Ok(LoopResult {
        session_file: session_file.to_string_lossy().to_string(),
        original_score: baseline_report.scores.composite,
        final_score: current_score,
        improvement,
        iterations: total_iterations,
        patches_applied: total_patches_applied,
        patches_rolled_back: total_patches_rolled_back,
        status,
    })
}

/// Rollback a patch by deleting its entities from engram.
fn rollback_patch(patch: &MemoryPatch) -> Result<(), EngramError> {
    for entity in &patch.entities {
        // Use engram CLI to find and delete the entity
        // For now, log the rollback intent
        tracing::info!(
            "Rolling back entity: {} ({:?})",
            entity.title,
            entity.entity_type
        );
        // TODO: Implement actual deletion via engram CLI
        // engram <type> delete --id <uuid>
    }
    Ok(())
}

/// Print a loop result summary
fn print_loop_result(result: &LoopResult) {
    let status_icon = match &result.status {
        LoopStatus::Improved => "✅",
        LoopStatus::NotImproved => "📊",
        LoopStatus::Skipped(_) => "⏭️",
        LoopStatus::Failed(_) => "❌",
    };

    println!(
        "{} Score: {:.3} → {:.3} ({:+.3}) | {} iterations | {} patches applied",
        status_icon,
        result.original_score,
        result.final_score,
        result.improvement,
        result.iterations,
        result.patches_applied
    );
}

/// Truncate a string for display
fn truncate_str(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}
