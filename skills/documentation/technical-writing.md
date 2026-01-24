---
name: engram-technical-writing
description: "Write clear technical documentation with diagrams, code examples, following style guides and accessibility standards."
---

# Technical Writing (Engram-Integrated)

## Overview

Write clear, comprehensive technical documentation that helps users understand and use systems effectively. Include diagrams, code examples, API references, and tutorials. Follow style guides and accessibility standards. Store documentation plans and reviews in Engram for consistency and quality.

## When to Use

Use this skill when:
- Writing API documentation
- Creating user guides and tutorials
- Documenting system architecture
- Writing README files
- Creating onboarding documentation
- Documenting design decisions (ADRs)
- Writing technical blog posts
- Creating internal documentation
- Need to standardize documentation across team

## The Pattern

### Step 1: Define Documentation Requirements

Identify what to document:

```bash
engram context create \
  --title "Documentation Requirements: [System/Feature]" \
  --content "## Purpose\n\n**What:** [What needs documentation]\n**Why:** [Why users need this documentation]\n**Audience:** [Who will read this - developers, end users, admins]\n**Skill Level:** [Beginner, intermediate, advanced]\n\n## Documentation Type\n\n- [ ] **Tutorial**: Step-by-step guide for beginners\n- [ ] **How-To Guide**: Task-focused instructions\n- [ ] **Reference**: Technical details, API specs\n- [ ] **Explanation**: Concepts, architecture, design decisions\n\n**Selected:** [Type]\n**Rationale:** [Why this type fits the need]\n\n## Content Requirements\n\n### Must Include\n- [ ] Overview/introduction\n- [ ] Prerequisites\n- [ ] Step-by-step instructions OR reference details\n- [ ] Code examples (working, tested)\n- [ ] Common errors and troubleshooting\n- [ ] Next steps / related documentation\n\n### Should Include\n- [ ] Diagrams (architecture, flow, sequence)\n- [ ] Screenshots or videos\n- [ ] Best practices\n- [ ] Performance considerations\n- [ ] Security considerations\n\n### Optional\n- [ ] Historical context\n- [ ] Alternative approaches\n- [ ] Advanced usage\n\n## Success Criteria\n\n**A reader should be able to:**\n1. [Goal 1 - e.g., Set up authentication in 15 minutes]\n2. [Goal 2 - e.g., Understand when to use sync vs async]\n3. [Goal 3 - e.g., Troubleshoot common errors]\n\n## Existing Documentation\n\n**What exists:**\n- [Link to current docs]\n- [What's good about them]\n- [What's missing or unclear]\n\n**What's needed:**\n- [Gaps to fill]\n- [Improvements to make]\n\n## Style and Tone\n\n**Style Guide:** [e.g., Google Developer Style Guide, Microsoft Writing Style Guide]\n**Tone:** [Formal, conversational, technical]\n**Voice:** [Second person (you), first person (we), third person]\n**Tense:** [Present tense preferred]\n\n## Accessibility\n\n- [ ] Alt text for images\n- [ ] Descriptive link text (not \"click here\")\n- [ ] Proper heading hierarchy (h1 → h2 → h3)\n- [ ] Code examples with syntax highlighting\n- [ ] Readable color contrast\n- [ ] Screen reader friendly\n\n## Maintenance\n\n**Update Frequency:** [On every release, quarterly, as needed]\n**Owner:** [Team/person responsible]\n**Review Process:** [Who reviews before publishing]" \
  --source "technical-writing" \
  --tags "documentation,requirements,[topic]"
```

### Step 2: Create Documentation Outline

Structure the content:

