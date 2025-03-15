# Bevy_RL2 Development Guidelines

## Build Commands

- Run the game: `cargo run`
- Run in dev mode with dynamic linking: `cargo run --features dev`
- Build for distribution: `cargo build --profile dist`
- Run in release mode: `cargo run --release`
- Run tests: `cargo test`
- Run a specific test: `cargo test test_name`
- Run clippy: `cargo clippy -- -D warnings`
- Format code: `cargo fmt --all`

## Style Guidelines

### Formatting

- Use `cargo fmt` before committing
- Follow Rust's standard naming conventions (snake_case for functions/variables, CamelCase for types)
- Group imports by crate, with Bevy imports first
- Enable clippy lints in files: `too_many_lines`, `if_not_else`, `explicit_iter_loop`, `explicit_into_iter_loop`, `exit`, `else_if_without_else`, `dbg_macro`

### Code Organization

- MVC-like architecture with controller/model/ui/view modules
- System sets defined in AppSet enum
- Game states defined in RunningState enum

### Types & Components

- Use Bevy's component/resource system for state management
- Implement Default trait for configuration structs
- Use #[derive(Resource, Reflect)] for settings and resources
- Use const functions when possible (#[must_use] where appropriate)

### Error Handling

- Use Result for fallible operations
- Use Option rather than null values
- Avoid unwrapping without context

### Documentation & Compatibility

- Document public functions and modules
- Maintain compatibility with Bevy 0.15
