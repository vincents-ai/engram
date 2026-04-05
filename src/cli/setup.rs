//! Setup command implementations

use crate::error::EngramError;
use serde::Serialize;
use std::env;
use std::fs;
use std::path::PathBuf;

/// Setup workspace command
pub fn setup_workspace(root_dir: Option<PathBuf>) -> Result<(), EngramError> {
    let engram_dir = root_dir
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".engram");
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
    root_dir: Option<PathBuf>,
) -> Result<(), EngramError> {
    let engram_dir = root_dir
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".engram");
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

/// Setup OpenCode skills command
pub fn setup_skills(config_dir: Option<PathBuf>, force: bool) -> Result<(), EngramError> {
    use crate::cli::skills::{install_skills, resolve_skills_source, scan_skills_from_dir};

    let opencode_dir = if let Some(dir) = config_dir {
        dir.join(".config").join("opencode")
    } else {
        env::var("HOME")
            .map(|home| PathBuf::from(home).join(".config").join("opencode"))
            .map_err(|_| EngramError::Validation("HOME environment variable not set".to_string()))?
    };

    let skills_dir = opencode_dir.join("skills");
    let source_dir = resolve_skills_source(None);

    println!("📂 Scanning skills from: {:?}", source_dir);
    let skills = scan_skills_from_dir(&source_dir)?;

    if skills.is_empty() {
        println!("⚠️  No skill files found in {:?}", source_dir);
        return Ok(());
    }

    println!("📦 Found {} skills", skills.len());
    println!();

    let mut writer = Vec::new();
    let (installed, updated, skipped) = install_skills(&mut writer, &skills, &skills_dir, force)?;
    let output = String::from_utf8(writer).map_err(|e| EngramError::Validation(e.to_string()))?;
    print!("{}", output);

    println!();
    println!("🎉 OpenCode skills setup complete!");
    println!("📁 Skills installed to: {:?}", skills_dir);
    println!(
        "📊 Installed: {}  Updated: {}  Skipped: {}",
        installed, updated, skipped
    );
    println!();
    println!("💡 Skills are now available in OpenCode with 'engram:' prefix");
    println!("   Example: @general can now use 'engram:use-engram-memory'");
    println!();
    println!("📖 To use skills:");
    println!("   1. Restart OpenCode to reload skills");
    println!("   2. Use @mention with skill name");
    println!("   3. Or call skill() tool with skill name");

    Ok(())
}

/// Setup OpenCode prompts command
pub fn setup_prompts(
    prompts_path: Option<&str>,
    config_dir: Option<PathBuf>,
) -> Result<(), EngramError> {
    let prompts_source = prompts_path.unwrap_or("./prompts");
    let prompts_source_path = PathBuf::from(prompts_source);

    // Get OpenCode config directory
    let opencode_dir = if let Some(dir) = config_dir {
        dir.join(".config").join("opencode")
    } else {
        env::var("HOME")
            .map(|home| PathBuf::from(home).join(".config").join("opencode"))
            .map_err(|_| EngramError::Validation("HOME environment variable not set".to_string()))?
    };

    let opencode_prompts_dir = opencode_dir.join("prompts");
    fs::create_dir_all(&opencode_prompts_dir).map_err(EngramError::Io)?;

    // Check if source prompts directory exists
    if !prompts_source_path.exists() {
        return Err(EngramError::Validation(format!(
            "Prompts source directory does not exist: {:?}",
            prompts_source_path
        )));
    }

    // Categories to install
    let categories = [
        (
            "agents",
            "Specialized subagents like Researcher, Coder, Reviewer",
        ),
        (
            "pipelines",
            "AI workflows for Bug Triage, Feature Dev, Refactoring",
        ),
        (
            "compliance",
            "Prompts for Security, GDPR, and Audit standards",
        ),
    ];

    let mut installed_count = 0;

    for (category, description) in categories {
        let source_dir = prompts_source_path.join(category);
        let target_dir = opencode_prompts_dir.join(category);

        if source_dir.exists() {
            // Copy entire category directory
            copy_dir_recursive(&source_dir, &target_dir)?;
            println!(
                "✅ Installed prompts category: {} ({})",
                category, description
            );
            installed_count += 1;
        } else {
            println!("⚠️  Prompts category '{}' not found in source", category);
        }
    }

    println!();
    println!("🎉 OpenCode prompts setup complete!");
    println!("📁 Prompts installed to: {:?}", opencode_prompts_dir);
    println!("📊 Total categories installed: {}", installed_count);
    println!();
    println!("💡 Prompts are now available in OpenCode");
    println!("   Skills: Guides like Git Workflow, Testing Guidelines");
    println!("   Agents: Specialized subagents like Researcher, Coder, Reviewer");
    println!("   Pipelines: AI workflows for Bug Triage, Feature Dev, Refactoring");
    println!("   Compliance: Prompts for Security, GDPR, and Audit standards");

    Ok(())
}

