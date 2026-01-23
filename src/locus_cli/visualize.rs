use clap::Subcommand;

#[derive(Subcommand)]
pub enum VisualizeCommands {
    Dashboard {
        #[arg(long, short)]
        agent: Option<String>,

        #[arg(long)]
        real_time: bool,
    },

    AgentMap {
        #[arg(long)]
        time_range: Option<String>,

        #[arg(long)]
        include_inactive: bool,
    },

    WorkflowStatus {
        #[arg(long, short)]
        workflow_id: Option<String>,
    },

    SystemHealth {
        #[arg(long)]
        detailed: bool,

        #[arg(long)]
        watch: bool,
    },
}
