# Entity Schema

## ADR

| Field | Type | Description |
|-------|------|-------------|
| id | `String` | UUID |
| title | `String` | ADR title |
| number | `u32` | Sequential number |
| status | `Enum` | proposed, accepted, deprecated, superseded |
| agent | `String` | Author |
| context | `String` | Context and problem statement |
| decision | `String` | Decision description |
| consequences | `String` | Consequences of the decision |
| alternatives | `Vec<Alternative>` | Options considered |
| implementation | `Option<String>` | Implementation notes |
| related_adrs | `Vec<String>` | Related ADR IDs |
| superseded_by | `Option<String>` | Newer ADR |
| supersedes | `Vec<String>` | Older ADRs replaced |
| stakeholders | `Vec<String>` | People involved |
| tags | `Vec<String>` | Tags |
| metadata | `HashMap` | Additional data |
| created_at | `DateTime` | Creation timestamp |
| updated_at | `DateTime` | Last update |
| decision_date | `Option<DateTime>` | When decided |

## AgentSandbox

| Field | Type | Description |
|-------|------|-------------|
| id | `String` | UUID |
| agent_id | `String` | Agent this sandbox applies to |
| sandbox_level | `Enum` | unrestricted, standard, restricted, isolated, training |
| permissions | `PermissionSet` | Allowed commands, paths, file ops, network, quality gates, workflows |
| resource_limits | `ResourceLimits` | Memory, CPU, disk, execution time, concurrency limits |
| command_filter | `CommandFilter` | Whitelist/blacklist mode, patterns, dangerous pattern detection |
| escalation_policy | `EscalationPolicy` | Auto-approve, human approval requirements, timeout, fallback |
| created_by | `String` | Who created this sandbox |
| agent | `String` | Owner |
| violation_count | `u32` | Number of violations recorded |
| metadata | `HashMap` | Additional data |
| created_at | `DateTime` | Creation timestamp |
| last_modified | `DateTime` | Last modification |

## BottleneckReport

| Field | Type | Description |
|-------|------|-------------|
| id | `String` | UUID |
| project_path | `String` | Repo path |
| computed_at | `DateTime` | Computation timestamp |
| agent | `String` | Author |
| slowest_tasks | `Vec<BottleneckEntry>` | Top-N longest-running tasks |
| blocked_tasks | `Vec<BottleneckEntry>` | All blocked tasks |
| total_analyzed | `u64` | Total tasks analyzed |
| blocked_count | `u64` | Number of blocked tasks |
| metadata | `HashMap` | Additional data |

## Compliance

| Field | Type | Description |
|-------|------|-------------|
| id | `String` | UUID |
| title | `String` | Requirement title |
| description | `String` | Requirement description |
| category | `String` | Compliance category |
| status | `Enum` | compliant, non_compliant, pending, exempt |
| severity | `Option<Enum>` | low, medium, high, critical |
| agent | `String` | Author |
| due_date | `Option<DateTime>` | Compliance deadline |
| evidence | `Vec<ComplianceEvidence>` | Supporting evidence |
| violations | `Vec<ComplianceViolation>` | Violations found |
| related_standards | `Vec<String>` | Referenced standard IDs |
| tags | `Vec<String>` | Tags |
| metadata | `HashMap` | Additional data |
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

## DocFragment

| Field | Type | Description |
|-------|------|-------------|
| id | `String` | UUID |
| topic | `String` | Topic (e.g. "adrs", "knowledge") |
| chunk_id | `String` | Chunk identifier within topic |
| title | `String` | Human-readable title |
| content | `String` | Markdown content |
| order | `u32` | Position within topic |
| written_at | `DateTime` | When written |
| agent | `String` | Author |
| source_entity_ids | `Vec<String>` | Source material entity IDs |
| stale | `bool` | Whether source entities changed since written |

## DoraMetricsReport

