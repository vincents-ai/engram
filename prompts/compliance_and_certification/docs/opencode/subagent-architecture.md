# Subagent Architecture Guide

> **Note:** This documentation describes multi-agent AI tool configuration patterns. Examples were originally written for OpenCode but the concepts apply to any AI coding tool.

This document explains how to design, implement, and manage specialized AI subagents using your AI coding tool's configuration system.

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

**Responsibilities:**
- React component development
- TypeScript implementation
- State management (Zustand, React Query)
- UI/UX implementation
- Frontend testing

#### Backend Developer

**Responsibilities:**
- API design and implementation
- Database schema and operations
- Authentication and authorization
- Performance optimization
- Backend testing

### Domain Experts

#### Game Designer

**Responsibilities:**
- Game mechanics design
- Player experience optimization
- Balance adjustments
- Content creation
- Progression systems

#### Domain Specialist (e.g. Cannabis Specialist)

**Responsibilities:**
- Domain-specific mechanics and accuracy
- Industry content and terminology
- Compliance and regulatory research
- Narrative development

### Infrastructure Specialists

#### Systems Architect (e.g. NixOS Systems Architect)

**Responsibilities:**
- Development environment setup
- Reproducible builds
- System configuration
- Package management
- CI/CD pipelines

#### World-Building Researcher

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
```
User: "Add a dark mode toggle to the settings page"
Primary Agent: 
  1. Analyzes task (UI + state management)
  2. Calls frontend-engineer for component implementation
  3. Calls backend-developer for preferences API
  4. Integrates all changes
  5. Runs tests and validation
```

### Domain-Specific Research
```
User: "Research cannabis cultivation laws for Oregon"
Primary Agent:
  1. Identifies research requirement
  2. Calls world-building-researcher with specific query
  3. Researcher uses webfetch for current legal data
  4. Calls domain-specialist for cultivation specifics
  5. Integrates research into game mechanics
```

### System Configuration
```
User: "Set up development environment with NixOS"
Primary Agent:
  1. Identifies infrastructure requirement
  2. Calls systems-architect
  3. Architect creates flake.nix and dev shell
  4. Tests build and dependency resolution
  5. Documents setup process
```

## Implementation Steps

### 1. Define Agent Roles
Identify the key domains in your project:
```
Project Analysis:
- Frontend: React, TypeScript, UI components
- Backend: Node.js, APIs, databases  
- DevOps: Docker, CI/CD, deployment
- Domain: Game mechanics, specialised knowledge
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
├── domain-specialist.md
├── systems-architect.md
└── world-building-researcher.md
```

### 3. Configure Tools and Permissions
Match tools to agent responsibilities. Follow the principle of least privilege — grant each agent only the tools it genuinely needs. Development agents typically need read/write/edit/bash/grep/glob/list, while research agents swap bash for webfetch, and review agents are read-only.

This architecture enables efficient, specialized AI assistance while maintaining clear boundaries and security controls.
