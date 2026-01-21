use crate::entities::{
    Entity, EntityRelationType, EntityRelationship, RelationshipDirection, RelationshipFilter,
    RelationshipStrength,
};
use crate::error::EngramError;
use crate::storage::{RelationshipStorage, Storage, TraversalAlgorithm};
use clap::Subcommand;
use uuid::Uuid;

#[derive(Debug, Clone, Subcommand)]
pub enum RelationshipCommands {
    /// Create a new relationship between entities
    Create {
        /// Source entity ID
        #[arg(long)]
        source_id: String,

        /// Source entity type
        #[arg(long)]
        source_type: String,

        /// Target entity ID
        #[arg(long)]
        target_id: String,

        /// Target entity type
        #[arg(long)]
        target_type: String,

        /// Relationship type
        #[arg(long, value_parser = parse_relationship_type)]
        relationship_type: EntityRelationType,

        /// Relationship direction (unidirectional, bidirectional, inverse)
        #[arg(long, default_value = "unidirectional")]
        direction: String,

        /// Relationship strength (weak, medium, strong, critical, or custom 0.0-1.0)
        #[arg(long, default_value = "medium")]
        strength: String,

        /// Optional description
        #[arg(long)]
        description: Option<String>,

        /// Agent creating the relationship
        #[arg(long)]
        agent: String,
    },

    /// List relationships with filtering
    List {
        /// Filter by entity ID (either source or target)
        #[arg(long)]
        entity_id: Option<String>,

        /// Filter by source entity ID
        #[arg(long)]
        source_id: Option<String>,

        /// Filter by target entity ID
        #[arg(long)]
        target_id: Option<String>,

        /// Filter by relationship type
        #[arg(long, value_parser = parse_relationship_type)]
        relationship_type: Option<EntityRelationType>,

        /// Filter by direction
        #[arg(long)]
        direction: Option<String>,

        /// Show only active relationships
        #[arg(long)]
        active_only: bool,

        /// Filter by agent
        #[arg(long)]
        agent: Option<String>,
    },

    /// Show relationship details
    Get {
        /// Relationship ID
        id: String,
    },

    Delete {
        /// Relationship ID
        id: String,

        /// Agent performing the deletion
        #[arg(long)]
        agent: String,
    },

    /// Find paths between entities
    FindPath {
        /// Source entity ID
        #[arg(long)]
        source_id: String,

        /// Target entity ID
        #[arg(long)]
        target_id: String,

        /// Traversal algorithm (bfs, dfs, dijkstra)
        #[arg(long, default_value = "dijkstra")]
        algorithm: String,

        /// Maximum depth for search
        #[arg(long)]
        max_depth: Option<usize>,
    },

    /// Get all entities connected to a given entity
    Connected {
        /// Entity ID to start from
        #[arg(long)]
        entity_id: String,

        /// Traversal algorithm (bfs, dfs)
        #[arg(long, default_value = "bfs")]
        algorithm: String,

        /// Maximum depth for traversal
        #[arg(long)]
        max_depth: Option<usize>,
    },

    /// Show relationship statistics
    Stats {},
}

fn parse_relationship_type(s: &str) -> Result<EntityRelationType, String> {
    match s.to_lowercase().as_str() {
        "depends_on" | "depends-on" => Ok(EntityRelationType::DependsOn),
        "contains" => Ok(EntityRelationType::Contains),
        "references" => Ok(EntityRelationType::References),
        "fulfills" => Ok(EntityRelationType::Fulfills),
        "implements" => Ok(EntityRelationType::Implements),
        "supersedes" => Ok(EntityRelationType::Supersedes),
        "associated_with" | "associated-with" => Ok(EntityRelationType::AssociatedWith),
        "influences" => Ok(EntityRelationType::Influences),
        custom => Ok(EntityRelationType::Custom(custom.to_string())),
    }
}

fn parse_direction(s: &str) -> Result<RelationshipDirection, String> {
    match s.to_lowercase().as_str() {
        "unidirectional" | "uni" => Ok(RelationshipDirection::Unidirectional),
        "bidirectional" | "bi" => Ok(RelationshipDirection::Bidirectional),
        "inverse" | "inv" => Ok(RelationshipDirection::Inverse),
        _ => Err(format!(
            "Invalid direction: {}. Use: unidirectional, bidirectional, or inverse",
            s
        )),
    }
}

