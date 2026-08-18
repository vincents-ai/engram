# Personas & Lessons

Personas describe reusable agent roles. Lessons capture reusable experience, prevention rules, and patterns learned during work.

## Persona CLI Usage

```bash
# Create a persona
engram persona create \
  --slug rust-expert \
  --title "Rust Expert" \
  --instructions "Review Rust code for correctness and maintainability" \
  --domain rust

# List and inspect personas
engram persona list
engram persona list --domain rust
engram persona show --id rust-expert

# Update or remove personas
engram persona update --id rust-expert --add-tag async
engram persona delete --id rust-expert
```

## Lesson CLI Usage

```bash
# Create and list lessons
engram lesson create --title "Validate slug input" --domain rust --category code
engram lesson list
engram lesson show --id <LESSON_ID>

# Promote lessons into standards or rules when applicable
engram lesson update --id <LESSON_ID> --severity high
```

## Usage

Use personas to select the right operating mode for an agent, then record lessons when work reveals durable knowledge that should guide future sessions.
