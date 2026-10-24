# AGENTS.md

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

## Issue triage lifecycle

Start **every new issue** with `status: needs triage` (also when an agent
pre-populates priority and size). The issue-opened workflow marks incoming
reports; `GITHUB_TOKEN` automation must include the label at creation because
its events do not trigger other workflows. Reporters are not required to
manage labels.

The maintainer/triager performs the following before declaring completion:

1. Check whether the reported problem is valid, already resolved or duplicated;
   inspect existing issues and PRs, evidence, local repository scope, issue
   type, dependencies, non-goals and acceptance criteria. Explain duplicate
   or invalid closures.
2. Compare the candidate's urgency with **all open issues in this repository**;
   reconsider existing relative priorities. Choose exactly one
   `priority: critical`, `priority: high`, `priority: medium` or
   `priority: low`, based on impact rather than creation order.
3. Assign exactly one `size: small` (focused), `size: medium` (contained
   multi-file), or `size: large` (cross-cutting), splitting unreviewably
   large implementation work and retaining parent acceptance.
4. Re-fetch the GitHub issue labels and confirm the type, one canonical
   priority and size, full-backlog comparison and any remaining blocker.
   **Only then** remove `status: needs triage`. Metadata alone never
   automatically completes triage.

For historical open issues, XS/S → small; M → medium; L/XL → large.
Preserve priority, assignees, type/area labels, discussions, closed issues
and existing validation/security requirements. Do not introduce a new
required PR/CI gate.