fn parse_strength(s: &str) -> Result<RelationshipStrength, String> {
    match s.to_lowercase().as_str() {
        "weak" => Ok(RelationshipStrength::Weak),
        "medium" => Ok(RelationshipStrength::Medium),
        "strong" => Ok(RelationshipStrength::Strong),
        "critical" => Ok(RelationshipStrength::Critical),
        custom => {
            if let Ok(value) = custom.parse::<f64>() {
                if value >= 0.0 && value <= 1.0 {
                    Ok(RelationshipStrength::Custom(value))
                } else {
                    Err("Custom strength must be between 0.0 and 1.0".to_string())
                }
            } else {
                Err(format!("Invalid strength: {}. Use: weak, medium, strong, critical, or a number between 0.0 and 1.0", s))
            }
        }
    }
}

fn parse_algorithm(s: &str) -> Result<TraversalAlgorithm, String> {
    match s.to_lowercase().as_str() {
        "bfs" | "breadth_first" | "breadth-first" => Ok(TraversalAlgorithm::BreadthFirst),
        "dfs" | "depth_first" | "depth-first" => Ok(TraversalAlgorithm::DepthFirst),
        "dijkstra" => Ok(TraversalAlgorithm::Dijkstra),
        _ => Err(format!(
            "Invalid algorithm: {}. Use: bfs, dfs, or dijkstra",
            s
        )),
    }
}

pub fn handle_relationship_command<S: RelationshipStorage>(
    storage: &mut S,
    command: RelationshipCommands,
) -> Result<(), EngramError> {
    match command {
        RelationshipCommands::Create {
            source_id,
            source_type,
            target_id,
            target_type,
            relationship_type,
            direction,
            strength,
            description,
            agent,
        } => create_relationship(
            storage,
            source_id,
            source_type,
            target_id,
            target_type,
            relationship_type,
            direction,
            strength,
            description,
            agent,
        ),

        RelationshipCommands::List {
            entity_id,
            source_id,
            target_id,
            relationship_type,
            direction,
            active_only,
            agent,
        } => list_relationships(
            storage,
            entity_id,
            source_id,
            target_id,
            relationship_type,
            direction,
            active_only,
            agent,
        ),

        RelationshipCommands::Get { id } => show_relationship(storage, &id),

        RelationshipCommands::Delete { id, agent } => delete_relationship(storage, &id, &agent),

        RelationshipCommands::FindPath {
            source_id,
            target_id,
            algorithm,
            max_depth,
        } => find_path(storage, &source_id, &target_id, &algorithm, max_depth),

        RelationshipCommands::Connected {
            entity_id,
            algorithm,
            max_depth,
        } => show_connected(storage, &entity_id, &algorithm, max_depth),

        RelationshipCommands::Stats {} => show_stats(storage),
    }
}

fn create_relationship<S: Storage>(
    storage: &mut S,
    source_id: String,
    source_type: String,
    target_id: String,
    target_type: String,
    relationship_type: EntityRelationType,
    direction_str: String,
    strength_str: String,
    description: Option<String>,
    agent: String,
) -> Result<(), EngramError> {
    let id = Uuid::new_v4().to_string();
    let direction =
        parse_direction(&direction_str).map_err(|e| EngramError::Validation(e.to_string()))?;
    let strength =
        parse_strength(&strength_str).map_err(|e| EngramError::Validation(e.to_string()))?;

    let mut relationship = EntityRelationship::new(
        id,
        agent,
        source_id,
        source_type,
        target_id,
        target_type,
        relationship_type,
    )
    .with_direction(direction)
    .with_strength(strength);

    if let Some(desc) = description {
        relationship = relationship.with_description(desc);
    }

    relationship
        .validate_entity()
        .map_err(|e| EngramError::Validation(e.to_string()))?;

    let generic = relationship.to_generic();
    storage.store(&generic)?;

    println!("✅ Relationship created successfully");
    println!("📋 ID: {}", relationship.id);
    println!(
        "🔗 {} --[{}]--> {}",
        relationship.source_id, relationship.relationship_type, relationship.target_id
    );

    Ok(())
}