```bash
engram reasoning create \
  --title "Documentation Outline: [Topic]" \
  --task-id [TASK_ID] \
  --content "## Document Structure\n\n### Title\n[Clear, descriptive title]\nExample: \"Getting Started with Webhooks\" (not \"Webhooks\")\n\n### Introduction (2-3 paragraphs)\n\n**Paragraph 1: What and Why**\n- What is this feature/system?\n- Why should readers care?\n- What problem does it solve?\n\n**Paragraph 2: Key Benefits**\n- Benefit 1\n- Benefit 2\n- Benefit 3\n\n**Paragraph 3: What You'll Learn**\n- After reading this guide, you'll be able to [X]\n- Time estimate: [e.g., 15 minutes]\n\n### Prerequisites\n\nBefore starting, you need:\n- [Requirement 1 - e.g., Node.js 16+]\n- [Requirement 2 - e.g., API key]\n- [Requirement 3 - e.g., Basic understanding of REST APIs]\n\n### Main Content\n\n#### Section 1: [Core Concept/First Step]\n\n**Concept Explanation:**\n- What is [concept]?\n- How does it work?\n- When should you use it?\n\n**Diagram:**\n```\n[ASCII or Mermaid diagram showing architecture/flow]\n```\n\n**Code Example:**\n```language\n[Complete, runnable code example]\n```\n\n**Explanation:**\n- Line-by-line breakdown of key parts\n- Why we did X instead of Y\n\n#### Section 2: [Next Concept/Step]\n\n(Same structure as Section 1)\n\n#### Section 3: [Advanced Usage/Edge Cases]\n\n**Common Scenarios:**\n\n**Scenario A:**\n- Use case: [When you need this]\n- Solution: [How to handle it]\n- Example:\n  ```language\n  [Code]\n  ```\n\n**Scenario B:**\n(Similar structure)\n\n### Troubleshooting\n\n#### Error: [Error message]\n\n**Symptom:** [What user sees]\n**Cause:** [Why this happens]\n**Solution:** [How to fix]\n**Example:**\n```\n[Code showing fix]\n```\n\n#### Issue: [Common problem]\n\n(Similar structure)\n\n### Best Practices\n\n**Do:**\n- ✓ [Good practice 1]\n- ✓ [Good practice 2]\n\n**Don't:**\n- ✗ [Anti-pattern 1]\n- ✗ [Anti-pattern 2]\n\n### Performance Considerations\n\n- [Consideration 1]\n- [Consideration 2]\n- Benchmarks: [Numbers if available]\n\n### Security Considerations\n\n- [Security concern 1]\n- [How to mitigate]\n\n### Next Steps\n\n**Now that you understand [topic], you can:**\n- [Next topic to learn]\n- [Related feature to explore]\n- [Advanced guide to read]\n\n**Related Documentation:**\n- [Link to related doc 1]\n- [Link to related doc 2]\n\n### API Reference (if applicable)\n\n#### `functionName(parameters)`\n\n**Description:** [What it does]\n\n**Parameters:**\n- `param1` (type): [Description]\n- `param2` (type, optional): [Description]\n\n**Returns:** type - [Description]\n\n**Example:**\n```language\nconst result = functionName(arg1, arg2);\nconsole.log(result);\n```\n\n**Throws:**\n- `ErrorType`: [When this error occurs]\n\n### Appendix (optional)\n\n- Glossary of terms\n- Complete code listing\n- Migration guide (if updating from old version)\n\n## Writing Guidelines\n\n**Clarity:**\n- Use simple, direct language\n- Define technical terms on first use\n- One idea per paragraph\n- Short sentences (< 25 words)\n\n**Code Examples:**\n- Complete and runnable\n- Include imports/setup\n- Show expected output\n- Comment only non-obvious parts\n- Test all examples before publishing\n\n**Diagrams:**\n- ASCII for simple flows (readable in plaintext)\n- Mermaid for complex diagrams (renders on GitHub)\n- Images for screenshots\n- Alt text for all images\n\n**Links:**\n- Descriptive link text: \"See the [authentication guide]\" (not \"click here\")\n- Link to related docs at end of sections\n- Deep link to specific sections when relevant\n\n**Confidence:** [0.0-1.0]" \
  --confidence [0.0-1.0] \
  --tags "documentation,outline,[topic]"
```

