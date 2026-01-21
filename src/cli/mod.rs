//! CLI module for Engram system
//!
//! Provides modular command-line interface with subcommands
//! for all entity types and operations.

pub mod adr;
pub mod compliance;
pub mod context;
pub mod convert;
pub mod escalation;
pub mod help;
pub mod info;
pub mod knowledge;
pub mod perkeep;
pub mod reasoning;
pub mod relationship;
pub mod rule;
#[cfg(feature = "sandbox")]
pub mod sandbox;
pub mod session;
pub mod setup;
pub mod standard;
pub mod sync;
pub mod task;
pub mod validation;
pub mod workflow;

pub use adr::*;
pub use compliance::*;
pub use context::*;
pub use convert::*;
pub use escalation::*;
pub use help::*;
pub use info::*;
pub use knowledge::*;
pub use perkeep::*;
pub use reasoning::*;
pub use relationship::*;
pub use rule::*;
#[cfg(feature = "sandbox")]
pub use sandbox::*;
pub use session::*;
pub use setup::*;
pub use standard::*;
pub use sync::*;
pub use task::*;
pub use validation::*;
pub use workflow::*;

use crate::ask::AskCommands;
use clap::{Parser, Subcommand};

/// Main CLI structure
#[derive(Parser)]
#[command(name = "engram")]
#[command(
    about = "Task memory system for LLM coding agents",
    long_about = "ENGRAM: Task-driven memory system for LLM coding agents\n\nPURPOSE: Maintains project state, tasks, and reasoning across coding sessions.\nEnforces disciplined development via Git commit validation requiring task references.\n\nWORKFLOW:\n1. engram setup workspace              # Initialize project\n2. engram task create --title \"...\"    # Create work items (returns UUIDs)\n3. engram context create --title \"...\" # Add background info\n4. engram reasoning create --task-id <uuid> # Document decisions\n5. engram relationship create ...       # Link entities (REQUIRED for validation)\n6. engram validate hook install        # Enable Git integration\n\nGIT INTEGRATION: Commits must reference task UUIDs: \"feat: implement auth [<uuid>]\"\nJSON I/O: Most commands support --json input/output for programmatic access.\n\nUse 'engram guide examples' for working command examples."
)]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(author = "Engram Team")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long, global = true)]
    pub json: bool,
}

/// Available CLI commands
#[derive(Subcommand)]
pub enum Commands {
    /// Initialize workspace or agent
    Setup {
        #[command(subcommand)]
        command: SetupCommands,
    },
    /// Convert from other formats (EXPERIMENTAL - Not yet implemented)
    Convert {
        /// Source format (openspec, beads, github)
        #[arg(long, short = 'o')]
        from: String,

        /// Source file path
        #[arg(long, short = 'f')]
        file: String,
    },
    /// Run test suite
    Test,
    /// Create/manage work items (returns UUIDs for commit references)
    Task {
        #[command(subcommand)]
        command: TaskCommands,
    },
    /// Background information and documentation
    Context {
        #[command(subcommand)]
        command: ContextCommands,
    },
    /// Natural Language Query Interface
    Ask {
        #[command(subcommand)]
        command: AskCommands,
    },
    /// Decision chains and rationale (required for task validation)
    Reasoning {
        #[command(subcommand)]
        command: ReasoningCommands,
    },
    /// Knowledge base management
    Knowledge {
        #[command(subcommand)]
        command: KnowledgeCommands,
    },
    /// Coding session tracking
    Session {
        #[command(subcommand)]
        command: SessionCommands,
    },
    /// Compliance requirements
    Compliance {
        #[command(subcommand)]
        command: ComplianceCommands,
    },
    /// Rules and policies
    Rule {
        #[command(subcommand)]
        command: RuleCommands,
    },
    /// Coding standards
    Standard {
        #[command(subcommand)]
        command: StandardCommands,
    },
    /// Architectural decisions
    Adr {
        #[command(subcommand)]
        command: AdrCommands,
    },
    /// State machines and process flows
    Workflow {
        #[command(subcommand)]
        command: WorkflowCommands,
    },
    /// Link entities (REQUIRED: task↔reasoning, task↔context for validation)
    Relationship {
        #[command(subcommand)]
        command: RelationshipCommands,
    },
    /// Git commit validation and pre-commit hooks
    Validate {
        #[command(subcommand)]
        command: validation::ValidationCommands,
    },
    /// Agent sandbox security and resource management
    #[cfg(feature = "sandbox")]
    Sandbox {
        #[command(subcommand)]
        command: SandboxCommands,
    },
    /// Escalation requests for sandbox permission denied operations
    Escalation {
        #[command(subcommand)]
        command: EscalationCommands,
    },
    /// Synchronize between agents
    Sync {
        #[command(subcommand)]
        command: SyncCommands,
    },
    /// Get next task and generate prompt
    Next {
        /// Optional specific task ID
        #[arg(long, short)]
        id: Option<String>,

        /// Output format (markdown, json)
        #[arg(long, default_value = "markdown")]
        format: String,
    },
    /// Display workspace and storage information
    Info,
    /// Migrate from dual-repository to Git refs storage
    Migration,
    /// Perkeep backup and restore operations
    Perkeep {
        #[command(subcommand)]
        command: PerkeepCommands,
    },
    #[command(name = "guide")]
    Guide {
        #[command(subcommand)]
        command: Option<HelpCommands>,
    },
}

/// Setup commands
#[derive(Subcommand)]
pub enum SetupCommands {
    /// Initialize workspace
    Workspace,
    /// Initialize agent profile
    Agent {
        /// Agent name
        #[arg(long, short)]
        name: String,

        /// Agent type (coder, reviewer, planner)
        #[arg(long, short, default_value = "coder")]
        agent_type: String,

        /// Agent specialization
        #[arg(long)]
        specialization: Option<String>,

        /// Agent email
        #[arg(long)]
        email: Option<String>,
    },
}
pub mod next;
