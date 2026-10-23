# Contributing to sea-orm-cubrid

Thank you for your interest in contributing to `sea-orm-cubrid`.

## Development Setup

### Prerequisites

- Rust 1.70+
- Git
- Docker (for integration tests)

### Installation

```bash
git clone https://github.com/cubrid-labs/sea-orm-cubrid.git
cd sea-orm-cubrid

cargo build
cargo test --lib
```

## Running Tests

### Offline tests

```bash
cargo test --lib
```

### Integration tests

```bash
docker compose up -d
export CUBRID_TEST_URL="cubrid://dba@localhost:33000/testdb"
cargo test
docker compose down -v
```

## Code Style

This project uses `rustfmt` for formatting and `clippy` for linting.

```bash
cargo fmt --check
cargo clippy -- -D warnings
```

To auto-fix:

```bash
cargo fmt
cargo clippy --fix --allow-dirty
```

## Pull Request Review and Merge Policy

The same contributor process applies to human-written and AI-assisted
changes (OpenCode, Codex, Claude Code, ChatGPT, Copilot, etc.).
Use the repository's GitHub Actions CI checks for authoritative final
validation, with local checks encouraged but not separately required.
Review the implementation, tests, security, compatibility and documentation.
Oracle, Copilot and `agent-review` output is optional advisory context.
The responsible maintainer makes the final merge decision after required
checks and actionable findings are resolved or documented. Security,
CI permissions and release/publishing changes get independent maintainer
scrutiny. No existing repository CI or release safeguards are waived.

## Pull Request Guidelines

1. Keep changes focused and explain the motivation in the PR description.
2. Add or update tests for behavior changes.
3. Ensure lint and offline tests pass before submitting.
4. Run integration tests for connection/backend-related updates.
5. Update `CHANGELOG.md` for user-visible changes.

## Reporting Issues

When filing an issue, include:

- Rust version
- CUBRID server version
- Minimal reproduction snippet
- Full error output