### Step 3: Write Content with Examples

Create the documentation:

```bash
engram context create \
  --title "Documentation Draft: [Topic]" \
  --content "# [Document Title]\n\n## Overview\n\n[Brief introduction paragraph explaining what this is and why it matters]\n\n**In this guide, you'll learn how to:**\n- [Learning objective 1]\n- [Learning objective 2]\n- [Learning objective 3]\n\n**Time to complete:** [X minutes]\n\n## Prerequisites\n\nBefore you begin, ensure you have:\n\n- [Prerequisite 1] - [Why needed]\n- [Prerequisite 2] - [Why needed]\n- [Prerequisite 3] - [Where to get it]\n\n## Quick Start\n\n**For readers who want to jump straight in:**\n\n```bash\n# Installation\nnpm install [package]\n\n# Basic usage\nconst client = new Client({ apiKey: 'your-key' });\nconst result = await client.doSomething();\nconsole.log(result);\n```\n\n**For detailed explanation, continue reading below.**\n\n---\n\n## Core Concepts\n\n### What is [Concept]?\n\n[Clear explanation of the concept in 2-3 paragraphs]\n\n**Example Use Case:**\nImagine you're building [scenario]. You need to [problem]. [This concept] solves this by [solution].\n\n### Architecture\n\n```\n┌─────────────┐\n│   Client    │\n└──────┬──────┘\n       │ 1. Request\n       ▼\n┌─────────────┐\n│  API Server │\n└──────┬──────┘\n       │ 2. Process\n       ▼\n┌─────────────┐\n│  Database   │\n└─────────────┘\n```\n\n**Flow:**\n1. Client sends request to API\n2. API processes and queries database\n3. API returns response to client\n\n---\n\n## Step-by-Step Guide\n\n### Step 1: Setup\n\n**Install the package:**\n\n```bash\nnpm install [package-name]\n```\n\n**Import into your project:**\n\n```javascript\nconst { Client } = require('[package-name]');\n// or with ES modules:\nimport { Client } from '[package-name]';\n```\n\n### Step 2: Configuration\n\n**Create a client instance:**\n\n```javascript\nconst client = new Client({\n  apiKey: process.env.API_KEY,  // Store keys in environment variables\n  baseUrl: 'https://api.example.com',\n  timeout: 5000,  // Optional: request timeout in ms\n});\n```\n\n**Configuration options:**\n\n| Option | Type | Required | Description |\n|--------|------|----------|-------------|\n| `apiKey` | string | Yes | Your API key from dashboard |\n| `baseUrl` | string | No | API endpoint (default: production) |\n| `timeout` | number | No | Request timeout in ms (default: 10000) |\n| `retries` | number | No | Number of retries on failure (default: 3) |\n\n### Step 3: Basic Usage\n\n**Fetch data:**\n\n```javascript\nasync function fetchUser(userId) {\n  try {\n    const user = await client.users.get(userId);\n    console.log(`User: ${user.name}`);\n    return user;\n  } catch (error) {\n    if (error.status === 404) {\n      console.error('User not found');\n    } else {\n      console.error('API error:', error.message);\n    }\n    throw error;\n  }\n}\n\nfetchUser('user_123');\n```\n\n**Expected output:**\n```\nUser: John Doe\n```\n\n**Explanation:**\n- Line 3: Call the `users.get()` method with user ID\n- Line 6-10: Handle errors gracefully with specific messages\n- Always wrap API calls in try-catch\n\n### Step 4: Advanced Usage\n\n**Batch operations:**\n\n```javascript\nasync function batchFetch(userIds) {\n  // Fetch all users in parallel\n  const promises = userIds.map(id => client.users.get(id));\n  \n  // Wait for all to complete\n  const users = await Promise.all(promises);\n  \n  return users;\n}\n\nconst users = await batchFetch(['user_1', 'user_2', 'user_3']);\n```\n\n**Pagination:**\n\n```javascript\nasync function getAllUsers() {\n  let allUsers = [];\n  let page = 1;\n  let hasMore = true;\n  \n  while (hasMore) {\n    const response = await client.users.list({\n      page: page,\n      limit: 100,\n    });\n    \n    allUsers = allUsers.concat(response.data);\n    hasMore = response.hasMore;\n    page++;\n  }\n  \n  return allUsers;\n}\n```\n\n---\n\n## Troubleshooting\n\n### Error: \"Invalid API key\"\n\n**Symptom:**\n```\nError: Invalid API key\nStatus: 401 Unauthorized\n```\n\n**Cause:**\nYour API key is missing, incorrect, or expired.\n\n**Solution:**\n1. Check your API key in the dashboard: https://dashboard.example.com/settings/api\n2. Ensure key is set in environment variable:\n   ```bash\n   echo $API_KEY  # Should print your key\n   ```\n3. Regenerate key if expired\n\n### Error: \"Rate limit exceeded\"\n\n**Symptom:**\n```\nError: Rate limit exceeded\nStatus: 429 Too Many Requests\nRetry-After: 60\n```\n\n**Cause:**\nYou've exceeded the API rate limit (default: 100 requests/minute).\n\n**Solution:**\n1. Add delay between requests:\n   ```javascript\n   await new Promise(resolve => setTimeout(resolve, 1000)); // Wait 1s\n   ```\n2. Implement exponential backoff:\n   ```javascript\n   const client = new Client({\n     apiKey: process.env.API_KEY,\n     retries: 3,\n     retryDelay: (attempt) => Math.pow(2, attempt) * 1000, // 1s, 2s, 4s\n   });\n   ```\n3. Upgrade to higher tier for increased limits\n\n### Issue: Slow response times\n\n**Symptom:**\nAPI calls take > 5 seconds.\n\n**Diagnosis:**\n```javascript\nconst start = Date.now();\nconst result = await client.users.get('user_123');\nconst duration = Date.now() - start;\nconsole.log(`Request took ${duration}ms`);\n```\n\n**Possible causes:**\n1. **Network latency:** Use a client in the same region as API\n2. **Large payload:** Use pagination or field filtering\n3. **Database slow:** Contact support if persistent\n\n---\n\n## Best Practices\n\n### Do ✓\n\n**Store API keys securely:**\n```javascript\n// ✓ Good: Use environment variables\nconst apiKey = process.env.API_KEY;\n\n// ✗ Bad: Hardcode in source\nconst apiKey = 'sk_live_abc123...';  // DON'T DO THIS\n```\n\n**Handle errors gracefully:**\n```javascript\n// ✓ Good: Specific error handling\ntry {\n  const user = await client.users.get(id);\n} catch (error) {\n  if (error.status === 404) {\n    // Handle not found\n  } else if (error.status === 429) {\n    // Handle rate limit\n  } else {\n    // Handle other errors\n  }\n}\n\n// ✗ Bad: Catch and ignore\ntry {\n  const user = await client.users.get(id);\n} catch (error) {\n  // Silent failure\n}\n```\n\n**Use timeouts:**\n```javascript\n// ✓ Good: Set reasonable timeout\nconst client = new Client({\n  timeout: 5000,  // 5 seconds\n});\n\n// ✗ Bad: No timeout (can hang forever)\nconst client = new Client();\n```\n\n### Don't ✗\n\n**Don't poll excessively:**\n```javascript\n// ✗ Bad: Poll every second\nsetInterval(async () => {\n  const status = await client.jobs.getStatus(jobId);\n}, 1000);\n\n// ✓ Good: Use webhooks or exponential backoff\nlet delay = 1000;\nwhile (status !== 'completed') {\n  await new Promise(resolve => setTimeout(resolve, delay));\n  status = await client.jobs.getStatus(jobId);\n  delay = Math.min(delay * 2, 60000); // Cap at 60s\n}\n```\n\n**Don't expose API keys:**\n```javascript\n// ✗ Bad: Log API keys\nconsole.log('API Key:', apiKey);\n\n// ✓ Good: Mask sensitive data\nconsole.log('API Key:', apiKey.slice(0, 7) + '...');\n```\n\n---\n\n## Performance\n\n### Benchmarks\n\n| Operation | Avg Latency | p99 Latency |\n|-----------|-------------|-------------|\n| Get User | 45ms | 120ms |\n| List Users (100) | 180ms | 350ms |\n| Create User | 90ms | 200ms |\n\n### Optimization Tips\n\n**1. Use batch operations:**\n```javascript\n// Slow: N requests\nfor (const id of userIds) {\n  await client.users.get(id);\n}\n\n// Fast: 1 request\nconst users = await client.users.batchGet(userIds);\n```\n\n**2. Cache responses:**\n```javascript\nconst cache = new Map();\n\nasync function getCachedUser(userId) {\n  if (cache.has(userId)) {\n    return cache.get(userId);\n  }\n  \n  const user = await client.users.get(userId);\n  cache.set(userId, user);\n  return user;\n}\n```\n\n**3. Use pagination efficiently:**\n```javascript\n// Request only what you need\nconst users = await client.users.list({\n  limit: 10,  // Small page size\n  fields: ['id', 'name'],  // Only needed fields\n});\n```\n\n---\n\n## Security\n\n### API Key Security\n\n**Never:**\n- Commit API keys to git\n- Log API keys\n- Share API keys in chat/email\n- Use production keys in development\n\n**Always:**\n- Store keys in environment variables or secret managers\n- Rotate keys regularly (every 90 days)\n- Use separate keys for dev/staging/production\n- Revoke compromised keys immediately\n\n### HTTPS Only\n\nAll API calls use HTTPS. Do not disable SSL verification:\n\n```javascript\n// ✗ NEVER DO THIS\nprocess.env.NODE_TLS_REJECT_UNAUTHORIZED = '0';\n```\n\n### Input Validation\n\nValidate all user input before sending to API:\n\n```javascript\nfunction validateUserId(userId) {\n  if (typeof userId !== 'string') {\n    throw new Error('User ID must be a string');\n  }\n  if (!/^user_[a-z0-9]+$/i.test(userId)) {\n    throw new Error('Invalid user ID format');\n  }\n  return true;\n}\n```\n\n---\n\n## Next Steps\n\n**Now that you understand the basics, explore:**\n\n- [Advanced Authentication](docs/auth.md) - OAuth, JWT, API keys\n- [Webhooks Guide](docs/webhooks.md) - Real-time notifications\n- [Error Handling](docs/errors.md) - Comprehensive error reference\n- [API Reference](api/) - Complete API documentation\n\n**Need help?**\n\n- [GitHub Issues](https://github.com/example/repo/issues) - Bug reports and feature requests\n- [Discord Community](https://discord.gg/example) - Chat with other developers\n- [Support](mailto:support@example.com) - Email support team\n\n---\n\n## API Reference\n\n### `Client`\n\n#### Constructor\n\n```javascript\nnew Client(options)\n```\n\n**Parameters:**\n- `options` (object): Configuration options\n  - `apiKey` (string, required): Your API key\n  - `baseUrl` (string, optional): API endpoint URL\n  - `timeout` (number, optional): Request timeout in milliseconds\n\n**Returns:** `Client` instance\n\n**Example:**\n```javascript\nconst client = new Client({\n  apiKey: 'sk_test_abc123',\n  timeout: 5000,\n});\n```\n\n#### `client.users.get()`\n\n```javascript\nawait client.users.get(userId)\n```\n\n**Parameters:**\n- `userId` (string, required): User ID to fetch\n\n**Returns:** `Promise<User>` - User object\n\n**Throws:**\n- `APIError` (404): User not found\n- `APIError` (401): Invalid API key\n- `APIError` (429): Rate limit exceeded\n\n**Example:**\n```javascript\nconst user = await client.users.get('user_123');\nconsole.log(user.name);\n```\n\n**User object:**\n```typescript\ninterface User {\n  id: string;\n  name: string;\n  email: string;\n  created_at: string;  // ISO 8601 timestamp\n}\n```\n\n---\n\n## Glossary\n\n**API Key**: Secret token used to authenticate API requests.\n\n**Rate Limit**: Maximum number of requests allowed per time period.\n\n**Webhook**: HTTP callback triggered when events occur.\n\n**Pagination**: Splitting large result sets into pages.\n\n---\n\n## Changelog\n\n**v1.2.0 (2026-01-24)**\n- Added batch operations\n- Improved error messages\n- Added retry logic\n\n**v1.1.0 (2026-01-01)**\n- Added pagination support\n- Performance improvements\n\n**v1.0.0 (2025-12-01)**\n- Initial release" \
  --source "technical-writing" \
  --tags "documentation,draft,[topic]"
