# Entity Schema

## Task

| Field | Type | Description |
|-------|------|-------------|
| id | `String` | UUID |
| title | `String` | Task title |
| description | `String` | Detailed description |
| status | `Enum` | pending, in_progress, completed, blocked |
| priority | `Enum` | high, medium, low |
| parent_id | `Option<String>` | Parent task ID |
| task_id | `Option<String>` | External task reference |
| agent | `String` | Owner |
| created_at | `DateTime` | Creation timestamp |
| updated_at | `DateTime` | Last update |

## Context

| Field | Type | Description |
|-------|------|-------------|
| id | `String` | UUID |
| title | `String` | Context title |
| content | `String` | Actual content |
| source | `Option<String>` | URL or source |
| agent | `String` | Owner |
| created_at | `DateTime` | Creation timestamp |

## Reasoning

| Field | Type | Description |
|-------|------|-------------|
| id | `String` | UUID |
| title | `String` | Decision title |
| description | `String` | Rationale |
| task_id | `Option<String>` | Related task |
| agent | `String` | Author |
| created_at | `DateTime` | Creation timestamp |

## Knowledge

| Field | Type | Description |
|-------|------|-------------|
| id | `String` | UUID |
| title | `String` | Knowledge title |
| content | `String` | The knowledge |
| knowledge_type | `Enum` | fact, pattern, rule, concept, procedure, heuristic |
| confidence | `f64` | 0.0-1.0 |
| source | `Option<String>` | Source URL |
| tags | `Vec<String>` | Tags |
| agent | `String` | Author |
| usage_count | `u32` | Times referenced |
| created_at | `DateTime` | Creation timestamp |

## Theory

| Field | Type | Description |
|-------|------|-------------|
| id | `String` | UUID |
| domain_name | `String` | Domain identifier |
| conceptual_model | `HashMap` | Concept → definition |
| system_mapping | `HashMap` | Concept → code location |
| design_rationale | `HashMap` | Decision → reason |
| invariants | `Vec<String>` | Must-be-true statements |
| iteration_count | `u32` | Theory version |
| reflection_ids | `Vec<String>` | Applied reflections |
| task_id | `Option<String>` | Associated task |
| agent | `String` | Author |
| created_at | `DateTime` | Creation timestamp |
| updated_at | `DateTime` | Last update |

## StateReflection

| Field | Type | Description |
|-------|------|-------------|
| id | `String` | UUID |
| theory_id | `String` | Theory being evaluated |
| observed_state | `String` | Raw observation/error |
| cognitive_dissonance | `Vec<String>` | Conflicts detected |
| proposed_theory_updates | `Vec<String>` | Proposed fixes |
| dissonance_score | `f64` | 0.0-1.0 |
| trigger_type | `Enum` | test_failure, runtime_error, etc. |
| severity | `Enum` | none, low, medium, high, critical |
| resolved | `bool` | Resolution status |
| new_theory_id | `Option<String>` | Updated theory |
| agent | `String` | Author |
| created_at | `DateTime` | Creation timestamp |

## Session

| Field | Type | Description |
|-------|------|-------------|
| id | `String` | UUID |
| agent | `String` | Agent name |
| status | `Enum` | active, reflecting, completed |
| active_theory_id | `Option<String>` | Bound theory |
| theory_ids | `Vec<String>` | Referenced theories |
| reflection_ids | `Vec<String>` | Generated reflections |
| start_time | `DateTime` | Session start |
| end_time | `Option<DateTime>` | Session end |
| metadata | `HashMap` | Additional data |

## Relationship

| Field | Type | Description |
|-------|------|-------------|
| id | `String` | UUID |
| source_id | `String` | Source entity ID |
| target_id | `String` | Target entity ID |
| relationship_type | `String` | Type (references, depends_on, etc.) |
| agent | `String` | Author |
| created_at | `DateTime` | Creation timestamp |

## Workflow

| Field | Type | Description |
|-------|------|-------------|
| id | `String` | UUID |
| title | `String` | Workflow title |
| description | `String` | Description |
| states | `Vec<String>` | State names |
| transitions | `Vec<Transition>` | Valid transitions |
| current_state | `String` | Current state |
| agent | `String` | Owner |
| created_at | `DateTime` | Creation timestamp |
