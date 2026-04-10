# Engram User Guide

Welcome to Engram! This guide is written for **human operators**—developers, product managers, and architects who want to use Engram to manage their software projects.

## What is Engram?

Engram is a "second brain" for your code. While Git tracks *what* changed (the code), Engram tracks *why* it changed (the reasoning), *how* you planned it (the tasks), and *what* you learned along the way (the context).

It lives inside your repository but doesn't clutter your files. Engram stores data directly in your `.git` database using custom references (`refs/engram/`). This means your project management is version-controlled without you ever having to `git add` a task file.

## Getting Started

Follow these steps to initialize Engram in your project.

### 1. Initialize Workspace
Run this in the root of your git repository. It creates the necessary internal structures in `.git/`.

```bash
engram setup workspace
```

### 2. Create Your Identity (Agent Profile)
Tell Engram who you are. This creates a "human" agent profile so your actions are attributed correctly.

```bash
engram setup agent --name "Your Name" --agent-type operator
```

*   `--name`: Your display name.
*   `--agent-type`: Use `operator` for humans. (Other types like `coder` or `reviewer` are for AI agents).

### 3. Install Validation Hook (Recommended)
The pre-commit hook ensures you never commit code without linking it to a task. This keeps your history clean and traceable.

```bash
# Check if hook is already installed
engram validate hook status

# Install the hook
engram validate hook install
```

Once installed, if you try `git commit -m "fix bug"` without a task ID, the commit will be rejected. You'll need to use: `git commit -m "fix bug [TASK-123]"`.

## Core Concepts

### 1. Tasks
Tasks are the units of work. Unlike a simple todo list, Engram tasks are hierarchical and stateful.

*   **Use for**: Planning features, tracking bugs, organizing research.
*   **Example**: "Implement OAuth2 Login" (Parent) -> "Design DB Schema" (Child).

### 2. Context
Context represents the raw materials of your knowledge work. It's where you dump information so you (and your AI agents) don't have to hold it in your head.

*   **Use for**: Storing documentation URLs, code snippets, error logs, meeting notes.
*   **Example**: "Stripe API Docs - Payment Intents", "Error Log from Production Crash".

### 3. Reasoning
This is Engram's superpower. A Reasoning entity captures the *decision-making process*.

*   **Use for**: Explaining why you chose a library, why you refactored a class, or why you closed a bug as "wontfix".
*   **Example**: "Chose PostgreSQL over Mongo because we need relational integrity for transactions."

### 4. Relationships
Entities don't live in isolation. You link them together to create a knowledge graph.

*   **Common links**:
    *   Task `depends_on` Task
    *   Task `references` Context
    *   Reasoning `justifies` Task

## Day-to-Day Workflow

### Step 1: Start Your Day (Session)
Tell Engram you're starting work. This helps track metrics and context switches.

```bash
engram session start --name human
```

### Step 2: Plan Your Work
Before you code, define what "done" looks like.

```bash
# Create a task
engram task create --title "Fix login timeout bug" --priority high

# Create a subtask for investigation
engram task create --title "Reproduce timeout locally" --parent-id [TASK-ID]
```

### Step 3: Capture Context
Found a relevant StackOverflow answer? Don't just bookmark it—save it.

```bash
engram context create --title "JWT Expiration Solution" --source "https://stackoverflow.com/..."
engram relationship create --source-id [TASK-ID] --target-id [CONTEXT-ID] --relationship-type references
```

### Step 4: Log Decisions
You decided to increase the timeout to 30 minutes. Record why.

```bash
engram reasoning create --title "Increased session timeout" --description "User feedback indicates 15m is too short for complex forms." --task-id [TASK-ID]
```

### Step 5: Query History
Need to remember why you did something last month?

```bash
# Search for reasoning related to "timeout"
engram reasoning list --task-id [TASK-ID]
```

## Working with AI Agents

Engram shines when you work with AI. Because your context and plans are structured, an AI agent can read your Engram workspace and immediately understand the project state.

**To hand off work to an agent:**

1.  **Be Explicit in Titles**: Use "Implement..." or "Refactor..." verbs.
2.  **Add Context Links**: Explicitly link relevant `Context` entities to the `Task`. The agent will read these first.
3.  **Define Acceptance Criteria**: Put this in the task description.

*Example Agent Handoff:*
> "I created task [TASK-123] 'Refactor Auth Middleware'. I've linked the [Context-456] 'New Security Standards' doc. Please pick this up."

## Theory Building

Based on Peter Naur's "Programming as Theory Building" (1985), capture the mental model behind your code.

```bash
# Create a theory about a domain
engram theory create "User Authentication" --agent your-agent

# Add conceptual model (concepts + definitions)
engram theory update --id <ID> --concept "User: A person who authenticates to the system"
engram theory update --id <ID> --concept "Session: A period of authenticated access"

# Add system mappings (how concepts map to code)
engram theory update --id <ID> --mapping "User: src/entities/user.rs (struct User)"
engram theory update --id <ID> --mapping "Session: src/entities/session.rs (struct Session)"

# Add design rationale (why decisions were made)
engram theory update --id <ID> --rationale "JWT Tokens: Stateless auth to avoid session storage"

# Add invariants (must-be-true statements)
engram theory update --id <ID> --invariant "User email must be unique"
```

## State Reflection

When code behavior conflicts with your theory, use reflection to detect and resolve the dissonance.

```bash
# Create a reflection when you encounter unexpected behavior
engram reflect create \
  --theory <THEORY_ID> \
  --observed "Test failed: expected User but got None" \
  --trigger-type test_failure

# Record specific dissonance points
engram reflect record-dissonance \
  --id <REFLECTION_ID> \
  --description "Theory claims User always exists, but code allows null"

# Propose theory updates
engram reflect propose-update \
  --id <REFLECTION_ID> \
  --update "Make User optional in Session, update invariant"

# Check if mutation required (dissonance >= 0.7)
engram reflect requires-mutation --id <REFLECTION_ID>
# Exit 0 = theory must be updated before code fixes
```

The 0.7 threshold enforces Naur's insight: bugs often indicate flawed mental models, not just typos.

## Tips & Tricks

*   **Automatic Storage**: Engram saves data instantly to your git repo's database (`.git/objects`). You do **not** need to `git add` or commit Engram data manually; it's handled automatically via `git refs`.
*   **Share with Team**: Since Engram uses standard git refs, you can push/pull them to share context with your team (requires configuring your git fetch/push refspecs).
*   **Hook Usage**: With the hook installed, use `engram task list --status in_progress` to find your active task ID before committing.
*   **Keep it Granular**: It's better to have 5 small tasks than 1 giant one. It makes reasoning easier to attach.
*   **Be Specific with AI**: When creating tasks for AI agents, include explicit file paths, expected outcomes, and success criteria. This prevents the "tool calling loop" issue where agents over-investigate instead of acting. See [Known Issues](./known-issues/README.md) for details.
