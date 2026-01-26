# Engram Prompt Engineering Guide

This guide explains how to construct effective prompts for agents using the Engram CLI. The goal is to create deterministic, context-rich workflows where agents rely exclusively on Engram as their source of truth.

## Philosophy

- **Single Source of Truth**: Agents should look up task context via `engram task show` and `engram relationship`, not rely on prompt injection.
- **Atomic Operations**: Workflows should be broken down into discrete steps tracked by Engram tasks.
- **Evidence-Based Validation**: Every step must be verifiable via `engram validate` with provide evidence-based validation for all final claims instead of unsubstantiated assertions.
- **Skills First**: Check for existing skills before creating new prompts.

## Available Prompts

Engram provides pre-built engram-adapted prompts for common workflows.

### Agent Prompts (170+)

Location: `./engram/prompts/agents/`

| Category | Count | Examples |
|----------|-------|----------|
| Core Orchestration | 3 | 01-the-one, 03-the-architect, 05-the-deconstructor |
| Development | 50+ | rustacean, gopher, type-safe, api-designer |
| Testing & Quality | 20+ | tester, qa-strategist, troubleshooter |
| Infrastructure | 30+ | devops-engineer, rustacean, gopher |
| Specialization | 60+ | accessibility-advocate, security-auditor, technical-writer |

### Pipeline Templates (100+)

Location: `./engram/prompts/ai/pipelines/`

| Category | Count | Examples |
|----------|-------|----------|
| Feature Development | 10+ | greenfield-feature-launch, ui-overhaul-refresh |
| Bug Fixing | 5+ | bug-hunt-triage |
| Infrastructure | 20+ | nixos-immutable-deploy, k8s-cluster-upgrade |
| Security | 10+ | security-penetration-test, red-team-exercise |
| Compliance | 15+ | gdpr-compliance, soc2-audit |

### Compliance Prompts (250+)

Location: `./engram/prompts/compliance_and_certification/prompts/audit_checkpoints/`

| Framework | Coverage |
|-----------|----------|
| iGaming | GLI, MGA, UKGC, G4 Standards |
| SaaS/IT | SOC2, ISO27001, PCI DSS |
| Data Protection | GDPR, CCPA, PIPEDA |
| EU Regulations | DSA, DMA, AI Act, NIS2, DORA, CSRD |
| Gaming Certification | RNG, RTP, Fairness |
| Software Development | OWASP, Microsoft SDL, ISO 12207 |
| German Compliance | GoBD, DSGVO, BSI IT-Grundschutz |
| Medical Device | IEC 62304 |
| Cybersecurity | NIST CSF, RMF, ISO 27002, CIS Controls |

## Evidence-Based Validation Requirements

**CRITICAL**: All prompts must enforce evidence-based validation for all final claims instead of unsubstantiated assertions.

### Required Evidence Types

Every assertion or claim must be supported by at least one of:

1. **Code Evidence**
   - Specific function names, file paths, and line numbers
   - Actual code snippets demonstrating the claim
   - Test results showing behavior

2. **Execution Evidence**
   - Command outputs with timestamps
   - Log files or error messages
   - Performance metrics or benchmarks

3. **Documentation Evidence**
   - Citations from official documentation
   - Links to specifications or standards
   - References to established patterns or frameworks

4. **Measurement Evidence**
   - Quantifiable metrics (e.g., "reduced from 500ms to 120ms")
   - Test coverage percentages
   - Before/after comparisons

### Evidence Format Requirements

```
## Claim: [Your assertion]

### Evidence:
- **Code Reference**: `src/file.rs:123` - Function `process_data()` shows...
- **Test Results**: `cargo test` output showing 100% pass rate
- **Execution Log**: `[2024-01-26 10:15:32] Successfully processed 1,247 records`
- **Documentation**: https://docs.rust-lang.org/std/result/enum.Result.html
```

### Invalid Examples

❌ "The code is more efficient"
❌ "This improves security"
❌ "The refactoring is better"

### Valid Examples

✅ "Reduced processing time from 500ms to 120ms (60% improvement) as measured by `hyperfine` benchmarks"
✅ "Fixed buffer overflow vulnerability in `src/parser.rs:45` by adding bounds check, verified by `cargo-audit` scan"
✅ "Increased test coverage from 72% to 95% in `src/utils/` module, confirmed by `cargo tarpaulin` report"

## Standard Prompt Template

Use this template to instruct agents. Replace `{{TASK_ID}}` with the actual UUID.

