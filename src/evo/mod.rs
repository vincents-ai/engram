//! Engram-Evo: Self-improving loop for pi agents.
//!
//! Provides trajectory evaluation and memory optimization capabilities:
//! - Ingest: Parse pi session JSONL files into Trajectory structs
//! - Evaluate: Run metrics (StepEfficiency, ToolCorrectness, PlanAdherence, TaskCompletion)
//! - Optimize: Analyze failures and generate Memory Patches
//! - Replay: Spawn pi --mode json and capture new trajectories
//! - Loop: Orchestrate the full improvement cycle
//! - Report: Format evaluation reports
//!
//! CLI subcommands:
//! - `engram evo ingest` — parse sessions into trajectories
//! - `engram evo evaluate` — run metrics on a trajectory
//! - `engram evo optimize` — generate memory patches from eval report
//! - `engram evo replay` — replay a task with injected patch
//! - `engram evo loop` — run the full improvement loop
//! - `engram evo report` — format evaluation reports

#[allow(unused_imports)]
pub mod capture;
pub mod cli;
#[allow(unused_imports)]
pub mod eval;
#[allow(unused_imports)]
pub mod ingest;
#[allow(unused_imports)]
pub mod llm;
#[allow(unused_imports)]
pub mod loop_;
#[allow(unused_imports)]
pub mod optimizer;
#[allow(unused_imports)]
pub mod replay;
#[allow(unused_imports)]
pub mod report;
pub mod types;

pub use cli::{
    EvaluateArgs, EvoCommands, IngestArgs, LoopArgs, OptimizeArgs, ReplayArgs, ReportArgs,
};
pub use types::*;

/// Handle the evo subcommand
pub fn handle_evo_command(cmd: EvoCommands) -> Result<(), crate::error::EngramError> {
    match cmd {
        EvoCommands::Ingest(args) => ingest::handle_ingest(args),
        EvoCommands::Evaluate(args) => eval::handle_evaluate(args),
        EvoCommands::Optimize(args) => optimizer::handle_optimize(args),
        EvoCommands::Replay(args) => replay::handle_replay(args),
        EvoCommands::Loop(args) => loop_::handle_loop(args),
        EvoCommands::Report(args) => report::handle_report(args),
    }
}
