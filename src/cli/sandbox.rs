//! Sandbox command implementations

use crate::entities::{AgentSandbox, Entity, SandboxLevel};
use crate::error::EngramError;
use crate::storage::Storage;
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Read, Write};

/// Sandbox input structure for JSON
#[derive(Debug, Deserialize)]
pub struct SandboxInput {
    pub agent_id: String,
    pub sandbox_level: String,
    pub created_by: Option<String>,
    pub agent: Option<String>,
}

/// Sandbox configuration update input
#[derive(Debug, Deserialize)]
pub struct SandboxUpdateInput {
    pub sandbox_level: Option<String>,
    pub permissions: Option<serde_json::Value>,
    pub resource_limits: Option<serde_json::Value>,
}

/// Sandbox validation request
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxValidationRequest {
    pub agent_id: String,
    pub operation: String,
    pub resource_type: String,
    pub parameters: serde_json::Value,
}

/// Sandbox commands
#[derive(Subcommand)]
pub enum SandboxCommands {
    /// Create a new agent sandbox configuration
    Create {
        /// Agent ID to create sandbox for
        #[arg(long, short)]
        agent_id: Option<String>,

        /// Sandbox security level (unrestricted, standard, restricted, isolated, training)
        #[arg(long, short, default_value = "standard")]
        level: String,

        /// Created by user
        #[arg(long)]
        created_by: Option<String>,

        /// Agent name
        #[arg(long)]
        agent: Option<String>,

        /// Read input from stdin as JSON
        #[arg(long, conflicts_with_all = ["agent_id"])]
        stdin: bool,

        /// Read input from file as JSON
        #[arg(long, conflicts_with_all = ["agent_id", "stdin"])]
        file: Option<String>,

        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },
    /// List all sandbox configurations
    List {
        /// Filter by agent ID
        #[arg(long)]
        agent_id: Option<String>,

        /// Filter by sandbox level
        #[arg(long)]
        level: Option<String>,

        /// Agent to filter by
        #[arg(long, short)]
        agent: Option<String>,

        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },
    /// Get sandbox configuration details
    Get {
        /// Sandbox ID
        #[arg()]
        id: String,

        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },
    /// Update sandbox configuration
    Update {
        /// Sandbox ID
        #[arg()]
        id: String,

        /// New sandbox level
        #[arg(long)]
        level: Option<String>,

        /// Read update data from stdin as JSON
        #[arg(long)]
        stdin: bool,

        /// Read update data from file as JSON
        #[arg(long, conflicts_with = "stdin")]
        file: Option<String>,

        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },
    /// Delete sandbox configuration
    Delete {
        /// Sandbox ID
        #[arg()]
        id: String,

        /// Confirm deletion without prompt
        #[arg(long)]
        force: bool,
    },
    /// Validate an operation against sandbox constraints
    Validate {
        /// Agent ID
        #[arg(long, short)]
        agent_id: Option<String>,

        /// Operation to validate
        #[arg(long, short)]
        operation: Option<String>,

        /// Resource type
        #[arg(long, short)]
        resource_type: Option<String>,

        /// Read validation request from stdin as JSON
        #[arg(long, conflicts_with_all = ["agent_id", "operation"])]
        stdin: bool,

        /// Read validation request from file as JSON
        #[arg(long, conflicts_with_all = ["agent_id", "operation", "stdin"])]
        file: Option<String>,

        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },
    /// Show sandbox statistics and usage
    Stats {
        /// Agent ID to show stats for
        #[arg(long, short)]
        agent_id: Option<String>,

        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },
    /// Reset sandbox configuration to defaults
    Reset {
        /// Agent ID to reset
        #[arg()]
        agent_id: String,

        /// Confirm reset without prompt
        #[arg(long)]
        force: bool,

        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },
}

