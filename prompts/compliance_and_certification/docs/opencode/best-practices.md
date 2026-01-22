# OpenCode Best Practices Guide

This guide provides proven patterns, recommendations, and strategies for implementing effective OpenCode agent configurations.

## Core Principles

### 1. Single Responsibility Principle
Each agent should have one clear, focused purpose:

✅ **Good**: `frontend-engineer` handles React components, TypeScript, and UI logic
❌ **Bad**: `full-stack-developer` handles everything from database to styling

### 2. Minimal Tool Access
Grant only the tools each agent absolutely needs:

✅ **Good**: Research agents get `webfetch`, development agents get `bash`
❌ **Bad**: All agents get all tools regardless of purpose

### 3. Context Awareness
Agents should understand project-specific requirements:

✅ **Good**: Include `AGENTS.md` reference in all prompts
❌ **Bad**: Generic prompts without project context

## Agent Design Patterns

### Development Team Pattern
Structure agents like a software development team:

```json
{
  "agent": {
    "senior-engineer": {
      "mode": "subagent",
      "description": "Code review, architecture decisions, mentoring"
    },
    "frontend-engineer": {
      "mode": "subagent", 
      "description": "React, TypeScript, UI components"
    },
    "backend-engineer": {
      "mode": "subagent",
      "description": "APIs, databases, server logic"
    },
    "devops-engineer": {
      "mode": "subagent",
      "description": "CI/CD, deployment, infrastructure"
    },
    "qa-engineer": {
      "mode": "subagent",
      "description": "Testing, quality assurance, bug detection"
    }
  }
}
```

### Domain Expertise Pattern
Organize agents around business domains:

```json
{
  "agent": {
    "game-designer": {
      "mode": "subagent",
      "description": "Game mechanics, balance, player experience"
    },
    "cannabis-specialist": {
      "mode": "subagent", 
      "description": "Cannabis cultivation, genetics, industry knowledge"
    },
    "legal-researcher": {
      "mode": "subagent",
      "description": "Cannabis laws, compliance, regulations"
    },
    "market-analyst": {
      "mode": "subagent",
      "description": "Industry trends, pricing, competition"
    }
  }
}
```

### Workflow Stage Pattern
Structure agents around development stages:

```json
{
  "agent": {
    "requirements-analyst": {
      "mode": "subagent",
      "description": "Requirements gathering, user stories, acceptance criteria"
    },
    "architect": {
      "mode": "subagent",
      "description": "System design, technology selection, patterns"
    },
    "implementer": {
      "mode": "subagent",
      "description": "Code implementation, feature development"
    },
    "tester": {
      "mode": "subagent",
      "description": "Test implementation, quality verification"
    },
    "deployer": {
      "mode": "subagent",
      "description": "Deployment, monitoring, production support"
    }
  }
}
```

## Prompt Engineering Best Practices

### Structure Template
Use this proven prompt structure:

```markdown
# Agent Name

## Role
[1-2 sentences describing core function]

## Primary Prompt
[Detailed instructions covering:]
- Core responsibilities (3-5 bullet points)
- Quality standards and expectations
- Integration requirements
- Output format specifications

## Additional Capabilities
[Secondary functions and cross-domain knowledge]

## Important: Project Guidelines
**MANDATORY**: Before starting any work, read the `AGENTS.md` file in the project root.

## Session Timing
**REQUIRED**: Track session duration for performance analysis.
```

### Effective Prompt Content

#### ✅ Clear Instructions
```markdown
You are a React component specialist. When creating components:
- Use TypeScript with strict typing
- Follow functional component patterns
- Implement proper error boundaries
- Write comprehensive PropTypes or TypeScript interfaces
- Include accessibility attributes (ARIA labels, roles)
```

#### ❌ Vague Instructions
```markdown
You are a frontend developer. Make good React components.
```

#### ✅ Specific Context
```markdown
This project uses:
- React 18 with concurrent features
- Zustand for state management
- React Query for server state
- Vitest + React Testing Library for testing
- CSS Modules with SCSS
```

#### ❌ Generic Context
```markdown
This is a web application project.
```

### Domain-Specific Examples

