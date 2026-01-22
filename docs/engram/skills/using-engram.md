# Using Engram

## Role Definition

You are an autonomous agent using Engram as your exclusive source of truth for task management, context discovery, and workflow execution. You NEVER rely on prompt injection or external context - all task information, relationships, and state must be retrieved from the Engram CLI.

## Source of Truth

**Strict Engram Reliance**: All context, task descriptions, relationships, and state must come from Engram CLI commands. Do not assume information from prompts or external sources.

## Skills Integration

Engram provides specialized skills for common workflows. Use these skills to ensure consistent, engram-integrated execution.

### Available Skills

```
./engram/skills/
├── meta/
│   ├── use-engram-memory.md      # Core memory integration skill
│   ├── delegate-to-agents.md     # Delegation using engram-adapted agents
│   └── audit-trail.md            # Complete audit documentation
├── workflow/
│   └── plan-feature.md           # Feature planning with engram tasks
└── compliance/
    └── check-compliance.md       # Compliance checking with engram storage
```

### Using Skills

When starting any work, check if a relevant skill exists:

```bash
# Check available skills
ls ./engram/skills/

# Use a skill by reading its documentation
cat ./engram/skills/meta/use-engram-memory.md
```

## Prompt Templates

Engram provides engram-adapted agent prompts that integrate with the memory system.

### Agent Prompts

```
./engram/prompts/agents/
├── 01-the-one.yaml              # Orchestrator (adapted)
├── 03-the-architect.yaml        # Architecture design (adapted)
├── 05-the-deconstructor.yaml    # Task breakdown (adapted)
├── _template-engram-adapted.yaml # Template for adapting agents
└── [160+ specialized agents]
```

### Pipeline Templates

```
./engram/prompts/ai/pipelines/
├── 01-greenfield-feature-launch.yaml  # Feature development pipeline (adapted)
├── _template-engram-adapted.yaml       # Template for adapting pipelines
└── [100+ specialized pipelines]
```

### Compliance Prompts

```
./engram/prompts/compliance_and_certification/
└── prompts/audit_checkpoints/
    ├── igaming/           # Gaming compliance (GLI, MGA, UKGC)
    ├── saas_it/           # SOC2, ISO27001, PCI DSS
    ├── data_protection/   # GDPR, CCPA, PIPEDA
    ├── eu_regulations/    # DSA, DMA, AI Act, NIS2, DORA
    ├── gaming_certification/  # RNG, RTP, Fairness
    ├── software_development/  # OWASP, SDL, ISO 12207
    ├── german_compliance/     # GoBD, DSGVO, BSI
    ├── medical_device/        # IEC 62304
    ├── cross_compliance/      # Multi-framework integration
    └── cybersecurity_policies/ # NIST CSF, RMF, ISO 27002
```

## Workflow Protocol

### 1. Task Initialization

```bash
# Update status to in_progress
engram task update {{TASK_ID}} --status inprogress

# Discover all related context
engram relationship connected --entity-id {{TASK_ID}}

# Read task description
engram task show {{TASK_ID}}
```

### 2. Context Discovery

```bash
# Find all references (context, docs, designs)
engram relationship connected --entity-id {{TASK_ID}} --relationship-type references

# Find reasoning (decisions, trade-offs)
engram relationship connected --entity-id {{TASK_ID}} --relationship-type documents

# Find subtasks (if task has children)
engram relationship connected --entity-id {{TASK_ID}} --relationship-type contains

# Get specific entity details
engram context show [CONTEXT_ID]
engram reasoning show [REASONING_ID]
```

### 3. Execution Phase

```bash
# Locate files via relationships
engram relationship connected --entity-id {{TASK_ID}} | grep -E "context|reasoning"

# Perform work (coding, writing, etc.)

# Store progress as reasoning
engram reasoning create \
  --title "[Work] Progress: [What was done]" \
  --task-id {{TASK_ID}} \
  --content "[Details of work completed]"

# Store final result as context
engram context create \
  --title "[Result] [Work Description]" \
  --content "[Output, findings, or artifact summary]"

# Link result to task
engram relationship create \
  --source-id {{TASK_ID}} \
  --target-id [RESULT_CONTEXT_ID] \
  --produces

# Commit with task reference
git commit -m "type: description [{{TASK_ID}}]"
```

### 4. Completion

```bash
# Validate workflow compliance
engram validate check

# Update task status
engram task update {{TASK_ID}} --status done --outcome "[Summary of work]"

# Output summary
echo "Task {{TASK_ID}} completed. Entities created: [CONTEXT_ID], [REASONING_ID]"
```

## Optimization Principles

- **Context-First**: Always gather complete context via Engram before starting execution
- **Relationship Navigation**: Use relationship traversal as the primary discovery mechanism
- **Entity Linking**: Link all created artifacts back to tasks for future discoverability
- **Progress Tracking**: Store progress as reasoning entities, not just in conversation
- **Skills Usage**: Check for relevant skills before starting work

## Error Handling

- If a task ID is invalid: Stop and report the error
- If required relationships are missing: Create them or request clarification
- If context is incomplete: Use relationship traversal to find more information
- Never proceed without full Engram-validated context

## Integration with External Prompts

When using external prompts (from `./engram/prompts/`), always adapt them:

1. **Add task_id parameter** to all prompts
2. **Include engram commands** in the prompt body
3. **Update response schema** to return engram entity IDs
4. **Link outputs** to the parent task via relationships

Example adaptation pattern:
```yaml
# Original parameter
parameters:
  properties:
    context:
      type: string
      description: "The context"

# Adapted parameter
parameters:
  properties:
    task_id:
      type: string
      description: "The engram task ID for this work"
    context:
      type: string
      description: "The context"
  required:
    - task_id
    - context
```
