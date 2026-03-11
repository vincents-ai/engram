# Context

Context represents the raw materials for your work—documentation, code snippets, URLs, notes.

## CLI Usage

```bash
# Create context
engram context create --title "Stripe API Docs" --source "https://stripe.com/docs/api"

# Create with content
engram context create --title "Error Log" --content "$(cat error.log)"

# List context
engram context list
```

## Use Cases

- Documentation URLs
- Code snippets
- Error logs
- Meeting notes
- External resources
