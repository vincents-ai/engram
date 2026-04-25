//! Setup, convert, and test command handlers

use engram::cli;
use engram::error::EngramError;

/// Handle setup commands
pub fn handle_setup_command(command: cli::SetupCommands) -> Result<(), EngramError> {
    match command {
        cli::SetupCommands::Workspace => cli::setup_workspace(None)?,
        cli::SetupCommands::Agent {
            name,
            agent_type,
            specialization,
            email,
            persona,
        } => {
            cli::setup_agent(
                &name,
                &agent_type,
                specialization.as_deref(),
                email.as_deref(),
                persona.as_deref(),
                None,
            )?;
        }
        cli::SetupCommands::Skills { force, dir, tool } => {
            cli::handle_skills_command(
                &mut std::io::stdout(),
                force,
                dir.as_deref(),
                tool.as_deref(),
                None,
            )?;
        }
        cli::SetupCommands::Prompts { path } => {
            cli::setup_prompts(path.as_deref(), None)?;
        }
    }
    Ok(())
}

/// Handle convert command
pub fn handle_convert_command(from: &str, file: &str) -> Result<(), EngramError> {
    println!("Converting from {} file: {}", from, file);
    println!("Conversion functionality will be implemented in a future version");
    Ok(())
}

/// Handle test command
pub fn handle_test_command() -> Result<(), EngramError> {
    println!("Engram Test Suite");
    println!("==================");

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
