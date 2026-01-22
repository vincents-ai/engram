# OpenCode Setup Guide

This guide provides step-by-step instructions for implementing OpenCode with specialized AI subagents in any repository.

## Prerequisites

- OpenCode installed (`npm install -g opencode`)
- Git repository
- Basic understanding of AI agent concepts

## Quick Setup (5 Minutes)

### 1. Initialize Configuration
```bash
# In your project root
touch opencode.json
mkdir prompts
```

### 2. Basic Configuration
Create `opencode.json`:
```json
{
  "$schema": "https://opencode.ai/config.json",
  "model": "github-copilot/claude-3.5-sonnet",
  "agent": {
    "frontend-engineer": {
      "mode": "subagent",
      "prompt": "{file:./prompts/frontend-engineer.md}",
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
}
```

### 3. Create Your First Agent
Create `prompts/frontend-engineer.md`:
```markdown
# Frontend Engineer AI

## Role
A specialized AI focused on frontend development.

## Primary Prompt
You are a senior frontend engineer. Focus on:
- Modern React and TypeScript development
- Component architecture and reusability
- Performance optimization
- Accessibility compliance

## Important: Project Guidelines
Always read the AGENTS.md file before starting work.
```

### 4. Test Configuration
```bash
opencode validate
opencode test-agent frontend-engineer
```

## Detailed Implementation

### Step 1: Project Analysis

Identify the key domains in your project:

```bash
# Analyze your codebase structure
find . -type f -name "*.js" -o -name "*.ts" -o -name "*.py" -o -name "*.go" | head -20

# Identify primary technologies
ls package.json requirements.txt go.mod pom.xml 2>/dev/null

# Check build and deployment systems
ls Dockerfile docker-compose.yml .github/workflows/ .gitlab-ci.yml 2>/dev/null
```

### Step 2: Define Agent Roles

Based on analysis, create specialized agents:

#### Web Development Project
```json
{
  "$schema": "https://opencode.ai/config.json",
  "agent": {
    "frontend-engineer": {
      "mode": "subagent",
      "prompt": "{file:./prompts/frontend-engineer.md}",
      "description": "React, TypeScript, modern web development"
    },
    "backend-developer": {
      "mode": "subagent", 
      "prompt": "{file:./prompts/backend-developer.md}",
      "description": "APIs, databases, server architecture"
    },
    "devops-engineer": {
      "mode": "subagent",
      "prompt": "{file:./prompts/devops-engineer.md}",
      "description": "CI/CD, deployment, infrastructure"
    }
  }
}
```

#### Data Science Project
```json
{
  "agent": {
    "data-scientist": {
      "mode": "subagent",
      "prompt": "{file:./prompts/data-scientist.md}",
      "description": "ML models, data analysis, experimentation"
    },
    "ml-engineer": {
      "mode": "subagent",
      "prompt": "{file:./prompts/ml-engineer.md}",
      "description": "Model deployment, MLOps, infrastructure"
    },
    "research-analyst": {
      "mode": "subagent",
      "prompt": "{file:./prompts/research-analyst.md}",
      "description": "Domain research, literature review, insights"
    }
  }
}
```

### Step 3: Create Agent Prompts

#### Template Structure
```markdown
# Agent Name

## Role
Clear description of the agent's primary function.

## Primary Prompt
Detailed instructions including:
- Core responsibilities
- Technical expertise
- Quality standards
- Integration requirements

## Additional Capabilities
- Secondary functions
- Cross-domain knowledge
- Research abilities

## Important: Project Guidelines
**MANDATORY**: Before starting any work, read the `AGENTS.md` file in the project root. It contains essential build commands, code style guidelines, git workflow requirements, and architectural conventions that must be followed for all work in this codebase.

## Session Timing
**REQUIRED**: For performance tracking:
1. Run `date` command at the very start of your session and note the timestamp
2. Run `date` command at the very end before responding with final outcome  
3. Calculate and report the total duration in your final response
```

