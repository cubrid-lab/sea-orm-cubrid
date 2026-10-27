# AGENTS.md

## Organization-wide issue and PR standards

See [the cubrid-lab standard](https://github.com/cubrid-lab/.github/blob/main/docs/ISSUE_PR_COMMIT_STANDARD.md). Use clear type-only **Issue** titles (`feat:`, `bug:`, `docs:`), no required module scope. State problem/goal and verifiable completion conditions, distinguishing evidence from assumptions. Triage establishes validity, duplicates, priority and size; **PRs and commits** keep their existing Conventional Commits and CI checks. Avoid AI-agent narration, personal references and opaque internal tracking codes in issue prose.


Project knowledge base for AI coding agents.

## Project Overview

`sea-orm-cubrid` provides a SeaORM backend for CUBRID through SeaORM's `ProxyDatabaseTrait` extension point.

- Language: Rust 2021
- Runtime: tokio
- ORM: SeaORM 1.x (proxy mode)
- Driver: cubrid-tokio
- Protocol model: cubrid-protocol

## Architecture

The crate follows SeaORM's proxy pattern:

1. SeaORM builds SQL with `DbBackend::MySql`.
2. `CubridProxy` receives `Statement` and parameters.
3. Conversion layer maps SeaQuery values to CUBRID values.
4. `cubrid-tokio` executes SQL and returns rows.
5. Results are converted into `ProxyRow` for SeaORM.

Key characteristics:

- No SeaORM fork.
- No custom SQL builder.
- Async-safe shared client via `Arc<tokio::sync::Mutex<_>>`.

## Module Responsibilities

- `src/lib.rs`: public API, `connect()`, module exports
- `src/proxy.rs`: `CubridProxy`, `ProxyDatabaseTrait` implementation, statement handling
- `src/convert.rs`: value conversion (`sea_query::Value` <-> `cubrid_protocol::Value`)
- `src/error.rs`: error conversion (`cubrid_tokio::Error` -> `sea_orm::DbErr`)
- `tests/`: offline behavior and conversion coverage

## Code Conventions

- Rust 2021 edition
- `#![deny(unsafe_code)]`
- Formatting: rustfmt, 100 char max line width
- Lints: clippy with warnings denied in CI
- Avoid `unwrap`/`expect` in library code
- Keep public API documented

## Development Workflow (cubrid-lab org standard)

Apply **identical PR review and merge criteria** regardless of whether
code was written by a person, OpenCode, Codex, Claude Code, ChatGPT,
Copilot or any other agent. No specific AI review is a prerequisite.

1. Implement a focused change with tests and affected documentation.
2. Required GitHub Actions CI checks, appropriate to this repository,
   are the authoritative verification at the final head. Local testing
   is recommended but not an additional required gate.
3. Review correctness, tests, security, API compatibility, maintainability,
   and scope. Oracle, Copilot and `agent-review` findings are optional,
   **advisory**, and never automatic approval or rejection.
4. Disposition outstanding findings and let the responsible maintainer
   decide on merge after checking required CI. Security, CI permissions,
   release and publication changes need explicit independent review.

Preserve existing branch protection, integration tests, compatibility and
release safeguards. Do not create tool-dependent Oracle/Copilot routes.

## Test Structure

- `tests/convert_tests.rs`: all important conversion paths
- `tests/proxy_tests.rs`: query/execute/transaction/ping delegation and row mapping
- `tests/error_tests.rs`: driver error to `DbErr` mapping
- `tests/connect_tests.rs`: connection helper behavior via mock factory and invalid DSN path

All tests are offline and deterministic.