| Field | Type | Description |
|-------|------|-------------|
| id | `String` | UUID |
| project_path | `String` | Repo path |
| computed_at | `DateTime` | Computation timestamp |
| window_start | `DateTime` | Time window start |
| window_end | `DateTime` | Time window end |
| commits_analyzed | `u64` | Commits analyzed |
| executions_analyzed | `u64` | Execution results analyzed |
| escalations_analyzed | `u64` | Escalation requests analyzed |
| deployment_frequency | `f64` | Deployments per week |
| lead_time_for_changes | `f64` | Days |
| change_failure_rate | `f64` | 0.0 - 1.0 |
| mean_time_to_recovery | `f64` | Hours |
| agent | `String` | Author |
| metadata | `HashMap` | Additional data |

## EscalationRequest

| Field | Type | Description |
|-------|------|-------------|
| id | `String` | UUID |
| agent_id | `String` | Requesting agent |
| session_id | `Option<String>` | Session when requested |
| operation_type | `Enum` | file_system_access, network_access, command_execution, privilege_escalation, quality_gate_override, workflow_modification, resource_limit_increase, custom |
| status | `Enum` | pending, approved, denied, expired, cancelled |
| priority | `Enum` | low, normal, high, critical |
| operation_context | `OperationContext` | Blocked operation details |
| justification | `String` | Agent's justification |
| impact_if_denied | `Option<String>` | Impact if request denied |
| suggested_reviewer | `Option<String>` | Suggested reviewer |
| reviewer | `Option<ReviewerInfo>` | Assigned reviewer |
| decision | `Option<ReviewDecision>` | Review decision |
| created_at | `DateTime` | Creation timestamp |
| updated_at | `DateTime` | Last update |
| expires_at | `DateTime` | Expiration timestamp |
| reviewed_at | `Option<DateTime>` | When reviewed |
| similar_request_count | `u32` | Similar past requests |
| agent | `String` | Owner |
| metadata | `HashMap` | Additional data |

## ExecutionResult

| Field | Type | Description |
|-------|------|-------------|
| id | `String` | UUID |
| task_id | `String` | Associated task |
| workflow_stage | `String` | Workflow stage when executed |
| command | `String` | Command executed |
| exit_code | `i32` | Exit code |
| stdout | `String` | Standard output |
| stderr | `String` | Standard error |
| timestamp | `DateTime` | Execution timestamp |
| duration_ms | `u64` | Duration in milliseconds |
| environment | `HashMap<String, String>` | Environment variables |
| file_changes | `Vec<String>` | Files changed |
| expected_result | `Option<Enum>` | success, failure, any |
| validation_status | `Enum` | passed, failed, skipped |
| quality_gate | `String` | Quality gate identifier |
| working_directory | `Option<String>` | Working directory |
| retry_count | `u32` | Retry count |
| previous_execution_id | `Option<String>` | Previous execution if retry |
| agent | `String` | Author |
| tags | `Vec<String>` | Tags |
| metadata | `HashMap` | Additional data |

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

## Lesson

| Field | Type | Description |
|-------|------|-------------|
| id | `String` | UUID |
| title | `String` | Short title summarising the lesson |
| mistake | `String` | Description of the mistake |
| correction | `String` | Correct approach |
| prevention_rule | `String` | Rule to prevent recurrence |
| domain | `String` | Domain (e.g. "rust", "postgres") |
| category | `Enum` | code, domain, process, design |
| severity | `Enum` | low, medium, high |
| agent | `String` | Author |
| tags | `Vec<String>` | Tags |
| created_at | `DateTime` | Creation timestamp |
| updated_at | `DateTime` | Last update |

## Persona

| Field | Type | Description |
|-------|------|-------------|
| id | `String` | UUID |
| slug | `String` | URL-safe slug (`[a-z0-9-]+`) |
| title | `String` | Display title |
| description | `String` | Short description |
| instructions | `String` | Full system-prompt instructions |
| domain | `String` | Specialisation domain |
| cov_questions | `Vec<String>` | Calibration of Values questions (3-5) |
| fap_table | `HashMap<String, String>` | Foundational Assumptions & Principles |
| ov_requirements | `Vec<String>` | Operational Values / requirements |
| base_persona | `Option<String>` | Base persona slug this extends |
| agent | `String` | Author |
| tags | `Vec<String>` | Tags |
| version | `String` | Semantic version (default "1.0.0") |
| created_at | `DateTime` | Creation timestamp |
| updated_at | `DateTime` | Last update |

