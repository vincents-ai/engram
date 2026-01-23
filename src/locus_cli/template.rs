use clap::Subcommand;

#[derive(Subcommand)]
pub enum TemplateCommands {
    Create {
        #[arg(long, short)]
        name: String,

        #[arg(long, short)]
        description: String,

        #[arg(long)]
        category: Option<String>,
    },

    List {
        #[arg(long)]
        category: Option<String>,

        #[arg(long, default_value = "table")]
        format: String,
    },

    Apply {
        #[arg(long, short)]
        template: String,

        #[arg(long, short)]
        target: String,
    },
}
