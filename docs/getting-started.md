# Getting Started with Engram

Engram is a distributed memory system for AI agents and human operators. It acts as a "second brain" for your software projects.

## Installation

```bash
# Clone the repository
git clone https://github.com/vincents-ai/engram.git
cd engram

# Build the binary
cargo build --release

# Add to PATH
export PATH="$PWD/target/release:$PATH"
```

## Quick Setup

```bash
# Initialize workspace
./target/release/engram setup workspace

# Create your profile
./target/release/engram setup agent --name "Your Name" --agent-type operator
```

## Core Workflow

The Engram workflow follows a simple cycle: **Plan → Execute → Remember**.

### 1. Plan

```bash
# Create a task
./target/release/engram task create --title "Add user authentication" --priority high
```

### 2. Execute & Document

```bash
# Store context
./target/release/engram context create --title "OAuth2 Spec" --source "https://oauth.net/2/"

# Link to task
./target/release/engram relationship create --source-id <TASK_ID> --target-id <CONTEXT_ID> --type references
```

### 3. Remember

```bash
# Record a decision
./target/release/engram reasoning create --title "Chose JWT for stateless auth" --task-id <TASK_ID>
```

## Next Steps

- [User Guide](user-guide.md) - Full workflow documentation
- [Theory Building](features/theory-building.md) - Capture mental models
- [CLI Reference](reference/cli.md) - Command reference
