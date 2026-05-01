//! Evaluation metrics for trajectory assessment.
//!
//! Four metrics:
//! - **StepEfficiency**: actual tool calls vs heuristic optimal count
//! - **ToolCorrectness**: % of tool results without errors
//! - **PlanAdherence**: how well execution matched planning (LLM-as-judge, optional)
//! - **TaskCompletion**: whether the task reached a successful conclusion
//!
//! Composite score = 0.25 * (step_efficiency + tool_correctness + plan_adherence + task_completion)

use crate::error::EngramError;
use crate::evo::cli::EvaluateArgs;
use crate::evo::types::*;
use chrono::Utc;
use std::fs;
use std::io::{self, Read as IoRead};

/// Handle the evaluate subcommand
pub fn handle_evaluate(args: EvaluateArgs) -> Result<(), EngramError> {
    // Read trajectory from file or stdin
    let trajectory_json = if args.trajectory == "-" {
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .map_err(|e| EngramError::Io(e))?;
        buf
    } else {
        fs::read_to_string(&args.trajectory).map_err(|e| EngramError::Io(e))?
    };

    // Parse — supports both single trajectory and array
    let trajectories: Vec<Trajectory> = if trajectory_json.trim_start().starts_with('[') {
        serde_json::from_str(&trajectory_json).map_err(|e| EngramError::Serialization(e))?
    } else {
        vec![serde_json::from_str(&trajectory_json).map_err(|e| EngramError::Serialization(e))?]
    };

    let mut reports = Vec::new();
    for traj in &trajectories {
        let report = evaluate_trajectory(traj, args.skip_llm);
        reports.push(report);
    }

    // Output
    let output_json =
        serde_json::to_string_pretty(&reports).map_err(|e| EngramError::Serialization(e))?;

    if args.output == "-" {
        println!("{}", output_json);
    } else {
        fs::write(&args.output, &output_json).map_err(|e| EngramError::Io(e))?;
        eprintln!("Wrote {} report(s) to {}", reports.len(), args.output);
    }

    Ok(())
}

/// Evaluate a single trajectory
pub fn evaluate_trajectory(trajectory: &Trajectory, skip_llm: bool) -> EvalReport {
    let step_eff = step_efficiency(trajectory);
    let tool_corr = tool_correctness(trajectory);
    let task_comp = task_completion(trajectory);
    let plan_adh = if skip_llm {
        0.5 // Neutral placeholder when LLM is skipped
    } else {
        match crate::evo::llm::LlmClient::new() {
            Ok(client) => match crate::evo::llm::evaluate_plan_adherence(&client, trajectory) {
                Ok(result) => {
                    tracing::info!(
                        "Plan adherence LLM score: {:.3} - {}",
                        result.score,
                        result.reasoning
                    );
                    result.score
                }
                Err(e) => {
                    tracing::warn!(
                        "LLM plan adherence failed, falling back to heuristic: {}",
                        e
                    );
                    plan_adherence_heuristic(trajectory)
                }
            },
            Err(e) => {
                tracing::warn!("No LLM client available, using heuristic: {}", e);
                plan_adherence_heuristic(trajectory)
            }
        }
    };

    let mut scores = EvalScores {
        step_efficiency: step_eff,
        tool_correctness: tool_corr,
        plan_adherence: plan_adh,
        task_completion: task_comp,
        composite: 0.0,
    };
    scores.compute_composite();

    // Compute per-turn scores
    let turn_scores: Vec<TurnScore> = trajectory
        .turns
        .iter()
        .enumerate()
        .map(|(i, turn)| TurnScore {
            turn_index: i,
            step_efficiency: turn_step_efficiency(turn),
            tool_correctness: turn_tool_correctness(turn),
            is_critical_failure: false, // Will be set below
        })
        .collect();

    // Identify critical failure turn
    let critical_failure_turn = identify_critical_failure(trajectory, &turn_scores);

    // Mark critical failure in turn scores
    let mut turn_scores = turn_scores;
    if let Some(idx) = critical_failure_turn {
        if let Some(ts) = turn_scores.get_mut(idx) {
            ts.is_critical_failure = true;
        }
    }

    // Generate improvement suggestions
    let suggestions = generate_suggestions(trajectory, &scores, critical_failure_turn);

    EvalReport {
        trajectory_id: trajectory.session_id.clone(),
        session_id: trajectory.session_id.clone(),
        scores,
        turn_scores,
        critical_failure_turn,
        improvement_suggestions: suggestions,
        evaluated_at: Utc::now(),
    }
}