/// Create a new sandbox configuration
pub fn create_sandbox<S: Storage>(
    storage: &mut S,
    agent_id: Option<String>,
    level: String,
    created_by: Option<String>,
    agent: Option<String>,
    stdin: bool,
    file: Option<String>,
    json: bool,
) -> Result<(), EngramError> {
    let sandbox_input = if stdin {
        read_sandbox_input_from_stdin()?
    } else if let Some(file_path) = file {
        read_sandbox_input_from_file(&file_path)?
    } else {
        let agent_id =
            agent_id.ok_or_else(|| EngramError::Validation("Agent ID is required".to_string()))?;

        SandboxInput {
            agent_id,
            sandbox_level: level,
            created_by,
            agent,
        }
    };

    let sandbox_level = parse_sandbox_level(&sandbox_input.sandbox_level)?;
    let created_by = sandbox_input
        .created_by
        .unwrap_or_else(|| "default".to_string());
    let agent = sandbox_input.agent.unwrap_or_else(|| "default".to_string());

    let sandbox = AgentSandbox::new(sandbox_input.agent_id, sandbox_level, created_by, agent);

    storage.store(&sandbox.to_generic())?;

    if json {
        println!("{}", serde_json::to_string_pretty(&sandbox.to_generic())?);
    } else {
        println!("✅ Sandbox created successfully:");
        println!("  ID: {}", sandbox.id);
        println!("  Agent: {}", sandbox.agent_id);
        println!("  Level: {:?}", sandbox.sandbox_level);
        println!("  Created by: {}", sandbox.created_by);
    }

    Ok(())
}

/// List sandbox configurations
pub fn list_sandboxes<S: Storage>(
    storage: &S,
    agent_id: Option<String>,
    level: Option<String>,
    agent: Option<String>,
    json: bool,
) -> Result<(), EngramError> {
    let ids = storage.list_ids("agent_sandbox")?;
    let mut sandboxes = Vec::new();

    for id in ids {
        if let Ok(Some(entity)) = storage.get(&id, "agent_sandbox") {
            match AgentSandbox::from_generic(entity) {
                Ok(sandbox) => {
                    // Apply filters
                    if let Some(filter_agent_id) = &agent_id {
                        if sandbox.agent_id != *filter_agent_id {
                            continue;
                        }
                    }

                    if let Some(filter_agent) = &agent {
                        if sandbox.agent != *filter_agent {
                            continue;
                        }
                    }

                    if let Some(filter_level) = &level {
                        if format!("{:?}", sandbox.sandbox_level).to_lowercase()
                            != filter_level.to_lowercase()
                        {
                            continue;
                        }
                    }

                    sandboxes.push(sandbox);
                }
                Err(_) => continue,
            }
        }
    }

    if json {
        let generic_sandboxes: Vec<_> = sandboxes.iter().map(|s| s.to_generic()).collect();
        println!("{}", serde_json::to_string_pretty(&generic_sandboxes)?);
    } else {
        if sandboxes.is_empty() {
            println!("No sandbox configurations found.");
        } else {
            println!("📋 Sandbox Configurations ({} found):", sandboxes.len());
            for sandbox in sandboxes {
                println!(
                    "  • {} [{}] - {} ({:?})",
                    sandbox.id, sandbox.agent_id, sandbox.agent, sandbox.sandbox_level
                );
            }
        }
    }

    Ok(())
}

/// Get sandbox configuration details
pub fn get_sandbox<S: Storage>(storage: &S, id: String, json: bool) -> Result<(), EngramError> {
    match storage.get(&id, "agent_sandbox")? {
        Some(entity) => {
            let sandbox =
                AgentSandbox::from_generic(entity).map_err(|e| EngramError::Validation(e))?;

            if json {
                println!("{}", serde_json::to_string_pretty(&sandbox.to_generic())?);
            } else {
                println!("🔒 Sandbox Configuration:");
                println!("  ID: {}", sandbox.id);
                println!("  Agent: {}", sandbox.agent_id);
                println!("  Level: {:?}", sandbox.sandbox_level);
                println!("  Created by: {}", sandbox.created_by);
                println!("  Created at: {}", sandbox.created_at);
                println!("  Last modified: {}", sandbox.last_modified);
                println!("  Violation count: {}", sandbox.violation_count);

                if !sandbox.metadata.is_empty() {
                    println!(
                        "  Metadata: {}",
                        serde_json::to_string_pretty(&sandbox.metadata)?
                    );
                }
            }
        }
        None => {
            return Err(EngramError::NotFound(format!(
                "Sandbox with ID {} not found",
                id
            )));
        }
    }

    Ok(())
}

