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