/// Step efficiency: ratio of optimal to actual steps.
///
/// Uses heuristics for "optimal" step count:
/// - A simple task (read + edit + test + commit) = 4 steps
/// - Each additional file to read adds 1
/// - Tool calls that repeat the same command with the same args are wasted
///
/// Returns 0.0-1.0 where 1.0 means perfect efficiency.
pub fn step_efficiency(trajectory: &Trajectory) -> f64 {
    let total_tool_calls: usize = trajectory.turns.iter().map(|t| t.tool_calls.len()).sum();

    if total_tool_calls == 0 {
        return 1.0; // No tool calls = nothing to be inefficient about
    }

    let optimal = estimate_optimal_steps(trajectory);
    if optimal == 0 {
        return 0.5; // Can't estimate, give neutral score
    }

    let ratio = optimal as f64 / total_tool_calls as f64;
    ratio.min(1.0) // Cap at 1.0
}

/// Estimate the optimal number of steps for a task.
///
/// Heuristic approach:
/// - Count unique tool names used (diversity is good)
/// - Count unique file paths referenced
/// - Add base overhead for typical task flow
fn estimate_optimal_steps(trajectory: &Trajectory) -> usize {
    let mut unique_tools = std::collections::HashSet::new();
    let mut unique_files = std::collections::HashSet::new();

    for turn in &trajectory.turns {
        for tc in &turn.tool_calls {
            unique_tools.insert(tc.name.clone());

            // Extract file paths from arguments
            if let Some(path) = tc.arguments.get("path").and_then(|v| v.as_str()) {
                unique_files.insert(path.to_string());
            }
            if let Some(path) = tc.arguments.get("command").and_then(|v| v.as_str()) {
                // Count files mentioned in bash commands
                if path.contains("cargo") || path.contains("rustc") {
                    unique_tools.insert("build".to_string());
                }
            }
        }
    }

    // Base: read each file once + edit needed files + verify (test/build) + commit
    let reads = unique_files.len().max(1);
    let base_flow = reads + 2; // +2 for verify + commit

    base_flow.max(2) // Minimum 2 steps
}

/// Tool correctness: percentage of tool results without errors.
///
/// Returns 0.0-1.0 where 1.0 means all tools succeeded.
pub fn tool_correctness(trajectory: &Trajectory) -> f64 {
    let tool_results: Vec<&ToolResult> = trajectory
        .turns
        .iter()
        .flat_map(|t| t.tool_results.iter())
        .collect();

    if tool_results.is_empty() {
        return 1.0; // No tool results = nothing to be incorrect about
    }

    let successful = tool_results.iter().filter(|r| !r.is_error).count();
    successful as f64 / tool_results.len() as f64
}