/// Update sandbox configuration
pub fn update_sandbox<S: Storage>(
    storage: &mut S,
    id: String,
    level: Option<String>,
    stdin: bool,
    file: Option<String>,
    json: bool,
) -> Result<(), EngramError> {
    let mut sandbox = match storage.get(&id, "agent_sandbox")? {
        Some(entity) => {
            AgentSandbox::from_generic(entity).map_err(|e| EngramError::Validation(e))?
        }
        None => {
            return Err(EngramError::NotFound(format!(
                "Sandbox with ID {} not found",
                id
            )))
        }
    };

    if stdin || file.is_some() {
        let update_input = if stdin {
            read_update_input_from_stdin()?
        } else {
            read_update_input_from_file(&file.unwrap())?
        };

        if let Some(new_level) = update_input.sandbox_level {
            sandbox.sandbox_level = parse_sandbox_level(&new_level)?;
        }
    } else if let Some(new_level) = level {
        sandbox.sandbox_level = parse_sandbox_level(&new_level)?;
    }

    sandbox.last_modified = chrono::Utc::now();

    storage.store(&sandbox.to_generic())?;

    if json {
        println!("{}", serde_json::to_string_pretty(&sandbox.to_generic())?);
    } else {
        println!("✅ Sandbox updated successfully:");
        println!("  ID: {}", sandbox.id);
        println!("  Level: {:?}", sandbox.sandbox_level);
    }

    Ok(())
}

/// Delete sandbox configuration
pub fn delete_sandbox<S: Storage>(
    storage: &mut S,
    id: String,
    force: bool,
) -> Result<(), EngramError> {
    if !force {
        print!("Are you sure you want to delete sandbox {}? (y/N): ", id);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if !input.trim().to_lowercase().starts_with('y') {
            println!("Operation cancelled.");
            return Ok(());
        }
    }

    storage.delete(&id, "agent_sandbox")?;
    println!("✅ Sandbox {} deleted successfully.", id);

    Ok(())
}

/// Validate an operation against sandbox constraints (simplified implementation)
pub fn validate_operation<S: Storage>(
    _storage: &S,
    agent_id: Option<String>,
    operation: Option<String>,
    resource_type: Option<String>,
    stdin: bool,
    file: Option<String>,
    json: bool,
) -> Result<(), EngramError> {
    let validation_request = if stdin {
        read_validation_request_from_stdin()?
    } else if let Some(file_path) = file {
        read_validation_request_from_file(&file_path)?
    } else {
        let agent_id =
            agent_id.ok_or_else(|| EngramError::Validation("Agent ID is required".to_string()))?;
        let operation = operation
            .ok_or_else(|| EngramError::Validation("Operation is required".to_string()))?;
        let resource_type = resource_type
            .ok_or_else(|| EngramError::Validation("Resource type is required".to_string()))?;

        SandboxValidationRequest {
            agent_id,
            operation,
            resource_type,
            parameters: serde_json::Value::Object(serde_json::Map::new()),
        }
    };

    // Simplified validation - just allow for now
    let result = serde_json::json!({
        "status": "allowed",
        "agent_id": validation_request.agent_id,
        "operation": validation_request.operation,
        "resource_type": validation_request.resource_type
    });

    if json {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("✅ Operation allowed:");
        println!("  Agent: {}", validation_request.agent_id);
        println!("  Operation: {}", validation_request.operation);
        println!("  Resource: {}", validation_request.resource_type);
    }

    Ok(())
}

/// Show sandbox statistics and usage
pub fn show_stats<S: Storage>(
    storage: &S,
    agent_id: Option<String>,
    json: bool,
) -> Result<(), EngramError> {
    let ids = storage.list_ids("agent_sandbox")?;
    let mut total_sandboxes = 0;
    let mut level_counts = std::collections::HashMap::new();
    let mut agent_sandboxes = Vec::new();

    for id in ids {
        if let Ok(Some(entity)) = storage.get(&id, "agent_sandbox") {
            if let Ok(sandbox) = AgentSandbox::from_generic(entity) {
                if let Some(filter_agent_id) = &agent_id {
                    if sandbox.agent_id == *filter_agent_id {
                        agent_sandboxes.push(sandbox.clone());
                    }
                } else {
                    total_sandboxes += 1;
                    *level_counts
                        .entry(format!("{:?}", sandbox.sandbox_level))
                        .or_insert(0) += 1;
                    agent_sandboxes.push(sandbox);
                }
            }
        }
    }

    if json {
        let stats = serde_json::json!({
            "total_sandboxes": if agent_id.is_some() { agent_sandboxes.len() } else { total_sandboxes },
            "level_distribution": level_counts,
            "sandboxes": agent_sandboxes.iter().map(|s| serde_json::json!({
                "id": s.id,
                "agent_id": s.agent_id,
                "level": format!("{:?}", s.sandbox_level),
                "violation_count": s.violation_count
            })).collect::<Vec<_>>()
        });
        println!("{}", serde_json::to_string_pretty(&stats)?);
    } else {
        if let Some(filter_agent_id) = agent_id {
            println!("📊 Sandbox Stats for Agent: {}", filter_agent_id);
            if agent_sandboxes.is_empty() {
                println!("  No sandbox configuration found for this agent.");
            } else {
                for sandbox in agent_sandboxes {
                    println!("  • Level: {:?}", sandbox.sandbox_level);
                    println!("    Violations: {}", sandbox.violation_count);
                }
            }
        } else {
            println!("📊 Sandbox Statistics:");
            println!("  Total sandboxes: {}", total_sandboxes);
            println!("  Level distribution:");
            for (level, count) in level_counts {
                println!("    {}: {}", level, count);
            }
        }
    }

    Ok(())
}

