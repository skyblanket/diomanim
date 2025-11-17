# Contributing to Diomanim

Thank you for your interest in contributing to Diomanim! This document provides guidelines and instructions for contributing.

## Code of Conduct

Be respectful, constructive, and collaborative. We aim to create a welcoming environment for all contributors.

## Getting Started

### Prerequisites
- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Git
- Basic understanding of Rust and animation concepts

### Setting Up Development Environment

```bash
# Clone the repository
git clone https://github.com/yourusername/diomanim.git
cd diomanim

# Build the project
cargo build

# Run tests
cargo test

# Run the demo
cargo run --release
```

## How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported in [Issues](https://github.com/yourusername/diomanim/issues)
2. If not, create a new issue with:
   - Clear title describing the bug
   - Steps to reproduce
   - Expected vs actual behavior
   - Your environment (OS, Rust version, GPU)
   - Error messages or logs if available

### Suggesting Features

1. Check if the feature has already been suggested
2. Create a new issue with:
   - Clear description of the feature
   - Use cases and motivation
   - Possible implementation approach (optional)

### Submitting Pull Requests

1. **Fork the repository** and create a new branch from `main`:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**:
   - Follow the Rust style guide (use `cargo fmt`)
   - Add tests for new functionality
   - Update documentation as needed
   - Keep commits focused and atomic

3. **Ensure quality**:
   ```bash
   # Format code
   cargo fmt

   # Run linter
   cargo clippy -- -D warnings

   # Run tests
   cargo test

   # Build successfully
   cargo build --release
   ```

4. **Commit your changes**:
   ```bash
   git commit -m "feat: add new animation interpolation method"
   ```

   Use conventional commit messages:
   - `feat:` for new features
   - `fix:` for bug fixes
   - `docs:` for documentation
   - `test:` for tests
   - `refactor:` for code refactoring
   - `perf:` for performance improvements

5. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

6. **Create a Pull Request**:
   - Provide a clear title and description
   - Reference any related issues
   - Include screenshots/videos if relevant
   - Wait for review and address feedback

## Development Guidelines

### Code Style

- Follow Rust naming conventions
- Use `cargo fmt` before committing
- Run `cargo clippy` and address warnings
- Write clear, self-documenting code
- Add comments for complex logic

### Documentation

- Add doc comments (`///`) for public APIs
- Include examples in doc comments when helpful
- Update README.md for major changes
- Keep inline comments concise and relevant

### Testing

- Write unit tests for new functionality
- Update existing tests if behavior changes
- Aim for good test coverage
- Test edge cases and error conditions

### Performance

- Use benchmarks for performance-critical code
- Avoid unnecessary allocations
- Leverage SIMD when applicable
- Profile before optimizing

## Project Structure

```
diomanim/
├── src/
│   ├── core/          # Core math and utilities
│   ├── animation/     # Animation system
│   ├── scene/         # Scene graph
│   ├── mobjects/      # Scene objects
│   ├── render/        # GPU rendering
│   ├── lib.rs         # Library entry point
│   └── main.rs        # Demo application
├── examples/          # Example animations
├── tests/             # Integration tests
├── benches/           # Benchmarks
└── docs/              # Additional documentation
```

## Areas Needing Help

- **Mobjects**: More shapes and geometric primitives
- **Animations**: Additional easing functions and animation types
- **Rendering**: Shader improvements and effects
- **Documentation**: Tutorials and examples
- **Testing**: Increase test coverage
- **Performance**: Profiling and optimization
- **Platform Support**: Testing on different OSes and GPUs

## Questions?

- Open a [Discussion](https://github.com/yourusername/diomanim/discussions)
- Join our community chat (link TBD)
- Check existing issues and PRs

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

Thank you for contributing to Diomanim! 🎨
