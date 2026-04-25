# engram-storage

Storage implementations for the [engram](https://github.com/vincents-ai/engram) AI memory layer.

## Overview

`engram-storage` provides concrete storage backends for engram entities:

- **`MemoryEntity`** — content-addressable entity representation with SHA-256 hashing, tags, references, and integrity verification
- **`MemoryStorage`** — in-memory storage backend for testing and development (implements `Storage` trait)
- **`RelationshipStorage`** — graph indexing, traversal (BFS/DFS), path finding, and relationship management

All implementations depend on `engram-core` for trait definitions and shared types.

## Usage

```rust
use engram_storage::{MemoryStorage, MemoryEntity, Storage};
use engram_core::entity_types::GenericEntity;

// In-memory storage for testing
let mut storage = MemoryStorage::new("my-agent");

let entity = GenericEntity {
    id: "task-1".to_string(),
    entity_type: "task".to_string(),
    agent: "my-agent".to_string(),
    timestamp: chrono::Utc::now(),
    data: serde_json::json!({"title": "My task"}),
};

storage.store(&entity).unwrap();
let retrieved = storage.get("task-1", "task").unwrap();
```

## Why This Crate Exists

The main `engram` crate is a binary (CLI + TUI). `engram-storage` provides the storage implementations as a library so they can be reused without pulling in the full CLI. `GitRefsStorage` remains in the main crate as it's deeply coupled to CLI configuration.

## Tests

```
cargo test -p engram-storage
```

11 tests covering MemoryEntity (creation, integrity, tags, fields, paths), MemoryStorage (store/get, delete, query-by-agent), and RelationshipStorage (indexing, paths).