/// Reset sandbox configuration to defaults
pub fn reset_sandbox<S: Storage>(
    storage: &mut S,
    agent_id: String,
    force: bool,
    json: bool,
) -> Result<(), EngramError> {
    if !force {
        print!(
            "Are you sure you want to reset sandbox configuration for agent {}? (y/N): ",
            agent_id
        );
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if !input.trim().to_lowercase().starts_with('y') {
            println!("Operation cancelled.");
            return Ok(());
        }
    }

    // Find existing sandbox for this agent
    let ids = storage.list_ids("agent_sandbox")?;
    let mut existing_sandbox = None;

    for id in ids {
        if let Ok(Some(entity)) = storage.get(&id, "agent_sandbox") {
            if let Ok(sandbox) = AgentSandbox::from_generic(entity) {
                if sandbox.agent_id == agent_id {
                    existing_sandbox = Some(sandbox);
                    break;
                }
            }
        }
    }

    let new_sandbox = if let Some(mut sandbox) = existing_sandbox {
        // Reset to standard level with default configuration
        sandbox.sandbox_level = SandboxLevel::Standard;
        sandbox.violation_count = 0;
        sandbox.last_modified = chrono::Utc::now();
        sandbox.metadata.clear();

        storage.store(&sandbox.to_generic())?;
        sandbox
    } else {
        // Create new default sandbox
        let sandbox = AgentSandbox::new(
            agent_id.clone(),
            SandboxLevel::Standard,
            "system".to_string(),
            "default".to_string(),
        );

        storage.store(&sandbox.to_generic())?;
        sandbox
    };

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&new_sandbox.to_generic())?
        );
    } else {
        println!("✅ Sandbox reset successfully:");
        println!("  Agent: {}", new_sandbox.agent_id);
        println!("  Level: {:?}", new_sandbox.sandbox_level);
        println!("  ID: {}", new_sandbox.id);
    }

    Ok(())
}

// Helper functions

fn read_sandbox_input_from_stdin() -> Result<SandboxInput, EngramError> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    Ok(serde_json::from_str(&input)?)
}

fn read_sandbox_input_from_file(file_path: &str) -> Result<SandboxInput, EngramError> {
    let content = fs::read_to_string(file_path)?;
    Ok(serde_json::from_str(&content)?)
}

fn read_update_input_from_stdin() -> Result<SandboxUpdateInput, EngramError> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    Ok(serde_json::from_str(&input)?)
}

fn read_update_input_from_file(file_path: &str) -> Result<SandboxUpdateInput, EngramError> {
    let content = fs::read_to_string(file_path)?;
    Ok(serde_json::from_str(&content)?)
}

fn read_validation_request_from_stdin() -> Result<SandboxValidationRequest, EngramError> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    Ok(serde_json::from_str(&input)?)
}

fn read_validation_request_from_file(
    file_path: &str,
) -> Result<SandboxValidationRequest, EngramError> {
    let content = fs::read_to_string(file_path)?;
    Ok(serde_json::from_str(&content)?)
}

fn parse_sandbox_level(level: &str) -> Result<SandboxLevel, EngramError> {
    match level.to_lowercase().as_str() {
        "unrestricted" => Ok(SandboxLevel::Unrestricted),
        "standard" => Ok(SandboxLevel::Standard),
        "restricted" => Ok(SandboxLevel::Restricted),
        "isolated" => Ok(SandboxLevel::Isolated),
        "training" => Ok(SandboxLevel::Training),
        _ => Err(EngramError::Validation(
            format!("Invalid sandbox level: {}. Must be one of: unrestricted, standard, restricted, isolated, training", level)
        )),
    }
}
