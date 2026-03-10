# Contributing

## Adding New Entities

1. Define entity in `src/entities/`
2. Implement `Entity` trait
3. Add CLI commands in `src/cli/`
4. Register storage handler
5. Add tests

## Adding New CLI Commands

1. Create file in `src/cli/`
2. Define command enum with `clap::Subcommand`
3. Implement handler functions
4. Register in `src/main.rs`

## Running Tests

```bash
# Run all tests
cargo test

# Run specific module
cargo test --lib entities

# Run with output
cargo test -- --nocapture
```