fn list_relationships<S: Storage>(
    _storage: &S,
    entity_id: Option<String>,
    source_id: Option<String>,
    target_id: Option<String>,
    relationship_type: Option<EntityRelationType>,
    direction: Option<String>,
    active_only: bool,
    agent: Option<String>,
) -> Result<(), EngramError> {
    let mut filter = RelationshipFilter::new();

    if let Some(entity) = entity_id {
        filter = filter.entity(entity);
    }
    if let Some(source) = source_id {
        filter = filter.source(source);
    }
    if let Some(target) = target_id {
        filter = filter.target(target);
    }
    if let Some(rel_type) = relationship_type {
        filter = filter.relationship_type(rel_type);
    }
    if let Some(dir_str) = direction {
        let dir = parse_direction(&dir_str).map_err(|e| EngramError::Validation(e.to_string()))?;
        filter = filter.direction(dir);
    }
    if active_only {
        filter = filter.active_only();
    }
    if let Some(ag) = agent {
        filter.agent = Some(ag);
    }

    let relationships = _storage.get_all("relationship")?;

    println!("🔗 Entity Relationships");
    println!("======================");

    if relationships.is_empty() {
        println!("No relationships found matching the criteria.");
        return Ok(());
    }

    println!("Found {} relationship(s):\n", relationships.len());

    for (i, rel_generic) in relationships.iter().enumerate() {
        // Parse the generic entity back to relationship
        if let Ok(rel_data) = serde_json::from_value::<EntityRelationship>(rel_generic.data.clone())
        {
            if filter.matches(&rel_data) {
                println!(
                    "{}. {} [{}]",
                    i + 1,
                    rel_data.id,
                    if rel_data.active {
                        "ACTIVE"
                    } else {
                        "INACTIVE"
                    }
                );
                println!(
                    "   🔗 {} --[{}]--> {}",
                    rel_data.source_id, rel_data.relationship_type, rel_data.target_id
                );
                println!(
                    "   📊 Direction: {:?} | Strength: {:.2}",
                    rel_data.direction,
                    rel_data.strength.weight()
                );
                println!(
                    "   👤 Agent: {} | 📅 Created: {}",
                    rel_data.agent,
                    rel_data.timestamp.format("%Y-%m-%d %H:%M")
                );
                if let Some(desc) = &rel_data.description {
                    println!("   📝 Description: {}", desc);
                }
                println!();
            }
        }
    }

    Ok(())
}

fn show_relationship<S: Storage>(storage: &S, id: &str) -> Result<(), EngramError> {
    let relationship = storage.get(id, "relationship")?;

    match relationship {
        Some(rel_generic) => {
            if let Ok(rel_data) =
                serde_json::from_value::<EntityRelationship>(rel_generic.data.clone())
            {
                println!("🔗 Relationship Details");
                println!("======================");
                println!("📋 ID: {}", rel_data.id);
                println!(
                    "🔗 {} --[{}]--> {}",
                    rel_data.source_id, rel_data.relationship_type, rel_data.target_id
                );
                println!("📊 Direction: {:?}", rel_data.direction);
                println!(
                    "💪 Strength: {:?} ({:.2})",
                    rel_data.strength,
                    rel_data.strength.weight()
                );
                println!(
                    "🏷️ Source Type: {} | Target Type: {}",
                    rel_data.source_type, rel_data.target_type
                );
                println!("✅ Active: {}", rel_data.active);
                println!("👤 Agent: {}", rel_data.agent);
                println!(
                    "📅 Created: {}",
                    rel_data.timestamp.format("%Y-%m-%d %H:%M:%S")
                );

                if let Some(desc) = &rel_data.description {
                    println!("📝 Description: {}", desc);
                }

                if !rel_data.metadata.is_empty() {
                    println!("📄 Metadata:");
                    for (key, value) in &rel_data.metadata {
                        println!("   - {}: {}", key, value);
                    }
                }

                println!("⚙️ Constraints:");
                if let Some(max_out) = rel_data.constraints.max_outbound {
                    println!("   - Max outbound: {}", max_out);
                }
                if let Some(max_in) = rel_data.constraints.max_inbound {
                    println!("   - Max inbound: {}", max_in);
                }
                println!("   - Allow cycles: {}", rel_data.constraints.allow_cycles);

                Ok(())
            } else {
                Err(EngramError::Validation(format!(
                    "Invalid relationship data for ID: {}",
                    id
                )))
            }
        }
        None => {
            println!("❌ Relationship not found: {}", id);
            Err(EngramError::NotFound(format!(
                "Relationship with ID '{}' not found",
                id
            )))
        }
    }
}