## ProgressiveGateConfig

| Field | Type | Description |
|-------|------|-------------|
| id | `String` | UUID |
| name | `String` | Config name |
| description | `String` | Description |
| gate_levels | `Vec<GateLevel>` | Defined gate levels with thresholds |
| escalation_rules | `Vec<EscalationRule>` | Escalation triggers and actions |
| optimization_settings | `OptimizationSettings` | Historical learning, caching, adaptive timeouts |
| active | `bool` | Whether config is active |
| agent | `String` | Author |
| created_at | `DateTime` | Creation timestamp |
| updated_at | `DateTime` | Last update |

## Reasoning

| Field | Type | Description |
|-------|------|-------------|
| id | `String` | UUID |
| title | `String` | Decision title |
| description | `String` | Rationale |
| task_id | `Option<String>` | Related task |
| agent | `String` | Author |
| created_at | `DateTime` | Creation timestamp |

## Relationship

| Field | Type | Description |
|-------|------|-------------|
| id | `String` | UUID |
| source_id | `String` | Source entity ID |
| target_id | `String` | Target entity ID |
| relationship_type | `String` | Type (references, depends_on, etc.) |
| agent | `String` | Author |
| created_at | `DateTime` | Creation timestamp |

## Rule

| Field | Type | Description |
|-------|------|-------------|
| id | `String` | UUID |
| title | `String` | Rule title |
| description | `String` | Rule description |
| rule_type | `Enum` | validation, transformation, enforcement, notification |
| status | `Enum` | active, inactive, deprecated |
| priority | `Enum` | low, medium, high, critical |
| condition | `JSON` | Rule condition logic |
| action | `JSON` | Rule action to execute |
| entity_types | `Vec<String>` | Entity types this rule applies to |
| execution_history | `Vec<RuleExecution>` | Past executions |
| tags | `Vec<String>` | Tags |
| related_rules | `Vec<String>` | Related rule IDs |
| metadata | `HashMap` | Additional data |
| agent | `String` | Author |
| created_at | `DateTime` | Creation timestamp |
| updated_at | `DateTime` | Last update |

## Session

| Field | Type | Description |
|-------|------|-------------|
| id | `String` | UUID |
| title | `String` | Session title |
| agent | `String` | Agent name |
| status | `Enum` | active, paused, completed, cancelled, reflecting |
| start_time | `DateTime` | Session start |
| end_time | `Option<DateTime>` | Session end |
| duration_seconds | `Option<u64>` | Calculated duration |
| task_ids | `Vec<String>` | Tasks worked on |
| context_ids | `Vec<String>` | Context items used |
| knowledge_ids | `Vec<String>` | Knowledge items referenced |
| active_theory_id | `Option<String>` | Bound theory |
| theory_ids | `Vec<String>` | Referenced theories |
| reflection_ids | `Vec<String>` | Generated reflections |
| goals | `Vec<String>` | Session goals |
| outcomes | `Vec<String>` | Session outcomes |
| space_metrics | `Option<SpaceMetrics>` | SPACE framework scores |
| dora_metrics | `Option<DoraMetrics>` | DORA metrics |
| tags | `Vec<String>` | Tags |
| metadata | `HashMap` | Additional data |

## Standard

| Field | Type | Description |
|-------|------|-------------|
| id | `String` | UUID |
| title | `String` | Standard title |
| description | `String` | Standard description |
| category | `Enum` | coding, testing, documentation, security, performance, process, architecture |
| status | `Enum` | draft, active, deprecated, superseded |
| version | `String` | Semantic version |
| agent | `String` | Author |
| effective_date | `DateTime` | When standard takes effect |
| superseded_by | `Option<String>` | Newer standard |
| supersedes | `Vec<String>` | Older standards replaced |
| related_standards | `Vec<String>` | Related standard IDs |
| requirements | `Vec<StandardRequirement>` | Requirements/guidelines |
| tags | `Vec<String>` | Tags |
| metadata | `HashMap` | Additional data |
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

## StaleTaskReport

