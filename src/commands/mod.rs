//! Command handler modules extracted from main.rs

pub mod setup;
pub mod task;
pub mod context;
pub mod reasoning;
pub mod knowledge;
pub mod persona;
pub mod session;
pub mod governance;
pub mod adr;
pub mod workflow;
pub mod sandbox;
pub mod escalation;
pub mod meta;
pub mod migration;
pub mod help;

// Re-export all handler functions for use in main.rs
pub use setup::{handle_convert_command, handle_setup_command, handle_test_command};
pub use task::handle_task_command;
pub use context::handle_context_command;
pub use reasoning::handle_reasoning_command;
pub use knowledge::{handle_knowledge_command, handle_lesson_command};
pub use persona::handle_persona_command;
pub use session::handle_session_command;
pub use governance::{handle_compliance_command, handle_rule_command, handle_standard_command};
pub use adr::handle_adr_command;
pub use workflow::handle_workflow_command;
pub use sandbox::handle_sandbox_command;
pub use escalation::handle_escalation_command;
pub use meta::{handle_reflection_command, handle_theory_command};
pub use migration::{handle_migrate_command, handle_migration_command};
pub use help::handle_help_command;
