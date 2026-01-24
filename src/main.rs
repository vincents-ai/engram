//! Main entry point for Engram CLI

use clap::Parser;
use engram::{
    ask::handle_ask_command,
    cli::{self, handle_relationship_command, handle_validation_command},
    error::EngramError,
    migration::Migration,
    storage::GitRefsStorage,
};

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
            engram::cli::git::handle_git_command(match command {
                engram::cli::git::GitCommands::External(args) => args,
            })?;
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
        cli::Commands::Next { id, format } => {
            let mut storage = GitRefsStorage::new(".", "default")?;
            engram::cli::next::handle_next_command(&mut storage, id, format)?;
        }
        cli::Commands::Info => {
            let storage = GitRefsStorage::new(".", "default")?;
            cli::info::info(&storage)?;
        }
        cli::Commands::Migration => handle_migration_command()?,
        cli::Commands::Guide { command } => handle_help_command(command)?,
        cli::Commands::Skills { command } => match command {
            cli::SkillsCommands::Setup => {
                cli::handle_skills_command(cli::SkillsCommands::Setup)?;
            }
            cli::SkillsCommands::List { format } => {
                cli::list_skills(&format, None)?;
            }
            cli::SkillsCommands::Show { name } => {
                cli::show_skill(&name, None)?;
            }
        },
        cli::Commands::Prompts { command } => match command {
            cli::PromptsCommands::List { category, format } => {
                cli::list_prompts(category.as_deref(), &format, None)?;
            }
            cli::PromptsCommands::Show { name } => {
                cli::show_prompt(&name, None)?;
            }
        },
        cli::Commands::Schema { command } => {
            cli::handle_schema_command(command)?;
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

/// Handle setup commands
fn handle_setup_command(command: cli::SetupCommands) -> Result<(), EngramError> {
    match command {
        cli::SetupCommands::Workspace => cli::setup_workspace(None)?,
        cli::SetupCommands::Agent {
            name,
            agent_type,
            specialization,
            email,
        } => {
            cli::setup_agent(
                &name,
                &agent_type,
                specialization.as_deref(),
                email.as_deref(),
                None,
            )?;
        }
        cli::SetupCommands::Skills => {
            cli::setup_skills(None)?;
        }
        cli::SetupCommands::Prompts { path } => {
            cli::setup_prompts(path.as_deref(), None)?;
        }
    }
    Ok(())
}

/// Handle convert command
fn handle_convert_command(from: &str, file: &str) -> Result<(), EngramError> {
    println!("Converting from {} file: {}", from, file);
    println!("Conversion functionality will be implemented in a future version");
    Ok(())
}

/// Handle test command
fn handle_test_command() -> Result<(), EngramError> {
    println!("Engram Test Suite");
    println!("==================");

    // Test basic functionality
    let workspace_dir = ".engram";
    if std::path::Path::new(workspace_dir).exists() {
        println!("✅ Workspace directory exists");
    } else {
        println!("❌ Workspace directory missing");
    }

    let agents_dir = ".engram/agents";
    if std::path::Path::new(agents_dir).exists() {
        println!("✅ Agents directory exists");
    } else {
        println!("❌ Agents directory missing");
    }

    println!("==================");
    println!("All tests completed");
    Ok(())
}

/// Handle task commands
fn handle_task_command<S: engram::storage::Storage + 'static>(
    command: cli::TaskCommands,
    storage: &mut S,
) -> Result<(), EngramError> {
    match command {
        cli::TaskCommands::Create {
            title,
            description,
            priority,
            agent,
            parent,
            tags,
            title_stdin,
            title_file,
            description_stdin,
            description_file,
            json,
            json_file,
        } => {
            cli::create_task(
                storage,
                title,
                description,
                &priority,
                agent,
                parent,
                tags,
                title_stdin,
                title_file,
                description_stdin,
                description_file,
                json,
                json_file,
            )?;
        }
        cli::TaskCommands::List {
            agent,
            status,
            limit,
        } => {
            cli::list_tasks(storage, agent.as_deref(), status.as_deref(), limit)?;
        }
        cli::TaskCommands::Show { id } => {
            cli::show_task(storage, &id)?;
        }
        cli::TaskCommands::Update {
            id,
            status,
            outcome,
        } => {
            cli::update_task(storage, &id, &status, outcome.as_deref())?;
        }
        cli::TaskCommands::Archive { id, reason } => {
            cli::archive_task(storage, &id, reason.as_deref())?;
        }
    }
    Ok(())
}

