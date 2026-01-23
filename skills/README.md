# Engram Skills

This directory contains skills for working with Engram memory system. Skills are reusable workflows that guide agents through common development tasks while storing all context, decisions, and outcomes in Engram entities.

## Overview

**Total Skills: 13**

Skills are organized by category and use Engram's entity system (context, reasoning, tasks, relationships) to create persistent, queryable memory across agent sessions.

## Skills by Category

### Meta Skills (4)
Core skills for working with Engram's memory system:

- **use-engram-memory** (`meta/use-engram-memory.md`) - Store context, decisions, and reasoning in Engram entities
- **delegate-to-agents** (`meta/delegate-to-agents.md`) - Break down work and delegate to specialized agents
- **audit-trail** (`meta/audit-trail.md`) - Create complete audit trail of all work
- **dispatching-parallel-agents** (`meta/dispatching-parallel-agents.md`) - Execute parallel agent coordination

### Workflow Skills (4)
Skills for planning and executing feature development:

- **brainstorming** (`workflow/brainstorming.md`) - Turn ideas into designs with engram context
- **writing-plans** (`workflow/writing-plans.md`) - Plan implementation with task hierarchy
- **plan-feature** (`workflow/plan-feature.md`) - Use pipeline templates for structured planning
- **requesting-code-review** (`workflow/requesting-code-review.md`) - Code review with engram tracking

### Development Skills (2)
Skills for implementing features:

- **test-driven-development** (`development/test-driven-development.md`) - TDD with engram evidence
- **subagent-driven-development** (`development/subagent-driven-development.md`) - Multi-agent development workflows

### Debugging Skills (1)
Skills for systematic problem-solving:

- **systematic-debugging** (`debugging/systematic-debugging.md`) - Debug with engram audit trail

### Testing Skills (1)
Skills for comprehensive testing:

- **testing** (`testing/skill.md`) - Track test execution, results, and coverage

### Compliance Skills (1)
Skills for ensuring regulatory compliance:

- **check-compliance** (`compliance/check-compliance.md`) - Validate against compliance frameworks

## Skill Format

Each skill follows this structure:

```markdown
---
name: engram-skill-name
description: "Brief description of what this skill does"
---

# Skill Title

## Overview
What this skill helps you do

## When to Use
Specific situations where this skill applies

## The Pattern
Step-by-step workflow

## Example
Concrete example with engram commands

## Related Skills
Links to other relevant skills
```

## Using Skills

### Discover Skills

```bash
# List all skills
ls engram/skills/**/*.md

# Search for specific skills
grep -r "TDD\|testing" engram/skills/
```

### Read a Skill

```bash
# Read skill content
cat engram/skills/workflow/brainstorming.md

# Or in your editor
vim engram/skills/development/test-driven-development.md
```

### Follow a Skill

Skills provide:
1. **When to Use** - Conditions for applying the skill
2. **The Pattern** - Step-by-step workflow
3. **Engram Commands** - Actual commands to run
4. **Example** - Concrete walkthrough

## Workflow Chains

Skills reference each other to form complete workflows:

### Feature Development Workflow
```
brainstorming → writing-plans → test-driven-development → requesting-code-review
```

### Parallel Agent Workflow
```
plan-feature → delegate-to-agents → dispatching-parallel-agents → subagent-driven-development
```

### Debugging Workflow
```
systematic-debugging → test-driven-development → audit-trail
```

### Compliance Workflow
```
check-compliance → testing → audit-trail
```

## Cross-References

All skills include a "Related Skills" section that references other skills using the format:
```
- `engram-skill-name` - Brief description
```

This creates a discoverable network of related workflows.

## Skill Principles

All skills follow these principles:

1. **Store Everything** - All context, decisions, and outcomes go into Engram
2. **Link Entities** - Create relationships between tasks, context, and reasoning
3. **Queryable** - Use `engram relationship connected` to retrieve full context
4. **Persistent** - Memory survives across agent sessions
5. **Traceable** - Complete audit trail of all work

## Examples

### Using brainstorming skill

```bash
# User asks: "Add authentication to the API"

# 1. Follow brainstorming skill to design the feature
# 2. Create engram context entities for each design section
engram context create --title "Design: Auth API - Architecture" \
  --content "JWT-based stateless auth with refresh tokens..."

# 3. Store trade-off analysis
engram reasoning create --title "Trade-off: JWT vs Sessions" \
  --task-id $TASK_ID --confidence 0.9

# 4. Link to task
engram relationship create --source-id $TASK_ID --target-id $CONTEXT_ID --references
```

### Using test-driven-development skill

```bash
# 1. Create test plan
engram context create --title "Test Plan: Auth API" \
  --content "Unit: 100% of auth logic\nIntegration: Login/logout/refresh"

# 2. Follow TDD cycle (red-green-refactor)
# 3. Store each phase
engram reasoning create --title "TDD: Red - Auth Tests" \
  --task-id $TASK_ID --confidence 1.0

# 4. Link test results
engram relationship create --source-id $TASK_ID --target-id $TEST_RESULTS --validates
```

### Using delegate-to-agents skill

```bash
# 1. Break down work
engram context create --title "Delegation Plan: Auth API" \
  --content "1. Architecture → 03-the-architect\n2. Implementation → 70-the-rustacean..."

# 2. Dispatch agents
# 3. Track outcomes
engram reasoning create --title "Delegation Complete: Auth API" \
  --task-id $TASK_ID --confidence 1.0
```

## Contributing Skills

When creating new skills:

1. **Use the standard format** - Follow the structure above
2. **Add YAML frontmatter** - Include name and description
3. **Provide concrete examples** - Show actual engram commands
4. **Add cross-references** - Link to related skills
5. **Focus on workflows** - Not just documentation

Place new skills in the appropriate category directory.

## Integration with Engram

Skills integrate with Engram's core features:

- **Context entities** - Background info, design docs, plans
- **Reasoning entities** - Decisions, trade-offs, confidence levels
- **Task entities** - Work items, implementation steps
- **Relationships** - Links between all entities
- **Validation** - Commit validation requires task+reasoning+context

## Skill Statistics

- **Total Skills**: 13
- **Total Lines**: 2,603
- **Shortest**: testing/skill.md (149 lines)
- **Longest**: dispatching-parallel-agents (352 lines)
- **Average**: 200 lines per skill

## Version History

- **v0.1.0** (2026-01-22) - Initial 6 skills (compliance, meta, workflow)
- **v0.2.0** (2026-01-23) - Added 7 superpowers skills + cross-references (13 total)

## Resources

- **Engram Documentation**: See engram README.md
- **Superpowers Skills**: Source at `obras_superpowers_skills/skills/`
- **Agent Prompts**: `~/code/prompts/goose/agents/`
- **Pipeline Templates**: `~/code/prompts/goose/ai/pipelines/`
- **Compliance Prompts**: `~/code/prompts/compliance_and_certification/`