/// Task completion: heuristic assessment of whether the task completed.
///
/// Signals of completion:
/// - Final assistant message contains success indicators
/// - No error stop reasons
/// - Session has multiple turns (agent did work)
/// - Final turn has assistant text (not just tool calls)
pub fn task_completion(trajectory: &Trajectory) -> f64 {
    if trajectory.turns.is_empty() {
        return 0.0;
    }

    let mut score = 0.0;

    // Did the session have multiple turns? (agent did substantive work)
    if trajectory.turns.len() >= 2 {
        score += 0.2;
    }

    // Did the final turn have assistant text (agent summarized)?
    if let Some(last_turn) = trajectory.turns.last() {
        if last_turn.assistant_text.is_some() {
            score += 0.3;
        }

        // Did it stop cleanly (not error/aborted)?
        match last_turn.stopped_reason {
            StopReason::Stop => score += 0.3,
            StopReason::ToolUse => score += 0.1, // Might be mid-work
            StopReason::Length => score += 0.0,  // Hit limit = probably incomplete
            StopReason::Error => score += 0.0,
            StopReason::Aborted => score += 0.0,
            StopReason::Unknown => score += 0.1,
        }

        // Check for success indicators in final text
        if let Some(text) = &last_turn.assistant_text {
            let lower = text.to_lowercase();
            if lower.contains("done")
                || lower.contains("complete")
                || lower.contains("finished")
                || lower.contains("success")
                || lower.contains("created")
                || lower.contains("implemented")
                || lower.contains("fixed")
                || lower.contains("resolved")
            {
                score += 0.2;
            }
        }
    }

    // Penalize if any turn had errors
    let error_count: usize = trajectory
        .turns
        .iter()
        .flat_map(|t| t.tool_results.iter())
        .filter(|r| r.is_error)
        .count();

    if error_count > 0 {
        score -= 0.1 * error_count.min(5) as f64;
    }

    score.clamp(0.0, 1.0)
}

/// Plan adherence heuristic (non-LLM version).
///
/// Checks:
/// - Does the agent's thinking correlate with its tool calls?
/// - Did the agent revisit topics it claimed to be done with?
/// - Did tool call sequences follow a logical pattern?
fn plan_adherence_heuristic(trajectory: &Trajectory) -> f64 {
    if trajectory.turns.is_empty() {
        return 0.5;
    }

    let mut score = 0.5; // Start neutral

    // Check: agent had thinking in most turns (was planning)
    let turns_with_thinking = trajectory
        .turns
        .iter()
        .filter(|t| t.assistant_thinking.is_some())
        .count();
    let thinking_ratio = turns_with_thinking as f64 / trajectory.turns.len() as f64;
    score += 0.2 * thinking_ratio;

    // Check: no excessive retry loops (same tool + similar args repeatedly)
    let retry_penalty = detect_retry_loops(trajectory);
    score -= 0.3 * retry_penalty;

    // Check: tool usage followed a logical sequence
    // (read before edit, build after edit, etc.)
    let sequence_score = detect_logical_sequence(trajectory);
    score += 0.3 * sequence_score;

    score.clamp(0.0, 1.0)
}

/// Detect retry loops: consecutive turns with the same tool and similar args.
fn detect_retry_loops(trajectory: &Trajectory) -> f64 {
    let mut retry_count = 0usize;
    let mut consecutive_errors = 0usize;

    for turn in &trajectory.turns {
        let has_error = turn.tool_results.iter().any(|r| r.is_error);
        if has_error {
            consecutive_errors += 1;
            if consecutive_errors > 2 {
                retry_count += 1;
            }
        } else {
            consecutive_errors = 0;
        }
    }

    // Normalize: 0 retries = 0.0, many retries approaches 1.0
    (retry_count as f64 / (trajectory.turns.len().max(1) as f64)).min(1.0)
}

/// Detect if tool usage follows a logical sequence.
///
/// Good patterns: read → edit → bash(test) → commit
/// Bad patterns: edit → read → edit (editing before reading)
fn detect_logical_sequence(trajectory: &Trajectory) -> f64 {
    let tool_sequence: Vec<&str> = trajectory
        .turns
        .iter()
        .flat_map(|t| t.tool_calls.iter().map(|tc| tc.name.as_str()))
        .collect();

    if tool_sequence.len() < 2 {
        return 0.5; // Not enough data
    }

    let mut good_patterns = 0usize;
    let mut bad_patterns = 0usize;

    for window in tool_sequence.windows(2) {
        match (window[0], window[1]) {
            // Good: read before edit
            ("read", "edit") | ("read", "write") => good_patterns += 1,
            // Good: edit before bash (test/build)
            ("edit", "bash") | ("write", "bash") => good_patterns += 1,
            // Good: bash before read (explore then read specific file)
            ("bash", "read") => good_patterns += 1,
            // Bad: edit before read
            ("edit", "read") | ("write", "read") => bad_patterns += 1,
            _ => {}
        }
    }

    let total = good_patterns + bad_patterns;
    if total == 0 {
        return 0.5;
    }

    good_patterns as f64 / total as f64
}

