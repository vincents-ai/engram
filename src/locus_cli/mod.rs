pub mod govern;
pub mod override_cmd;
pub mod template;
pub mod visualize;
pub mod workflow;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum LocusCommands {
    Workflow {
        #[command(subcommand)]
        subcommand: workflow::WorkflowCommands,
    },
    Template {
        #[command(subcommand)]
        subcommand: template::TemplateCommands,
    },
    Visualize {
        #[command(subcommand)]
        subcommand: visualize::VisualizeCommands,
    },
    Govern {
        #[command(subcommand)]
        subcommand: govern::GovernCommands,
    },
    Override {
        #[command(subcommand)]
        subcommand: override_cmd::OverrideCommands,
    },
}