```markdown
You are an autonomous agent using Engram for state management.
**Task ID**: `{{TASK_ID}}`

### Protocol

1. **Initialize**:
   ```bash
   engram task update {{TASK_ID}} --status inprogress
   engram relationship connected --entity-id {{TASK_ID}} to find related context/reasoning.
   ```

2. **Execute**:
   - Read the task description: `engram task show {{TASK_ID}}`
   - Access linked contexts: `engram context show [CONTEXT_ID]`
   - Store progress: `engram reasoning create --title "[Progress]" --task-id {{TASK_ID}} --content "[Details]"`

3. **Validate**:
   - Run `engram validate check`
   - Run tests/linters as required by the task
   - **Evidence-Based Validation**: Provide evidence-based validation for all final claims instead of unsubstantiated assertions. Every assertion must be supported by:
     - Concrete evidence from code, tests, logs, or documentation
     - Verifiable measurements or test results
     - Specific examples or citations
     - Quantifiable metrics where applicable

4. **Complete**:
   - Store result: `engram context create --title "[Result]" --content "[Output]"`
   - Link result: `engram relationship create --source-id {{TASK_ID}} --target-id [RESULT_ID] --produces`
   - Update status: `engram task update {{TASK_ID}} --status done`
```

## Engram-Adapted Prompt Pattern

All engram-adapted prompts follow this pattern:

### 1. Parameters Include task_id

```yaml
parameters:
  properties:
    task_id:
      type: string
      description: "The engram task ID for this work"
    context:
      type: string
      description: "Additional context"
  required:
    - task_id
    - context
```

### 2. Engram Commands in Prompt Body

```markdown
YOUR WORK:

STEP 1: Retrieve task
```bash
engram task show {{task_id}}
```

STEP 2: Get related context
```bash
engram relationship connected --entity-id {{task_id}} --references
```

STEP 3: Store progress
```bash
engram reasoning create --title "[Work] Progress" --task-id {{task_id}} --content "[What you did]"
```

STEP 4: Complete
```bash
engram context create --title "[Result]" --content "[Output]"
engram relationship create --source-id {{task_id}} --target-id [RESULT_ID] --produces
engram task update {{task_id}} --status done --outcome "[Summary]"
```
```

### 3. Response Includes Entity IDs

```yaml
response:
  format: json
  schema:
    type: object
    properties:
      task_id:
        type: string
        description: "The task ID (echoed for confirmation)"
      status:
        type: string
        enum: ["completed", "in_progress"]
        description: "Status of work"
      result_context_id:
        type: string
        description: "The engram context ID for the result"
    required:
      - task_id
      - status
      - result_context_id
```

## Workflow Examples

### 1. Research & Planning

For tasks requiring investigation before implementation.

```markdown
**Phase**: Research
**Goal**: Analyze requirements and create a plan.

1. Initialize:
   ```bash
   engram task update {{TASK_ID}} --status inprogress
   ```

2. Gather Info:
   ```bash
   engram relationship connected --entity-id {{TASK_ID}} --references
   engram context list | grep -i [keyword]
   ```

3. Document Plan:
   ```bash
   engram reasoning create \
     --title "Research: [Topic] - Findings" \
     --task-id {{TASK_ID}} \
     --content "[Research findings]"
   
   engram context create \
     --title "Plan: [Feature] Implementation" \
     --content "[Implementation approach]"
   
   engram relationship create \
     --source-id {{TASK_ID}} \
     --target-id [PLAN_CONTEXT_ID] \
     --references
   ```

4. Complete:
   ```bash
   engram task update {{TASK_ID}} --status done --outcome "Research complete, plan created"
   ```

### 2. Using Engram-Adapted Agent

For delegating work to specialized agents.

```markdown
**Agent**: [Agent Name from ./engram/prompts/agents/]
**Task ID**: {{TASK_ID}}

1. The agent should retrieve its task:
   ```bash
   engram task show {{TASK_ID}}
   ```

2. The agent should get context:
   ```bash
   engram relationship connected --entity-id {{TASK_ID}} --references
   ```

3. The agent should store progress:
   ```bash
   engram reasoning create --title "[Agent] Progress" --task-id {{TASK_ID}} --content "[What was done]"
   ```

4. The agent should complete:
   ```bash
   engram context create --title "[Agent] Result" --content "[Output]"
   engram relationship create --source-id {{TASK_ID}} --target-id [RESULT_ID] --produces
   engram task update {{TASK_ID}} --status done --outcome "[Summary]"
   ```

