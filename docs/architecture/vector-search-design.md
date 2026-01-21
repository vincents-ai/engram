# Vector Search Layer Design

## Overview

Add semantic similarity search to Engram without disrupting the existing Git refs storage architecture.

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                     Engram CLI                          │
└─────────────────────────────────────────────────────────┘
                          │
        ┌─────────────────┴─────────────────┐
        │                                   │
        ▼                                   ▼
┌──────────────────┐              ┌──────────────────┐
│  GitRefsStorage  │              │  VectorStorage   │
│  (Primary)       │              │  (Optional)      │
├──────────────────┤              ├──────────────────┤
│ • Entity CRUD    │              │ • Embeddings     │
│ • Relationships  │              │ • Similarity     │
│ • Git refs       │              │ • Search         │
│ • Blobs          │              │                  │
└──────────────────┘              └──────────────────┘
        │                                   │
        ▼                                   ▼
┌──────────────────┐              ┌──────────────────┐
│   .git/refs/     │              │ .engram/         │
│   engram/        │              │ vectors.db       │
└──────────────────┘              └──────────────────┘
```

## Storage Layout

```
.git/
  refs/
    engram/
      task/{uuid}        # Task entities (existing)
      context/{uuid}     # Context entities (existing)
      reasoning/{uuid}   # Reasoning entities (existing)
      
.engram/
  vectors.db             # SQLite database with vector embeddings
    tables:
      - embeddings       # entity_id, entity_type, vector, model, timestamp
      - models           # model_name, dimensions, provider
      - search_history   # query, results, timestamp
```

## Key Design Decisions

### 1. Optional and Non-Blocking
- Vector search is **opt-in** via feature flag or config
- Existing operations continue without vector storage
- Embedding generation happens lazily or via explicit command

### 2. Multiple Embedding Providers
```rust
pub trait EmbeddingProvider {
    async fn embed(&self, text: &str) -> Result<Vec<f32>>;
    fn dimensions(&self) -> usize;
    fn model_name(&self) -> &str;
}

// Implementations:
- LocalOnnxProvider (offline, fast)
- OpenAIProvider (API-based, high quality)
- CohereProvider (API-based, code-focused)
```

### 3. Lazy Indexing Strategy
- Embeddings generated on-demand, not at entity creation
- Background indexing job: `engram index rebuild`
- Cache embeddings in SQLite to avoid regeneration

### 4. Query Interface
```bash
# Semantic search
engram search "authentication implementation" --type context --limit 10

# Hybrid search (keyword + semantic)
engram search "JWT tokens" --hybrid --threshold 0.7

# Show similar items
engram context similar <context-id> --limit 5
engram task similar <task-id>
```

## Implementation Phases

### Phase 1: Core Infrastructure (This PR)
- [ ] Add `VectorStorage` trait
- [ ] Implement SQLite + sqlite-vss backend
- [ ] Add embedding generation trait
- [ ] Implement local ONNX provider (MiniLM model)
- [ ] Basic search command

### Phase 2: Search Features
- [ ] Hybrid search (keyword + semantic)
- [ ] Similarity threshold tuning
- [ ] Result ranking and scoring
- [ ] Search history and analytics

### Phase 3: Advanced Features
- [ ] Multiple embedding models
- [ ] API-based providers (OpenAI, Cohere)
- [ ] Background indexing service
- [ ] Vector index optimization

### Phase 4: Perkeep Integration
- [ ] Backup/restore with Perkeep
- [ ] Cloud storage backend support
- [ ] Incremental backup
- [ ] Restore to specific timestamps

## Database Schema

```sql
-- Embedding storage
CREATE TABLE embeddings (
    id TEXT PRIMARY KEY,
    entity_id TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    vector BLOB NOT NULL,        -- Vector embedding
    model TEXT NOT NULL,          -- Model identifier
    dimensions INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(entity_id, model)
);

-- Virtual table for vector search (sqlite-vss)
CREATE VIRTUAL TABLE vss_embeddings USING vss0(
    vector(384)                   -- Dimension based on model
);

