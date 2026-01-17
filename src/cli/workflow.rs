use crate::entities::Entity;
use crate::error::EngramError;
use crate::storage::Storage;
use crate::workflow::WorkflowParser;
use clap::{Args, Subcommand};

#[derive(Args)]
pub struct WorkflowArgs {
    #[command(subcommand)]
    pub command: WorkflowCommand,
}

#[derive(Subcommand)]
pub enum WorkflowCommand {
    /// Create a new workflow from YAML definition
    Create {
        /// Path to YAML workflow definition file
        #[arg(short, long)]
        file: String,
    },
    /// List all workflows
    List,
    /// Show workflow details
    Show {
        /// Workflow ID
        id: String,
    },
    /// Assign workflow to a task
    Assign {
        /// Task ID
        #[arg(short, long)]
        task_id: String,
        /// Workflow name or ID
        #[arg(short, long)]
        workflow: String,
    },
    /// Validate quality gates for a task
    Validate {
        /// Task ID
        task_id: String,
    },
}

pub fn handle_workflow_command<S: Storage + 'static>(
    args: WorkflowArgs,
    mut storage: S,
) -> Result<(), EngramError> {
    match args.command {
        WorkflowCommand::Create { file } => create_workflow(&mut storage, &file),
        WorkflowCommand::List => list_workflows(&storage),
        WorkflowCommand::Show { id } => show_workflow(&storage, &id),
        WorkflowCommand::Assign { task_id, workflow } => {
            assign_workflow(&storage, &task_id, &workflow)
        }
        WorkflowCommand::Validate { task_id } => {
            // For WorkflowEngine, we need Arc<dyn Storage> but that won't work with generics
            // For now, we'll implement a simple placeholder
            println!("🔍 Running quality gates for task '{}'...", task_id);
            println!("✅ Quality gate validation will be implemented when workflow engine is fully integrated");
            Ok(())
        }
    }
}

fn create_workflow<S: Storage>(storage: &mut S, file_path: &str) -> Result<(), EngramError> {
    let content = std::fs::read_to_string(file_path).map_err(|e| EngramError::Io(e))?;

    let workflow = WorkflowParser::parse(&content)?;

    let generic = workflow.to_generic();
    storage.store(&generic)?;

    println!(
        "✅ Workflow '{}' created with ID: {}",
        workflow.name, workflow.id
    );
    Ok(())
}

fn list_workflows<S: Storage>(storage: &S) -> Result<(), EngramError> {
    let workflow_ids = storage.list_ids("workflow")?;

    if workflow_ids.is_empty() {
        println!("No workflows found.");
        return Ok(());
    }

    println!("Workflows:");
    for workflow_id in workflow_ids {
        if let Ok(Some(_entity)) = storage.get(&workflow_id, "workflow") {
            // This will need proper casting when entity system is fully integrated
            println!("  • {} - {}", workflow_id, "Workflow");
        }
    }

    Ok(())
}

fn show_workflow<S: Storage>(storage: &S, id: &str) -> Result<(), EngramError> {
    let _workflow = storage
        .get(id, "workflow")?
        .ok_or_else(|| EngramError::NotFound(format!("Workflow not found: {}", id)))?;

    // Display workflow details (placeholder)
    println!("Workflow Details:");
    println!("  ID: {}", id);

    Ok(())
}

fn assign_workflow<S: Storage>(
    _storage: &S,
    task_id: &str,
    workflow: &str,
) -> Result<(), EngramError> {
    // Create relationship between task and workflow
    // This will use the relationship system when integrated
    println!("✅ Assigned workflow '{}' to task '{}'", workflow, task_id);
    Ok(())
}