/// Handle context commands
fn handle_context_command<S: engram::storage::Storage>(
    command: engram::cli::ContextCommands,
    storage: &mut S,
) -> Result<(), EngramError> {
    match command {
        cli::ContextCommands::Create {
            title,
            content,
            source,
            relevance,
            source_id,
            agent,
            tags,
            title_stdin,
            title_file,
            content_stdin,
            content_file,
            json,
            json_file,
        } => {
            cli::create_context(
                storage,
                title,
                content,
                source,
                &relevance,
                source_id,
                agent,
                tags,
                title_stdin,
                title_file,
                content_stdin,
                content_file,
                json,
                json_file,
            )?;
        }
        cli::ContextCommands::List {
            agent,
            relevance,
            limit,
        } => {
            cli::list_contexts(storage, agent.as_deref(), relevance.as_deref(), limit)?;
        }
        cli::ContextCommands::Show { id } => {
            cli::show_context(storage, &id)?;
        }
        cli::ContextCommands::Update { id, content } => {
            cli::update_context(storage, &id, &content)?;
        }
        cli::ContextCommands::Delete { id } => {
            cli::delete_context(storage, &id)?;
        }
    }
    Ok(())
}

/// Handle reasoning commands
fn handle_reasoning_command<S: engram::storage::Storage>(
    command: engram::cli::ReasoningCommands,
    storage: &mut S,
) -> Result<(), EngramError> {
    match command {
        cli::ReasoningCommands::Create {
            title,
            task_id,
            agent,
            confidence,
            content,
            tags,
            title_stdin,
            title_file,
            content_stdin,
            content_file,
            json,
            json_file,
        } => {
            cli::create_reasoning(
                storage,
                title,
                task_id,
                agent,
                confidence,
                content,
                tags,
                title_stdin,
                title_file,
                content_stdin,
                content_file,
                json,
                json_file,
            )?;
        }
        cli::ReasoningCommands::AddStep {
            id,
            description,
            conclusion,
            confidence,
            description_stdin,
            description_file,
            conclusion_stdin,
            conclusion_file,
        } => {
            cli::add_reasoning_step(
                storage,
                &id,
                description,
                conclusion,
                confidence,
                description_stdin,
                description_file,
                conclusion_stdin,
                conclusion_file,
            )?;
        }
        cli::ReasoningCommands::Conclude {
            id,
            conclusion,
            confidence,
            conclusion_stdin,
            conclusion_file,
        } => {
            cli::conclude_reasoning(
                storage,
                &id,
                conclusion,
                confidence,
                conclusion_stdin,
                conclusion_file,
            )?;
        }
        cli::ReasoningCommands::List {
            agent,
            task_id,
            limit,
        } => {
            cli::list_reasoning(storage, agent.as_deref(), task_id.as_deref(), limit)?;
        }
        cli::ReasoningCommands::Show { id } => {
            cli::show_reasoning(storage, &id)?;
        }
        cli::ReasoningCommands::Delete { id } => {
            cli::delete_reasoning(storage, &id)?;
        }
    }
    Ok(())
}

/// Handle knowledge commands
fn handle_knowledge_command<S: engram::storage::Storage>(
    command: engram::cli::KnowledgeCommands,
    storage: &mut S,
) -> Result<(), EngramError> {
    match command {
        cli::KnowledgeCommands::Create {
            title,
            content,
            knowledge_type,
            confidence,
            source,
            agent,
            tags,
            title_stdin,
            title_file,
            content_stdin,
            content_file,
            json,
            json_file,
        } => {
            cli::create_knowledge(
                storage,
                title,
                content,
                knowledge_type,
                confidence,
                source,
                agent,
                tags,
                title_stdin,
                title_file,
                content_stdin,
                content_file,
                json,
                json_file,
            )?;
        }
        cli::KnowledgeCommands::List { agent, kind, limit } => {
            cli::list_knowledge(storage, agent, kind, limit)?;
        }
        cli::KnowledgeCommands::Show { id } => {
            cli::show_knowledge(storage, &id)?;
        }
        cli::KnowledgeCommands::Update { id, field, value } => {
            cli::update_knowledge(storage, &id, &field, &value)?;
        }
        cli::KnowledgeCommands::Delete { id } => {
            cli::delete_knowledge(storage, &id)?;
        }
    }
    Ok(())
}