/// Per-turn step efficiency
fn turn_step_efficiency(turn: &Turn) -> f64 {
    if turn.tool_calls.is_empty() {
        return 1.0;
    }
    // Simple: 1 tool call = perfect, more = decreasing
    let count = turn.tool_calls.len();
    if count <= 2 {
        1.0
    } else {
        2.0 / count as f64
    }
}

/// Per-turn tool correctness
fn turn_tool_correctness(turn: &Turn) -> f64 {
    if turn.tool_results.is_empty() {
        return 1.0;
    }
    let successful = turn.tool_results.iter().filter(|r| !r.is_error).count();
    successful as f64 / turn.tool_results.len() as f64
}

/// Identify the critical failure turn (the one most responsible for poor score).
///
/// Priority: first turn with errors, or the turn with lowest combined score.
fn identify_critical_failure(trajectory: &Trajectory, turn_scores: &[TurnScore]) -> Option<usize> {
    // First: find first turn with tool errors
    for (i, turn) in trajectory.turns.iter().enumerate() {
        if turn.tool_results.iter().any(|r| r.is_error) {
            return Some(i);
        }
    }

    // Second: find turn with lowest combined score
    turn_scores
        .iter()
        .enumerate()
        .filter(|(_, ts)| ts.step_efficiency < 0.5 || ts.tool_correctness < 0.5)
        .map(|(i, ts)| (i, ts.step_efficiency + ts.tool_correctness))
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(i, _)| i)
}

/// Generate improvement suggestions based on evaluation results.
fn generate_suggestions(
    trajectory: &Trajectory,
    scores: &EvalScores,
    critical_turn: Option<usize>,
) -> Vec<String> {
    let mut suggestions = Vec::new();

    if scores.step_efficiency < 0.5 {
        suggestions.push(
            "Low step efficiency: agent used more tool calls than necessary. Consider adding knowledge about efficient workflows.".to_string()
        );
    }

    if scores.tool_correctness < 0.7 {
        let error_tools: Vec<String> = trajectory
            .turns
            .iter()
            .flat_map(|t| t.tool_results.iter())
            .filter(|r| r.is_error)
            .map(|r| r.tool_name.clone())
            .collect();
        let unique_errors: std::collections::HashSet<_> = error_tools.into_iter().collect();
        suggestions.push(format!(
            "Tool errors detected in: {:?}. Consider adding knowledge about correct usage of these tools.",
            unique_errors
        ));
    }

    if scores.task_completion < 0.5 {
        suggestions.push(
            "Task appears incomplete. Agent may need clearer task instructions or better completion detection.".to_string()
        );
    }

    if scores.plan_adherence < 0.5 {
        suggestions.push(
            "Plan adherence is low. Agent may be acting without sufficient planning. Consider adding reasoning guidelines.".to_string()
        );
    }

    if let Some(idx) = critical_turn {
        if let Some(turn) = trajectory.turns.get(idx) {
            let error_results: Vec<&ToolResult> =
                turn.tool_results.iter().filter(|r| r.is_error).collect();
            if !error_results.is_empty() {
                suggestions.push(format!(
                    "Critical failure at turn {}: {} error(s) in tool calls ({})",
                    idx,
                    error_results.len(),
                    error_results
                        .iter()
                        .map(|r| r.tool_name.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
        }
    }

    // Check for retry loops
    let consecutive_errors: usize = trajectory
        .turns
        .iter()
        .flat_map(|t| t.tool_results.iter())
        .filter(|r| r.is_error)
        .count();

    if consecutive_errors > 3 {
        suggestions.push(format!(
            "Agent encountered {} tool errors total. May indicate a missing knowledge entry about a tool limitation or error recovery pattern.",
            consecutive_errors
        ));
    }

    if suggestions.is_empty() {
        suggestions.push(
            "No major issues detected. Trajectory scored well across all metrics.".to_string(),
        );
    }

    suggestions
}