Example agents:
- **Architecture**: Use `./engram/prompts/agents/03-the-architect.yaml`
- **Task Breakdown**: Use `./engram/prompts/agents/05-the-deconstructor.yaml`
- **Testing**: Use `./engram/prompts/agents/15-the-tester.yaml`
- **Documentation**: Use `./engram/prompts/agents/41-the-technical-writer.yaml`

### 3. Using Pipeline

For multi-stage workflows.

```markdown
**Pipeline**: Feature Development
**Template**: ./engram/prompts/ai/pipelines/01-greenfield-feature-launch.yaml

1. Creates engram workflow with stages: strategy → architecture → breakdown
2. Calls engram-adapted agents for each stage
3. Stores all outputs in engram entities
4. Returns workflow ID and created entity IDs

Parameters:
- feature_idea: "Description of feature"
- parent_task_id: "{{TASK_ID}}"
```

### 4. Compliance Checking

For verifying work against compliance frameworks.

```markdown
**Compliance Check**: [Framework]
**Prompts**: ./engram/prompts/compliance_and_certification/prompts/audit_checkpoints/[framework]/

1. Get context:
   ```bash
   engram context list | grep [compliance-area]
   ```

2. Run compliance check:
   ```bash
   # Use relevant compliance prompts from:
   # ./engram/prompts/compliance_and_certification/prompts/audit_checkpoints/
   ```

3. Store results:
   ```bash
   engram compliance create \
     --title "Compliance: [Feature] - [Framework]" \
     --category [security/privacy/quality] \
     --requirements "✅ [Requirement 1]\n✅ [Requirement 2]"

   engram context create \
     --title "Audit: [Feature] - [Framework]" \
     --content "[Audit findings]"
   
   engram relationship create \
     --source-id {{TASK_ID}} \
     --target-id [AUDIT_CONTEXT_ID] \
     --fulfills
   ```

### 5. Documentation

For writing docs.

```markdown
**Phase**: Documentation
**Goal**: Update documentation to match code/specs.

1. Initialize:
   ```bash
   engram task update {{TASK_ID}} --status inprogress
   ```

2. Get source:
   ```bash
   engram relationship connected --entity-id {{TASK_ID}} --references
   ```

3. Write:
   - Update markdown files
   - Reference engram entities in docs

4. Store:
   ```bash
   engram context create \
     --title "Documentation: [File/Feature]" \
     --content "[Summary of docs updated]"
   
   engram relationship create \
     --source-id {{TASK_ID}} \
     --target-id [DOC_CONTEXT_ID] \
     --produces
   ```

5. Commit:
   ```bash
   git commit -m "docs: update [file] [{{TASK_ID}}]"
   ```

6. Complete:
   ```bash
   engram task update {{TASK_ID}} --status done --outcome "Documentation complete"
   ```

## Best Practices

- **No Hallucinations**: If a UUID isn't found, stop and error out. Don't invent IDs.
- **Link Everything**: If you create a new artifact (file, module), create a corresponding Engram entity and link it to the task.
- **Status Hygiene**: Always move tasks to `inprogress` before starting and `done` only after validation.
- **Evidence-Based Validation**: Provide evidence-based validation for all final claims instead of unsubstantiated assertions. Every claim must be backed by:
  - Verifiable evidence (code, tests, logs, documentation)
  - Specific examples and citations
  - Quantifiable metrics and measurements
  - Test results or execution outputs
- **Skills First**: Check `./engram/skills/` for existing skills before creating new prompts.
- **Use Adapted Prompts**: Prefer engram-adapted prompts from `./engram/prompts/` over generic prompts.
- **Store Progress**: Don't just complete tasks - store intermediate progress as reasoning entities.

## Prompt Locations Quick Reference

| Purpose | Location |
|---------|----------|
| Orchestration | `./engram/prompts/agents/01-the-one.yaml` |
| Architecture | `./engram/prompts/agents/03-the-architect.yaml` |
| Task Breakdown | `./engram/prompts/agents/05-the-deconstructor.yaml` |
| Feature Pipeline | `./engram/prompts/ai/pipelines/01-greenfield-feature-launch.yaml` |
| Security Compliance | `./engram/prompts/compliance_and_certification/prompts/audit_checkpoints/saas_it/` |
| Data Privacy | `./engram/prompts/compliance_and_certification/prompts/audit_checkpoints/data_protection/` |
| EU Regulations | `./engram/prompts/compliance_and_certification/prompts/audit_checkpoints/eu_regulations/` |
| All Skills | `./engram/skills/` |
