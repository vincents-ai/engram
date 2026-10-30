# Schema

Engram can generate JSON Schema documents for built-in entity types and publish those schemas into the git-ref-backed store.

## CLI Usage

```bash
# Generate a schema to stdout
engram schema generate --entity task
engram schema generate --entity reasoning

# Write a schema to a file
engram schema generate --entity workflow --output workflow.schema.json

# Publish all built-in schemas as engram refs
engram schema publish

# Backward-compatible workflow-only generator
engram schema workflow --output workflow.schema.json
```

## Supported Entities

`schema generate` accepts short names such as `task` and full schema IDs such as `engram-task`.

| Schema ID | Namespace pattern |
|-----------|-------------------|
| `engram-task` | `refs/engram/task/*` |
| `engram-context` | `refs/engram/context/*` |
| `engram-reasoning` | `refs/engram/reasoning/*` |
| `engram-knowledge` | `refs/engram/knowledge/*` |
| `engram-session` | `refs/engram/session/*` |
| `engram-adr` | `refs/engram/adr/*` |
| `engram-theory` | `refs/engram/theory/*` |
| `engram-state-reflection` | `refs/engram/state_reflection/*` |
| `engram-doc-fragment` | `refs/engram/doc_fragment/*` |
| `engram-compliance` | `refs/engram/compliance/*` |
| `engram-workflow` | `refs/engram/workflow/*` |

## Publish Behavior

`engram schema publish` serializes every built-in schema wrapper and stores it as a generic `schema` entity owned by the `engram` agent. Published schemas include:

- `id`
- `namespace_pattern`
- `title`
- crate `version`
- JSON Schema payload
- optional UI hints field

Use published schemas when external tools need to discover engram entity structure from the repository itself.