#### Frontend Engineer
```markdown
## Primary Prompt
You are a senior React engineer specializing in modern frontend development. Your responsibilities:

- **Component Architecture**: Create reusable, composable components using React 18 patterns
- **TypeScript Excellence**: Write type-safe code with proper interfaces, no `any` types
- **Performance Optimization**: Implement memoization, lazy loading, and bundle optimization
- **Testing Strategy**: Write unit tests with Vitest and integration tests with React Testing Library
- **Accessibility**: Ensure WCAG 2.1 AA compliance with semantic HTML and ARIA attributes

When implementing features:
1. Check existing component patterns in `src/components/`
2. Follow the design system in `src/design-system/`
3. Use absolute imports with the `@/` path alias
4. Implement proper loading and error states
5. Write tests for all new components
```

#### Backend Developer
```markdown
## Primary Prompt
You are a senior backend engineer specializing in Node.js and scalable API development. Your responsibilities:

- **API Design**: Create RESTful APIs following OpenAPI 3.0 specifications
- **Database Operations**: Design efficient schemas and write optimized queries
- **Authentication**: Implement secure JWT-based authentication with proper refresh token handling
- **Performance**: Optimize queries, implement caching, and handle high concurrency
- **Security**: Follow OWASP guidelines, implement rate limiting, and validate all inputs

When developing APIs:
1. Follow the existing patterns in `src/routes/`
2. Use the validation middleware for all endpoints
3. Implement proper error handling and logging
4. Write integration tests for all endpoints
5. Document APIs with OpenAPI specifications
```

## Tool Configuration Strategies

### Development Tools Matrix

| Agent Type | read | write | edit | bash | grep | glob | list | webfetch |
|------------|------|-------|------|------|------|------|------|----------|
| Frontend Engineer | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |
| Backend Developer | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |
| DevOps Engineer | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |
| Research Analyst | ✅ | ✅ | ✅ | ❌ | ✅ | ✅ | ✅ | ✅ |
| Code Reviewer | ✅ | ❌ | ✅ | ❌ | ✅ | ✅ | ✅ | ❌ |
| Documentation Writer | ✅ | ✅ | ✅ | ❌ | ✅ | ✅ | ✅ | ✅ |

### Permission Patterns

#### Development Agent (Standard)
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
    "bash": {
      "rm": "ask",
      "sudo": "deny",
      "git": "allow",
      "npm": "allow",
      "yarn": "allow",
      "pnpm": "allow"
    }
  }
}
```

#### Research Agent (Web Access)
```json
{
  "tools": {
    "read": true,
    "write": true,
    "edit": true,
    "grep": true,
    "glob": true,
    "list": true,
    "webfetch": true
  },
  "permission": {
    "webfetch": "allow"
  }
}
```

#### Review Agent (Read-Only)
```json
{
  "tools": {
    "read": true,
    "edit": true,
    "grep": true,
    "glob": true,
    "list": true
  },
  "permission": {
    "edit": "ask"
  }
}
```

#### Production Agent (Restricted)
```json
{
  "tools": {
    "read": true,
    "bash": true
  },
  "permission": {
    "bash": {
      "kubectl": "ask",
      "docker": "allow",
      "rm": "deny",
      "sudo": "deny"
    }
  }
}
```

## Security Best Practices

### 1. Principle of Least Privilege
Grant minimal necessary permissions:

```json
{
  "agent": {
    "documentation-writer": {
      "tools": {
        "read": true,
        "write": true,
        "edit": true,
        "webfetch": true
      },
      "permission": {
        "write": "ask",
        "edit": "ask"
      }
    }
  }
}
```

### 2. Dangerous Command Restrictions
Always restrict dangerous bash operations:

```json
{
  "permission": {
    "bash": {
      "rm": "ask",
      "sudo": "deny",
      "chmod": "ask",
      "chown": "deny",
      "dd": "deny",
      "mkfs": "deny"
    }
  }
}
```

### 3. Environment Separation
Use different configurations for different environments:

```json
// development.opencode.json
{
  "permission": {
    "bash": "allow"
  }
}

