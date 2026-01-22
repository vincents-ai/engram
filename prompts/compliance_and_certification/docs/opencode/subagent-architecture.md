# Subagent Architecture Guide

This document explains how to design, implement, and manage specialized AI subagents using OpenCode's configuration system.

## Overview

Subagents are specialized AI assistants that focus on specific domains (frontend, backend, DevOps, etc.). They provide expert knowledge and capabilities while working within controlled boundaries.

## Architecture Principles

### 1. Single Responsibility Principle
Each agent should have one clear purpose:
- **Frontend Engineer**: React, TypeScript, UI components
- **Backend Developer**: APIs, databases, server architecture  
- **DevOps Engineer**: Deployment, CI/CD, infrastructure
- **Game Designer**: Game mechanics, balance, player experience

### 2. Controlled Access
Agents only have access to tools they need:
- **File Tools**: `read`, `write`, `edit` for code changes
- **Search Tools**: `grep`, `glob`, `list` for code exploration
- **System Tools**: `bash` for builds, tests, deployment
- **External Tools**: `webfetch` for research (limited)

### 3. Context Awareness
Each agent understands:
- Project structure and conventions
- Domain-specific best practices
- Team workflow requirements
- Security constraints

## Subagent Types by Role

### Development Specialists

#### Frontend Engineer
```json
{
  "frontend-typescript-engineer": {
    "mode": "subagent",
    "model": "github-copilot/claude-3.5-sonnet",
    "prompt": "{file:./prompts/frontend-typescript-engineer.md}",
    "tools": {
      "read": true,
      "write": true,
      "edit": true,
      "bash": true,
      "grep": true,
      "glob": true,
      "list": true
    }
  }
}
```

**Responsibilities:**
- React component development
- TypeScript implementation
- State management (Zustand, React Query)
- UI/UX implementation
- Frontend testing

#### Backend Developer
```json
{
  "backend-developer": {
    "mode": "subagent",
    "model": "github-copilot/claude-3.5-sonnet", 
    "prompt": "{file:./prompts/backend-developer.md}",
    "tools": {
      "read": true,
      "write": true,
      "edit": true,
      "bash": true,
      "grep": true,
      "glob": true,
      "list": true
    }
  }
}
```

**Responsibilities:**
- API design and implementation
- Database schema and operations
- Authentication and authorization
- Performance optimization
- Backend testing

### Domain Experts

#### Game Designer
```json
{
  "game-designer": {
    "mode": "subagent",
    "model": "github-copilot/claude-3.5-sonnet",
    "prompt": "{file:./prompts/game-designer.md}",
    "tools": {
      "read": true,
      "write": true,
      "edit": true,
      "bash": true,
      "grep": true,
      "glob": true,
      "list": true
    }
  }
}
```

**Responsibilities:**
- Game mechanics design
- Player experience optimization
- Balance adjustments
- Content creation
- Progression systems

#### Cannabis Specialist
```json
{
  "cannabis-specialist": {
    "mode": "subagent",
    "model": "github-copilot/claude-3.5-sonnet",
    "prompt": "{file:./prompts/cannabis-specialist.md}",
    "tools": {
      "read": true,
      "write": true,
      "edit": true,
      "bash": true,
      "grep": true,
      "glob": true,
      "list": true,
      "webfetch": true
    }
  }
}
```

**Responsibilities:**
- Cannabis cultivation mechanics
- Strain genetics and breeding
- Industry-accurate content
- Legal compliance research
- Narrative development

### Infrastructure Specialists

#### NixOS Systems Architect
```json
{
  "nixos-systems-architect": {
    "mode": "subagent",
    "model": "github-copilot/claude-3.5-sonnet",
    "prompt": "{file:./prompts/nixos-systems-architect.md}",
    "tools": {
      "read": true,
      "write": true,
      "edit": true,
      "bash": true,
      "grep": true,
      "glob": true,
      "list": true
    }
  }
}
```

**Responsibilities:**
- Development environment setup
- Reproducible builds
- System configuration
- Package management
- CI/CD pipelines

