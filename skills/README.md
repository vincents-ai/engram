# Engram Skills

This directory contains skills for working with Engram memory system. Skills are reusable workflows that guide agents through common development tasks while storing all context, decisions, and outcomes in Engram entities.

## Overview

**Total Skills: 59**

Skills are organized by category and use Engram's entity system (context, reasoning, tasks, relationships) to create persistent, queryable memory across agent sessions.

## Skills by Category

### Meta Skills (6)
Core skills for working with Engram's memory system:

- **use-engram-memory** (`meta/use-engram-memory.md`) - Store context, decisions, and reasoning in Engram entities
- **commit-convention** (`meta/commit-convention.md`) - Commit format, hook setup, rejection handling, and why --no-verify is prohibited
- **delegate-to-agents** (`meta/delegate-to-agents.md`) - Break down work and delegate to specialized agents
- **audit-trail** (`meta/audit-trail.md`) - Create complete audit trail of all work
- **dispatching-parallel-agents** (`meta/dispatching-parallel-agents.md`) - Execute parallel agent coordination
- **persona-architect** (`meta/persona-architect.md`) - 8-step Persona Construction Protocol (PCP) with CoV/FAP/OV structured expert prompting methodology

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

### Planning Skills (7)
Skills for project planning and risk management:

- **risk-assessment** (`planning/risk-assessment.md`) - Identify and mitigate project risks
- **spike-investigation** (`planning/spike-investigation.md`) - Time-boxed research and exploration
- **dependency-mapping** (`planning/dependency-mapping.md`) - Map technical and team dependencies
- **backlog-refinement** (`planning/backlog-refinement.md`) - Refine user stories with acceptance criteria
- **capacity-planning** (`planning/capacity-planning.md`) - Estimate team capacity and velocity
- **roadmap-planning** (`planning/roadmap-planning.md`) - Create strategic product roadmaps
- **release-planning** (`planning/release-planning.md`) - Plan and coordinate releases

### Documentation Skills (6)
Skills for creating and maintaining documentation:

- **adr** (`documentation/adr.md`) - Architecture Decision Records with context
- **api-docs** (`documentation/api-docs.md`) - API documentation and specifications
- **technical-writing** (`documentation/technical-writing.md`) - Clear technical documentation
- **runbooks** (`documentation/runbooks.md`) - Operational runbooks for incidents
- **knowledge-transfer** (`documentation/knowledge-transfer.md`) - Transfer knowledge between team members
- **changelog** (`documentation/changelog.md`) - Maintain changelogs following conventions

### Architecture Skills (8)
Skills for system architecture and design:

- **system-design** (`architecture/system-design.md`) - High-level system architecture
- **security-architecture** (`architecture/security-architecture.md`) - Security design and threat modeling
- **scalability-analysis** (`architecture/scalability-analysis.md`) - Analyze scalability requirements
- **api-design** (`architecture/api-design.md`) - REST/GraphQL API design patterns
- **data-modeling** (`architecture/data-modeling.md`) - Database schema and data architecture
- **integration-patterns** (`architecture/integration-patterns.md`) - Service integration patterns
- **observability-design** (`architecture/observability-design.md`) - Logging, metrics, and tracing
- **refactoring-strategy** (`architecture/refactoring-strategy.md`) - Strategic code refactoring

### Quality Skills (5)
Skills for ensuring code quality and performance:

- **assumption-validation** (`quality/assumption-validation.md`) - Validate technical assumptions
- **edge-cases** (`quality/edge-cases.md`) - Identify and handle edge cases
- **tech-debt** (`quality/tech-debt.md`) - Track and manage technical debt
- **performance-analysis** (`quality/performance-analysis.md`) - Analyze performance bottlenecks
- **accessibility** (`quality/accessibility.md`) - Ensure WCAG accessibility compliance

### Review Skills (4)
Skills for code review and retrospectives:

- **security-review** (`review/security-review.md`) - Security-focused code review
- **code-quality** (`review/code-quality.md`) - Comprehensive code quality review
- **post-mortem** (`review/post-mortem.md`) - Blameless incident post-mortems
- **retrospective** (`review/retrospective.md`) - Team retrospectives and improvement

### Go-to-Market Skills (3)
Skills for product strategy, market validation, and launch execution:

- **market-validation** (`go-to-market/market-validation.md`) - Validate product ideas through interactive discovery and agent-driven research
- **gtm-strategy** (`go-to-market/gtm-strategy.md`) - Define ICP, positioning, pricing, channels, and success metrics
- **launch-execution** (`go-to-market/launch-execution.md`) - Build tactical launch plans with content calendar, outreach, and runbook

