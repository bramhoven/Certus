# Contributing to Certus

Thank you for your interest in contributing to Certus! We welcome contributions from the community to help improve this trading execution and backtesting engine.

## How to Contribute

### Reporting Issues

If you find a bug or have a feature request, please [open an issue](https://github.com/yourusername/certus/issues) on GitHub. Provide as much detail as possible, including:

- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- Environment details (Rust version, OS, etc.)

### Submitting Pull Requests

1. Fork the repository and create a new branch for your changes.
2. Ensure your code follows the project's style guidelines.
3. Write tests for new functionality.
4. Run the test suite to make sure everything passes.
5. Update documentation if necessary.
6. Submit a pull request with a clear description of the changes.

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Git

### Building the Project

```bash
git clone https://github.com/yourusername/certus.git
cd certus
cargo build
```

### Running Tests

```bash
cargo test
```

### Code Formatting

We use `rustfmt` for code formatting. Run it with:

```bash
cargo fmt
```

### Linting

Use `clippy` for linting:

```bash
cargo clippy
```

## Code Style

- Follow the standard Rust style guidelines.
- Use descriptive variable and function names.
- Add comments for complex logic.
- Keep functions small and focused.

## Testing

- Write unit tests for new functions.
- Include integration tests for larger features.
- Ensure all tests pass before submitting a PR.

## Commit Messages

Use clear, descriptive commit messages. Follow the format:

```
<type>(<scope>): <description>

[optional body]
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

## Code of Conduct

Please be respectful and inclusive in all interactions. We follow a code of conduct to ensure a positive community.

## License

By contributing to Certus, you agree that your contributions will be licensed under the same license as the project (MIT).

Thank you for contributing!