/// Helper function to recursively copy directories
fn copy_dir_recursive(source: &PathBuf, target: &PathBuf) -> Result<(), EngramError> {
    fs::create_dir_all(target).map_err(EngramError::Io)?;

    for entry in fs::read_dir(source).map_err(EngramError::Io)? {
        let entry = entry.map_err(EngramError::Io)?;
        let source_path = entry.path();
        let target_path = target.join(entry.file_name().to_str().unwrap());

        if source_path.is_dir() {
            copy_dir_recursive(&source_path, &target_path)?;
        } else {
            fs::copy(&source_path, &target_path).map_err(EngramError::Io)?;
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_setup_workspace() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path().to_path_buf();

        setup_workspace(Some(root.clone())).unwrap();

        let engram_dir = root.join(".engram");
        assert!(engram_dir.exists());
        assert!(engram_dir.join("agents").exists());
        assert!(engram_dir.join("workspaces").exists());
        assert!(engram_dir.join("templates").exists());
        assert!(engram_dir.join("config.yaml").exists());

        // Verify config content
        let config_content = fs::read_to_string(engram_dir.join("config.yaml")).unwrap();
        assert!(config_content.contains("agents:"));
        assert!(config_content.contains("workspaces:"));
    }

    #[test]
    fn test_setup_agent() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path().to_path_buf();

        setup_agent(
            "test-agent",
            "implementation",
            Some("rust"),
            Some("test@example.com"),
            Some(root.clone()),
        )
        .unwrap();

        let agent_file = root.join(".engram/agents/test-agent.yaml");
        assert!(agent_file.exists());

        let content = fs::read_to_string(agent_file).unwrap();
        assert!(content.contains("name: test-agent"));
        assert!(content.contains("agent_type: implementation"));
        assert!(content.contains("specialization: rust"));
        assert!(content.contains("email: test@example.com"));
    }

    #[test]
    fn test_setup_skills() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path().to_path_buf();

        // Setup mock HOME structure
        let config_dir = root.clone();

        setup_skills(Some(config_dir.clone()), false).unwrap();

        let skills_dir = config_dir.join(".config/opencode/skills");
        assert!(skills_dir.exists());

        // Check for a few expected skills
        assert!(skills_dir
            .join("engram-use-engram-memory/SKILL.md")
            .exists());
        assert!(skills_dir
            .join("engram-test-driven-development/SKILL.md")
            .exists());

        // Verify content
        let skill_content =
            fs::read_to_string(skills_dir.join("engram-use-engram-memory/SKILL.md")).unwrap();
        assert!(!skill_content.is_empty());

        // Verify existing skills are not overwritten
        let test_file = skills_dir.join("engram-use-engram-memory/SKILL.md");
        fs::write(&test_file, "modified content").unwrap();

        setup_skills(Some(config_dir), false).unwrap();

        let new_content = fs::read_to_string(test_file).unwrap();
        assert_eq!(new_content, "modified content");
    }

    #[test]
    fn test_setup_prompts() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path().to_path_buf();

        // Create mock source prompts
        let source_prompts = root.join("source_prompts");
        fs::create_dir_all(source_prompts.join("agents")).unwrap();
        fs::write(source_prompts.join("agents/test_agent.md"), "test content").unwrap();

        // Create mock destination config dir
        let config_dir = root.join("config");

        setup_prompts(
            Some(source_prompts.to_str().unwrap()),
            Some(config_dir.clone()),
        )
        .unwrap();

        let installed_prompts = config_dir.join(".config/opencode/prompts");
        assert!(installed_prompts.exists());
        assert!(installed_prompts.join("agents/test_agent.md").exists());

        let content = fs::read_to_string(installed_prompts.join("agents/test_agent.md")).unwrap();
        assert_eq!(content, "test content");
    }

    #[test]
    fn test_setup_prompts_missing_source() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path().to_path_buf();
        let config_dir = root.join("config");

        // Point to non-existent source
        let result = setup_prompts(Some("/non/existent/path"), Some(config_dir));

        assert!(matches!(result, Err(EngramError::Validation(_))));
    }

    #[test]
    fn test_setup_prompts_partial_categories() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path().to_path_buf();

        // Create source with only 'agents' category
        let source_prompts = root.join("source_prompts");
        fs::create_dir_all(source_prompts.join("agents")).unwrap();

        let config_dir = root.join("config");

        // Should succeed but warn (print) about missing categories
        let result = setup_prompts(
            Some(source_prompts.to_str().unwrap()),
            Some(config_dir.clone()),
        );

        assert!(result.is_ok());

        let installed_prompts = config_dir.join(".config/opencode/prompts");
        assert!(installed_prompts.join("agents").exists());
        assert!(!installed_prompts.join("pipelines").exists()); // Wasn't in source
    }
}