#### World-Building Researcher
```json
{
  "world-building-researcher": {
    "mode": "subagent",
    "model": "github-copilot/claude-3.5-sonnet",
    "prompt": "{file:./prompts/world-building-researcher.md}",
    "tools": {
      "read": true,
      "write": true,
      "edit": true,
      "bash": true,
      "grep": true,
      "glob": true,
      "list": true,
      "webfetch": true
    }
  }
}
```

**Responsibilities:**
- Real-world data research
- Legal framework analysis
- Geographic information
- Market research
- Content authenticity

## Prompt Structure

### Standard Prompt Template

```markdown
# Agent Name

## Role
Brief description of the agent's primary function and expertise area.

## Primary Prompt
Detailed instructions for the agent's core responsibilities:
- Specific tasks they should handle
- Domain expertise they should apply
- Output format and quality standards
- Integration with project workflow

## Additional Capabilities
Secondary functions and extended use cases:
- Related tasks they can assist with
- Cross-functional collaboration
- Research and analysis capabilities

## Important: Project Guidelines
**MANDATORY**: Before starting any work, read the `AGENTS.md` file in the project root. It contains essential build commands, code style guidelines, git workflow requirements, and architectural conventions that must be followed for all work in this codebase.

## Session Timing
**REQUIRED**: For performance tracking:
1. Run `date` command at the very start of your session and note the timestamp
2. Run `date` command at the very end before responding with final outcome
3. Calculate and report the total duration in your final response
```

### Example: Frontend Engineer Prompt

```markdown
# Frontend TypeScript Engineer AI

## Role
A specialized AI focused on modern frontend development using React, TypeScript, and contemporary web technologies.

## Primary Prompt
You are a senior frontend engineer AI specializing in React and TypeScript development. Your primary responsibilities include:

- **Component Development**: Create reusable, well-typed React components following modern patterns
- **State Management**: Implement and maintain state using Zustand and React Query
- **TypeScript Implementation**: Write type-safe code with proper interfaces and type definitions
- **Performance Optimization**: Implement best practices for React performance
- **Testing**: Write comprehensive unit and integration tests

## Additional Capabilities
- UI/UX implementation and improvement suggestions
- Accessibility compliance and WCAG guidelines
- Frontend build optimization and bundling
- Integration with backend APIs
- Code review and refactoring recommendations

## Important: Project Guidelines
**MANDATORY**: Before starting any work, read the `AGENTS.md` file in the project root.

## Session Timing
**REQUIRED**: Track session duration for performance analysis.
```

## Tool Configuration Strategies

### Minimal Access Pattern
Grant only essential tools:
```json
{
  "tools": {
    "read": true,
    "edit": true,
    "grep": true
  }
}
```

### Development Pattern
Standard development capabilities:
```json
{
  "tools": {
    "read": true,
    "write": true,
    "edit": true,
    "bash": true,
    "grep": true,
    "glob": true,
    "list": true
  }
}
```

### Research Pattern
Include external access for research:
```json
{
  "tools": {
    "read": true,
    "write": true,
    "edit": true,
    "bash": true,
    "grep": true,
    "glob": true,
    "list": true,
    "webfetch": true
  }
}
```

## Agent Coordination

### Primary Agent Workflow
1. **User Request**: User provides task requirements
2. **Task Analysis**: Primary agent analyzes complexity and domain
3. **Agent Selection**: Choose appropriate specialist agent
4. **Task Delegation**: Call specialist via Task tool
5. **Integration**: Primary agent integrates specialist work
6. **Quality Check**: Ensure consistency and completeness

### Specialist Agent Workflow
1. **Context Loading**: Read project guidelines and current state
2. **Domain Analysis**: Apply specialist knowledge to task
3. **Implementation**: Execute within granted tool permissions
4. **Documentation**: Provide clear explanation of changes
5. **Handoff**: Return results to primary agent

### Cross-Agent Communication
Agents communicate through:
- **Shared Files**: `AGENTS.md`, `README.md`, documentation
- **Code Comments**: Domain-specific notes and explanations
- **Git Commits**: Detailed commit messages with agent attribution
- **Task Results**: Structured output from specialist agents

## Implementation Steps