```

### Step 4: Review and Edit

Create review checklist:

```bash
engram context create \
  --title "Documentation Review Checklist" \
  --content "# Documentation Review Checklist\n\n## Content Quality\n\n- [ ] **Accurate**: All information is correct and up-to-date\n- [ ] **Complete**: Covers all necessary topics\n- [ ] **Clear**: Easy to understand for target audience\n- [ ] **Concise**: No unnecessary words or complexity\n- [ ] **Relevant**: Focused on user needs\n\n## Structure\n\n- [ ] **Logical flow**: Content progresses naturally\n- [ ] **Proper headings**: H1 → H2 → H3 hierarchy\n- [ ] **Table of contents**: For long documents\n- [ ] **Short paragraphs**: 3-4 sentences max\n- [ ] **Lists**: Use for multiple items\n\n## Code Examples\n\n- [ ] **Complete**: All examples are runnable\n- [ ] **Tested**: Verified to work as shown\n- [ ] **Syntax highlighting**: Correct language specified\n- [ ] **Comments**: Only where needed (not obvious)\n- [ ] **Output shown**: Expected results included\n\n## Diagrams and Images\n\n- [ ] **Alt text**: All images have descriptive alt text\n- [ ] **Readable**: Clear at different screen sizes\n- [ ] **Up-to-date**: Match current UI/architecture\n- [ ] **Necessary**: Each image adds value\n\n## Links\n\n- [ ] **Working**: All links return 200 OK\n- [ ] **Descriptive**: Link text describes destination\n- [ ] **Relevant**: Links add value\n- [ ] **Internal vs external**: Clearly distinguished\n\n## Style\n\n- [ ] **Consistent tone**: Matches style guide\n- [ ] **Active voice**: \"Click the button\" not \"The button should be clicked\"\n- [ ] **Present tense**: \"The API returns\" not \"The API will return\"\n- [ ] **Second person**: \"You can\" not \"One can\" or \"Users can\"\n- [ ] **Simple words**: \"Use\" not \"Utilize\"\n\n## Grammar and Spelling\n\n- [ ] **Spell check**: No typos\n- [ ] **Grammar check**: Correct sentence structure\n- [ ] **Punctuation**: Proper comma and period usage\n- [ ] **Capitalization**: Consistent (e.g., API not Api)\n\n## Accessibility\n\n- [ ] **Heading hierarchy**: No skipped levels\n- [ ] **Color contrast**: Text readable on background\n- [ ] **Link text**: Not \"click here\" or \"read more\"\n- [ ] **Alt text**: Describes image content\n- [ ] **Code blocks**: Properly formatted\n\n## Completeness\n\n- [ ] **Prerequisites**: Listed upfront\n- [ ] **Troubleshooting**: Common issues covered\n- [ ] **Next steps**: Where to go next\n- [ ] **Related docs**: Linked appropriately\n\n## Technical Accuracy\n\n- [ ] **Correct API calls**: Match actual API\n- [ ] **Correct parameters**: All required params included\n- [ ] **Correct responses**: Match actual responses\n- [ ] **Error handling**: Shows realistic error scenarios\n\n## Final Checks\n\n- [ ] **Peer review**: At least one other person reviewed\n- [ ] **Subject matter expert review**: Technical accuracy verified\n- [ ] **User testing**: Test docs with target audience\n- [ ] **Version noted**: Document version matches product version" \
  --source "technical-writing" \
  --tags "documentation,review-checklist"