/// Handle session commands
fn handle_session_command<S: engram::storage::Storage>(
    command: engram::cli::SessionCommands,
    storage: &mut S,
) -> Result<(), EngramError> {
    use engram::cli::session::*;

    match command {
        engram::cli::SessionCommands::Start { name, auto_detect } => {
            start_session(storage, name, auto_detect)?;
        }
        engram::cli::SessionCommands::Status { id, metrics } => {
            show_session_status(storage, id, metrics)?;
        }
        engram::cli::SessionCommands::End {
            id,
            generate_summary,
        } => {
            end_session(storage, id, generate_summary)?;
        }
        engram::cli::SessionCommands::List { agent, limit } => {
            list_sessions(storage, agent, limit)?;
        }
    }

    Ok(())
}

/// Handle compliance commands
fn handle_compliance_command<S: engram::storage::Storage>(
    command: engram::cli::ComplianceCommands,
    storage: &mut S,
) -> Result<(), EngramError> {
    match command {
        cli::ComplianceCommands::Create {
            title,
            description,
            category,
            severity: _,
            agent,
        } => {
            cli::create_compliance(storage, title, description, category, agent)?;
        }
        cli::ComplianceCommands::List {
            agent,
            category,
            limit,
        } => {
            cli::list_compliance(storage, agent.as_deref(), category.as_deref(), limit)?;
        }
        cli::ComplianceCommands::Show { id } => {
            cli::show_compliance(storage, &id)?;
        }
        cli::ComplianceCommands::Update { id, field, value } => {
            cli::update_compliance(storage, &id, &field, &value)?;
        }
        cli::ComplianceCommands::Delete { id } => {
            cli::delete_compliance(storage, &id)?;
        }
    }
    Ok(())
}

/// Handle rule commands
fn handle_rule_command<S: engram::storage::Storage>(
    command: engram::cli::RuleCommands,
    storage: &mut S,
) -> Result<(), EngramError> {
    match command {
        cli::RuleCommands::Create {
            title,
            description,
            rule_type,
            priority,
            entity_types,
            condition,
            action,
            agent,
        } => {
            cli::create_rule(
                storage,
                title,
                description,
                rule_type,
                priority,
                entity_types,
                condition,
                action,
                agent,
            )?;
        }
        cli::RuleCommands::Get { id } => {
            cli::get_rule(storage, &id)?;
        }
        cli::RuleCommands::Update {
            id,
            title,
            description,
            rule_type,
            priority,
            entity_types,
            condition,
            action,
            status,
        } => {
            cli::update_rule(
                storage,
                &id,
                title,
                description,
                rule_type,
                priority,
                entity_types,
                condition,
                action,
                status,
            )?;
        }
        cli::RuleCommands::Delete { id } => {
            cli::delete_rule(storage, &id)?;
        }
        cli::RuleCommands::List {
            rule_type,
            priority,
            entity_type,
            status,
            search,
            limit,
            offset,
        } => {
            cli::list_rules(
                storage,
                rule_type,
                priority,
                entity_type,
                status,
                search,
                limit,
                offset,
            )?;
        }
        cli::RuleCommands::Execute {
            id,
            entity_id,
            entity_type,
        } => {
            cli::execute_rule(storage, &id, entity_id, entity_type)?;
        }
    }
    Ok(())
}

/// Handle standard commands
fn handle_standard_command<S: engram::storage::Storage>(
    command: engram::cli::StandardCommands,
    storage: &mut S,
) -> Result<(), EngramError> {
    match command {
        cli::StandardCommands::Create {
            title,
            description,
            category,
            version,
            effective_date,
            agent,
        } => {
            cli::create_standard(
                storage,
                title,
                description,
                category,
                version,
                effective_date,
                agent,
            )?;
        }
        cli::StandardCommands::Get { id } => {
            cli::get_standard(storage, &id)?;
        }
        cli::StandardCommands::Update {
            id,
            title,
            description,
            category,
            version,
            status,
            effective_date,
            superseded_by,
        } => {
            cli::update_standard(
                storage,
                &id,
                title,
                description,
                category,
                version,
                status,
                effective_date,
                superseded_by,
            )?;
        }
        cli::StandardCommands::Delete { id } => {
            cli::delete_standard(storage, &id)?;
        }
        cli::StandardCommands::List {
            category,
            status,
            search,
            limit,
            offset,
        } => {
            cli::list_standards(storage, category, status, search, limit, offset)?;
        }
        cli::StandardCommands::AddRequirement {
            id,
            title,
            description,
            mandatory,
            priority,
            evidence_required,
        } => {
            cli::add_requirement(
                storage,
                &id,
                title,
                description,
                mandatory,
                priority,
                evidence_required,
            )?;
        }
    }
    Ok(())
}

