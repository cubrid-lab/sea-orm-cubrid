# Engineering invariants for sea-orm-cubrid

> **Status:** Proposed repository-specific engineering checklist, not evidence that existing tests pass.

sea-orm-cubrid owns the Rust SeaORM/SeaQuery CUBRID adapter. Correctness requirements follow SeaORM/SeaQuery extension feasibility, Tokio driver integration, SQL generation, error and value mapping, Cargo/MSRV, and actual CUBRID tests. Use this repository's implementation, public API, relevant live-CUBRID evidence and official CUBRID/JDBC results as appropriate; never assume that another language's driver or ORM test result establishes behavior here.

## Evidence and adoption method

The Python projects' reusable contribution is **how correctness was established**, not a shared API: model physical CAS lifecycle, test uncertain SQL execution without replay, invalidate stale handles, verify transaction rollback, preserve type fidelity, compare semantic outcomes with an independent official driver/JDBC, refuse vacuous live-test passes, and publish only after final candidate checks.

Sources: [CAS reconnect/session tests](https://github.com/cubrid-lab/pycubrid/blob/main/tests/test_cas_session_persistence.py), [cursor handle ownership](https://github.com/cubrid-lab/pycubrid/blob/main/tests/test_autocommit_handle_ownership.py), [live state machine](https://github.com/cubrid-lab/pycubrid/blob/main/tests/test_connection_state_machine.py), [official driver differential](https://github.com/cubrid-lab/pycubrid/blob/main/tests/test_official_differential.py), [SQLAlchemy CUBRID architectural boundaries](https://github.com/cubrid-lab/sqlalchemy-cubrid/blob/main/docs/ARCHITECTURE.md), [DDL rollback tests](https://github.com/cubrid-lab/sqlalchemy-cubrid/blob/main/test/test_transactional_ddl.py), [integration gate tests](https://github.com/cubrid-lab/sqlalchemy-cubrid/blob/main/test/test_integration_gate.py), [release note style](https://github.com/cubrid-lab/sqlalchemy-cubrid/blob/main/docs/RELEASE_NOTES_STYLE.md).

Do **not** infer CUBRID support from syntactic similarity with MySQL. In particular, CUBRID DDL rollback is exercised by live CUBRID results for the specified database version; do not state that CUBRID always auto-commits DDL without version-and-transaction-specific real evidence.

## SeaORM/SeaQuery Rust-specific invariants

1. **SQL is built by the ORM, not by the proxy:** `Database::connect_proxy(DbBackend::MySql, ...)` still generates MySQL SQL. `ProxyDatabaseTrait` only delegates execution. A future independent CUBRID SQL dialect must prove actual SeaORM/SeaQuery public extension support or a reviewed upstream solution; do not string-rewrite MySQL SQL as a substitute.
2. **No fake successful transactions:** `src/proxy.rs` currently ignores errors returned by the driver's begin/commit/rollback calls. Its upstream proxy trait may lack error return values: treat this as a feasibility blocker, not proof of support. Document an upstream fix or a different transaction contract before advertising 1.0.
3. **No invented results:** current `last_insert_id: 0` is not a database-supplied generated key. Unsupported or unavailable generated-ID semantics must be explicit. Columns with duplicate names can collapse when stored in a map; ensure aliases and ordering remain correct.
4. **Ownership and cancellation:** `Arc<tokio::sync::Mutex<_>>` serializes one client, not independent transaction isolation. A transaction must own its physical connection until completion; Tokio cancellation/failure must not silently leak work or allow a second task into its transaction.
5. **Data fidelity:** preserve SeaQuery Value and CUBRID `Option`/NULL, i64, precise numeric and collection semantics. Fail unsupported conversions clearly rather than returning a default; verify in live CUBRID.
6. **DDL and migrations:** test catalog reflection, transactional DDL rollback, supported schema changes and any migration claims against CUBRID. Never infer MySQL DDL compatibility.
7. **Truthful proof and packaging:** the current `cargo test` suite is offline. A passing coverage job cannot justify live DB claims. Verify a clean Cargo consumer without `[patch.crates-io]` and the exact Rust/SeaORM/driver version matrix.

## Implementation and release checklist

- [ ] Demonstrate a CUBRID-native SeaORM query generated without `DbBackend::MySql` before claiming native backend support.
- [ ] Prove transaction begin/commit/rollback failures propagate through the **actual public SeaORM API**; blocked if upstream trait cannot represent it.
- [ ] Test real CRUD, join, generated-key, null/precision/collection and rollback behavior on supported DB versions using `cubrid-tokio`.
- [ ] Assert nonzero real SQL cases in release candidate CI, with actual CUBRID/driver/SeaORM version and no skipped required checks.
- [ ] Keep published support claims, crate MSRV, patch overrides and release notes accurate.

Tracked: [native integration #28](https://github.com/cubrid-lab/sea-orm-cubrid/issues/28), [live DB tests #31](https://github.com/cubrid-lab/sea-orm-cubrid/issues/31), [coverage claims #27](https://github.com/cubrid-lab/sea-orm-cubrid/issues/27), [publication #1](https://github.com/cubrid-lab/sea-orm-cubrid/issues/1). Follow [release policy](RELEASE_POLICY.md) before [architecture draft PR #29](https://github.com/cubrid-lab/sea-orm-cubrid/pull/29).
