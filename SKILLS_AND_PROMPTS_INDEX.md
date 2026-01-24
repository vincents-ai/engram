# Engram Skills and Prompts Index

This index provides quick access to all skills, prompts, and workflows available in engram, categorized by user goal.

## Quick Start

1. **New to engram?** Start with [Using Engram](docs/engram/skills/using-engram.md)
2. **Creating prompts?** See the [Prompt Engineering Guide](docs/engram/skills/prompt_guide.md)
3. **Need a skill?** Browse the categories below

## 1. Core Capabilities
Essential skills for memory, task management, and agent orchestration.

### Skills
Location: `./skills/meta/`

| Skill | File | Purpose |
|-------|------|---------|
| Use Engram Memory | `meta/use-engram-memory.md` | Core skill for using engram as persistent memory |
| Delegate to Agents | `meta/delegate-to-agents.md` | Delegation using engram-adapted agents |
| Audit Trail | `meta/audit-trail.md` | Complete audit documentation |
| Dispatch Parallel | `meta/dispatching-parallel-agents.md` | Patterns for parallel agent execution |

### Key Prompts
Location: `./prompts/agents/`

| Agent | File | Purpose |
|-------|------|---------|
| The One | `01-the-one.yaml` | Primary orchestrator - creates tasks and delegates |
| The Sidekick | `02-the-sidekick.yaml` | General assistance and quick tasks |
| The Deconstructor | `05-the-deconstructor.yaml` | Breaks complex tasks into atomic units |
| The Task Manager | `166-the-task-manager.yaml` | Manages task lifecycle and state |

## 2. Planning & Architecture
Resources for system design, requirements gathering, and technical planning.

### Skills
Location: `./skills/architecture/` & `./skills/planning/`

| Category | Skill | File | Purpose |
|----------|-------|------|---------|
| **Architecture** | System Design | `architecture/system-design.md` | High-level system architecture patterns |
| | API Design | `architecture/api-design.md` | REST/GraphQL/gRPC interface design |
| | Data Modeling | `architecture/data-modeling.md` | Database schema and entity design |
| **Planning** | Roadmap | `planning/roadmap-planning.md` | Strategic timeline planning |
| | Risk Assessment | `planning/risk-assessment.md` | Identifying and mitigating technical risks |
| | Brainstorming | `workflow/brainstorming.md` | Ideation and option generation |

### Key Prompts
Location: `./prompts/agents/`

| Agent | File | Purpose |
|-------|------|---------|
| The Architect | `03-the-architect.yaml` | System design and technical decision making |
| API Designer | `17-the-api-designer.yaml` | Interface specification and contract definition |
| The Visualiser | `29-the-visualisation-expert.yaml` | Creating diagrams and visual models |
| Project Kickoff | `../ai/pipelines/00-project-kickoff.yaml` | Initial project setup pipeline |

## 3. Development & Implementation
Resources for coding, debugging, infrastructure, and feature delivery.

### Skills
Location: `./skills/development/` & `./skills/debugging/`

| Skill | File | Purpose |
|-------|------|---------|
| Plan Feature | `workflow/plan-feature.md` | Step-by-step feature implementation guide |
| Systematic Debugging | `debugging/systematic-debugging.md` | Rigorous bug isolation and fixing |
| TDD | `development/test-driven-development.md` | Test-first development workflow |
| Technical Writing | `documentation/technical-writing.md` | Creating clear technical documentation |

### Development Agents
Location: `./prompts/agents/`

| Specialty | Agent | File |
|-----------|-------|------|
| **Rust** | The Rustacean | `70-the-rustacean.yaml` |
| **Go** | The Gopher | `71-the-gopher.yaml` |
| **JS/TS** | The Type Safe | `72-the-type-safe.yaml` |
| **DevOps** | DevOps Engineer | `45-the-devops-engineer.yaml` |
| **Shell** | The Shell | `77-the-shell.yaml` |
| **Database** | DB Specialist | `21-the-database-specialist.yaml` |

### Pipeline Templates
Location: `./prompts/ai/pipelines/`

| Pipeline | File | Use Case |
|----------|------|----------|
| Feature Launch | `01-greenfield-feature-launch.yaml` | End-to-end new feature delivery |
| Bug Triage | `03-bug-hunt-triage.yaml` | Systematic bug fixing |
| API Modernisation | `04-api-modernisation.yaml` | Updating legacy APIs |
| Infrastructure | `11-nixos-immutable-deploy.yaml` | Immutable infrastructure deployment |

## 4. Quality & Compliance
Resources for testing, auditing, security, and regulatory compliance.

### Skills
Location: `./skills/compliance/` & `./skills/quality/`

| Skill | File | Purpose |
|-------|------|---------|
| Check Compliance | `compliance/check-compliance.md` | Validating against regulatory frameworks |
| Code Quality | `review/code-quality.md` | Code review standards and patterns |
| Security Review | `review/security-review.md` | Security auditing checklist |
| Accessibility | `quality/accessibility.md` | WCAG and inclusive design checks |

### Quality Agents
Location: `./prompts/agents/`

| Agent | File | Purpose |
|-------|------|---------|
| The Tester | `15-the-tester.yaml` | Writing and running tests |
| QA Strategist | `48-the-qa-strategist.yaml` | Test planning and coverage analysis |
| Ethics Auditor | `14-the-ethics-auditor.yaml` | AI ethics and bias checking |
| Critical Roller | `20-the-critical-roller.yaml` | Critical system verification |

### Compliance Frameworks
Location: `./prompts/compliance_and_certification/prompts/audit_checkpoints/`

| Framework | Directory | Coverage |
|-----------|-----------|----------|
| **iGaming** | `igaming/` | GLI-11, GLI-19, GLI-33, MGA, UKGC, G4 |
| **SaaS/IT** | `saas_it/` | SOC2, ISO27001, PCI DSS |
| **Data Protection** | `data_protection/` | GDPR, CCPA, PIPEDA |
| **EU Regulations** | `eu_regulations/` | DSA, DMA, AI Act, NIS2, DORA |
| **Cybersecurity** | `cybersecurity_policies/` | NIST CSF, RMF, ISO 27002 |
| **Medical** | `medical_device/` | IEC 62304 |

## Reference: File Structure

```
./
├── skills/                    # Skills documentation
│   ├── meta/                  # Core capabilities (memory, delegation)
│   ├── architecture/          # System design skills
│   ├── planning/              # Project planning skills
│   ├── development/           # Coding & implementation skills
│   ├── quality/               # QA & accessibility skills
│   ├── compliance/            # Regulatory skills
│   └── workflow/              # Process guides
│
├── prompts/                   # Prompt templates
│   ├── agents/                # 170+ agent prompts
│   │   ├── 01-the-one.yaml
│   │   └── ...
│   │
│   ├── ai/
│   │   └── pipelines/         # 100+ workflow pipelines
│   │
│   └── compliance_and_certification/
│       └── prompts/
│           └── audit_checkpoints/ # 250+ compliance checks
│
└── docs/
    └── engram/
        └── skills/
            ├── using-engram.md
            └── prompt_guide.md
```

## Getting Help

- **For workflows**: See [Using Engram](docs/engram/skills/using-engram.md)
- **For prompts**: See [Prompt Engineering Guide](docs/engram/skills/prompt_guide.md)
- **For compliance**: Use `check-compliance.md` skill
- **For delegation**: Use `delegate-to-agents.md` skill
