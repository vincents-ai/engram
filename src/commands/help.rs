//! Help/guide command handler

use engram::cli;
use engram::error::EngramError;

/// Handle help command
pub fn handle_help_command(command: Option<cli::HelpCommands>) -> Result<(), EngramError> {
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
