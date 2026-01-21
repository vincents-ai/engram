//! Main Locus integration with Engram storage
//!
//! Handles loading configuration and setting up integration layer

use crate::config::Config;
use crate::entities::TaskStatus;
use crate::error::EngramError;
use crate::locus_integration::LocusIntegration;
use crate::storage::{GitStorage, RelationshipStorage, Storage};
use clap::Subcommand;
use std::io;

pub async fn handle_locus_command<S: Storage + RelationshipStorage>(
    integration: &mut LocusIntegration<S>,
    command: crate::locus_cli::LocusCommands,
) -> io::Result<()> {
    match command {
        crate::locus_cli::LocusCommands::Workflow { subcommand } => {
            handle_workflow_integration(integration, subcommand).await
        }
        crate::locus_cli::LocusCommands::Template { subcommand } => {
            handle_template_integration(integration, subcommand).await
        }
        crate::locus_cli::LocusCommands::Visualize { subcommand } => {
            handle_visualize_integration(integration, subcommand).await
        }
        crate::locus_cli::LocusCommands::Govern { subcommand } => {
            handle_govern_integration(integration, subcommand).await
        }
        crate::locus_cli::LocusCommands::Override { subcommand } => {
            handle_override_integration(integration, subcommand).await
        }
    }
}

async fn handle_workflow_integration<S: Storage + RelationshipStorage>(
    integration: &mut LocusIntegration<S>,
    command: crate::locus_cli::workflow::WorkflowCommands,
) -> io::Result<()> {
    match command {
        crate::locus_cli::workflow::WorkflowCommands::List {
            workflow_type: _,
            format,
        } => {
            println!("📋 Loading workflows from Engram...");

            match integration.get_workflows() {
                Ok(workflows) => {
                    println!("Found {} workflows", workflows.len());

                    for workflow in workflows {
                        match format.as_str() {
                            "json" => {
                                println!("{}", serde_json::to_string_pretty(&workflow).unwrap())
                            }
                            "yaml" => println!("{}", serde_yaml::to_string(&workflow).unwrap()),
                            _ => {
                                println!("🏗️  {}", workflow.title);
                                println!("   {}", workflow.description);
                                println!("   Status: {:?}", workflow.status);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Error loading workflows: {}", e);
                }
            }
        }

        crate::locus_cli::workflow::WorkflowCommands::Show { name, with_history } => {
            println!("📄 Loading workflow details...");

            match integration.get_workflow(&name) {
                Ok(Some(workflow)) => {
                    println!("🏗️  {}", workflow.title);
                    println!("   {}", workflow.description);
                    println!("   Status: {:?}", workflow.status);

                    if with_history {
                        println!("📊 Loading execution history...");
                        println!("🚧 Execution history not yet implemented");
                    }
                }
                Ok(None) => {
                    eprintln!("❌ Workflow '{}' not found", name);
                }
                Err(e) => {
                    eprintln!("❌ Error loading workflow: {}", e);
                }
            }
        }

        _ => {
            println!("🚧 Command requires full Engram integration - not yet implemented");
        }
    }

    Ok(())
}

async fn handle_template_integration<S: Storage + RelationshipStorage>(
    _integration: &mut LocusIntegration<S>,
    _command: crate::locus_cli::template::TemplateCommands,
) -> io::Result<()> {
    println!("🧩 Template management requires Engram integration - not yet implemented");
    Ok(())
}

async fn handle_visualize_integration<S: Storage + RelationshipStorage>(
    integration: &mut LocusIntegration<S>,
    command: crate::locus_cli::visualize::VisualizeCommands,
) -> io::Result<()> {
    match command {
        crate::locus_cli::visualize::VisualizeCommands::Dashboard { agent, real_time } => {
            println!("🚀 Loading dashboard data from Engram...");

            match integration.get_tasks(agent.as_deref()) {
                Ok(tasks) => {
                    println!("📊 Dashboard Overview:");
                    println!("   Total Tasks: {}", tasks.len());

                    let active_tasks = tasks
                        .iter()
                        .filter(|t| t.status == TaskStatus::InProgress)
                        .count();
                    println!("   Active Tasks: {}", active_tasks);

                    let completed_tasks = tasks
                        .iter()
                        .filter(|t| t.status == TaskStatus::Done)
                        .count();
                    println!("   Completed Tasks: {}", completed_tasks);

                    if let Some(agent_name) = agent {
                        println!("   Filtered by agent: {}", agent_name);
                    }

                    if real_time {
                        println!("🔄 Real-time updates: enabled (not yet implemented)");
                    }
                }
                Err(e) => {
                    eprintln!("❌ Error loading dashboard data: {}", e);
                }
            }
        }

        crate::locus_cli::visualize::VisualizeCommands::SystemHealth { detailed, watch } => {
            println!("💓 Loading system health from Engram...");

            match integration.get_system_health() {
                Ok(health) => {
                    println!("🏥 System Health:");
                    println!("   Total Tasks: {}", health.total_tasks);
                    println!("   Total Workflows: {}", health.total_workflows);
                    println!("   Error Count: {}", health.error_count);
                    println!("   Last Updated: {}", health.last_updated);

                    if detailed {
                        println!("🔍 Detailed metrics (not yet implemented)");
                    }

                    if watch {
                        println!("👀 Watch mode: enabled (not yet implemented)");
                    }
                }
                Err(e) => {
                    eprintln!("❌ Error loading system health: {}", e);
                }
            }
        }

        _ => {
            println!("🚧 Visualization command requires Engram integration - not yet implemented");
        }
    }

    Ok(())
}

async fn handle_govern_integration<S: Storage + RelationshipStorage>(
    _integration: &mut LocusIntegration<S>,
    _command: crate::locus_cli::govern::GovernCommands,
) -> io::Result<()> {
    println!("⚖️ Governance requires Engram integration - not yet implemented");
    Ok(())
}

async fn handle_override_integration<S: Storage + RelationshipStorage>(
    integration: &mut LocusIntegration<S>,
    command: crate::locus_cli::override_cmd::OverrideCommands,
) -> io::Result<()> {
    match command {
        crate::locus_cli::override_cmd::OverrideCommands::Approve {
            task_id,
            reason,
            override_type,
        } => {
            println!("✅ Creating emergency override...");

            match integration.create_emergency_override(&task_id, &reason, &override_type) {
                Ok(override_id) => {
                    println!("✅ Override created: {}", override_id);
                    println!("Task: {}", task_id);
                    println!("Reason: {}", reason);
                    println!("Type: {}", override_type);
                }
                Err(e) => {
                    eprintln!("❌ Error creating override: {}", e);
                }
            }
        }

        crate::locus_cli::override_cmd::OverrideCommands::Emergency { subcommand } => {
            match subcommand {
                crate::locus_cli::override_cmd::EmergencyCommands::Halt { reason, scope } => {
                    println!("🛑 EMERGENCY HALT");
                    println!("Reason: {}", reason);
                    if let Some(s) = scope {
                        println!("Scope: {}", s);
                    }
                    println!("🚧 Emergency halt requires system-level integration");
                }

                _ => {
                    println!("🚧 Emergency command requires full integration");
                }
            }
        }

        _ => {
            println!("🚧 Override command requires Engram integration - not yet implemented");
        }
    }

    Ok(())
}