| Field | Type | Description |
|-------|------|-------------|
| id | `String` | UUID |
| computed_at | `DateTime` | Computation timestamp |
| stale_threshold_hours | `i64` | Hours before considered stale |
| total_in_progress | `usize` | Total in-progress tasks |
| total_stale | `usize` | Stale tasks found |
| stale_tasks | `Vec<StaleTaskEntry>` | Stale task details |
| metadata | `HashMap` | Additional data |

## Task

| Field | Type | Description |
|-------|------|-------------|
| id | `String` | UUID |
| title | `String` | Task title |
| description | `String` | Detailed description |
| status | `Enum` | todo, in_progress, done, blocked, cancelled |
| priority | `Enum` | low, medium, high, critical |
| agent | `String` | Owner |
| start_time | `DateTime` | Start timestamp |
| end_time | `Option<DateTime>` | End timestamp |
| parent | `Option<String>` | Parent task ID |
| children | `Vec<String>` | Child task IDs |
| tags | `Vec<String>` | Tags |
| context_ids | `Vec<String>` | Associated context IDs |
| knowledge | `Vec<String>` | Knowledge items |
| files | `Vec<String>` | Related files |
| outcome | `Option<String>` | Task outcome |
| block_reason | `Option<String>` | Reason for blocking |
| workflow_id | `Option<String>` | Associated workflow ID |
| workflow_state | `Option<String>` | Current workflow state |
| metadata | `HashMap` | Additional data |

## TaskDurationReport

| Field | Type | Description |
|-------|------|-------------|
| id | `String` | UUID |
| project_path | `String` | Repo path |
| computed_at | `DateTime` | Computation timestamp |
| agent | `String` | Author |
| total_tasks_analyzed | `u64` | Total tasks |
| completed_tasks | `u64` | Completed tasks |
| task_durations | `Vec<TaskDurationEntry>` | Per-task duration details |
| median_duration_hours | `f64` | Median (completed only) |
| mean_duration_hours | `f64` | Mean (completed only) |
| min_duration_hours | `f64` | Min (completed only) |
| max_duration_hours | `f64` | Max (completed only) |
| metadata | `HashMap` | Additional data |

## Theory

| Field | Type | Description |
|-------|------|-------------|
| id | `String` | UUID |
| domain_name | `String` | Domain identifier |
| conceptual_model | `HashMap` | Concept to definition |
| system_mapping | `HashMap` | Concept to code location |
| design_rationale | `HashMap` | Decision to reason |
| invariants | `Vec<String>` | Must-be-true statements |
| iteration_count | `u32` | Theory version |
| reflection_ids | `Vec<String>` | Applied reflections |
| task_id | `Option<String>` | Associated task |
| agent | `String` | Author |
| created_at | `DateTime` | Creation timestamp |
| updated_at | `DateTime` | Last update |

## Workflow

| Field | Type | Description |
|-------|------|-------------|
| id | `String` | UUID |
| title | `String` | Workflow title |
| description | `String` | Description |
| status | `Enum` | active, inactive, draft, archived |
| agent | `String` | Owner |
| states | `Vec<WorkflowState>` | State definitions |
| transitions | `Vec<WorkflowTransition>` | Valid transitions |
| initial_state | `String` | Starting state |
| final_states | `Vec<String>` | Terminal states |
| entity_types | `Vec<String>` | Applicable entity types |
| permission_schemes | `Vec<PermissionScheme>` | Permission rules |
| event_handlers | `Vec<EventHandler>` | Event handlers |
| tags | `Vec<String>` | Tags |
| metadata | `HashMap` | Additional data |
| created_at | `DateTime` | Creation timestamp |
| updated_at | `DateTime` | Last update |

## WorkflowInstance

| Field | Type | Description |
|-------|------|-------------|
| id | `String` | UUID |
| workflow_id | `String` | Associated workflow ID |
| current_state | `String` | Current state name |
| context | `WorkflowExecutionContext` | Execution context (agent, variables, permissions) |
| status | `Enum` | Workflow execution status |
| started_at | `DateTime` | Start timestamp |
| updated_at | `DateTime` | Last update timestamp |
| completed_at | `Option<DateTime>` | Completion timestamp |
| execution_history | `Vec<WorkflowExecutionEvent>` | Execution history |
| step_count | `u64` | Transitions executed |
