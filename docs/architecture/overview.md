# Architecture Overview

## System Design

Engram is a distributed memory system that stores entity data in Git refs.

## Core Components

```
┌─────────────────────────────────────────┐
│              CLI Layer                  │
│  (src/cli/*.rs)                        │
└────────────────┬────────────────────────┘
                 │
┌────────────────▼────────────────────────┐
│            Entity Layer                 │
│  (src/entities/*.rs)                   │
│  - Task, Context, Reasoning, Knowledge  │
│  - Theory, StateReflection              │
│  - Session, Workflow, Relationship      │
└────────────────┬────────────────────────┘
                 │
┌────────────────▼────────────────────────┐
│           Storage Layer                │
│  (src/storage/*.rs)                    │
│  - GitStorage                          │
│  - MemoryStorage                       │
└─────────────────────────────────────────┘
```

## Storage Backends

| Backend | Description |
|---------|-------------|
| GitStorage | Persists to `.git/refs/engram/` |
| MemoryStorage | In-memory for testing |

## Entity Flow

1. User creates entity via CLI
2. Entity is validated
3. Converted to GenericEntity
4. Stored in storage backend
5. Can be retrieved, listed, updated, deleted
