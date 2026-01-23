use clap::Subcommand;

#[derive(Subcommand)]
pub enum GovernCommands {
    Policy {
        #[command(subcommand)]
        subcommand: PolicyCommands,
    },

    QualityGate {
        #[command(subcommand)]
        subcommand: QualityGateCommands,
    },

    Compliance {
        #[command(subcommand)]
        subcommand: ComplianceCommands,
    },
}

#[derive(Subcommand)]
pub enum PolicyCommands {
    Create {
        #[arg(long, short)]
        name: String,

        #[arg(long, short)]
        rule: String,

        #[arg(long)]
        scope: Option<String>,
    },

    List {
        #[arg(long)]
        scope: Option<String>,
    },

    Enforce {
        #[arg(long, short)]
        policy: String,

        #[arg(long)]
        dry_run: bool,
    },
}

#[derive(Subcommand)]
pub enum QualityGateCommands {
    Create {
        #[arg(long, short)]
        name: String,

        #[arg(long, short)]
        command: String,

        #[arg(long)]
        required: bool,
    },

    List {
        #[arg(long)]
        category: Option<String>,
    },

    Validate {
        #[arg(long, short)]
        gate: String,

        #[arg(long)]
        context: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum ComplianceCommands {
    Check {
        #[arg(long)]
        standard: Option<String>,

        #[arg(long)]
        report_format: Option<String>,
    },

    Report {
        #[arg(long, short)]
        period: String,

        #[arg(long)]
        output: Option<String>,
    },

    Audit {
        #[arg(long, short)]
        target: String,

        #[arg(long)]
        deep_scan: bool,
    },
}
