# FAQ

## What is Engram?

Engram is a distributed memory system for AI agents and human operators. It stores context, reasoning, and knowledge in Git refs rather than files.

## How is data stored?

Data is stored in `.git/refs/engram/` as JSON files. This keeps your working directory clean while maintaining version control.

## Can I use Engram without Git?

Yes, use memory-only storage for testing or non-Git projects.

## What's the difference between Context and Knowledge?

- **Context**: Raw materials for a specific task (docs, snippets, logs)
- **Knowledge**: Reusable patterns that transcend tasks

## What is Theory Building?

Based on Peter Naur's "Programming as Theory Building" (1985). It captures the mental model developers have of their code—why it was designed a certain way, what invariants must hold, etc.

## How does State Reflection work?

When code behavior conflicts with an agent's theory, the agent creates a StateReflection. If dissonance is high (≥0.7), the theory must be updated before code can be fixed.