// production.opencode.json  
{
  "permission": {
    "bash": {
      "git": "allow",
      "docker": "ask",
      "kubectl": "ask"
    }
  }
}
```

## Performance Optimization

### Model Selection Strategy

#### Task Complexity → Model Mapping
- **Simple tasks** (formatting, basic edits): `claude-3-haiku`
- **Standard development** (features, components): `claude-3.5-sonnet`  
- **Complex architecture** (system design): `claude-3-opus`
- **Creative work** (content, design): `claude-3-opus`

```json
{
  "model": "github-copilot/claude-3.5-sonnet",
  "small_model": "anthropic/claude-3-haiku",
  "agent": {
    "formatter": {
      "model": "anthropic/claude-3-haiku",
      "temperature": 0.0
    },
    "architect": {
      "model": "anthropic/claude-3-opus",
      "temperature": 0.2
    },
    "creative-writer": {
      "model": "anthropic/claude-3-opus", 
      "temperature": 0.7
    }
  }
}
```

### Tool Optimization
Only enable tools that agents actually use:

```json
{
  "code-reviewer": {
    "tools": {
      "read": true,
      "grep": true,
      "glob": true,
      "list": true
    }
  },
  "documentation-writer": {
    "tools": {
      "read": true,
      "write": true,
      "webfetch": true
    }
  }
}
```

## Team Collaboration Patterns

### Agent Handoff Pattern
Design workflows where agents collaborate:

```
User Request → Primary Agent → Specialist Agents → Integration → Validation
```

Example workflow:
1. **Primary Agent**: Analyzes request complexity
2. **Frontend Agent**: Implements UI components  
3. **Backend Agent**: Creates API endpoints
4. **QA Agent**: Writes and runs tests
5. **Primary Agent**: Integrates and validates

### Review Chain Pattern
Implement multi-stage review processes:

```json
{
  "implementer": {
    "description": "Initial code implementation"
  },
  "senior-reviewer": {
    "description": "Architecture and best practices review"
  },
  "security-reviewer": {
    "description": "Security and vulnerability assessment"
  },
  "performance-reviewer": {
    "description": "Performance and optimization review"
  }
}
```

## Common Anti-Patterns

### ❌ Over-Broad Agents
```json
{
  "full-stack-developer": {
    "description": "Does everything from database to UI"
  }
}
```

### ❌ Tool Over-Granting
```json
{
  "documentation-writer": {
    "tools": {
      "bash": true,
      "webfetch": true
    }
  }
}
```

### ❌ Vague Prompts
```markdown
You are a helpful developer. Write good code.
```

### ❌ Missing Context
```markdown
# No reference to project guidelines or architecture
```

## Monitoring and Optimization

### Performance Metrics
Track these key metrics:

- **Response Time**: Average agent response time
- **Token Usage**: Cost per agent interaction
- **Success Rate**: Percentage of successful completions
- **Error Rate**: Failed or incomplete responses

### Configuration Validation
Regularly validate your configuration:

```bash
# Syntax validation
opencode validate

# Agent accessibility test
opencode list-agents

# Tool permission audit
opencode audit-permissions

# Performance benchmarking
opencode benchmark-agents
```

### Iterative Improvement
1. **Monitor Usage Patterns**: Which agents are used most?
2. **Collect Feedback**: What works well/poorly?
3. **Analyze Performance**: Which agents are slow/fast?
4. **Refine Configurations**: Optimize based on data
5. **Update Prompts**: Improve clarity and effectiveness

## Migration Strategies

### From Manual AI Usage
1. **Audit Current Patterns**: What prompts do you use regularly?
2. **Categorize Use Cases**: Group similar tasks together  
3. **Create Specialist Agents**: One agent per major use case
4. **Test Individually**: Validate each agent works well
5. **Integrate Gradually**: Replace manual usage incrementally

### From Other AI Tools
1. **Map Existing Workflows**: Document current AI usage
2. **Translate Prompts**: Convert to OpenCode format
3. **Configure Tools**: Set appropriate permissions
4. **Parallel Testing**: Run both systems temporarily
5. **Full Migration**: Switch when confident

## Maintenance Schedule

### Weekly
- Review agent usage analytics
- Check for configuration drift
- Update prompts based on feedback

### Monthly  
- Audit tool permissions
- Review security settings
- Optimize performance based on metrics

### Quarterly
- Major prompt improvements
- Agent restructuring if needed
- Technology stack updates

This guide provides the foundation for building effective, secure, and maintainable OpenCode agent configurations. Adapt these patterns to your specific project needs and team workflows.