fn delete_relationship<S: Storage>(
    storage: &mut S,
    id: &str,
    _agent: &str,
) -> Result<(), EngramError> {
    let relationship = storage.get(id, "relationship")?;

    match relationship {
        Some(_) => {
            storage.delete(id, "relationship")?;
            println!("✅ Relationship deleted successfully: {}", id);
            Ok(())
        }
        None => {
            println!("❌ Relationship not found: {}", id);
            Err(EngramError::NotFound(format!(
                "Relationship with ID '{}' not found",
                id
            )))
        }
    }
}

fn find_path<S: RelationshipStorage>(
    storage: &S,
    source_id: &str,
    target_id: &str,
    algorithm_str: &str,
    max_depth: Option<usize>,
) -> Result<(), EngramError> {
    let algorithm =
        parse_algorithm(algorithm_str).map_err(|e| EngramError::Validation(e.to_string()))?;

    println!(
        "🔍 Finding path from {} to {} using {:?}",
        source_id, target_id, algorithm
    );

    if let Some(depth) = max_depth {
        println!("📊 Maximum depth: {}", depth);
    }

    match storage.find_paths(source_id, target_id, algorithm, max_depth) {
        Ok(paths) => {
            if paths.is_empty() {
                println!("❌ No path found between {} and {}", source_id, target_id);
            } else {
                println!("✅ Found {} path(s):", paths.len());
                for (i, path) in paths.iter().enumerate() {
                    println!("🛤️  Path {}: {}", i + 1, path.entities.join(" → "));
                    println!("   Weight: {:.2}", path.total_weight);
                }
            }
        }
        Err(e) => {
            println!("❌ Error finding paths: {}", e);
            return Err(e);
        }
    }

    Ok(())
}

fn show_connected<S: RelationshipStorage>(
    storage: &S,
    entity_id: &str,
    algorithm_str: &str,
    max_depth: Option<usize>,
) -> Result<(), EngramError> {
    let algorithm =
        parse_algorithm(algorithm_str).map_err(|e| EngramError::Validation(e.to_string()))?;

    println!(
        "🕸️ Finding entities connected to {} using {:?}",
        entity_id, algorithm
    );

    if let Some(depth) = max_depth {
        println!("📊 Maximum depth: {}", depth);
    }

    match storage.get_connected_entities(entity_id, algorithm, max_depth) {
        Ok(connected) => {
            if connected.is_empty() {
                println!("❌ No connected entities found for {}", entity_id);
            } else {
                println!("✅ Found {} connected entities:", connected.len());
                for entity in connected {
                    println!("🔗 {}", entity);
                }
            }
        }
        Err(e) => {
            println!("❌ Error finding connected entities: {}", e);
            return Err(e);
        }
    }

    Ok(())
}

fn show_stats<S: RelationshipStorage>(storage: &S) -> Result<(), EngramError> {
    println!("📊 Relationship Statistics");
    println!("========================");

    match storage.get_relationship_stats() {
        Ok(stats) => {
            println!("📈 Total relationships: {}", stats.total_relationships);
            println!(
                "🔄 Bidirectional relationships: {}",
                stats.bidirectional_count
            );
            println!(
                "⚖️  Average connections per entity: {:.2}",
                stats.average_connections_per_entity
            );
            println!("🔗 Relationship density: {:.2}", stats.relationship_density);

            println!("\n📋 Relationships by type:");
            for (rel_type, count) in &stats.relationships_by_type {
                println!("   - {:?}: {}", rel_type, count);
            }

            if let Some(most_connected) = &stats.most_connected_entity {
                println!(
                    "\n🌟 Most connected entity: {} ({} connections)",
                    most_connected.0, most_connected.1
                );
            }
        }
        Err(e) => {
            println!("❌ Error retrieving statistics: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
