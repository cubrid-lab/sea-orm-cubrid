## Summary

Describe the user-visible behavior, CUBRID or SeaORM constraint, and the change.

## Related issues

Link issues, or state N/A.

## Tests and evidence

State the actual commands and results. **Offline tests are not proof of a live CUBRID transaction.** Include supported CUBRID/Rust/SeaORM/cubrid-tokio versions and pass/fail/skip counts for real DB tests, or explicitly state why live validation is not available.

## CUBRID correctness and regression evidence

<!-- Review docs/ENGINEERING_INVARIANTS.md; check only relevant items, with N/A reason for unrelated documentation changes. -->

- [ ] I reviewed whether generated SQL depends on `DbBackend::MySql`; any remaining dependency is disclosed, not described as a native CUBRID dialect.
- [ ] Transaction begin/commit/rollback failures are propagated or limitations are explicitly documented. No invented successful operation or generated insert ID is reported.
- [ ] Affected async connection/transaction ownership and cancellation behavior is checked, including driver and CUBRID error paths.
- [ ] Precision/NULL/collection values, column alias handling, and migration correctness are covered where applicable.
- [ ] I distinguished live CUBRID evidence from mocked tests and reported the actual tested versions and skipped/failed cases.
- [ ] I reviewed crate SemVer, MSRV, dependency resolution, documentation and release-note impacts.

## Validation status

- `cargo fmt --all --check`:
- `cargo clippy --all-targets --all-features -- -D warnings`:
- `cargo test --all-targets --all-features`:
- Real CUBRID integration (if applicable):
- Packaging/external Cargo consumer (if applicable):

## Test-first evidence and design approval

<!-- Applicable to behavioral implementation. For docs-only or design feasibility work, write N/A and the actual evidence. See docs/adr/0004-test-first-development.md and docs/DESIGN_SIGNOFF.md (in the design PR where applicable). -->

- [ ] **Design:** The public behavior/SQL/transaction/error contract and owning layer were reviewed before implementation (link ADR/issue or explain N/A).
- [ ] **Red:** A targeted test failed for the expected **behavioral** reason; summarize the observed failure or state why test-first did not apply.
- [ ] **Green:** The smallest implementation passes the new test; name the exact test command and outcome.
- [ ] **Refactor:** Related regression tests and required final-head checks pass, with observed CI links when available.
- [ ] **Live CUBRID:** For behavior depending on the server, distinguish real DB tests from mocks; include exact server/runtime/driver versions and passed/failed/skipped cases, or explicitly mark the behavior unverified.
- [ ] **Release:** No version/support claim or publication is inferred from a green mock or skipped integration job; final release candidates follow the separately reviewed full DB gate.

## Development environment and dependency verification

<!-- For a behavior/CI/dependency change, give observed results. For docs-only or unrelated work, state N/A with a reason. See docs/DEVELOPMENT_SETUP.md and ADR 0005. -->

- [ ] Clean checkout and native dependency install resolve without unexpected manifest/lockfile changes; report runtime/compiler version and result, or explain why not run.
- [ ] Do not confuse local build/offline tests with actual live CUBRID SQL execution. Record the database/version and count of executed integration cases when claiming that behavior.
- [ ] Public package engines, Go module path, Cargo MSRV/features and versioned dependency constraints are reviewed if relevant; no silent compatibility-breaking setup change.
- [ ] Required final-head CI and release checks remain in place; missing/failed/skipped checks are reported truthfully and a docs-only shortcut is **not** a release verification.
