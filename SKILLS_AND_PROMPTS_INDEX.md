# Engram Skills and Prompts Index

This index provides quick access to all skills, prompts, and workflows available in engram.

## Quick Start

1. **New to engram?** Start with [Using Engram](docs/engram/skills/using-engram.md)
2. **Creating prompts?** See the [Prompt Engineering Guide](docs/engram/skills/prompt_guide.md)
3. **Need a skill?** Check the Skills section below

## Skills

Skills are documented workflows that guide agents through common patterns.

### Location
```
./engram/skills/
```

### Available Skills

| Skill | File | Purpose |
|-------|------|---------|
| Use Engram Memory | `meta/use-engram-memory.md` | Core skill for using engram as persistent memory |
| Delegate to Agents | `meta/delegate-to-agents.md` | Delegation using engram-adapted agents |
| Audit Trail | `meta/audit-trail.md` | Complete audit documentation |
| Plan Feature | `workflow/plan-feature.md` | Feature planning with engram tasks |
| Check Compliance | `compliance/check-compliance.md` | Compliance checking with engram storage |

### Using Skills

```bash
# List available skills
ls ./engram/skills/

# Read a skill
cat ./engram/skills/meta/use-engram-memory.md
```

## Prompts

Prompts are YAML templates for agent orchestration.

### Location
```
./engram/prompts/
```

### Agent Prompts (170+)

```
./engram/prompts/agents/
```

| Category | Count | Key Files |
|----------|-------|-----------|
| Core Orchestration | 3 | `01-the-one.yaml`, `02-the-sidekick.yaml`, `05-the-deconstructor.yaml` |
| Architecture | 5 | `03-the-architect.yaml`, `17-the-api-designer.yaml` |
| Development | 50+ | `70-the-rustacean.yaml`, `71-the-gopher.yaml`, `72-the-type-safe.yaml` |
| Testing | 10+ | `15-the-tester.yaml`, `48-the-qa-strategist.yaml` |
| Infrastructure | 30+ | `45-the-devops-engineer.yaml`, `77-the-shell.yaml` |
| Security | 10+ | `20-the-critical-roller.yaml`, `14-the-ethics-auditor.yaml` |
| Documentation | 5 | `41-the-technical-writer.yaml`, `32-the-knowledge-base-curator.yaml` |
| Specialization | 60+ | See full list below |

#### Core Agents (Engram-Adapted)

| File | Purpose | Engram Integration |
|------|---------|-------------------|
| `01-the-one.yaml` | Orchestrator - creates engram tasks and delegates | Creates tasks, stores delegation plans in context |
| `03-the-architect.yaml` | System design and architecture | Stores design in context, decisions in reasoning |
| `05-the-deconstructor.yaml` | Task breakdown into atomic units | Creates engram subtasks with hierarchy |

#### Development Agents

| File | Language/Specialty |
|------|-------------------|
| `70-the-rustacean.yaml` | Rust development |
| `71-the-gopher.yaml` | Go development |
| `72-the-type-safe.yaml` | TypeScript/JavaScript |
| `17-the-api-designer.yaml` | API design |
| `18-the-integration-specialist.yaml` | Integration |
| `19-the-performance-tuner.yaml` | Performance |
| `21-the-database-specialist.yaml` | Database |
| `37-the-prompt-engineer.yaml` | Prompt engineering |

#### Quality Agents

| File | Purpose |
|------|---------|
| `15-the-tester.yaml` | Testing |
| `48-the-qa-strategist.yaml` | QA strategy |
| `34-the-troubleshooter.yaml` | Debugging |
| `20-the-critical-roller.yaml` | Critical systems |

### Pipeline Templates (100+)

```
./engram/prompts/ai/pipelines/
```

| Category | Count | Examples |
|----------|-------|----------|
| Feature Development | 10+ | `01-greenfield-feature-launch.yaml`, `02-ui-overhaul-refresh.yaml` |
| Bug Fixing | 5+ | `03-bug-hunt-triage.yaml` |
| API/Backend | 10+ | `04-api-modernisation.yaml`, `05-database-migration.yaml` |
| DevOps/Infrastructure | 20+ | `11-nixos-immutable-deploy.yaml`, `16-k8s-cluster-upgrade.yaml` |
| Security | 10+ | `13-security-penetration-test.yaml`, `48-red-team-exercise.yaml` |
| Compliance | 15+ | Various compliance check pipelines |

#### Core Pipelines (Engram-Adapted)

| File | Purpose | Engram Integration |
|------|---------|-------------------|
| `01-greenfield-feature-launch.yaml` | New feature from idea to task breakdown | Creates engram workflow with stages |

### Compliance Prompts (250+)

```
./engram/prompts/compliance_and_certification/prompts/audit_checkpoints/
```

