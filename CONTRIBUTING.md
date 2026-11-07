# Contributing to sea-orm-cubrid

For issues, use clear type-only titles (`feat:`, `bug:`, etc.) without module scope and state observable acceptance criteria. PRs and commits use the repository's Conventional Commits/CI rules. Read the [shared issue, triage, PR and commit standard](https://github.com/cubrid-lab/.github/blob/main/docs/ISSUE_PR_COMMIT_STANDARD.md).

Thank you for your interest in contributing to `sea-orm-cubrid`.

## Development Setup

### Prerequisites

- Rust 1.88 or later (the minimum in `Cargo.toml`)
- Git
- A live CUBRID database only for separately arranged manual testing

### Installation

```bash
git clone https://github.com/cubrid-lab/sea-orm-cubrid.git
cd sea-orm-cubrid

cargo build
cargo test --all-targets --all-features
```



## Test-driven implementation

For bug fixes or behavioral features, start from a reviewed interface/SQL/transaction design. Use the [TDD decision](docs/adr/0004-test-first-development.md): **Red** (run a new focused test and capture the genuine target failure), **Green** (smallest production change passing that test), **Refactor** (rerun related tests and final PR CI). In the PR description record the failing assertion/error **actually observed**, the passing assertion, which regressions ran, and whether real CUBRID validation ran or remains unverified. A mock-only pass cannot establish server compatibility; the exact release candidate still requires full live DB validation. No deliberate failing tests or skipped release checks may be left to satisfy the TDD process. For documentation and upstream feasibility work, describe the design/probe evidence rather than forcing an irrelevant Red test. The ADR and design contracts are proposals until reviewed and accepted.

## CUBRID correctness and regression evidence

For changes to CUBRID protocol, connection lifecycle, error handling, results, native SQL, ORM migrations, supported versions or releases, read [this repository's engineering invariants](docs/ENGINEERING_INVARIANTS.md) **before** implementation and review. The checks are adapted from the real bugs and tests of `pycubrid` and `sqlalchemy-cubrid`, not a Python API port. In the pull request, describe the behavioral invariant affected, the normal and failure cases tested, and which checks were offline, real CUBRID or independently compared with JDBC. State the exact CUBRID/runtime/driver versions and test outcomes when claiming database compatibility. **Do not mark tests as passed when they were skipped, missing or never executed.**

For documentation-only or nonbehavioral changes, explain why a database run is unnecessary rather than manufacturing an integration result. The final **release candidate** still follows the repository's full live-CUBRID release gate, separate from ordinary PR path optimization. These expectations supplement, and do not weaken, the established review/CI rules.

## Running Tests

### Offline tests (the current CI contract)

```bash
cargo test --all-targets --all-features
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
```

The current `tests/` modules verify conversion, proxy forwarding and error
mapping without a live database. This repository does **not** have a
`docker-compose.yml` or a live-DB CI job today. Do not report
`cargo test` or mock proxy tests as proof of a real CUBRID transaction.
If the change needs CUBRID behavior evidence, record a separately arranged
live run, environment and limitations; adding an actual integration lane
requires its own scoped issue and reviewed PR.

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

## Repository-local issues and agent ownership

File focused changes in this repository and check the latest code, duplicates
and open PRs before implementing. New issues start with `status: needs triage`;
maintainers verify evidence, type, scope and priority relative to all local
open issues. Apply exactly one `priority: critical/high/medium/low` and one
`size: XS/S/M/L/XL`; remove the status only after confirming saved labels
and the review outcome. A priority label is not a merge decision.

Outside human contributors can work on unclaimed issues without a required
AI reviewer or prior assignment. Maintainer-operated autonomous agents require
explicit authorization and a confirmed assignee for reserved work. Preserve
proxy transaction/error behavior, typed value conversions and SeaORM API
compatibility; run the actual Rust CI commands rather than introducing a
tool-specific Oracle or Copilot gate.

## Reporting Issues

When filing an issue, include:

- Rust version
- CUBRID server version
- Minimal reproduction snippet
- Full error output
