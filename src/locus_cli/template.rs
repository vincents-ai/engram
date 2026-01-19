use clap::Subcommand;
use std::io;

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

pub async fn handle_template_command(command: TemplateCommands) -> io::Result<()> {
    match command {
        TemplateCommands::Create {
            name,
            description,
            category,
        } => {
            println!("Creating template: {}", name);
            println!("Description: {}", description);
            if let Some(cat) = category {
                println!("Category: {}", cat);
            }
        }

        TemplateCommands::List { category, format } => {
            println!("Listing templates with format: {}", format);
            if let Some(cat) = category {
                println!("Category: {}", cat);
            }
        }

        TemplateCommands::Apply { template, target } => {
            println!("Applying template: {} to: {}", template, target);
        }
    }

    Ok(())
}
