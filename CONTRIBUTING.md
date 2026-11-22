# Contributing to sea-orm-cubrid

For issues, use clear type-only titles (`feat:`, `bug:`, etc.) without module scope and state observable acceptance criteria. PRs and commits use the repository's Conventional Commits/CI rules. Read [AGENTS.md](AGENTS.md) for this repository's review guidance.

Thank you for your interest in contributing to `sea-orm-cubrid`.


## Share improvement ideas and recognize problem discovery

Noticed confusing documentation, onboarding friction, a missing test,
unexpected behavior, or an improvement while using this project? File an
**Improvement Idea** through [New issue](https://github.com/cubrid-lab/sea-orm-cubrid/issues/new/choose).
A short observed problem is enough: you do **not** need to know the cause,
design the fix or provide full acceptance criteria to suggest it. Maintainers
triage evidence, duplicates, type, priority and scope before authorizing work.

Check for related issues when practical; keep credentials, sensitive logs,
user data and undisclosed vulnerabilities out of public reports. Proposals
do not bypass the project's existing code ownership, safety, CI or release
rules. A small direct PR needs no additional issue merely for ceremony if
the repository already permits it; check ownership and open PRs first.

When an issue leads to a PR, link the origin and acknowledge a **verified
reporter as a reporter**, distinct from verified code author/co-author
credits. Where the release-note style permits it, use separate
`reported by @handle` wording when useful. Never fabricate reporter
identity or dilute project release-note provenance requirements.

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

## PR review outcomes

Use the same decision labels for every PR, whether human-authored or AI-assisted:

- **`APPROVE`** a reviewed, non-blocking change; optional suggestions can accompany approval instead of delaying it.
- **`REQUEST_CHANGES`** for an actionable must-fix defect, with the affected code and required correction clearly identified.
- **`COMMENT`** when asking a question or when a review is not yet decisive. The author cannot approve their own PR; never imply an AI review is an independent human approval.
- Code approval and GitHub Actions **run approval** are separate. `action_required`, pending, failed or stale required CI checks do not permit merge, even if a code review is approved. Confirm final-head CI, resolved blocking findings, applicable independent/security/integration requirements and the maintainer's merge decision before merging.

See [AGENTS.md — PR review outcomes](AGENTS.md#pr-review-outcomes-common-decision-rules). Existing repository-specific branch, release, security and required-review safeguards continue to apply.

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
5. Title the PR as a Conventional Commit, `type: description` or
   `type(scope): description` (`feat!:` for a breaking change);
   `.github/workflows/pr-title.yml` checks it. PRs are squash-merged with the
   PR title as the commit subject on `main` ([required repository
   settings](docs/RELEASE_PROCESS.md#repository-settings)), and Release Please
   uses that subject to choose the next version. A PR made with GitHub's
   **Revert** button is titled `Revert "..."`; retitle it `revert: ...`.
6. Do **not** edit `CHANGELOG.md`. Release Please generates it in the Release
   PR; describe user-visible changes in the PR description instead. See
   [the release process](docs/RELEASE_PROCESS.md).

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


## Issue handoff and development progress

Use the issue and PR's actual state as evidence. Classification, authorization,
implementation, review and completion are **different decisions**:

1. **Intake / triage:** A newly filed issue starts with `status: needs triage`.
   Check evidence or reproduction, duplicate open/closed issues and overlapping
   PRs, repository scope, type, priority, size, dependencies, and verifiable
   acceptance criteria. A maintainer removes the intake label only after
   confirming the saved GitHub metadata. Do not infer priority, size, or triage
   completion automatically from the title or an AI suggestion.
2. **Ready:** Triage completion alone does not authorize development. Record
   the Ready decision in the issue only when the intended outcome and non-goals are bounded, blockers
   have been resolved, and any repository-specific product, architecture,
   safety or compatibility decisions have been accepted. Unresolved research
   remains investigation; blocked work records its dependency and next action.
3. **In Progress:** Before maintainer-directed or autonomous implementation,
   identify the actual implementer in the GitHub Assignee field, check whether
   another open PR already owns the change, and link the implementation PR to
   its issue. Human-led outside contributors may propose a change on an
   unclaimed issue without prior assignment or an AI-tool-specific approval;
   coordinate with existing assignees and follow the local contribution rules.
4. **In Review:** A PR is in review only when its scoped change and verification
   evidence are available. Check the required CI on the final PR head, test
   coverage appropriate to the risk, unresolved review findings and any
   repository-specific independent review or live-system validation. A bot
   review or passing CI cannot replace an outstanding product/safety decision.
5. **Done:** Record the actual disposition: a merged and validated change,
   a completed investigation with evidence, or an explained duplicate/not-
   planned closure. Use an issue-closing PR keyword only if merging actually
   satisfies its acceptance criteria. A merged PR is not proof that any
   deferred integration, fault, security or release verification has passed.

**Tracking without boards:** GitHub Issues and pull requests are the only
workflow tracker. The sole lifecycle label is `status: needs triage` for
unreviewed intake. Once triage is completed, remove that label after checking
the actual saved issue metadata; do not create `status: ready`, `status: in
progress`, or other duplicate workflow labels. Record the **Ready** decision,
remaining dependencies, and next action in the issue. For **In Progress**,
identify the actual implementer using Assignees and add a concise start/handoff
comment; link the working PR when it exists. **In Review** is visible from the
linked open, non-draft PR and its actual review/check status. A **Blocked**
issue records the blocker, responsible party, and unblock condition in a
comment. **Done** is the issue's Closed state with the recorded validated
outcome or explained disposition, not merely the existence of a merged PR.
Do not configure or depend on GitHub Projects/boards. Priority measures impact,
and size estimates effort; neither represents work-in-progress.
This guidance introduces no new automated merge gate and does not supersede
this repository's design, testing, security, or release requirements.
