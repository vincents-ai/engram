//! Main entry point for Engram CLI

mod commands;

use clap::Parser;
use engram::{
    ask::handle_ask_command,
    cli::{self, handle_relationship_command, handle_validation_command},
    error::EngramError,
    storage::GitRefsStorage,
};

use commands::*;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let json_mode = args.iter().any(|arg| arg == "--json");

    if let Err(e) = run().await {
        if json_mode {
            let error_msg = serde_json::json!({
                "error": e.to_string()
            });
            println!("{}", error_msg);
        } else {
            eprintln!("Error: {}", e);
        }
        std::process::exit(1);
    }
}

async fn run() -> Result<(), EngramError> {
    let args = cli::Cli::parse();

    match args.command {
        cli::Commands::Setup { command } => handle_setup_command(command)?,
        cli::Commands::Convert { from, file } => handle_convert_command(&from, &file)?,
        cli::Commands::Doc { command } => {
            let mut storage = GitRefsStorage::new(".", "default")?;
            cli::handle_doc_command(command, &mut storage)?;
        }
        cli::Commands::Import { command } => {
            let mut storage = GitRefsStorage::new(".", "default")?;
            cli::handle_import_command(command, &mut storage)?;
        }
        cli::Commands::Test => handle_test_command()?,
        cli::Commands::Task { command } => {
            let mut storage = GitRefsStorage::new(".", "default")?;
            handle_task_command(command, &mut storage)?;
        }
        cli::Commands::Context { command } => {
            let mut storage = GitRefsStorage::new(".", "default")?;
            handle_context_command(command, &mut storage)?;
        }
        cli::Commands::Ask { command } => {
            handle_ask_command(command).await?;
        }
        cli::Commands::Reasoning { command } => {
            let mut storage = GitRefsStorage::new(".", "default")?;
            handle_reasoning_command(command, &mut storage)?;
        }
        cli::Commands::Knowledge { command } => {
            let mut storage = GitRefsStorage::new(".", "default")?;
            handle_knowledge_command(command, &mut storage)?;
        }
        cli::Commands::Lesson { command } => {
            let mut storage = GitRefsStorage::new(".", "default")?;
            handle_lesson_command(command, &mut storage)?;
        }
        cli::Commands::Persona { command } => {
            let mut storage = GitRefsStorage::new(".", "default")?;
            handle_persona_command(command, &mut storage)?;
        }
        cli::Commands::Session { command } => {
            let mut storage = GitRefsStorage::new(".", "default")?;
            handle_session_command(command, &mut storage)?;
        }
        cli::Commands::Compliance { command } => {
            let mut storage = GitRefsStorage::new(".", "default")?;
            handle_compliance_command(command, &mut storage)?;
        }
        cli::Commands::Rule { command } => {
            let mut storage = GitRefsStorage::new(".", "default")?;
            handle_rule_command(command, &mut storage)?;
        }
        cli::Commands::Standard { command } => {
            let mut storage = GitRefsStorage::new(".", "default")?;
            handle_standard_command(command, &mut storage)?;
        }
        cli::Commands::Adr { command } => {
            let mut storage = GitRefsStorage::new(".", "default")?;
            handle_adr_command(command, &mut storage)?;
        }
        cli::Commands::Workflow { command } => {
            let mut storage = GitRefsStorage::new(".", "default")?;
            handle_workflow_command(command, &mut storage)?;
        }
        cli::Commands::Relationship { command } => {
            let mut storage = GitRefsStorage::new(".", "default")?;
            handle_relationship_command(&mut storage, command)?;
        }
        cli::Commands::Git { command } => {
            engram::cli::git::handle_git_command(command)?;
        }
        cli::Commands::Validate { command } => {
            let storage = GitRefsStorage::new(".", "default")?;
            handle_validation_command(command, storage)?;
        }
        cli::Commands::Sandbox { command } => {
            let mut storage = GitRefsStorage::new(".", "default")?;
            handle_sandbox_command(command, &mut storage)?;
        }
        cli::Commands::Escalation { command } => {
            let mut storage = GitRefsStorage::new(".", "default")?;
            handle_escalation_command(command, &mut storage)?;
        }
        cli::Commands::Sync { command } => {
            let mut storage = GitRefsStorage::new(".", "default")?;
            engram::cli::sync::handle_sync_command(&mut storage, &command)?;
        }
        cli::Commands::Next {
            id,
            format,
            agent,
            parent,
            scope_agent,
            session,
            tag,
        } => {
            let mut storage = GitRefsStorage::new(".", "default")?;
            engram::cli::next::handle_next_command(
                &mut storage,
                id,
                format,
                agent,
                parent,
                scope_agent,
                session,
                tag,
            )?;
        }
        cli::Commands::Info => {
            let storage = GitRefsStorage::new(".", "default")?;
            cli::info::info(&storage)?;
        }
        cli::Commands::Migration => handle_migration_command()?,
        cli::Commands::Migrate { command } => handle_migrate_command(command)?,
        cli::Commands::Guide { command } => handle_help_command(command)?,
        cli::Commands::Skills { command } => match command {
            cli::SkillsCommands::Setup {
                force,
                dir,
                tool,
                source,
            } => {
                cli::handle_skills_command(
                    &mut std::io::stdout(),
                    force,
                    dir.as_deref(),
                    tool.as_deref(),
                    source.as_deref(),
                )?;
            }
            cli::SkillsCommands::List { format, verbose } => {
                cli::list_skills(&mut std::io::stdout(), &format, verbose, None)?;
            }
            cli::SkillsCommands::Show { name } => {
                cli::show_skill(&mut std::io::stdout(), &name, None)?;
            }
        },
        cli::Commands::Prompts { command } => match command {
            cli::PromptsCommands::List {
                category,
                format,
                verbose,
            } => {
                cli::list_prompts(category.as_deref(), &format, None, verbose)?;
            }
            cli::PromptsCommands::Show { name } => {
                cli::show_prompt(&name, None)?;
            }
            cli::PromptsCommands::Validate { category, fix } => {
                cli::validate_prompts(category.as_deref(), fix, None)?;
            }
        },
        cli::Commands::Schema { command } => {
            let mut storage = GitRefsStorage::new(".", "default")?;
            cli::handle_schema_command(command, &mut storage)?;
        }
        cli::Commands::Theory { command } => {
            let mut storage = GitRefsStorage::new(".", "default")?;
            handle_theory_command(command, &mut storage)?;
        }
        cli::Commands::Reflect { command } => {
            let mut storage = GitRefsStorage::new(".", "default")?;
            handle_reflection_command(command, &mut storage)?;
        }
        cli::Commands::Analytics { command } => {
            let mut storage = GitRefsStorage::new(".", "default")?;
            cli::handle_analytics_command(&mut storage, command)?;
        }
        cli::Commands::Health { command } => {
            let mut storage = GitRefsStorage::new(".", "default")?;
            cli::health::handle_health_command(&mut storage, command)?;
        }
        cli::Commands::Evo { command } => {
            engram::evo::handle_evo_command(command)?;
        }
        cli::Commands::Perkeep { command } => {
            use engram::cli::perkeep::{
                perkeep_backup, perkeep_health, perkeep_list, perkeep_restore,
            };
            let mut storage = GitRefsStorage::new(".", "default")?;
            match command {
                cli::PerkeepCommands::Backup {
                    entity_type,
                    include_relationships,
                    description,
                } => {
                    perkeep_backup(&storage, entity_type, include_relationships, description)
                        .await?;
                }
                cli::PerkeepCommands::Restore {
                    blobref,
                    agent,
                    dry_run,
                } => {
                    perkeep_restore(&mut storage, blobref, agent, dry_run).await?;
                }
                cli::PerkeepCommands::List { detailed } => {
                    perkeep_list(detailed).await?;
                }
                cli::PerkeepCommands::Health => {
                    perkeep_health().await?;
                }
                cli::PerkeepCommands::Config {
                    server,
                    auth_token,
                    save: _,
                } => {
                    println!("Perkeep configuration");
                    if let Some(server) = server {
                        println!("   Server: {}", server);
                    }
                    if let Some(_auth_token) = auth_token {
                        println!("   Auth token: [REDACTED]");
                    }
                    println!("Note: Configuration via environment variables PERKEEP_SERVER and PERKEEP_AUTH_TOKEN");
                }
            }
        }
    }

    Ok(())
}
