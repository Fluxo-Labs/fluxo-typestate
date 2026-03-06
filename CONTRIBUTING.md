# Contributing to Fluxo Typestate

Thank you for your interest in contributing to Fluxo Typestate! This document provides guidelines and information to help you get started.

## Getting Started

### Prerequisites

- Rust 2024 edition or later
- Git

### Setup

1. Fork the repository
2. Clone your fork locally
3. Create a new branch for your feature or bugfix
4. Make your changes
5. Ensure all tests pass
6. Submit a pull request

```bash
git clone https://github.comFluxo-Labs/fluxo-typestate.git
cd fluxo-typestate
git checkout -b feature/your-feature-name
```

## Development Workflow

### Building

```bash
cargo build
```

### Testing

Run all tests:

```bash
cargo test --workspace
```

Run tests for a specific package:

```bash
cargo test -p fluxo-typestate
cargo test -p fluxo-typestate-macros
```

### Formatting

We use `rustfmt` for consistent code formatting:

```bash
cargo fmt --all
```

### Linting

We use `clippy` for linting:

```bash
cargo clippy --workspace -- -D warnings
```

## Code Style

- Follow Rust idioms and conventions
- Use meaningful variable and function names
- Add documentation comments (`///`) for public APIs
- Keep changes focused and atomic
- Ensure all public APIs have examples in their documentation

## Submitting Changes

### Pull Request Process

1. Update the README.md with details of changes if applicable
2. Update the CHANGELOG.md following the Keep a Changelog format
3. Ensure your PR description clearly describes the problem and solution
4. Link any relevant issues in your PR description
5. Wait for code review and address any feedback

### Commit Messages

Use clear and descriptive commit messages:

```
type(scope): brief description

More detailed explanatory text if necessary. Wrap at 72 characters.
```

Types:
- `feat`: new feature
- `fix`: bug fix
- `docs`: documentation changes
- `style`: formatting, missing semi-colons, etc.
- `refactor`: refactoring code without feature changes
- `test`: adding or updating tests
- `chore`: maintenance tasks

## Testing

### Unit Tests

Write unit tests for new functionality:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_feature() {
        // Test implementation
    }
}
```

### Integration Tests

Add integration tests in the `tests/` directory for end-to-end testing.

### Documentation Tests

Include examples in documentation that double as tests:

```rust
/// Adds two numbers together.
/// 
/// # Examples
/// 
/// ```
/// use fluxo_typestate::add;
/// 
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

## Release Process

Releases are handled by the maintainers. For version bumps:

1. Update version numbers in workspace `Cargo.toml`
2. Update CHANGELOG.md
3. Create a git tag
4. Publish to crates.io
5. Create GitHub release

## Getting Help

- Check existing [issues](https://github.com/fluxo-labs/fluxo-typestate/issues) for similar problems
- Create a new issue for bugs or feature requests
- Join discussions in [GitHub Discussions](https://github.com/fluxo-labs/fluxo-typestate/discussions)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
