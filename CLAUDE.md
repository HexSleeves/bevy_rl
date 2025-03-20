# Bevy_RL2 Development Guidelines

## Build Commands

- Run the game: `cargo run`
- Run in dev mode with hot reload: `cargo run --features dev`
- Build for distribution: `cargo build --profile dist`
- Run in release mode: `cargo run --release`
- Run tests: `cargo test`
- Run a specific test: `cargo test test_name`
- Run clippy: `cargo clippy -- -D warnings`
- Format code: `cargo fmt --all`

## Code Organization

- MVC architecture: model/ (game state), view/ (rendering), controller/ (input), ui/ (interface)
- System execution order follows AppSet order: RecordInput → Visibility → Update → Render
- Define components/resources/systems in their respective module directories

## Style Guidelines

- Format with `cargo fmt` before committing
- Use snake_case for functions/variables/modules, CamelCase for types, SCREAMING_CASE for constants
- Group imports: Bevy imports first, then external crates, then local modules
- Enable clippy lints: `too_many_lines`, `if_not_else`, `explicit_iter_loop`, `exit`, `else_if_without_else`
- Avoid `dbg_macro` and `println!` in production code - use tracing macros instead

## Types & Components

- Register components with `app.register_type::<YourComponent>()` for reflection support
- Use #[derive(Resource, Reflect)] for resources that need inspection
- Implement Default trait for configuration structs
- Use custom macros for repetitive patterns (see src/macros/)
- Prefer const functions when appropriate (#[must_use] for functions with return values)

## Error Handling

- Use Result<T, Error> with thiserror for fallible operations
- Define error types with thiserror::Error for better context
- Use Option rather than null values
- Avoid unwrapping (.unwrap(), .expect()) without clear context
- Use anyhow::Result for propagating errors in complex functions

## Documentation & Testing

- Document public APIs with /// comments
- Maintain compatibility with Bevy 0.15
- Follow ECS best practices from the Bevy documentation
- Write unit tests for critical game systems
- Use integration tests for verifying game mechanics work together properly
- Update ARCHITECTURE.md when adding new modules, components, or design patterns
- Use conventional commits format for documentation changes (docs: update...)

## Version Control Practices

- Autocommit changes after completing a task to prevent accumulation of unstaged files
- Use conventional commit format (feat:, fix:, docs:, chore:, etc.)
- Prefer small, focused commits over large changesets
- Include clear descriptions of changes and their purpose
- Remove unused files with appropriate commit messages (chore: remove...)