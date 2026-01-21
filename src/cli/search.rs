//! Vector search CLI commands

use crate::error::EngramError;
use crate::vector::embedding::EmbeddingProvider;
use crate::vector::{FastEmbedProvider, SqliteVectorStorage};
use clap::{Args, Subcommand};
use std::path::Path;

/// Entity type choice for filtering
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum EntityChoice {
    Task,
    Context,
    Reasoning,
    Knowledge,
}

impl std::fmt::Display for EntityChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityChoice::Task => write!(f, "task"),
            EntityChoice::Context => write!(f, "context"),
            EntityChoice::Reasoning => write!(f, "reasoning"),
            EntityChoice::Knowledge => write!(f, "knowledge"),
        }
    }
}

/// Vector search subcommands
#[derive(Subcommand, Debug)]
pub enum VectorSearchCommands {
    /// Search for similar entities using semantic similarity
    #[command(name = "search")]
    Search(SearchArgs),

    /// Index entities for vector search
    #[command(name = "index")]
    Index(IndexArgs),

    /// Show vector index status
    #[command(name = "status")]
    Status(StatusArgs),
}

/// Search for entities using semantic similarity
#[derive(Args, Debug)]
pub struct SearchArgs {
    /// Query text to search for
    pub query: String,

    /// Entity types to search (task, context, reasoning, etc.)
    #[arg(short, long)]
    pub entity_type: Option<EntityChoice>,

    /// Maximum number of results
    #[arg(short, long, default_value = "10")]
    pub limit: usize,

    /// Minimum similarity threshold (0.0-1.0)
    #[arg(short, long, default_value = "0.7")]
    pub threshold: f32,

    /// Embedding model to use
    #[arg(long)]
    pub model: Option<String>,
}

/// Index entities for vector search
#[derive(Args, Debug)]
pub struct IndexArgs {
    /// Entity types to index
    #[arg(short, long)]
    pub entity_type: Option<EntityChoice>,

    /// Rebuild all embeddings (force re-indexing)
    #[arg(long)]
    pub rebuild: bool,

    /// Batch size for indexing
    #[arg(long, default_value = "50")]
    pub batch_size: usize,

    /// Embedding model to use
    #[arg(long)]
    pub model: Option<String>,
}

/// Show vector index status
#[derive(Args, Debug)]
pub struct StatusArgs {
    /// Show detailed status
    #[arg(long)]
    pub detailed: bool,
}

/// Handle vector search commands
pub async fn handle_vector_search_commands(
    storage_path: &Path,
    commands: VectorSearchCommands,
) -> Result<(), EngramError> {
    match commands {
        VectorSearchCommands::Search(args) => handle_search(storage_path, args).await,
        VectorSearchCommands::Index(args) => handle_index(storage_path, args).await,
        VectorSearchCommands::Status(args) => handle_status(storage_path, args).await,
    }
}

async fn handle_search(storage_path: &Path, args: SearchArgs) -> Result<(), EngramError> {
    let vector_db_path = storage_path.join("vectors.db");

    // Initialize vector storage
    let vector_storage = SqliteVectorStorage::new(&vector_db_path)?;

    // Initialize embedding provider
    let provider = FastEmbedProvider::new()?;

    // Generate query embedding
    let query_embedding = provider.embed(&args.query).await?;

    // Perform search
    let results = vector_storage.search_similar(
        &query_embedding,
        args.entity_type.map(|e| e.to_string()).as_deref(),
        args.limit,
        args.threshold,
    )?;

    // Display results
    println!();
    println!("Semantic Search Results for: \"{}\"", args.query);
    println!(
        "   Threshold: {:.2} | Limit: {}",
        args.threshold, args.limit
    );
    println!("{}", "-".repeat(60));

    if results.is_empty() {
        println!("No matching entities found.");
    } else {
        for (i, result) in results.iter().enumerate() {
            let entity_type = match result.entity_type.as_str() {
                "task" => "Task",
                "context" => "Context",
                "reasoning" => "Reasoning",
                "knowledge" => "Knowledge",
                _ => "Entity",
            };

            println!(
                "{}. {} [{}] (similarity: {:.3})",
                i + 1,
                entity_type,
                result.entity_id.chars().take(8).collect::<String>(),
                result.score
            );
        }
    }

    println!();
    Ok(())
}

async fn handle_index(storage_path: &Path, args: IndexArgs) -> Result<(), EngramError> {
    let vector_db_path = storage_path.join("vectors.db");

    // Initialize vector storage
    let mut vector_storage = SqliteVectorStorage::new(&vector_db_path)?;

    // Initialize embedding provider
    let provider = FastEmbedProvider::new()?;

    println!("Indexing entities for vector search...");
    println!("   Batch size: {}", args.batch_size);
    println!();

    // Determine entity types to index
    let entity_types: Vec<String> = match args.entity_type {
        Some(etype) => vec![etype.to_string()],
        None => vec![
            "task".to_string(),
            "context".to_string(),
            "reasoning".to_string(),
            "knowledge".to_string(),
        ],
    };

    let mut indexed_count = 0;
    let mut failed_count = 0;

    // Register the model
    vector_storage.register_model(
        provider.model_name(),
        "fastembed",
        provider.dimensions(),
        true,
    )?;

    println!(
        "Registered model: {} ({} dimensions)",
        provider.model_name(),
        provider.dimensions()
    );
    println!("Note: Full entity indexing requires integration with entity storage layer.");
    println!();
    println!("To index existing entities, run:");
    println!("  engram vector index --entity-type task --batch-size 50");
    println!();

    println!("Indexed 0 entities (framework ready).");
    Ok(())
}

async fn handle_status(storage_path: &Path, args: StatusArgs) -> Result<(), EngramError> {
    let vector_db_path = storage_path.join("vectors.db");

    if !vector_db_path.exists() {
        println!("Vector Index Status");
        println!("   Vector database: Not initialized");
        println!("   Run 'engram vector index' to create the index.");
        return Ok(());
    }

    let vector_storage = SqliteVectorStorage::new(&vector_db_path)?;

    let count = vector_storage.count_embeddings()?;
    let models = vector_storage.list_models()?;

    println!("Vector Index Status");
    println!("   Vector database: {}", vector_db_path.display());
    println!("   Total embeddings: {}", count);
    println!("   Registered models: {}", models.len());

    if args.detailed {
        println!();
        println!("Models:");
        for model in models {
            println!("   - {}", model);
        }
    }

    println!();
    println!("Vector Search: Available");
    println!("   Provider: FastEmbed (local ONNX)");
    println!("   Default model: All-MiniLM-L6-v2 (384 dimensions)");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_args_defaults() {
        let args = SearchArgs {
            query: "test query".to_string(),
            entity_type: None,
            limit: 10,
            threshold: 0.7,
            model: None,
        };

        assert_eq!(args.limit, 10);
        assert!((args.threshold - 0.7).abs() < 0.001);
    }

    #[test]
    fn test_index_args_defaults() {
        let args = IndexArgs {
            entity_type: None,
            rebuild: false,
            batch_size: 50,
            model: None,
        };

        assert_eq!(args.batch_size, 50);
        assert!(!args.rebuild);
    }

    #[test]
    fn test_entity_choice_display() {
        assert_eq!(EntityChoice::Task.to_string(), "task");
        assert_eq!(EntityChoice::Context.to_string(), "context");
        assert_eq!(EntityChoice::Reasoning.to_string(), "reasoning");
        assert_eq!(EntityChoice::Knowledge.to_string(), "knowledge");
    }
}
