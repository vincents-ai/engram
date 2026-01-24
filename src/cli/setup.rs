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
pub fn setup_skills(config_dir: Option<PathBuf>) -> Result<(), EngramError> {
    use std::env;

    // Get OpenCode config directory
    let opencode_dir = if let Some(dir) = config_dir {
        dir.join(".config").join("opencode")
    } else {
        env::var("HOME")
            .map(|home| PathBuf::from(home).join(".config").join("opencode"))
            .map_err(|_| EngramError::Validation("HOME environment variable not set".to_string()))?
    };

    let skills_dir = opencode_dir.join("skills");
    fs::create_dir_all(&skills_dir).map_err(EngramError::Io)?;

    // List of built-in Engram skills to install with their content
    let skills: &[(&str, &str)] = &[
        // Meta Skills
        (
            "engram-use-engram-memory",
            include_str!("../../skills/meta/use-engram-memory.md"),
        ),
        (
            "engram-delegate-to-agents",
            include_str!("../../skills/meta/delegate-to-agents.md"),
        ),
        (
            "engram-audit-trail",
            include_str!("../../skills/meta/audit-trail.md"),
        ),
        (
            "engram-dispatching-parallel-agents",
            include_str!("../../skills/meta/dispatching-parallel-agents.md"),
        ),
        // Workflow Skills
        (
            "engram-brainstorming",
            include_str!("../../skills/workflow/brainstorming.md"),
        ),
        (
            "engram-writing-plans",
            include_str!("../../skills/workflow/writing-plans.md"),
        ),
        (
            "engram-plan-feature",
            include_str!("../../skills/workflow/plan-feature.md"),
        ),
        (
            "engram-requesting-code-review",
            include_str!("../../skills/workflow/requesting-code-review.md"),
        ),
        // Development Skills
        (
            "engram-test-driven-development",
            include_str!("../../skills/development/test-driven-development.md"),
        ),
        (
            "engram-subagent-driven-development",
            include_str!("../../skills/development/subagent-driven-development.md"),
        ),
        // Debugging Skills
        (
            "engram-systematic-debugging",
            include_str!("../../skills/debugging/systematic-debugging.md"),
        ),
        // Compliance Skills
        (
            "engram-check-compliance",
            include_str!("../../skills/compliance/check-compliance.md"),
        ),
        // Planning Skills (7)
        (
            "engram-risk-assessment",
            include_str!("../../skills/planning/risk-assessment.md"),
        ),
        (
            "engram-spike-investigation",
            include_str!("../../skills/planning/spike-investigation.md"),
        ),
        (
            "engram-dependency-mapping",
            include_str!("../../skills/planning/dependency-mapping.md"),
        ),
        (
            "engram-capacity-planning",
            include_str!("../../skills/planning/capacity-planning.md"),
        ),
        (
            "engram-release-planning",
            include_str!("../../skills/planning/release-planning.md"),
        ),
        (
            "engram-backlog-refinement",
            include_str!("../../skills/planning/backlog-refinement.md"),
        ),
        (
            "engram-roadmap-planning",
            include_str!("../../skills/planning/roadmap-planning.md"),
        ),
        // Documentation Skills (6)
        (
            "engram-adr",
            include_str!("../../skills/documentation/adr.md"),
        ),
        (
            "engram-api-docs",
            include_str!("../../skills/documentation/api-docs.md"),
        ),
        (
            "engram-knowledge-transfer",
            include_str!("../../skills/documentation/knowledge-transfer.md"),
        ),
        (
            "engram-runbooks",
            include_str!("../../skills/documentation/runbooks.md"),
        ),
        (
            "engram-changelog",
            include_str!("../../skills/documentation/changelog.md"),
        ),
        (
            "engram-technical-writing",
            include_str!("../../skills/documentation/technical-writing.md"),
        ),
        // Architecture Skills (8)
        (
            "engram-system-design",
            include_str!("../../skills/architecture/system-design.md"),
        ),
        (
            "engram-security-architecture",
            include_str!("../../skills/architecture/security-architecture.md"),
        ),
        (
            "engram-data-modeling",
            include_str!("../../skills/architecture/data-modeling.md"),
        ),
        (
            "engram-api-design",
            include_str!("../../skills/architecture/api-design.md"),
        ),
        (
            "engram-scalability-analysis",
            include_str!("../../skills/architecture/scalability-analysis.md"),
        ),
        (
            "engram-refactoring-strategy",
            include_str!("../../skills/architecture/refactoring-strategy.md"),
        ),
        (
            "engram-observability-design",
            include_str!("../../skills/architecture/observability-design.md"),
        ),
        (
            "engram-integration-patterns",
            include_str!("../../skills/architecture/integration-patterns.md"),
        ),
        // Quality Skills (5)
        (
            "engram-assumption-validation",
            include_str!("../../skills/quality/assumption-validation.md"),
        ),
        (
            "engram-edge-cases",
            include_str!("../../skills/quality/edge-cases.md"),
        ),
        (
            "engram-tech-debt",
            include_str!("../../skills/quality/tech-debt.md"),
        ),
        (
            "engram-performance-analysis",
            include_str!("../../skills/quality/performance-analysis.md"),
        ),
        (
            "engram-accessibility",
            include_str!("../../skills/quality/accessibility.md"),
        ),
        // Review Skills (4)
        (
            "engram-security-review",
            include_str!("../../skills/review/security-review.md"),
        ),
        (
            "engram-code-quality",
            include_str!("../../skills/review/code-quality.md"),
        ),
        (
            "engram-post-mortem",
            include_str!("../../skills/review/post-mortem.md"),
        ),
        (
            "engram-retrospective",
            include_str!("../../skills/review/retrospective.md"),
        ),
    ];

    let mut installed_count = 0;

    for (skill_name, skill_content) in skills {
        let skill_dir = skills_dir.join(skill_name);

        // Skip if skill already exists (user skill takes precedence)
        if skill_dir.exists() {
            println!(
                "⚠️  Skill '{}' already exists, skipping (user skill preserved)",
                skill_name
            );
            continue;
        }

        fs::create_dir_all(&skill_dir).map_err(EngramError::Io)?;

        // Create SKILL.md file with embedded content
        let skill_file = skill_dir.join("SKILL.md");
        fs::write(&skill_file, skill_content).map_err(EngramError::Io)?;

        println!("✅ Installed skill: {}", skill_name);
        installed_count += 1;
    }

    println!();
    println!("🎉 OpenCode skills setup complete!");
    println!("📁 Skills installed to: {:?}", skills_dir);
    println!("📊 Total skills installed: {}", installed_count);
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
            Some(root.clone())
        ).unwrap();
        
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
        
        setup_skills(Some(config_dir.clone())).unwrap();
        
        let skills_dir = config_dir.join(".config/opencode/skills");
        assert!(skills_dir.exists());
        
        // Check for a few expected skills
        assert!(skills_dir.join("engram-use-engram-memory/SKILL.md").exists());
        assert!(skills_dir.join("engram-test-driven-development/SKILL.md").exists());
        
        // Verify content
        let skill_content = fs::read_to_string(skills_dir.join("engram-use-engram-memory/SKILL.md")).unwrap();
        assert!(!skill_content.is_empty());
        
        // Verify existing skills are not overwritten
        let test_file = skills_dir.join("engram-use-engram-memory/SKILL.md");
        fs::write(&test_file, "modified content").unwrap();
        
        setup_skills(Some(config_dir)).unwrap();
        
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
            Some(config_dir.clone())
        ).unwrap();
        
        let installed_prompts = config_dir.join(".config/opencode/prompts");
        assert!(installed_prompts.exists());
        assert!(installed_prompts.join("agents/test_agent.md").exists());
        
        let content = fs::read_to_string(installed_prompts.join("agents/test_agent.md")).unwrap();
        assert_eq!(content, "test content");
    }
}
