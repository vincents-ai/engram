# Multi-Agent Best Practices Guide

> **Note:** This documentation describes multi-agent AI tool configuration patterns. Examples were originally written for OpenCode but the concepts apply to any AI coding tool.

This guide provides proven patterns, recommendations, and strategies for implementing effective agent configurations in your AI coding tool.

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
Structure agents like a software development team with one agent per role: senior engineer for code review and architecture decisions, frontend engineer for React/TypeScript/UI, backend engineer for APIs and databases, devops engineer for CI/CD and infrastructure, and a QA engineer for testing and quality assurance. Keeping each role distinct ensures that each agent carries only the context and permissions relevant to its domain.

### Domain Expertise Pattern
Organise agents around business domains when the project calls for specialised knowledge. For example, a game project might have a game designer for mechanics and balance, a domain specialist for industry-accurate content, a legal researcher for compliance, and a market analyst for industry trends. Each agent's prompt is focused tightly on that domain, preventing context bleed between unrelated areas.

### Workflow Stage Pattern
A third approach is to map agents to the stages of development: requirements analyst, architect, implementer, tester, and deployer. This is useful when you want strict gate-keeping between phases — for example, ensuring the architect approves a design before the implementer begins writing code.

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

## Permission Patterns and Security

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

**Development Agent (Standard):** Grant read/write/edit/bash/grep/glob/list. Restrict dangerous bash operations: `rm` → ask, `sudo` → deny, `git`/`npm`/`yarn`/`pnpm` → allow.

**Research Agent (Web Access):** Grant read/write/edit/grep/glob/list/webfetch. No bash access. Webfetch allowed.

**Review Agent (Read-Only):** Grant read/edit/grep/glob/list only. Set edit → ask so any proposed change is confirmed before it lands.

**Production Agent (Restricted):** Grant read and bash only. Restrict bash strictly: `kubectl` and `docker` → ask, `rm` and `sudo` → deny.

### Security Best Practices

#### 1. Principle of Least Privilege
Grant minimal necessary permissions. Documentation writers, for example, need read/write/edit and webfetch but have no reason for bash access. Require confirmation (`ask`) for any write operations if the agent rarely needs to modify files.

#### 2. Dangerous Command Restrictions
Always restrict dangerous bash operations: `rm` → ask, `sudo` → deny, `chmod` → ask, `chown` → deny, `dd` → deny, `mkfs` → deny.

#### 3. Environment Separation
Use different configuration files for different environments. In `config.dev.json`, bash access can be broad to enable fast iteration. In `config.prod.json`, restrict bash to only the commands needed for deployment (e.g. `git` and `docker` on ask, no `kubectl` without confirmation).

## Performance and Model Selection

### Task Complexity → Model Mapping
- **Simple tasks** (formatting, basic edits): a fast/cheap model (e.g. Haiku-class)
- **Standard development** (features, components): a balanced model (e.g. Sonnet-class)
- **Complex architecture** (system design): a powerful model (e.g. Opus-class)
- **Creative work** (content, design): a powerful model with higher temperature

Configure your AI coding tool to set per-agent model preferences where supported, using the cheapest model capable of the task to keep costs down.

### Tool Optimization
Only enable tools that agents actually use. A code reviewer only needs read/grep/glob/list — enabling write or bash would be unnecessary overhead and a security risk. A documentation writer needs read/write/webfetch but not bash.

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
Implement multi-stage review processes by chaining specialist reviewers: an implementer produces the initial code, a senior reviewer checks architecture and best practices, a security reviewer assesses vulnerabilities, and a performance reviewer identifies optimisation opportunities. Each reviewer has read-only access to enforce the separation of concerns.

## Common Anti-Patterns

### ❌ Over-Broad Agents
Avoid agents like `full-stack-developer` that handle everything from database to UI. The lack of focus means the agent's prompt becomes too large to be effective and its tool permissions grow dangerously wide.

### ❌ Tool Over-Granting
Avoid giving agents tools they don't need. A documentation writer does not need bash access. A formatter does not need webfetch. Every extra tool is a potential security surface.

### ❌ Vague Prompts
```markdown
You are a helpful developer. Write good code.
```
Prompts like this produce inconsistent results. Be explicit about responsibilities, standards, and how the agent fits into the project.

### ❌ Missing Context
Agents with no reference to project guidelines or architecture will guess at conventions, produce inconsistent output, and require more correction. Always reference `AGENTS.md` or an equivalent project context file.

## Monitoring and Optimization

### Performance Metrics
Track these key metrics:

- **Response Time**: Average agent response time
- **Token Usage**: Cost per agent interaction
- **Success Rate**: Percentage of successful completions
- **Error Rate**: Failed or incomplete responses

### Configuration Validation
Regularly review your configuration manually or with your tool's built-in validation support. Check:
- Agent prompt clarity and specificity
- Tool permissions against the least-privilege matrix
- Model assignments against task complexity
- Any drift between documented and actual configuration

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
2. **Translate Prompts**: Convert to your AI coding tool's format
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

This guide provides the foundation for building effective, secure, and maintainable agent configurations in your AI coding tool. Adapt these patterns to your specific project needs and team workflows.