| Framework | Location | Coverage |
|-----------|----------|----------|
| **iGaming** | `igaming/` | GLI-11, GLI-19, GLI-33, MGA, UKGC, G4 |
| **SaaS/IT** | `saas_it/` | SOC2, ISO27001, PCI DSS |
| **Data Protection** | `data_protection/` | GDPR, CCPA, PIPEDA |
| **EU Regulations** | `eu_regulations/` | DSA, DMA, AI Act, NIS2, DORA, CSRD, CSDDD |
| **Gaming Certification** | `gaming_certification/` | RNG, RTP, Fairness |
| **Software Development** | `software_development/` | OWASP, Microsoft SDL, ISO 12207 |
| **German Compliance** | `german_compliance/` | GoBD, DSGVO, BSI IT-Grundschutz |
| **Medical Device** | `medical_device/` | IEC 62304 |
| **Cybersecurity** | `cybersecurity_policies/` | NIST CSF, RMF, ISO 27002, CIS Controls |
| **Cross-Compliance** | `cross_compliance/` | Multi-framework integration |

#### Compliance Categories

| Area | Frameworks |
|------|------------|
| Security | SOC2, ISO27001, PCI DSS, NIST CSF, CIS Controls |
| Privacy | GDPR, CCPA, PIPEDA, DSGVO |
| EU Digital | DSA, DMA, AI Act, Data Act |
| EU Cyber | NIS2, DORA |
| Gaming | GLI, MGA, UKGC, G4, RNG, RTP |
| Development | OWASP, SDL, ISO 12207 |
| Medical | IEC 62304 |

## Templates

Use these templates to create new engram-adapted prompts:

| Template | Location | Purpose |
|----------|----------|---------|
| Agent Template | `agents/_template-engram-adapted.yaml` | Adapt any agent for engram |
| Pipeline Template | `ai/pipelines/_template-engram-adapted.yaml` | Adapt any pipeline for engram |

### Using Templates

```bash
# Copy agent template
cp ./engram/prompts/agents/_template-engram-adapted.yaml ./engram/prompts/agents/XX-my-new-agent.yaml

# Copy pipeline template
cp ./engram/prompts/ai/pipelines/_template-engram-adapted.yaml ./engram/prompts/ai/pipelines/XX-my-new-pipeline.yaml
```

## Documentation

| Document | Location | Purpose |
|----------|----------|---------|
| Using Engram | `docs/engram/skills/using-engram.md` | Core workflow protocol |
| Prompt Engineering Guide | `docs/engram/skills/prompt_guide.md` | Prompt construction patterns |
| Validation | `docs/validation.md` | Commit validation rules |

## Workflow Examples

### Basic Feature Development

1. **Plan**: Use `01-greenfield-feature-launch.yaml` pipeline
2. **Design**: Use `03-the-architect.yaml` for architecture
3. **Breakdown**: Use `05-the-deconstructor.yaml` for tasks
4. **Implement**: Use appropriate language agent (e.g., `70-the-rustacean.yaml`)
5. **Test**: Use `15-the-tester.yaml`
6. **Document**: Use `41-the-technical-writer.yaml`

### Security Compliance Check

1. **Identify Framework**: Choose from `compliance_and_certification/prompts/audit_checkpoints/`
2. **Run Check**: Use relevant compliance prompts
3. **Store Results**: Use `check-compliance.md` skill
4. **Verify**: `engram validate check`

### Delegation Workflow

1. **Analyze**: Use `01-the-one.yaml` to create tasks
2. **Delegate**: Use `delegate-to-agents.md` skill
3. **Track**: Monitor via `engram relationship connected`
4. **Review**: Check results with `engram context list`

## File Structure

```
./engram/
├── skills/                    # Skills documentation
│   ├── meta/
│   │   ├── use-engram-memory.md
│   │   ├── delegate-to-agents.md
│   │   └── audit-trail.md
│   ├── workflow/
│   │   └── plan-feature.md
│   └── compliance/
│       └── check-compliance.md
│
├── prompts/                   # Prompt templates
│   ├── agents/               # 170+ agent prompts
│   │   ├── 01-the-one.yaml (adapted)
│   │   ├── 03-the-architect.yaml (adapted)
│   │   ├── 05-the-deconstructor.yaml (adapted)
│   │   ├── _template-engram-adapted.yaml
│   │   └── [160+ more agents]
│   │
│   ├── ai/
│   │   └── pipelines/        # 100+ pipeline templates
│   │       ├── 01-greenfield-feature-launch.yaml (adapted)
│   │       ├── _template-engram-adapted.yaml
│   │       └── [100+ more pipelines]
│   │
│   └── compliance_and_certification/  # 250+ compliance prompts
│       └── prompts/audit_checkpoints/
│           ├── igaming/
│           ├── saas_it/
│           ├── data_protection/
│           ├── eu_regulations/
│           ├── gaming_certification/
│           ├── software_development/
│           ├── german_compliance/
│           ├── medical_device/
│           ├── cross_compliance/
│           └── cybersecurity_policies/
│
├── docs/
│   └── engram/
│       └── skills/
│           ├── using-engram.md
│           └── prompt_guide.md
│
└── README.md                  # This file
```

## Statistics

| Category | Count |
|----------|-------|
| Skills | 5 |
| Agent Prompts | 170+ |
| Pipeline Templates | 100+ |
| Compliance Prompts | 160+ (Markdown) |
| Documentation Files | 4 |

## Getting Help

- **For workflows**: See [Using Engram](docs/engram/skills/using-engram.md)
- **For prompts**: See [Prompt Engineering Guide](docs/engram/skills/prompt_guide.md)
- **For compliance**: Use `check-compliance.md` skill
- **For delegation**: Use `delegate-to-agents.md` skill