/// Handle ADR commands
fn handle_adr_command<S: engram::storage::Storage>(
    command: engram::cli::AdrCommands,
    storage: &mut S,
) -> Result<(), EngramError> {
    match command {
        cli::AdrCommands::Create {
            title,
            number,
            context,
            agent,
        } => {
            cli::create_adr(storage, title, number, context, agent)?;
        }
        cli::AdrCommands::Get { id } => {
            cli::get_adr(storage, &id)?;
        }
        cli::AdrCommands::Update {
            id,
            title,
            status,
            context,
            decision,
            consequences,
            implementation,
            superseded_by,
        } => {
            cli::update_adr(
                storage,
                &id,
                title,
                status,
                context,
                decision,
                consequences,
                implementation,
                superseded_by,
            )?;
        }
        cli::AdrCommands::Delete { id } => {
            cli::delete_adr(storage, &id)?;
        }
        cli::AdrCommands::List {
            status,
            search,
            limit,
            offset,
        } => {
            cli::list_adrs(storage, status, search, limit, offset)?;
        }
        cli::AdrCommands::Accept {
            id,
            decision,
            consequences,
        } => {
            cli::accept_adr(storage, &id, decision, consequences)?;
        }
        cli::AdrCommands::AddAlternative { id, description } => {
            cli::add_alternative(storage, &id, description)?;
        }
        cli::AdrCommands::AddStakeholder { id, stakeholder } => {
            cli::add_stakeholder(storage, &id, stakeholder)?;
        }
    }
    Ok(())
}

/// Handle workflow commands
fn handle_workflow_command<S: engram::storage::Storage>(
    command: engram::cli::WorkflowCommands,
    storage: &mut S,
) -> Result<(), EngramError> {
    match command {
        cli::WorkflowCommands::Create {
            title,
            description,
            entity_types,
            agent,
        } => {
            cli::create_workflow(storage, title, description, entity_types, agent)?;
        }
        cli::WorkflowCommands::Get { id } => {
            cli::get_workflow(storage, &id)?;
        }
        cli::WorkflowCommands::Update {
            id,
            title,
            description,
            status,
            entity_types,
            initial_state,
        } => {
            cli::update_workflow(
                storage,
                &id,
                title,
                description,
                status,
                entity_types,
                initial_state,
            )?;
        }
        cli::WorkflowCommands::Delete { id } => {
            cli::delete_workflow(storage, &id)?;
        }
        cli::WorkflowCommands::List {
            status,
            search,
            limit,
            offset,
        } => {
            cli::list_workflows(storage, status, search, limit, offset)?;
        }
        cli::WorkflowCommands::AddState {
            id,
            name,
            state_type,
            description,
            is_final,
        } => {
            cli::add_state(storage, &id, name, state_type, description, is_final)?;
        }
        cli::WorkflowCommands::AddTransition {
            id,
            name,
            from_state,
            to_state,
            transition_type,
            description,
        } => {
            cli::add_transition(
                storage,
                &id,
                name,
                from_state,
                to_state,
                transition_type,
                description,
            )?;
        }
        cli::WorkflowCommands::Activate { id } => {
            cli::activate_workflow(storage, &id)?;
        }
        cli::WorkflowCommands::Start {
            workflow_id,
            entity_id,
            entity_type,
            agent,
            variables,
        } => {
            let storage_for_workflow = GitRefsStorage::new(".", "default")?;
            cli::start_workflow_instance(
                storage_for_workflow,
                workflow_id,
                entity_id,
                entity_type,
                agent,
                variables,
            )?;
        }
        cli::WorkflowCommands::Transition {
            instance_id,
            transition,
            agent,
        } => {
            let storage_for_workflow = GitRefsStorage::new(".", "default")?;
            cli::execute_workflow_transition(storage_for_workflow, instance_id, transition, agent)?;
        }
        cli::WorkflowCommands::Status { instance_id } => {
            let storage_for_workflow = GitRefsStorage::new(".", "default")?;
            cli::get_workflow_instance_status(storage_for_workflow, instance_id)?;
        }
        cli::WorkflowCommands::Instances {
            workflow_id,
            agent,
            running_only,
        } => {
            let storage_for_workflow = GitRefsStorage::new(".", "default")?;
            cli::list_workflow_instances(storage_for_workflow, workflow_id, agent, running_only)?;
        }
        cli::WorkflowCommands::Cancel {
            instance_id,
            agent,
            reason,
        } => {
            let storage_for_workflow = GitRefsStorage::new(".", "default")?;
            cli::cancel_workflow_instance(storage_for_workflow, instance_id, agent, reason)?;
        }
        cli::WorkflowCommands::ExecuteAction {
            action_type,
            command,
            args,
            working_directory,
            environment,
            timeout_seconds,
            message,
            entity_id,
            entity_type,
        } => {
            let storage_for_workflow = GitRefsStorage::new(".", "default")?;
            cli::execute_action(
                storage_for_workflow,
                action_type,
                command,
                args,
                working_directory,
                environment,
                timeout_seconds,
                message,
                entity_id,
                entity_type,
            )?;
        }
        cli::WorkflowCommands::QueryActions {
            workflow_id,
            state_id,
        } => {
            cli::query_workflow_actions(storage, workflow_id, state_id)?;
        }
    }
    Ok(())
}

