//! CLI module for Engram system
//!
//! Provides modular command-line interface with subcommands
//! for all entity types and operations.

pub mod adr;
pub mod compliance;
pub mod context;
pub mod convert;
pub mod help;
pub mod knowledge;
pub mod reasoning;
pub mod relationship;
pub mod rule;
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
pub use help::*;
pub use knowledge::*;
pub use reasoning::*;
pub use relationship::*;
pub use rule::*;
pub use session::*;
pub use setup::*;
pub use standard::*;
pub use sync::*;
pub use task::*;
pub use validation::*;
pub use workflow::*;

use clap::{Parser, Subcommand};

/// Main CLI structure
#[derive(Parser)]
#[command(name = "engram")]
#[command(about = "Distributed memory system for AI agents", long_about = None)]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(author = "Engram Team")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Available CLI commands
#[derive(Subcommand)]
pub enum Commands {
    /// Initialize workspace or agent
    Setup {
        #[command(subcommand)]
        command: SetupCommands,
    },
    /// Convert from other formats
    Convert {
        /// Source format (openspec, beads, github)
        #[arg(long, short)]
        from: String,

        /// Source file path
        #[arg(long, short)]
        file: String,
    },
    /// Run test suite
    Test,
    /// Manage tasks
    Task {
        #[command(subcommand)]
        command: TaskCommands,
    },
    /// Manage context
    Context {
        #[command(subcommand)]
        command: ContextCommands,
    },
    /// Manage reasoning chains
    Reasoning {
        #[command(subcommand)]
        command: ReasoningCommands,
    },
    /// Manage knowledge
    Knowledge {
        #[command(subcommand)]
        command: KnowledgeCommands,
    },
    /// Manage sessions
    Session {
        #[command(subcommand)]
        command: SessionCommands,
    },
    /// Manage compliance requirements
    Compliance {
        #[command(subcommand)]
        command: ComplianceCommands,
    },
    /// Manage rules
    Rule {
        #[command(subcommand)]
        command: RuleCommands,
    },
    /// Manage standards
    Standard {
        #[command(subcommand)]
        command: StandardCommands,
    },
    /// Manage architectural decision records
    Adr {
        #[command(subcommand)]
        command: AdrCommands,
    },
    /// Manage workflows
    Workflow {
        #[command(subcommand)]
        command: WorkflowCommands,
    },
    /// Manage entity relationships
    Relationship {
        #[command(subcommand)]
        command: RelationshipCommands,
    },
    /// Manage validation and hooks
    Validate {
        #[command(subcommand)]
        command: ValidationCommands,
    },
    /// Synchronize between agents
    Sync {
        #[command(subcommand)]
        command: SyncCommands,
    },
    Update,
    Migration,
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
