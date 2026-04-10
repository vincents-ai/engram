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
│  - Session, Workflow, WorkflowInstance  │
│  - ADR, Compliance, Standard, Rule      │
│  - Lesson, Persona, DocFragment         │
│  - EscalationRequest, ExecutionResult   │
│  - AgentSandbox, ProgressiveGateConfig  │
│  - BottleneckReport, DoraMetricsReport  │
│  - TaskDurationReport, StaleTaskReport  │
└────────────────┬────────────────────────┘
                 │
┌────────────────▼────────────────────────┐
│           Storage Layer                │
│  (src/storage/*.rs)                    │
│  - GitRefsStorage (primary)            │
│  - MemoryOnlyStorage (testing)         │
└─────────────────────────────────────────┘
```

## Modules

| Module | Description |
|--------|-------------|
| ask | Semantic search and query engine |
| cli | Command-line interface |
| config | Configuration management |
| engines | Workflow engine, rule engine, NLQ engine |
| entities | All 23 entity types |
| error | Error types |
| feedback | Structured feedback interface |
| locus_cli | Locus agent CLI |
| locus_handlers | Locus event handlers |
| locus_integration | Locus integration layer |
| locus_tui | Terminal UI (feature-gated) |
| migration | Data migrations |
| nlq | Natural language query processing |
| perkeep | Perkeep backup integration |
| personas | Persona management |
| sandbox | Sandbox enforcement (feature-gated) |
| storage | Storage backends |
| validation | Input validation |
| vector | Vector search |
| version | Version info |

## Storage Backends

| Backend | Description |
|---------|-------------|
| GitRefsStorage | Primary — persists to `.git/refs/engram/` |
| MemoryOnlyStorage | In-memory for testing |

## Entity Flow

1. User creates entity via CLI
2. Entity is validated
3. Converted to GenericEntity
4. Stored in storage backend
5. Can be retrieved, listed, updated, deleted