/// Handle migration command
fn handle_migration_command() -> Result<(), EngramError> {
    let args: Vec<String> = std::env::args().collect();
    let dry_run = args.contains(&String::from("--dry-run"));
    let backup_only = args.contains(&String::from("--backup-only"));

    if backup_only {
        println!("📦 Creating backup of .engram directory...");
        let migration = Migration::new(".", "default", true, backup_only)?;
        migration.create_backup()?;
        println!("✅ Backup completed successfully");
        return Ok(());
    }

    let mut migration = Migration::new(".", "default", dry_run, false)?;

    // Pre-flight validation
    if let Err(e) = Migration::validate_migration_readiness(".") {
        eprintln!("❌ Migration pre-check failed: {}", e);
        return Err(e);
    }

    println!("🚀 Starting migration from dual-repository to Git refs storage");
    if dry_run {
        println!("📝 DRY RUN: No changes will be made");
    } else {
        println!("🔄 MIGRATION: Converting data to Git refs storage");
    }

    match migration.execute() {
        Ok(stats) => {
            println!("\n🏁 Migration Summary:");
            println!("  📊 Total processed: {}", stats.entities_processed);
            println!("  ✅ Successfully migrated: {}", stats.entities_migrated);
            if stats.entities_failed > 0 {
                println!("  ❌ Failed: {}", stats.entities_failed);
            }
            if !dry_run && stats.entities_migrated > 0 {
                println!("\n💾 Backup created at: .engram_backup_<timestamp>");
            }
            println!("\n✅ Migration completed successfully!");
        }
        Err(e) => {
            eprintln!("❌ Migration failed: {}", e);
        }
    }

    Ok(())
}

/// Handle sandbox commands
fn handle_sandbox_command<S: engram::storage::Storage>(
    command: engram::cli::SandboxCommands,
    storage: &mut S,
) -> Result<(), EngramError> {
    use engram::cli::sandbox::*;

    match command {
        engram::cli::SandboxCommands::Create {
            agent,
            level,
            created_by,
            stdin,
            file,
            json,
        } => {
            create_sandbox(storage, agent, level, created_by, stdin, file, json)?;
        }
        engram::cli::SandboxCommands::List {
            agent_id,
            level,
            agent,
            json,
        } => {
            list_sandboxes(storage, agent_id, level, agent, json)?;
        }
        engram::cli::SandboxCommands::Get { id, json } => {
            get_sandbox(storage, id, json)?;
        }
        engram::cli::SandboxCommands::Update {
            id,
            level,
            stdin,
            file,
            json,
        } => {
            update_sandbox(storage, id, level, stdin, file, json)?;
        }
        engram::cli::SandboxCommands::Delete { id, force } => {
            delete_sandbox(storage, id, force)?;
        }
        engram::cli::SandboxCommands::Validate {
            agent_id,
            operation,
            resource_type,
            stdin,
            file,
            json,
        } => {
            validate_operation(
                storage,
                agent_id,
                operation,
                resource_type,
                stdin,
                file,
                json,
            )?;
        }
        engram::cli::SandboxCommands::Stats { agent_id, json } => {
            show_stats(storage, agent_id, json)?;
        }
        engram::cli::SandboxCommands::Reset {
            agent_id,
            force,
            json,
        } => {
            reset_sandbox(storage, agent_id, force, json)?;
        }
    }

    Ok(())
}

