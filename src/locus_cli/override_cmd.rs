use clap::Subcommand;

#[derive(Subcommand)]
pub enum OverrideCommands {
    Approve {
        #[arg(long, short)]
        task_id: String,

        #[arg(long, short)]
        reason: String,

        #[arg(long)]
        override_type: String,
    },

    Reject {
        #[arg(long, short)]
        task_id: String,

        #[arg(long, short)]
        reason: String,

        #[arg(long)]
        block_future: bool,
    },

    Emergency {
        #[command(subcommand)]
        subcommand: EmergencyCommands,
    },

    List {
        #[arg(long)]
        status: Option<String>,

        #[arg(long)]
        agent: Option<String>,

        #[arg(long, default_value = "table")]
        format: String,
    },
}

#[derive(Subcommand)]
pub enum EmergencyCommands {
    Halt {
        #[arg(long, short)]
        reason: String,

        #[arg(long)]
        scope: Option<String>,
    },

    Rollback {
        #[arg(long, short)]
        task_id: String,

        #[arg(long)]
        rollback_point: String,
    },

    Bypass {
        #[arg(long, short)]
        gate: String,

        #[arg(long, short)]
        duration: String,

        #[arg(long, short)]
        justification: String,
    },
}