-- Model registry
CREATE TABLE models (
    name TEXT PRIMARY KEY,
    provider TEXT NOT NULL,
    dimensions INTEGER NOT NULL,
    is_default BOOLEAN DEFAULT 0,
    config TEXT                   -- JSON config
);

-- Search history
CREATE TABLE search_history (
    id TEXT PRIMARY KEY,
    query TEXT NOT NULL,
    model TEXT NOT NULL,
    results TEXT,                 -- JSON array of results
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

## Configuration

```yaml
# .engram/config.yaml
vector_search:
  enabled: true
  database: .engram/vectors.db
  
  default_provider: local_onnx
  
  providers:
    local_onnx:
      model: all-MiniLM-L6-v2
      dimensions: 384
      device: cpu
      
    openai:
      model: text-embedding-3-small
      dimensions: 1536
      api_key_env: OPENAI_API_KEY
      
  indexing:
    auto_index: false           # Index on entity creation
    batch_size: 100
    
  search:
    default_limit: 10
    similarity_threshold: 0.7
    hybrid_weight: 0.5          # Balance keyword vs semantic
```

## API Examples

### Rust API
```rust
use engram::vector::{VectorStorage, LocalOnnxProvider};

// Initialize
let vector_store = VectorStorage::new(".engram/vectors.db")?;
let provider = LocalOnnxProvider::new("all-MiniLM-L6-v2")?;

// Generate embedding
let text = "Implement user authentication with JWT tokens";
let embedding = provider.embed(text).await?;

// Store
vector_store.store_embedding(
    "task-123", 
    "task", 
    &embedding, 
    provider.model_name()
)?;

// Search
let results = vector_store.search(
    &query_embedding,
    10,     // limit
    0.7     // threshold
)?;

// Similar items
let similar = vector_store.find_similar(
    "task-123",
    5       // limit
)?;
```

### CLI Examples
```bash
# Index existing entities
engram index rebuild
engram index rebuild --type context --model all-MiniLM-L6-v2

# Search
engram search "database migration strategy"
engram search "error handling" --type task --limit 5

# Find similar
engram context similar <context-id>
engram task similar <task-id> --threshold 0.8

# Check index status
engram index status
```

## Performance Considerations

### Embedding Generation
- Local ONNX: ~10-50ms per text (CPU)
- OpenAI API: ~100-300ms per text (network)
- Batch processing: 10x faster than sequential

### Vector Search
- HNSW index: O(log n) search time
- 10k embeddings: <10ms search
- 100k embeddings: <50ms search
- Memory: ~4KB per 384-dim vector

### Disk Usage
- 384-dim vectors: ~1.5KB per entity
- 10k entities: ~15MB
- 100k entities: ~150MB

## Migration Path

For existing installations:

```bash
# Step 1: Enable vector search
engram config set vector_search.enabled true

# Step 2: Index existing entities (optional)
engram index rebuild --background

# Step 3: Use semantic search
engram search "your query"
```

No changes to existing Git refs storage or workflows.

## Testing Strategy

### Unit Tests
- Embedding generation (mock and real)
- Vector storage CRUD operations
- Similarity search correctness
- Threshold filtering

### Integration Tests
- End-to-end search workflow
- Multiple embedding models
- Large dataset performance
- Concurrent access

### BDD Tests
```gherkin
Feature: Vector Search
  Scenario: Semantic search finds similar contexts
    Given I have contexts about "authentication" and "authorization"
    When I search for "user login security"
    Then both contexts should be returned
    And they should be ranked by similarity
```

## Future Extensions

### Vector Compression
- Product quantization for smaller storage
- Binary embeddings for faster search

### Multi-Modal Embeddings
- Code embeddings (CodeBERT, GraphCodeBERT)
- Diagram embeddings (CLIP for architecture diagrams)

### Distributed Search
- Replicate vector index across agents
- Federated search across team members

### LLM Integration
- Re-ranking with LLM
- Query expansion
- Explanation generation
