//! Setup command implementations

use crate::error::EngramError;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

/// Setup workspace command
pub fn setup_workspace() -> Result<(), EngramError> {
    let engram_dir = PathBuf::from(".engram");
    fs::create_dir_all(&engram_dir).map_err(EngramError::Io)?;

    // Create subdirectories
    let subdirs = ["agents", "workspaces", "templates"];
    for subdir in &subdirs {
        fs::create_dir_all(engram_dir.join(subdir)).map_err(EngramError::Io)?;
    }

    // Create default configuration
    let config = WorkspaceSetup {
        agents: std::collections::HashMap::from([
            (
                "coder".to_string(),
                AgentSetup {
                    agent_type: "implementation".to_string(),
                    description: "Handles code changes and technical implementation tasks"
                        .to_string(),
                },
            ),
            (
                "reviewer".to_string(),
                AgentSetup {
                    agent_type: "quality_assurance".to_string(),
                    description: "Reviews code for quality and standards compliance".to_string(),
                },
            ),
            (
                "planner".to_string(),
                AgentSetup {
                    agent_type: "architecture".to_string(),
                    description: "Handles system design, planning, and architectural decisions"
                        .to_string(),
                },
            ),
        ]),
        workspaces: std::collections::HashMap::from([(
            "default".to_string(),
            WorkspaceEntry {
                agents: vec!["coder".to_string(), "reviewer".to_string()],
                sync_strategy: "merge_with_conflict_resolution".to_string(),
            },
        )]),
    };

    let config_path = engram_dir.join("config.yaml");
    let config_yaml = serde_yaml::to_string(&config)
        .map_err(|e| EngramError::Validation(format!("Failed to serialize config: {}", e)))?;

    fs::write(&config_path, config_yaml).map_err(EngramError::Io)?;

    println!("✅ Workspace initialized for Engram team collaboration");
    println!("📝 Configuration created at: {:?}", config_path);

    Ok(())
}

/// Setup agent command
pub fn setup_agent(
    name: &str,
    agent_type: &str,
    specialization: Option<&str>,
    email: Option<&str>,
) -> Result<(), EngramError> {
    let engram_dir = PathBuf::from(".engram");
    fs::create_dir_all(&engram_dir.join("agents")).map_err(EngramError::Io)?;

    let agent_profile = AgentProfile {
        name: name.to_string(),
        agent_type: agent_type.to_string(),
        specialization: specialization.map(|s| s.to_string()).unwrap_or_default(),
        email: email.map(|e| e.to_string()),
        created_at: chrono::Utc::now().format("%Y-%m-%d").to_string(),
        version: "1.0.0".to_string(),
        capabilities: vec![
            "memory_storage".to_string(),
            "task_management".to_string(),
            "context_tracking".to_string(),
            "reasoning_chains".to_string(),
            "knowledge_graph".to_string(),
            "team_collaboration".to_string(),
        ],
        commands: vec![
            format!("engram task create --title \"Task Title\" --agent {}", name),
            format!(
                "engram context create --source \"documentation\" --agent {}",
                name
            ),
            format!(
                "engram reasoning create --task-id \"task-123\" --agent {}",
                name
            ),
            "engram sync --agents \"agent1,agent2\" --strategy \"merge_with_conflict_resolution\""
                .to_string(),
        ],
        workspace_access: WorkspaceAccess {
            repositories: vec!["./".to_string()],
            tools: vec!["git".to_string(), "rust".to_string(), "cargo".to_string()],
        },
    };

    let agent_file = engram_dir.join("agents").join(format!("{}.yaml", name));
    let agent_yaml = serde_yaml::to_string(&agent_profile).map_err(|e| {
        EngramError::Validation(format!("Failed to serialize agent profile: {}", e))
    })?;

    fs::write(&agent_file, agent_yaml).map_err(EngramError::Io)?;

    println!("✅ Agent '{}' ({}) profile created", name, agent_type);
    println!("📝 Profile saved to: {:?}", agent_file);

    Ok(())
}

/// Workspace setup configuration structure
#[derive(Debug, Serialize)]
struct WorkspaceSetup {
    agents: std::collections::HashMap<String, AgentSetup>,
    workspaces: std::collections::HashMap<String, WorkspaceEntry>,
}

/// Agent setup configuration
#[derive(Debug, Serialize)]
struct AgentSetup {
    agent_type: String,
    description: String,
}

/// Workspace entry configuration
#[derive(Debug, Serialize)]
struct WorkspaceEntry {
    agents: Vec<String>,
    sync_strategy: String,
}

/// Agent profile structure
#[derive(Debug, Serialize)]
struct AgentProfile {
    name: String,
    agent_type: String,
    specialization: String,
    email: Option<String>,
    created_at: String,
    version: String,
    capabilities: Vec<String>,
    commands: Vec<String>,
    workspace_access: WorkspaceAccess,
}

/// Workspace access configuration
#[derive(Debug, Serialize)]
struct WorkspaceAccess {
    repositories: Vec<String>,
    tools: Vec<String>,
}
