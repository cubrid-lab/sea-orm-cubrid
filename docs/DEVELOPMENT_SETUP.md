# Development setup — Rust SeaORM CUBRID adapter

> **Status:** Reconciled to inspected `Cargo.toml`, CI and README. **Not** a statement that the build, DB connection or crate publish succeeded. [ADR 0005](adr/0005-development-setup.md) is Proposed.

## Toolchain and dependency source

Use Rust **1.88 or later**, the crate's declared `rust-version`; this is **not** interchangeable with the `cubrid-rs` workspace's Rust 1.75 claim. SeaORM `1.x` with `proxy`, SeaQuery `0.x`, and `cubrid-tokio`/`cubrid-protocol` `0.1` are current manifest declarations, **not** a proven compatibility matrix. Root `Cargo.toml` contains `[patch.crates-io]` references to GitHub `cubrid-labs/cubrid-rs`; these overrides may be unavailable to an external registry consumer. Do not silently rewrite them or claim publishing is ready.

```sh
git clone https://github.com/cubrid-lab/sea-orm-cubrid.git
cd sea-orm-cubrid
rustc --version
cargo --version
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

If Cargo dependency fetch fails, capture the exact Git URL and resolution problem and track [#1](https://github.com/cubrid-lab/sea-orm-cubrid/issues/1); avoid silently substituting random local paths.

## Integration and consumer setup are blocked

Current CI runs formatting, clippy, offline tests and coverage. The repo has **no checked-in Docker Compose or real CUBRID integration lane**. Mocked `ProxyDatabaseTrait`/type tests do not establish actual CUBRID-generated SQL, rollback, generated keys or transaction failure handling. Set up a disposable broker and add genuine SeaORM SQL execution tests as a separate reviewed change in [#31](https://github.com/cubrid-lab/sea-orm-cubrid/issues/31).

The current adapter invokes `DbBackend::MySql`, discards transaction errors and returns `last_insert_id: 0`. Validate the upstream SeaORM/SeaQuery extension and error contract ([#28](https://github.com/cubrid-lab/sea-orm-cubrid/issues/28), [#32](https://github.com/cubrid-lab/sea-orm-cubrid/issues/32), [#33](https://github.com/cubrid-lab/sea-orm-cubrid/issues/33)) before claiming a native backend. A clean external Cargo consumer without root-workspace patches is a separate 1.0 gate.

## Setup exit criteria

- [ ] Rust 1.88+, formatter, clippy and offline tests run with observed results.
- [ ] Cargo Git override and public crate dependencies resolve in a clean consumer, without unreviewed module/repository changes.
- [ ] Real CUBRID harness and nonzero ORM-generated SQL assertions exist before live support is advertised.
- [ ] SeaORM public extension feasibility and transaction error propagation are demonstrable.
- [ ] [Architecture design PR #29](https://github.com/cubrid-lab/sea-orm-cubrid/pull/29) and [release policy](RELEASE_POLICY.md) remain prerequisites.