/// Handle escalation commands
fn handle_escalation_command<S: engram::storage::Storage>(
    command: engram::cli::EscalationCommands,
    storage: &mut S,
) -> Result<(), EngramError> {
    use engram::cli::escalation::*;

    match command {
        engram::cli::EscalationCommands::Create {
            agent,
            operation_type,
            operation,
            block_reason,
            justification,
            priority,
            impact,
            reviewer,
            stdin,
            file,
            json,
        } => {
            create_escalation(
                storage,
                agent,
                operation_type,
                operation,
                block_reason,
                justification,
                priority,
                impact,
                reviewer,
                stdin,
                file,
                json,
            )?;
        }
        engram::cli::EscalationCommands::List {
            agent_id,
            status,
            priority,
            operation_type,
            expired_only,
            actionable_only,
            agent,
            json,
        } => {
            list_escalations(
                storage,
                agent_id,
                status,
                priority,
                operation_type,
                expired_only,
                actionable_only,
                agent,
                json,
            )?;
        }
        engram::cli::EscalationCommands::Get { id, json } => {
            get_escalation(storage, id, json)?;
        }
        engram::cli::EscalationCommands::Review {
            id,
            status,
            reason,
            reviewer_id,
            reviewer_name,
            duration,
            create_policy,
            notes,
            stdin,
            file,
            json,
        } => {
            review_escalation(
                storage,
                id,
                status,
                reason,
                reviewer_id,
                reviewer_name,
                duration,
                create_policy,
                notes,
                stdin,
                file,
                json,
            )?;
        }
        engram::cli::EscalationCommands::Cancel {
            id,
            reason,
            force,
            json,
        } => {
            cancel_escalation(storage, id, reason, force, json)?;
        }
        engram::cli::EscalationCommands::Cleanup { apply, json } => {
            cleanup_escalations(storage, apply, json)?;
        }
        engram::cli::EscalationCommands::Stats {
            agent_id,
            days,
            json,
        } => {
            show_escalation_stats(storage, agent_id, days, json)?;
        }
    }

    Ok(())
}
/// Handle help command
fn handle_help_command(command: Option<cli::HelpCommands>) -> Result<(), EngramError> {
    match command {
        Some(cli::HelpCommands::Onboarding) => {
            println!("ENGRAM - Task Memory System for LLM Coding Agents");
            println!("==================================================");
            println!();
            println!(
                "PURPOSE: Maintains project state, tasks, and reasoning across coding sessions."
            );
            println!("Enforces disciplined development via Git commit validation requiring task references.");
            println!();
            println!("CORE WORKFLOW:");
            println!("1. engram setup workspace              # Initialize project");
            println!(
                "2. engram task create --title \"...\"    # Create work items (returns UUIDs)"
            );
            println!("3. engram context create --title \"...\" # Add background info");
            println!("4. engram reasoning create --task-id <uuid> # Document decisions");
            println!(
                "5. engram relationship create ...       # Link entities (REQUIRED for validation)"
            );
            println!("6. engram validate hook install        # Enable Git integration");
            println!();
            println!("ESSENTIAL COMMANDS:");
            println!(
                "  task        Create/manage work items (returns UUIDs for commit references)"
            );
            println!("  context     Background information and documentation");
            println!("  reasoning   Decision chains and rationale");
            println!("  relationship Link entities (required: task↔reasoning, task↔context)");
            println!("  validate    Git commit validation and hooks");
            println!();
            println!(
                "JSON I/O: Most commands support --json input/output for programmatic access."
            );
            println!("Git Integration: Commits must reference task UUIDs: \"feat: implement auth [<uuid>]\"");
            println!();
            println!("Use 'engram guide examples' for working command examples.");
        }
        Some(cli::HelpCommands::GettingStarted) => {
            println!("ENGRAM Quick Start for LLM Agents");
            println!("=================================");
            println!();
            println!("STEP 1: Initialize workspace");
            println!("  engram setup workspace");
            println!();
            println!("STEP 2: Create your first task");
            println!("  engram task create --title \"Implement user authentication\"");
            println!("  # Returns UUID like: a1b2c3d4-e5f6-7890-abcd-ef1234567890");
            println!();
            println!("STEP 3: Add supporting entities");
            println!("  engram context create --title \"Auth requirements\" --source \"requirements.md\"");
            println!("  engram reasoning create --task-id <TASK_UUID> --title \"JWT vs Session approach\"");
            println!();
            println!("STEP 4: Create required relationships");
            println!("  engram relationship create \\");
            println!("    --source-id <TASK_UUID> --source-type task \\");
            println!("    --target-id <CONTEXT_UUID> --target-type context \\");
            println!("    --relationship-type references --agent default");
            println!();
            println!("STEP 5: Enable Git validation");
            println!("  engram validate hook install");
            println!();
            println!("STEP 6: Make commits with task references");
            println!("  git commit -m \"feat: add login endpoint [<TASK_UUID>]\"");
            println!();
            println!("For examples with real UUIDs: engram guide examples");
        }
        Some(cli::HelpCommands::Examples) => {
            println!("ENGRAM Command Examples for LLM Agents");
            println!("======================================");
            println!();
            println!("# 1. SETUP WORKFLOW");
            println!("engram setup workspace");
            println!();
            println!("# 2. CREATE ENTITIES");
            println!("# Create task (save UUID for later steps)");
            println!(
                "TASK_ID=$(engram task create --title \"Add OAuth support\" --json | jq -r '.id')"
            );
            println!();
            println!("# Create context");
            println!("CTX_ID=$(engram context create --title \"OAuth 2.0 specification\" --source \"RFC 6749\" --json | jq -r '.id')");
            println!();
            println!("# Create reasoning");
            println!("REASON_ID=$(engram reasoning create --task-id $TASK_ID --title \"Why OAuth over custom auth\" --json | jq -r '.id')");
            println!();
            println!("# 3. CREATE RELATIONSHIPS (REQUIRED FOR VALIDATION)");
            println!("engram relationship create \\");
            println!("  --source-id $TASK_ID --source-type task \\");
            println!("  --target-id $CTX_ID --target-type context \\");
            println!("  --relationship-type references --agent default");
            println!();
            println!("engram relationship create \\");
            println!("  --source-id $TASK_ID --source-type task \\");
            println!("  --target-id $REASON_ID --target-type reasoning \\");
            println!("  --relationship-type references --agent default");
            println!();
            println!("# 4. VALIDATION SETUP");
            println!("engram validate hook install");
            println!("engram validate commit --message \"feat: add OAuth endpoint [$TASK_ID]\" --dry-run");
            println!();
            println!("# 5. JSON PROGRAMMATIC ACCESS");
            println!("# List all tasks as JSON");
            println!("engram task list --agent default | jq '.[].id'");
            println!();
            println!("# Create task from JSON input");
            println!("echo '{{\"title\": \"Database migration\", \"priority\": \"high\"}}' | engram task create --json");
            println!();
            println!("# 6. RELATIONSHIP QUERIES");
            println!("# Find all entities connected to a task");
            println!(
                "engram relationship connected --entity-id $TASK_ID --relationship-type references"
            );
            println!();
            println!("# Find path between entities");
            println!("engram relationship find-path --source-id $TASK_ID --target-id $CTX_ID");
            println!();
            println!("# 7. VALIDATION WORKFLOW");
            println!("# Test commit validation");
            println!(
                "engram validate commit --message \"feat: implement OAuth [$TASK_ID]\" --dry-run"
            );
            println!();
            println!("# Check validation setup");
            println!("engram validate hook status");
            println!();
            println!("For more details: engram <command> --help");
        }
        None => {
            println!("ENGRAM Guide - Task Memory System for LLM Coding Agents");
            println!("==========================================================");
            println!();
            println!("Available guide sections:");
            println!("  getting-started  Step-by-step setup and first tasks");
            println!("  examples         Complete command examples with real workflows");
            println!("  onboarding       Overview and core concepts");
            println!();
            println!("Usage:");
            println!("  engram guide getting-started   # Quick start tutorial");
            println!("  engram guide examples          # Copy-paste examples");
            println!("  engram guide onboarding        # Detailed overview");
            println!();
            println!("For specific command help:");
            println!("  engram <command> --help        # Help for individual commands");
            println!("  engram --help                  # Show all available commands");
        }
    }

    Ok(())
}
