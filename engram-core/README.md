# engram-core

Core traits, types, and errors for the [engram](https://github.com/vincents-ai/engram) AI memory layer.

## Overview

`engram-core` provides the foundational abstractions shared across all engram crates:

- **`Storage`** trait — CRUD, querying, branching, and sync for memory backends
- **`Entity`** trait — uniform interface for all 27 entity types (task, context, reasoning, knowledge, etc.)
- **`GenericEntity`** — dynamic entity representation for storage without compile-time type knowledge
- **`EngramError` / `StorageError` / `ConfigError`** — error types used across all engram crates
- **`QueryFilter` / `QueryResult`** — structured query types
- **`EntityRegistry`** — dynamic entity type resolution

## Why This Crate Exists

The main `engram` crate is a binary (CLI + TUI). Other crates that need engram types — like integration crates in `agentic-repos` — can't depend on a binary crate. `engram-core` extracts just the trait contracts and shared types with minimal dependencies.

## Usage

```rust
use engram_core::{Storage, GenericEntity, EngramError, QueryFilter};

fn query_tasks<S: Storage>(storage: &S) -> Result<Vec<GenericEntity>, EngramError> {
    let filter = QueryFilter {
        entity_type: Some("task".to_string()),
        ..Default::default()
    };
    let result = storage.query(&filter)?;
    Ok(result.entities)
}
```

## Dependencies

Minimal by design — only `serde`, `chrono`, `uuid`, `thiserror`, `schemars`, `serde_yaml`. No git libraries, no async runtimes, no CLI frameworks.
