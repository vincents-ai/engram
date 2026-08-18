//! CLI types for engram evo subcommand

use clap::Parser;

/// CLI subcommand group for engram evo
#[derive(Parser)]
#[command(
    about = "Self-improving loop for pi agents",
    long_about = "ENGRAM EVO: Self-improving loop for pi agents.\n\nParses pi session trajectories, evaluates with metrics, generates memory patches,\nand verifies improvements through automated replays.\n\nSubcommands:\n  ingest    - Parse session files into trajectories\n  evaluate  - Run evaluation metrics\n  optimize  - Generate memory patches\n  replay   - Replay a task with injected patch\n  loop     - Run full improvement loop\n  report   - Format evaluation reports"
)]
pub enum EvoCommands {
    /// Parse pi session files into Trajectory objects
    Ingest(IngestArgs),
    /// Run evaluation metrics on a trajectory
    Evaluate(EvaluateArgs),
    /// Generate Memory Patches from evaluation
    Optimize(OptimizeArgs),
    /// Replay a task with patch injection
    Replay(ReplayArgs),
    /// Run the full improvement loop
    Loop(LoopArgs),
    /// Format and display evaluation reports
    Report(ReportArgs),
}

#[derive(Parser)]
pub struct IngestArgs {
    /// Sessions directory (default: ~/.pi/agent/sessions)
    #[arg(long, short)]
    pub sessions_dir: Option<String>,
    /// Output file for trajectory JSON
    #[arg(long, short, default_value = "trajectories.json")]
    pub output: String,
    /// Filter by session name pattern
    #[arg(long)]
    pub filter: Option<String>,
    /// Maximum sessions to process
    #[arg(long, default_value = "10")]
    pub limit: usize,
}

#[derive(Parser)]
pub struct EvaluateArgs {
    /// Trajectory input file (or - for stdin)
    #[arg(long, short, default_value = "-")]
    pub trajectory: String,
    /// Output file for eval report (or - for stdout)
    #[arg(long, short, default_value = "-")]
    pub output: String,
    /// Skip metrics requiring LLM (PlanAdherence)
    #[arg(long)]
    pub skip_llm: bool,
}

#[derive(Parser)]
pub struct OptimizeArgs {
    /// Eval report file
    #[arg(long, short)]
    pub eval_report: String,
    /// Trajectory file (for context)
    #[arg(long, short)]
    pub trajectory: String,
    /// Output file for patch (or - for stdout)
    #[arg(long, short, default_value = "-")]
    pub output: String,
}

#[derive(Parser)]
pub struct ReplayArgs {
    /// Patch file to inject
    #[arg(long, short)]
    pub patch: String,
    /// Original task prompt
    #[arg(long, short)]
    pub task: String,
    /// Model to use for replay
    #[arg(long)]
    pub model: Option<String>,
    /// Output file for replay trajectory
    #[arg(long, short, default_value = "-")]
    pub output: String,
}

#[derive(Parser)]
pub struct LoopArgs {
    /// Sessions directory
    #[arg(long)]
    pub sessions_dir: Option<String>,
    /// Maximum iterations
    #[arg(long, default_value = "3")]
    pub max_iterations: usize,
    /// Minimum improvement threshold
    #[arg(long, default_value = "0.1")]
    pub min_improvement: f64,
    /// Model to use for replays
    #[arg(long)]
    pub model: Option<String>,
    /// Maximum sessions to process
    #[arg(long, default_value = "5")]
    pub limit: usize,
}

#[derive(Parser)]
pub struct ReportArgs {
    /// Eval report file (or - for stdin)
    #[arg(long, short, default_value = "-")]
    pub eval_report: String,
    /// Output format (text, json)
    #[arg(long, short, default_value = "text")]
    pub format: String,
}