```

### Step 5: Link Documentation to System

```bash
# Link documentation to system/feature task
engram relationship create \
  --source-id [SYSTEM_TASK_ID] --source-type task \
  --target-id [DOCS_DRAFT_ID] --target-type context \
  --relationship-type documents --agent default

engram relationship create \
  --source-id [SYSTEM_TASK_ID] --source-type task \
  --target-id [DOCS_OUTLINE_ID] --target-type reasoning \
  --relationship-type documents --agent default
```

## Example

Write API documentation for webhook feature.

### Step 1: Requirements

```bash
REQ=$(engram context create \
  --title "Documentation Requirements: Webhooks" \
  --content "Purpose: Document webhook feature for API users\nAudience: Developers integrating our API\nSkill: Intermediate (understand REST APIs)\nType: How-To Guide + Reference\n\nMust Include:\n- How to set up webhooks\n- Event types available\n- Payload examples\n- Security (signature verification)\n- Troubleshooting\n\nSuccess: Reader can set up webhooks in 15 minutes" \
  --source "technical-writing" \
  --tags "documentation,requirements,webhooks" \
  --json | jq -r '.id')
```

### Step 2: Outline

```bash
OUTLINE=$(engram reasoning create \
  --title "Documentation Outline: Webhooks" \
  --task-id webhooks-123 \
  --content "1. Introduction\n   - What are webhooks\n   - Why use instead of polling\n   - Quick example\n\n2. Setup (Step-by-step)\n   - Create webhook URL endpoint\n   - Register in dashboard\n   - Test webhook\n\n3. Event Types\n   - List all event types\n   - Payload schema for each\n\n4. Security\n   - Signature verification\n   - Replay attack prevention\n\n5. Troubleshooting\n   - Webhook not firing\n   - Failed deliveries\n   - Retry logic\n\n6. API Reference\n   - Create webhook endpoint\n   - List webhooks endpoint\n   - Delete webhook endpoint" \
  --confidence 0.90 \
  --tags "documentation,outline,webhooks" \
  --json | jq -r '.id')
```

### Step 3: Write Draft

(See full example in Step 3 of The Pattern above)

### Step 4: Review

```bash
# Run through review checklist
# - Code examples tested: ✓
# - Alt text for diagrams: ✓
# - Links working: ✓
# - Peer reviewed: ✓
```

### Step 5: Publish

```bash
# Publish to documentation site
cp webhook-guide.md docs/guides/webhooks.md
git add docs/guides/webhooks.md
git commit -m "docs: Add webhook setup guide"
git push

# Update docs site
cd docs-site
npm run build
npm run deploy
```

## Querying Documentation

```bash
# Get all documentation drafts
engram context list | grep "Documentation Draft"

# Find documentation for specific topic
engram context list | grep -i "webhooks"

# Get all documentation outlines
engram reasoning list | grep "Documentation Outline"

# Find documentation requirements
engram context list | grep "Documentation Requirements"
```

## Related Skills

This skill integrates with:
- `engram-api-design` - Document API endpoints and schemas
- `engram-system-design` - Document system architecture
- `engram-changelog` - Create user-facing release notes
- `engram-code-review` - Review documentation in PRs
- `engram-onboarding` - Create onboarding documentation
- `engram-accessibility` - Ensure documentation is accessible
