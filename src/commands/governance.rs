//! Governance command handlers: compliance, rules, standards

use engram::cli;
use engram::error::EngramError;

/// Handle compliance commands
pub fn handle_compliance_command<S: engram::storage::Storage>(
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
            all,
            offset,
        } => {
            cli::list_compliance(
                storage,
                agent.as_deref(),
                category.as_deref(),
                limit,
                all,
                offset,
            )?;
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
pub fn handle_rule_command<S: engram::storage::Storage>(
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
            all,
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
                all,
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
pub fn handle_standard_command<S: engram::storage::Storage>(
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
            all,
        } => {
            cli::list_standards(
                &mut std::io::stdout(),
                storage,
                category,
                status,
                search,
                limit,
                offset,
                all,
            )?;
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