### Screenplay Skills (11)
Skills for writing screenplays with a co-writer using engram as persistent project memory:

- **session-start** (`screenplay/session-start.md`) - Start a session: load project context, confirm the goal, enforce co-writer rules
- **beat-sheet-builder** (`screenplay/beat-sheet-builder.md`) - Build and maintain the Save the Cat! 15-beat structure
- **outliner** (`screenplay/outliner.md`) - Build a scene-level outline from the beat sheet before drafting
- **logline-writer** (`screenplay/logline-writer.md`) - Craft and test the single-sentence story spine
- **theme-developer** (`screenplay/theme-developer.md`) - Develop and track the central theme throughout the script
- **world-builder** (`screenplay/world-builder.md`) - Define and maintain the rules, locations, and systems of the story world
- **character-developer** (`screenplay/character-developer.md`) - Build deep, consistent characters with arc, flaw, and voice
- **scene-writer** (`screenplay/scene-writer.md`) - Write individual scenes in correct Fountain format
- **dialogue-refiner** (`screenplay/dialogue-refiner.md`) - Polish dialogue for authenticity, subtext, and distinct character voice
- **plot-hole-finder** (`screenplay/plot-hole-finder.md`) - Systematically find and fix logic, motivation, and structural inconsistencies
- **rewriter** (`screenplay/rewriter.md`) - Guide the rewrite process through sequential passes

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

### Planning Workflow
```
risk-assessment → spike-investigation → dependency-mapping → backlog-refinement → release-planning
```

### Architecture Workflow
```
system-design → security-architecture → scalability-analysis → api-design → observability-design
```

### Documentation Workflow
```
adr → technical-writing → api-docs → runbooks → knowledge-transfer
```

### Quality Workflow
```
assumption-validation → edge-cases → performance-analysis → tech-debt → accessibility
```

### Review Workflow
```
security-review → code-quality → post-mortem → retrospective
```

### Go-to-Market Workflow
```
market-validation → gtm-strategy → launch-execution
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

- **Total Skills**: 58
- **Total Lines**: ~17,000
- **Shortest**: planning/roadmap-planning.md (126 lines)
- **Longest**: review/code-quality.md (1,007 lines)
- **Average**: ~300 lines per skill

## Version History

- **v0.1.0** (2026-01-22) - Initial 6 skills (compliance, meta, workflow)
- **v0.2.0** (2026-01-23) - Added 7 superpowers skills + cross-references (13 total)
- **v0.3.0** (2026-01-24) - Added 30 new skills across 5 categories (43 total)
  - Planning Skills: 7 (risk, spike, dependencies, backlog, capacity, roadmap, release)
  - Documentation Skills: 6 (adr, api-docs, technical-writing, runbooks, knowledge-transfer, changelog)
  - Architecture Skills: 8 (system-design, security, scalability, api, data-modeling, integration, observability, refactoring)
  - Quality Skills: 5 (assumption, edge-cases, tech-debt, performance, accessibility)
  - Review Skills: 4 (security-review, code-quality, post-mortem, retrospective)
- **v0.4.0** (2026-03-29) - Added 3 go-to-market skills (46 total)
  - Go-to-Market Skills: 3 (market-validation, gtm-strategy, launch-execution)
  - Corresponding pipelines: 101-market-validation, 102-gtm-strategy, 103-launch-execution
- **v0.5.0** (2026-04-04) - Added 11 screenplay skills (57 total)
  - Screenplay Skills: 11 (session-start, beat-sheet-builder, outliner, logline-writer, theme-developer, world-builder, character-developer, scene-writer, dialogue-refiner, plot-hole-finder, rewriter)
- **v0.6.0** (2026-04-09) - Added engram-persona-architect skill + Rust persona/lesson entities (58 total)
  - Meta Skills: +1 (persona-architect — 8-step PCP with CoV/FAP/OV structured expert prompting)
  - New Rust entities: Persona, Lesson with full CLI commands (create/list/show/update/delete)
  - Migrated 172 persona YAMLs with domain-adapted CoV/FAP/OV sections

## Resources

- **Engram Documentation**: See engram README.md
- **Superpowers Skills**: Source at `obras_superpowers_skills/skills/`
- **Agent Prompts**: `~/code/prompts/goose/agents/`
- **Pipeline Templates**: `~/code/prompts/goose/ai/pipelines/`
- **Compliance Prompts**: `~/code/prompts/compliance_and_certification/`
