//! Report formatting for evaluation reports.
//!
//! Supports two output formats:
//! - **text**: Human-readable summary with scores and suggestions
//! - **json**: Raw JSON output (default evaluation output)

use crate::error::EngramError;
use crate::evo::cli::ReportArgs;
use crate::evo::types::EvalReport;
use std::fs;
use std::io::{self, Read as IoRead};

/// Handle the report subcommand
pub fn handle_report(args: ReportArgs) -> Result<(), EngramError> {
    // Read eval report from file or stdin
    let report_json = if args.eval_report == "-" {
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .map_err(|e| EngramError::Io(e))?;
        buf
    } else {
        fs::read_to_string(&args.eval_report).map_err(|e| EngramError::Io(e))?
    };

    // Parse — supports single report or array
    let reports: Vec<EvalReport> = if report_json.trim_start().starts_with('[') {
        serde_json::from_str(&report_json).map_err(|e| EngramError::Serialization(e))?
    } else {
        vec![serde_json::from_str(&report_json).map_err(|e| EngramError::Serialization(e))?]
    };

    match args.format.as_str() {
        "json" => {
            let output = serde_json::to_string_pretty(&reports)
                .map_err(|e| EngramError::Serialization(e))?;
            println!("{}", output);
        }
        "text" | _ => {
            for report in &reports {
                print_text_report(report);
            }
        }
    }

    Ok(())
}

/// Print a human-readable evaluation report
fn print_text_report(report: &EvalReport) {
    println!("╔══════════════════════════════════════════════════╗");
    println!("║          ENGRAM-EVO EVALUATION REPORT            ║");
    println!("╠══════════════════════════════════════════════════╣");
    println!(
        "║  Session: {:<38}  ║",
        truncate_str(&report.session_id, 38)
    );
    println!(
        "║  Evaluated: {:<36}  ║",
        truncate_str(&report.evaluated_at.to_rfc3339(), 36)
    );
    println!("╠══════════════════════════════════════════════════╣");
    println!("║  SCORES                                          ║");
    println!("╠══════════════════════════════════════════════════╣");
    print_score_line("Step Efficiency", report.scores.step_efficiency);
    print_score_line("Tool Correctness", report.scores.tool_correctness);
    print_score_line("Plan Adherence", report.scores.plan_adherence);
    print_score_line("Task Completion", report.scores.task_completion);
    println!("║                                                  ║");
    print_score_line("COMPOSITE", report.scores.composite);
    println!("╠══════════════════════════════════════════════════╣");

    if let Some(idx) = report.critical_failure_turn {
        println!("║  ⚠ Critical Failure: Turn {:<22}  ║", idx);
        println!("╠══════════════════════════════════════════════════╣");
    }

    if !report.improvement_suggestions.is_empty() {
        println!("║  SUGGESTIONS                                     ║");
        println!("╠══════════════════════════════════════════════════╣");
        for suggestion in &report.improvement_suggestions {
            let wrapped = wrap_text(suggestion, 46);
            for line in &wrapped {
                println!("║  {}  ║", pad_str(line, 46));
            }
            println!("║                                                  ║");
        }
    }

    // Per-turn details for turns with issues
    let problem_turns: Vec<_> = report
        .turn_scores
        .iter()
        .filter(|ts| ts.is_critical_failure || ts.tool_correctness < 0.5)
        .collect();

    if !problem_turns.is_empty() {
        println!("║  PROBLEM TURNS                                   ║");
        println!("╠══════════════════════════════════════════════════╣");
        for ts in problem_turns {
            let flag = if ts.is_critical_failure { "⚠" } else { " " };
            println!(
                "║  {} Turn {}: eff={:.2} corr={:.2}               ",
                flag, ts.turn_index, ts.step_efficiency, ts.tool_correctness
            );
        }
        println!("╚══════════════════════════════════════════════════╝");
    } else {
        println!("╚══════════════════════════════════════════════════╝");
    }
    println!();
}

/// Print a score line with a visual bar
fn print_score_line(label: &str, score: f64) {
    let bar_width = 20;
    let filled = (score * bar_width as f64).round() as usize;
    let empty = bar_width - filled;
    let bar: String = "█".repeat(filled) + &"░".repeat(empty);
    println!("║  {:<17} {} {:.3}  ║", label, bar, score);
}

/// Truncate a string to max_len characters
fn truncate_str(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

/// Pad a string to exactly len characters
fn pad_str(s: &str, len: usize) -> String {
    format!("{:<width$}", s, width = len)
}

/// Simple word-wrap
fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        if current.is_empty() {
            current = word.to_string();
        } else if current.len() + 1 + word.len() <= max_width {
            current.push(' ');
            current.push_str(word);
        } else {
            lines.push(current);
            current = word.to_string();
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}
