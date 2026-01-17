//! Main entry point for Engram CLI

use clap::Parser;
use engram::{
    cli::{self, handle_relationship_command, handle_validation_command},
    error::EngramError,
    storage::GitStorage,
};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), EngramError> {
    let args = cli::Cli::parse();

    match args.command {
        cli::Commands::Setup { command } => handle_setup_command(command)?,
        cli::Commands::Convert { from, file } => handle_convert_command(&from, &file)?,
        cli::Commands::Test => handle_test_command()?,
        cli::Commands::Task { command } => {
            let mut storage = GitStorage::new(".", "default")?;
            handle_task_command(command, &mut storage)?;
        }
        cli::Commands::Context { command } => {
            let mut storage = GitStorage::new(".", "default")?;
            handle_context_command(command, &mut storage)?;
        }
        cli::Commands::Reasoning { command } => {
            let mut storage = GitStorage::new(".", "default")?;
            handle_reasoning_command(command, &mut storage)?;
        }
        cli::Commands::Knowledge { command } => {
            let mut storage = GitStorage::new(".", "default")?;
            handle_knowledge_command(command, &mut storage)?;
        }
        cli::Commands::Session { command } => {
            let mut storage = GitStorage::new(".", "default")?;
            handle_session_command(command, &mut storage)?;
        }
        cli::Commands::Compliance { command } => {
            let mut storage = GitStorage::new(".", "default")?;
            handle_compliance_command(command, &mut storage)?;
        }
        cli::Commands::Rule { command } => {
            let mut storage = GitStorage::new(".", "default")?;
            handle_rule_command(command, &mut storage)?;
        }
        cli::Commands::Standard { command } => {
            let mut storage = GitStorage::new(".", "default")?;
            handle_standard_command(command, &mut storage)?;
        }
        cli::Commands::Adr { command } => {
            let mut storage = GitStorage::new(".", "default")?;
            handle_adr_command(command, &mut storage)?;
        }
        cli::Commands::Workflow { command } => {
            let mut storage = GitStorage::new(".", "default")?;
            handle_workflow_command(command, &mut storage)?;
        }
        cli::Commands::Relationship { command } => {
            let mut storage = GitStorage::new(".", "default")?;
            handle_relationship_command(&mut storage, command)?;
        }
        cli::Commands::Validate { command } => {
            let storage = GitStorage::new(".", "default")?;
            handle_validation_command(command, storage)?;
        }
        cli::Commands::Sync { command } => {
            let mut storage = GitStorage::new(".", "default")?;
            engram::cli::sync::handle_sync_command(&mut storage, &command)?;
        }
        cli::Commands::Update => handle_update_command()?,
        cli::Commands::Migration => handle_migration_command()?,
        cli::Commands::Guide { command } => handle_help_command(command)?,
    }

    Ok(())
}

/// Handle setup commands
fn handle_setup_command(command: cli::SetupCommands) -> Result<(), EngramError> {
    match command {
        cli::SetupCommands::Workspace => cli::setup_workspace()?,
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
            )?;
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
fn handle_task_command<S: engram::storage::Storage>(
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
        cli::TaskCommands::Delete { id } => {
            cli::delete_task(storage, &id)?;
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
            tags,
            title_stdin,
            title_file,
            json,
            json_file,
        } => {
            cli::create_reasoning(
                storage,
                title,
                task_id,
                agent,
                confidence,
                tags,
                title_stdin,
                title_file,
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
            let storage_for_workflow = GitStorage::new(".", "default")?;
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
            let storage_for_workflow = GitStorage::new(".", "default")?;
            cli::execute_workflow_transition(storage_for_workflow, instance_id, transition, agent)?;
        }
        cli::WorkflowCommands::Status { instance_id } => {
            let storage_for_workflow = GitStorage::new(".", "default")?;
            cli::get_workflow_instance_status(storage_for_workflow, instance_id)?;
        }
        cli::WorkflowCommands::Instances {
            workflow_id,
            agent,
            running_only,
        } => {
            let storage_for_workflow = GitStorage::new(".", "default")?;
            cli::list_workflow_instances(storage_for_workflow, workflow_id, agent, running_only)?;
        }
        cli::WorkflowCommands::Cancel {
            instance_id,
            agent,
            reason,
        } => {
            let storage_for_workflow = GitStorage::new(".", "default")?;
            cli::cancel_workflow_instance(storage_for_workflow, instance_id, agent, reason)?;
        }
    }
    Ok(())
}

/// Handle update command
fn handle_update_command() -> Result<(), EngramError> {
    println!("Update functionality will be implemented in a future version");
    Ok(())
}

/// Handle migration command
fn handle_migration_command() -> Result<(), EngramError> {
    println!("Migration functionality will be implemented in a future version");
    Ok(())
}

/// Handle help command
fn handle_help_command(_command: Option<cli::HelpCommands>) -> Result<(), EngramError> {
    println!("Engram - Distributed Memory System for AI Agents");
    println!();
    println!("Available commands:");
    println!("  setup      Initialize workspace or agent");
    println!("  convert     Convert from other formats");
    println!("  test        Run test suite");
    println!("  task        Manage tasks");
    println!("  context     Manage context");
    println!("  reasoning    Manage reasoning chains");
    println!("  knowledge   Manage knowledge");
    println!("  session     Manage sessions");
    println!("  compliance  Manage compliance requirements");
    println!("  rules       Manage rules");
    println!("  standards   Manage standards");
    println!("  adr         Manage architectural decision records");
    println!("  workflow    Manage workflows");
    println!("  sync        Synchronize between agents");
    println!("  update      Update project files with engram blocks");
    println!("  migration   Migrate storage to hierarchical organization");
    println!("  help        Show help and onboarding information");
    println!();
    println!("Use 'engram <command> --help' for detailed help on each command.");

    Ok(())
}
