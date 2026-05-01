//! Memory Optimizer: Generate Memory Patches from evaluation reports.
//!
//! Takes an EvalReport + Trajectory, identifies the critical failure,
//! and uses an LLM to analyze what went wrong and generate a MemoryPatch
//! (a specification for engram entities to write).

use crate::error::EngramError;
use crate::evo::cli::OptimizeArgs;
use crate::evo::llm::LlmClient;
use crate::evo::types::*;
use std::fs;
use std::io::{self, Read as IoRead};

/// Handle the optimize subcommand
pub fn handle_optimize(args: OptimizeArgs) -> Result<(), EngramError> {
    // Read eval report
    let report_json = if args.eval_report == "-" {
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .map_err(|e| EngramError::Io(e))?;
        buf
    } else {
        fs::read_to_string(&args.eval_report).map_err(|e| EngramError::Io(e))?
    };

    let reports: Vec<EvalReport> = if report_json.trim_start().starts_with('[') {
        serde_json::from_str(&report_json).map_err(|e| EngramError::Serialization(e))?
    } else {
        vec![serde_json::from_str(&report_json).map_err(|e| EngramError::Serialization(e))?]
    };

    // Read trajectory
    let traj_json = if args.trajectory == "-" {
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .map_err(|e| EngramError::Io(e))?;
        buf
    } else {
        fs::read_to_string(&args.trajectory).map_err(|e| EngramError::Io(e))?
    };

    let trajectories: Vec<Trajectory> = if traj_json.trim_start().starts_with('[') {
        serde_json::from_str(&traj_json).map_err(|e| EngramError::Serialization(e))?
    } else {
        vec![serde_json::from_str(&traj_json).map_err(|e| EngramError::Serialization(e))?]
    };

    // Create LLM client
    let client = LlmClient::new()?;

    let mut patches = Vec::new();
    for report in &reports {
        // Find matching trajectory
        let trajectory = trajectories
            .iter()
            .find(|t| t.session_id == report.session_id)
            .ok_or_else(|| {
                EngramError::NotFound(format!(
                    "No trajectory found for session {}",
                    report.session_id
                ))
            })?;

        let patch = generate_patch(&client, report, trajectory)?;
        patches.push(patch);
    }

    // Output
    let output_json =
        serde_json::to_string_pretty(&patches).map_err(|e| EngramError::Serialization(e))?;

    if args.output == "-" {
        println!("{}", output_json);
    } else {
        fs::write(&args.output, &output_json).map_err(|e| EngramError::Io(e))?;
        eprintln!("Wrote {} patch(es) to {}", patches.len(), args.output);
    }

    Ok(())
}

/// Generate a memory patch from an evaluation report and trajectory.
///
/// Uses an LLM to:
/// 1. Identify the critical failure turn
/// 2. Analyze what went wrong
/// 3. Generate specific engram entities that would prevent the failure
pub fn generate_patch(
    client: &LlmClient,
    report: &EvalReport,
    trajectory: &Trajectory,
) -> Result<MemoryPatch, EngramError> {
    let system_prompt = r#"You are an expert at analyzing AI coding agent failures and generating targeted knowledge entries that would prevent similar failures.

Given an evaluation report and trajectory context, you must:
1. Identify the root cause of the failure (not just the symptom)
2. Generate 1-3 engram entities that would prevent this failure if available to the agent
3. Each entity should be specific and actionable

Entity types available:
- knowledge (with type: fact, pattern, rule, concept, procedure, heuristic, skill, technique)
- lesson (what went wrong, the fix, and prevention rule)
- context (background information the agent was missing)

Respond in JSON format:
{
  "target_failure_turn": <int>,
  "rationale": "<explanation of root cause>",
  "entities": [
    {
      "entity_type": "knowledge|lesson|context",
      "title": "<concise title>",
      "content": "<detailed content the agent needs>",
      "knowledge_type": "rule|heuristic|procedure|fact|pattern|technique",
      "tags": ["relevant", "tags"],
      "confidence": 0.9
    }
  ]
}

Rules:
- Be specific, not generic. "When cargo build fails with linking errors, check if the target is installed via rustup" not "Build errors are bad"
- Focus on the root cause, not symptoms
- Each entity should be independently useful (the agent may only see one)
- Titles should be searchable and descriptive
- Confidence should reflect how certain you are this would help (0.7-0.95)"#;

    // Build context for the LLM
    let mut context = String::new();

    // Evaluation scores
    context.push_str("## Evaluation Report\n\n");
    context.push_str(&format!(
        "Composite Score: {:.3}\n",
        report.scores.composite
    ));
    context.push_str(&format!(
        "Step Efficiency: {:.3} | Tool Correctness: {:.3} | Plan Adherence: {:.3} | Task Completion: {:.3}\n",
        report.scores.step_efficiency,
        report.scores.tool_correctness,
        report.scores.plan_adherence,
        report.scores.task_completion
    ));

    if let Some(idx) = report.critical_failure_turn {
        context.push_str(&format!("\nCritical Failure Turn: {}\n", idx));
    }

    context.push_str("\n### Suggestions from Heuristic Evaluation\n");
    for s in &report.improvement_suggestions {
        context.push_str(&format!("- {}\n", s));
    }

    // Trajectory context around the failure
    context.push_str("\n## Trajectory Context\n\n");
    context.push_str(&format!(
        "Task: {}\n",
        trajectory.task_description.as_deref().unwrap_or("Unknown")
    ));
    context.push_str(&format!(
        "Model: {} | Provider: {} | Total Turns: {}\n",
        trajectory.model,
        trajectory.provider,
        trajectory.turns.len()
    ));

    // Include turns around the failure point
    let failure_idx = report.critical_failure_turn.unwrap_or(0);
    let start = failure_idx.saturating_sub(2);
    let end = (failure_idx + 3).min(trajectory.turns.len());

    context.push_str(&format!(
        "\n### Turns {}-{} (around failure point):\n\n",
        start,
        end - 1
    ));

    for turn in &trajectory.turns[start..end] {
        context.push_str(&format!(
            "**Turn {}** (stop: {:?})\n",
            turn.index, turn.stopped_reason
        ));

        if let Some(thinking) = &turn.assistant_thinking {
            let truncated = if thinking.len() > 500 {
                format!("{}...", &thinking[..500])
            } else {
                thinking.clone()
            };
            context.push_str(&format!("  Agent thinking: {}\n", truncated));
        }

        if let Some(text) = &turn.assistant_text {
            let truncated = if text.len() > 300 {
                format!("{}...", &text[..300])
            } else {
                text.clone()
            };
            context.push_str(&format!("  Agent said: {}\n", truncated));
        }

        for tc in &turn.tool_calls {
            let args_str = if tc.arguments.to_string().len() > 200 {
                format!("{}...", &tc.arguments.to_string()[..200])
            } else {
                tc.arguments.to_string()
            };
            context.push_str(&format!("  Tool call: {}({})\n", tc.name, args_str));
        }

        for tr in &turn.tool_results {
            let status = if tr.is_error { "ERROR" } else { "OK" };
            let content_preview = if tr.content.len() > 300 {
                format!("{}...", &tr.content[..300])
            } else {
                tr.content.clone()
            };
            context.push_str(&format!(
                "  Tool result [{}]: {} → {}\n",
                status, tr.tool_name, content_preview
            ));
        }
        context.push('\n');
    }

    let result: MemoryPatch = client.complete_json(system_prompt, &context)?;

    Ok(result)
}
