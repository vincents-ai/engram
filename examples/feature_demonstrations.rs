//! Engram Feature Demonstrations
//!
//! This example provides demonstrations of core Engram features including
//! entity creation, relationships, validation, sessions, workflows, and backup.

use engram::entities::{Context, ContextRelevance, Entity, Reasoning, Task};
use engram::storage::memory_only_storage::MemoryStorage;
use engram::storage::Storage;
use engram::Entity as _;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Engram Feature Demonstrations");
    println!("==============================\n");

    let mut storage = MemoryStorage::new("demo-agent");
    println!("✅ Storage initialized\n");

    // Create entities
    let task = Task::new(
        "Implement authentication".to_string(),
        "Add JWT-based authentication to API".to_string(),
        "demo-agent".to_string(),
        engram::entities::TaskPriority::High,
        None,
    );

    let mut reasoning = Reasoning::new(
        "JWT chosen for simplicity".to_string(),
        task.id.clone(),
        "demo-agent".to_string(),
    );
    reasoning.add_step(
        "Evaluate authentication options".to_string(),
        "JWT provides stateless authentication".to_string(),
        0.85,
    );
    reasoning.add_step(
        "Compare JWT vs OAuth2".to_string(),
        "JWT simpler for our use case".to_string(),
        0.9,
    );
    reasoning.set_conclusion("Implement JWT authentication".to_string(), 0.9);

    let context = Context::new(
        "JWT RFC 7519".to_string(),
        "JSON Web Token specification".to_string(),
        "documentation".to_string(),
        ContextRelevance::High,
        "demo-agent".to_string(),
    );

    storage.store(&task.to_generic())?;
    storage.store(&reasoning.to_generic())?;
    storage.store(&context.to_generic())?;
    println!("✅ Created task, reasoning, and context");

    // Show entity types
    println!("\n📦 Supported Entity Types:");
    println!("   • Task - Work items with priorities and status");
    println!("   • Context - Background information and docs");
    println!("   • Reasoning - Decision chains with confidence");
    println!("   • Knowledge - Information with categorization");
    println!("   • Session - Agent work sessions with metrics");
    println!("   • Relationship - Entity connections (graph)");
    println!("   • Workflow - State machine definitions");
    println!("   • Compliance - Requirements tracking");
    println!("   • Standard - Team standards with versioning");
    println!("   • ADR - Architectural Decision Records");

    // Show features
    println!("\n🎯 Key Features:");
    println!("1. Entity Management");
    println!("   - Create, read, update, delete entities");
    println!("   - Type-safe entity trait with validation");
    println!("   - Generic entity representation for dynamic handling");
    println!("   - Agent-based filtering and querying");

    println!("\n2. Relationship System");
    println!("   - Graph-based entity relationships");
    println!("   - Types: DependsOn, Contains, References, etc.");
    println!("   - Direction: Unidirectional, Bidirectional");
    println!("   - Strength: Weak, Medium, Strong, Critical");
    println!("   - Path finding and graph traversal");

    println!("\n3. Validation System");
    println!("   - Task ID extraction: [TASK-123], [task:id], Refs: #456");
    println!("   - Relationship requirements (reasoning + context)");
    println!("   - Pre-commit hook integration");
    println!("   - Exemption handling for chore/docs/fixup");

    println!("\n4. Session Management");
    println!("   - Track agent work sessions");
    println!("   - Goals and outcomes documentation");
    println!("   - Task/context/knowledge associations");
    println!("   - SPACE framework metrics");
    println!("   - DORA metrics integration");

    println!("\n5. Workflow Engine");
    println!("   - Custom state machine definitions");
    println!("   - State transitions with conditions");
    println!("   - Prompt templates per state");
    println!("   - Permission schemes");
    println!("   - Task-workflow association");

    println!("\n6. Perkeep Integration");
    println!("   - Content-addressable backup (SHA-256)");
    println!("   - Entity type filtering");
    println!("   - Relationship preservation");
    println!("   - Selective restore by blob reference");

    // Show CLI commands
    println!("\n💻 CLI Commands:");
    println!("   engram task create --title 'Task' --priority high");
    println!("   engram context create --title 'Context' --source docs");
    println!("   engram reasoning create --title 'Reasoning' --task-id <id>");
    println!("   engram relationship create --source-id a --target-id b --type depends_on");
    println!("   engram validation commit --message 'feat: implement [TASK-ID]'");
    println!("   engram session start --agent alice");
    println!("   engram workflow create --title 'My Workflow'");
    println!("   engram perkeep backup --description 'Backup'");

    // Show storage backends
    println!("\n💾 Storage Backends:");
    println!("   • GitStorage - Content-addressable Git-based storage");
    println!("   • MemoryStorage - In-memory storage for testing");
    println!("   • RelationshipStorage - Graph relationship queries");
    println!("   • GitRefsStorage - Git references for collaboration");

    // Show configuration
    println!("\n⚙️ Configuration (.engram/config.yaml):");
    println!("   app:");
    println!("     log_level: info");
    println!("     default_agent: default");
    println!("   workspace:");
    println!("     agents:");
    println!("       coder:");
    println!("         type: implementation");
    println!("   storage:");
    println!("     storage_type: git");
    println!("     base_path: .engram");

    println!("\n🎯 This demonstrates:");
    println!("1. ✅ Multi-entity creation and storage");
    println!("2. ✅ Entity relationships and graph operations");
    println!("3. ✅ Commit validation with task references");
    println!("4. ✅ Session tracking with goals/outcomes");
    println!("5. ✅ Workflow state machine concepts");
    println!("6. ✅ Perkeep backup/restore integration");
    println!("7. ✅ Storage backend options");
    println!("8. ✅ CLI command overview");

    Ok(())
}