#### Frontend Engineer Example
```markdown
# Frontend Engineer AI

## Role
A specialized AI focused on modern frontend development using React, TypeScript, and contemporary web technologies.

## Primary Prompt
You are a senior frontend engineer AI specializing in React and TypeScript development. Your primary responsibilities include:

- **Component Development**: Create reusable, well-typed React components following modern patterns
- **State Management**: Implement and maintain state using appropriate libraries (Redux, Zustand, Context)
- **TypeScript Implementation**: Write type-safe code with proper interfaces and type definitions
- **Performance Optimization**: Implement best practices for React performance (memoization, lazy loading, etc.)
- **Testing**: Write comprehensive unit and integration tests using modern testing frameworks
- **Build Optimization**: Configure and optimize webpack, Vite, or similar build tools
- **Accessibility**: Ensure WCAG compliance and semantic HTML

When working on tasks:
1. Always check existing code patterns and follow project conventions
2. Use TypeScript strictly - avoid `any` types
3. Implement proper error boundaries and error handling
4. Follow the project's component architecture patterns
5. Write tests for new functionality
6. Consider performance implications of your implementations

## Additional Capabilities
- UI/UX implementation and improvement suggestions
- CSS-in-JS and styling system optimization
- Progressive Web App (PWA) implementation
- Frontend build pipeline optimization
- Code review and refactoring recommendations
- Integration with backend APIs and real-time features

## Important: Project Guidelines
**MANDATORY**: Before starting any work, read the `AGENTS.md` file in the project root. It contains essential build commands, code style guidelines, git workflow requirements, and architectural conventions that must be followed for all work in this codebase.

## Session Timing
**REQUIRED**: For performance tracking:
1. Run `date` command at the very start of your session and note the timestamp
2. Run `date` command at the very end before responding with final outcome
3. Calculate and report the total duration in your final response
```

### Step 4: Configure Tools and Permissions

#### Basic Development Setup
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

#### Research-Enabled Setup
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

#### Security-Conscious Setup
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
  },
  "permission": {
    "edit": "ask",
    "bash": {
      "rm": "ask",
      "sudo": "deny",
      "git": "allow",
      "npm": "allow",
      "docker": "ask"
    }
  }
}
```

### Step 5: Project Integration

#### Create AGENTS.md
```markdown
# AGENTS.md - Codebase Guide for AI Agents

## Build Commands
- **Build**: `npm run build` 
- **Dev**: `npm run dev`
- **Test**: `npm test`
- **Lint**: `npm run lint`
- **Format**: `npm run format`

## Code Style
- **Language**: TypeScript strict mode
- **Formatting**: Prettier with 2 spaces, single quotes
- **Imports**: Use absolute imports with path mapping
- **Components**: Function components with TypeScript

## Architecture
- Frontend: React + TypeScript + Vite
- State: Zustand for global state, React Query for server state
- Styling: CSS Modules with SCSS
- Testing: Vitest + React Testing Library

## Git Workflow
- **Commits**: Conventional commits with detailed descriptions
- **Branches**: feature/task-description format
- **Reviews**: All changes require review before merge
```

#### Update package.json Scripts
```json
{
  "scripts": {
    "build": "vite build",
    "dev": "vite",
    "test": "vitest",
    "test:ui": "vitest --ui",
    "lint": "eslint . --ext ts,tsx --report-unused-disable-directives --max-warnings 0",
    "lint:fix": "eslint . --ext ts,tsx --fix",
    "format": "prettier --write \"src/**/*.{ts,tsx,js,jsx,json,css,md}\"",
    "format:check": "prettier --check \"src/**/*.{ts,tsx,js,jsx,json,css,md}\"",
    "typecheck": "tsc --noEmit"
  }
}
```

### Step 6: Validation and Testing

#### Validate Configuration
```bash
# Check syntax and schema
opencode validate