### 1. Define Agent Roles
Identify the key domains in your project:
```
Project Analysis:
- Frontend: React, TypeScript, UI components
- Backend: Node.js, APIs, databases  
- DevOps: Docker, CI/CD, deployment
- Domain: Game mechanics, cannabis cultivation
- Research: Legal compliance, market data
```

### 2. Create Prompt Files
Structure your prompts directory:
```
prompts/
├── README.md                      # Agent overview
├── frontend-typescript-engineer.md
├── backend-developer.md
├── game-designer.md
├── cannabis-specialist.md
├── nixos-systems-architect.md
└── world-building-researcher.md
```

### 3. Configure Tools and Permissions
Match tools to agent responsibilities:
```json
{
  "agent": {
    "frontend-engineer": {
      "tools": {
        "read": true,
        "write": true,
        "edit": true,
        "bash": true,
        "grep": true,
        "glob": true,
        "list": true
      }
    },
    "researcher": {
      "tools": {
        "read": true,
        "write": true,
        "edit": true,
        "webfetch": true,
        "grep": true,
        "glob": true,
        "list": true
      }
    }
  }
}
```

### 4. Test Agent Interactions
Validate your configuration:
```bash
# Test individual agents
opencode test-agent frontend-engineer
opencode test-agent backend-developer

# Test cross-agent workflows
opencode validate-workflow
```

## Best Practices

### Agent Design
1. **Clear Boundaries**: Each agent has distinct responsibilities
2. **Consistent Prompts**: Use standardized prompt structure
3. **Context Awareness**: Agents understand project conventions
4. **Tool Minimalism**: Grant only necessary permissions

### Workflow Integration
1. **Project Guidelines**: Maintain `AGENTS.md` with current requirements
2. **Naming Conventions**: Use descriptive, hyphenated agent names
3. **Documentation**: Keep agent purposes and capabilities documented
4. **Version Control**: Track agent configuration changes

### Security and Permissions
1. **Least Privilege**: Minimum necessary tool access
2. **Sensitive Operations**: Use "ask" permission for destructive commands
3. **External Access**: Carefully control webfetch permissions
4. **Command Restrictions**: Limit dangerous bash operations

### Performance Optimization
1. **Model Selection**: Use appropriate models for complexity
2. **Prompt Efficiency**: Keep prompts focused and actionable
3. **Tool Optimization**: Enable only required tools
4. **Session Tracking**: Monitor agent performance and timing

## Common Patterns

### Multi-Agent Collaboration
```bash
User: "Add a dark mode toggle to the settings page"
Primary Agent: 
  1. Analyzes task (UI + state management)
  2. Calls frontend-engineer for component implementation
  3. Calls backend-developer for preferences API
  4. Calls game-designer for UX integration
  5. Integrates all changes
  6. Runs tests and validation
```

### Domain-Specific Research
```bash
User: "Research cannabis cultivation laws for Oregon"
Primary Agent:
  1. Identifies research requirement
  2. Calls world-building-researcher with specific query
  3. Researcher uses webfetch for current legal data
  4. Calls cannabis-specialist for cultivation specifics
  5. Integrates research into game mechanics
```

### System Configuration
```bash
User: "Set up development environment with NixOS"
Primary Agent:
  1. Identifies infrastructure requirement
  2. Calls nixos-systems-architect
  3. Architect creates flake.nix and dev shell
  4. Tests build and dependency resolution
  5. Documents setup process
```

## Troubleshooting

### Common Issues
1. **Agent Not Responding**: Check agent name spelling and configuration
2. **Tool Access Denied**: Review tool permissions in opencode.json
3. **Prompt Errors**: Validate prompt file paths and syntax
4. **Cross-Agent Confusion**: Ensure clear agent boundaries

### Debugging Steps
1. **Validate Configuration**: `opencode validate`
2. **Test Individual Agents**: `opencode test-agent <name>`
3. **Check Tool Permissions**: Review agent tool access
4. **Examine Prompt Files**: Ensure prompts are clear and actionable

### Performance Issues
1. **Model Selection**: Use appropriate models for task complexity
2. **Tool Optimization**: Disable unnecessary tools
3. **Prompt Optimization**: Streamline prompt instructions
4. **Session Monitoring**: Track agent response times

This architecture enables efficient, specialized AI assistance while maintaining clear boundaries and security controls.