---
name: engram-agent-types
description: "Understand and use engram agent types: the free-form --agent-type label stored in agent profiles. Covers built-in conventions, custom type registration, workspace config updates, discovery patterns, and the --specialization flag."
---

# Engram Agent Types

## Overview

`--agent-type` is a **free-form string label** stored in an agent's profile YAML (`.engram/agents/<name>.yaml`) and reflected in workspace config. It communicates a peer agent's role to orchestrators, tools, and other agents. The CLI accepts any string — there is no enum validation. The three names shown in `engram setup agent --help` (`coder`, `reviewer`, `planner`) are documentation defaults only.

## When to Use

Use this skill when:
- Registering a new agent with a custom role
- Querying what agents exist in a workspace and what their responsibilities are
- Writing an orchestrator that dispatches work by agent role
- Designing a multi-agent team and choosing type labels
- Auditing a workspace to understand agent coverage

## Built-in Types

These three types are written by `engram setup workspace` and have internal role mappings:

| `--agent-type` | Internal role mapping | Typical responsibility |
|---|---|---|
| `coder` | `implementation` | Write and modify code |
| `reviewer` | `quality_assurance` | Review PRs, flag issues |
| `planner` | `architecture` | Design, ADRs, task decomposition |

These are conventions, not constraints. You can rename or extend them freely.

## Custom Types

Any string is a valid agent type. Recommended custom types:

| `--agent-type` | Purpose |
|---|---|
| `orchestrator` | Coordinates subagents; owns task hierarchy |
| `tester` | Test writing and execution |
| `security-auditor` | OWASP review, threat modelling, CVE triage |
| `data-engineer` | Data pipelines, ETL, schema design |
| `deployer` | CI/CD automation, release management |
| `documenter` | Technical writing, runbooks, API docs |
| `researcher` | Investigation, spike work, literature review |

## The `--specialization` Flag

Use `--specialization` alongside `--agent-type` to add finer-grained detail that the type label alone cannot express. While agent-type describes the role category, specialization expresses tooling expertise, domain focus, or language specialisation.

Examples:
- `--agent-type "tester" --specialization "property-based testing, fuzzing, Rust"`
- `--agent-type "data-engineer" --specialization "dbt, Snowflake, streaming pipelines"`
- `--agent-type "security-auditor" --specialization "OWASP, threat modelling, CVE triage"`

Both fields are stored verbatim in the agent YAML and are searchable via `engram ask query`.

## Pattern

### Register a Custom Agent

```bash
engram setup agent \
  --name "security-bot" \
  --agent-type "security-auditor" \
  --specialization "OWASP, threat modelling, CVE triage"
```

This writes `.engram/agents/security-bot.yaml` with the provided type and specialization.

### Update Workspace Config

After creating the agent, add it to `.engram/config.yaml` under the workspace agents list so it participates in sync and is discoverable by other agents:

```yaml
# .engram/config.yaml
workspaces:
  default:
    agents:
      - coder
      - reviewer
      - planner
      - security-bot   # <-- add custom agent here
```

Without this entry, the agent profile exists on disk but is not part of the active workspace roster.

### Discover Agent Types

```bash
# Natural language search across all agent profiles and context
engram ask query "agent type security-auditor"
engram ask query "agents with specialization testing"

# List all registered agents by reading profiles directly
ls .engram/agents/
```

Each `.engram/agents/<name>.yaml` contains the `agent_type` and `specialization` fields for that agent.

## Naming Conventions

- **Lowercase, hyphen-separated**: `security-auditor`, `data-engineer`, not `SecurityAuditor` or `data_engineer`
- **Descriptive of role, not tooling**: `tester` not `pytest-runner`; `deployer` not `github-actions-bot`
- **Stable across sessions**: once registered and added to workspace config, the type label becomes a stable identifier that other agents and orchestrators depend on
- **Singular noun or noun-phrase**: `orchestrator`, `security-auditor`, not `orchestrating` or `does-security`

## Example

Register a dedicated test agent and wire it into the workspace:

```bash
# 1. Register the agent
engram setup agent \
  --name "test-runner" \
  --agent-type "tester" \
  --specialization "unit tests, integration tests, property-based testing, Rust"

# 2. Add to workspace config (.engram/config.yaml)
#    workspaces.default.agents: [..., test-runner]

# 3. Verify discovery
engram ask query "agent type tester"
```

An orchestrator can then dispatch by type:

```bash
# Find the tester agent UUID for task assignment
engram ask query "tester agent specialization"

# Assign a task to the tester
engram task update <TASK_UUID> --status in_progress
# Pass TASK_UUID to the test-runner agent
```

## Related Skills

- `engram-orchestrator` — dispatching tasks to agents by role
- `engram-subagent-register` — how a subagent claims a task and stores findings
- `engram-dispatching-parallel-agents` — running multiple typed agents concurrently
- `engram-delegate-to-agents` — agent catalog and delegation patterns
- `engram-use-engram-memory` — storing and retrieving agent context