# Test specific agents
opencode test-agent frontend-engineer
opencode test-agent backend-developer

# Check agent accessibility
opencode list-agents
```

#### Test Agent Workflows
```bash
# Test basic functionality
opencode --agent frontend-engineer "Create a simple React component"

# Test tool permissions
opencode --agent devops-engineer "Run the build process"

# Test research capabilities
opencode --agent research-analyst "Find documentation for React 18 features"
```

## Advanced Configuration

### Multi-Model Setup
```json
{
  "model": "github-copilot/claude-3.5-sonnet",
  "small_model": "anthropic/claude-3-haiku",
  "agent": {
    "frontend-engineer": {
      "model": "github-copilot/claude-3.5-sonnet",
      "temperature": 0.1
    },
    "creative-writer": {
      "model": "anthropic/claude-3-opus",
      "temperature": 0.7
    },
    "code-reviewer": {
      "model": "anthropic/claude-3-haiku",
      "temperature": 0.0
    }
  }
}
```

### Environment-Specific Configuration
```json
{
  "agent": {
    "production-deployer": {
      "mode": "subagent",
      "prompt": "{file:./prompts/production-deployer.md}",
      "tools": {
        "read": true,
        "bash": true
      },
      "permission": {
        "bash": {
          "kubectl": "ask",
          "docker": "allow",
          "rm": "deny"
        }
      }
    }
  }
}
```

### Integration with Existing Tools

#### GitHub Actions Integration
```yaml
# .github/workflows/ai-review.yml
name: AI Code Review
on: [pull_request]
jobs:
  ai-review:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run AI Review
        run: |
          opencode --agent code-reviewer "Review this PR for issues"
```

#### Pre-commit Hooks
```bash
# .git/hooks/pre-commit
#!/bin/bash
opencode --agent code-reviewer "Check code quality before commit"
```

## Troubleshooting

### Common Issues

#### Agent Not Found
```bash
# Check agent names in configuration
cat opencode.json | jq '.agent | keys'

# Validate configuration
opencode validate
```

#### Permission Denied
```bash
# Check tool permissions
cat opencode.json | jq '.agent.["agent-name"].tools'

# Review permission settings
cat opencode.json | jq '.agent.["agent-name"].permission'
```

#### Prompt File Errors
```bash
# Verify prompt file exists
ls -la prompts/

# Check file path in configuration
cat opencode.json | jq '.agent.["agent-name"].prompt'
```

### Performance Optimization

#### Model Selection
- Use `claude-3-haiku` for simple tasks
- Use `claude-3.5-sonnet` for complex development
- Use `claude-3-opus` for creative tasks

#### Tool Optimization
- Only enable necessary tools
- Use specific bash permissions
- Limit webfetch for security

#### Prompt Optimization
- Keep prompts focused and specific
- Include relevant context and examples
- Reference project guidelines clearly

## Migration from Other Systems

### From Manual AI Usage
1. Extract common prompts into agent files
2. Define tool boundaries for each use case
3. Create agent-specific configurations
4. Test workflows with new agents

### From Other AI Tools
1. Map existing workflows to agent responsibilities
2. Convert prompts to OpenCode format
3. Configure tool permissions appropriately
4. Validate functionality with test scenarios

## Best Practices Summary

### Configuration
- Use schema validation
- Follow naming conventions
- Document agent purposes
- Version control all changes

### Security
- Apply principle of least privilege
- Use "ask" permission for sensitive operations
- Regularly review and audit permissions
- Avoid hardcoding secrets

### Maintenance
- Keep prompts updated with project evolution
- Monitor agent performance and accuracy
- Collect feedback and iterate on configurations
- Document changes and decisions

### Team Adoption
- Provide training on agent capabilities
- Create usage guidelines and examples
- Share successful patterns and configurations
- Establish review processes for agent changes

This setup guide provides the foundation for implementing OpenCode agents in any project. Adapt the examples and configurations to match your specific technology stack and team requirements.