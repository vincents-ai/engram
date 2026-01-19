use clap::Subcommand;
use std::io;

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

pub async fn handle_override_command(command: OverrideCommands) -> io::Result<()> {
    match command {
        OverrideCommands::Approve {
            task_id,
            reason,
            override_type,
        } => {
            println!("✅ Approving override for task: {}", task_id);
            println!("Reason: {}", reason);
            println!("Type: {}", override_type);
        }

        OverrideCommands::Reject {
            task_id,
            reason,
            block_future,
        } => {
            println!("❌ Rejecting override for task: {}", task_id);
            println!("Reason: {}", reason);
            if block_future {
                println!("Future overrides blocked");
            }
        }

        OverrideCommands::Emergency { subcommand } => handle_emergency_command(subcommand).await?,

        OverrideCommands::List {
            status,
            agent,
            format,
        } => {
            println!("📋 Listing overrides...");
            if let Some(st) = status {
                println!("Status: {}", st);
            }
            if let Some(a) = agent {
                println!("Agent: {}", a);
            }
            println!("Format: {}", format);
        }
    }

    Ok(())
}

async fn handle_emergency_command(command: EmergencyCommands) -> io::Result<()> {
    match command {
        EmergencyCommands::Halt { reason, scope } => {
            println!("🛑 EMERGENCY HALT");
            println!("Reason: {}", reason);
            if let Some(s) = scope {
                println!("Scope: {}", s);
            }
        }

        EmergencyCommands::Rollback {
            task_id,
            rollback_point,
        } => {
            println!("⏪ EMERGENCY ROLLBACK");
            println!("Task ID: {}", task_id);
            println!("Rollback point: {}", rollback_point);
        }

        EmergencyCommands::Bypass {
            gate,
            duration,
            justification,
        } => {
            println!("🚧 EMERGENCY BYPASS");
            println!("Gate: {}", gate);
            println!("Duration: {}", duration);
            println!("Justification: {}", justification);
        }
    }

    Ok(())
}
