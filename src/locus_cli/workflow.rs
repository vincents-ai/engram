use clap::Subcommand;

#[derive(Subcommand)]
pub enum WorkflowCommands {
    /// Create a new workflow design
    Create {
        /// Workflow name
        #[arg(long, short)]
        name: String,

        /// Workflow description
        #[arg(long, short)]
        description: String,

        /// Output file path (optional, defaults to stdout)
        #[arg(long, short)]
        output: Option<String>,
    },

    /// Visual workflow builder with drag-and-drop composition
    Design {
        /// Workflow name to edit
        #[arg(long, short)]
        name: Option<String>,

        /// Launch in interactive mode
        #[arg(long, short)]
        interactive: bool,
    },

    /// List existing workflows
    List {
        /// Filter by type
        #[arg(long)]
        workflow_type: Option<String>,

        /// Output format (table, json, yaml)
        #[arg(long, default_value = "table")]
        format: String,
    },

    /// Show workflow details
    Show {
        /// Workflow name or ID
        name: String,

        /// Include execution history
        #[arg(long)]
        with_history: bool,
    },

    /// Validate workflow configuration
    Validate {
        /// Workflow file path
        #[arg(long, short)]
        file: String,

        /// Show detailed validation results
        #[arg(long)]
        detailed: bool,
    },
}
