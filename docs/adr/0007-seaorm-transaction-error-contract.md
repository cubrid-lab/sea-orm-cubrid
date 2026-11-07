# ADR 0007: Error-bearing SeaORM transaction contract before native support

- **Status:** Proposed — feasibility blocker, **not resolved**
- **Date:** 2026-11-08
- **Scope:** SeaORM CUBRID adapter transaction begin/commit/rollback semantics
- **Issues:** [#28](https://github.com/cubrid-lab/sea-orm-cubrid/issues/28), [#32](https://github.com/cubrid-lab/sea-orm-cubrid/issues/32), [#31](https://github.com/cubrid-lab/sea-orm-cubrid/issues/31)

## Verified upstream interface limitation

In the supported `sea-orm = "1"` family, the upstream
[SeaORM `1.1.x` proxy trait](https://github.com/SeaQL/sea-orm/blob/1.1.x/src/database/proxy.rs)
declares:

```rust
async fn begin(&self) {}
async fn commit(&self) {}
async fn rollback(&self) {}
```

The upstream [current `master` trait](https://github.com/SeaQL/sea-orm/blob/master/src/database/proxy.rs)
also returns `()` for these three methods (inspected on 2026-11-08).
By contrast, `query`, `execute`, and `ping` can return `Result<_, DbErr>`.

Our [`src/proxy.rs`](../../src/proxy.rs) calls an error-bearing
`cubrid_tokio` client, but uses `let _ = ...` in begin/commit/rollback,
discarding failures. **It is not safe or honest to say these operations
succeed because the proxy callbacks return normally.** The upstream trait
cannot transmit `DbErr` as part of those callbacks. A real CUBRID fault
injection test has not yet established server-specific behavior.

## Design decision to be resolved before implementation

**Do not certify transactional SeaORM `DatabaseConnection` operations
until errors can reach the public transaction caller on the same operation.**
The following are alternative solution paths to investigate in parallel;
none is approved merely by writing this ADR.

1. **Upstream trait change**: propose error-bearing `begin`, `commit`,
   `rollback` with `Result<(), DbErr>`, plus downstream changes to every
   proxy implementation and error propagation. This may require a new
   compatible upstream major version; confirm the real supported release and
   migration path before adopting it.
2. **Independent native SeaORM backend extension**: prove an official
   public extension point that can implement transaction methods returning
   `Result` while generating CUBRID SQL directly, without pretending the
   `DbBackend::MySql` compiler or proxy callbacks can do so. Require a
   compile-only feasibility spike **before** broad implementation.
3. **Narrow the advertised API** while blocked: explicitly mark SeaORM
   transaction operations as unverified/unsafe for release and avoid exposing
   claims of transaction success. A new fallible convenience method on
   `CubridProxy` **alone** does not repair a public SeaORM
   `DatabaseConnection::begin()/commit()/rollback()` call.

A failure in the transaction-start command itself must also be handled:
the current `execute("BEGIN", &[])` is **not** established as a valid
transaction-start mechanism for every advertised CUBRID version. CUBRID's
autocommit/session transaction semantics need a separate live-DB contract.

## Rejected substitutes

- **Logging and continuing:** caller still sees success.
- **Panic in the proxy callbacks:** violates the library's fallible Rust API
  and can abort an application.
- **Sticky last-error returned only on a later query:** a failed commit may
  be the last operation, so the caller can already report a false success.
- **Retrying commit or SQL after a transport error:** ambiguous execution can
  duplicate work.
- **An unrelated `query()`/mock success:** proves neither the public
  transaction result nor the server's transaction status.

## TDD and verification plan

- [ ] Record a RED fake-client case for each of begin/commit/rollback failure
  `cubrid_tokio::Error::ConnectionClosed`; assert that the **public
  SeaORM transaction operation** rejects instead of returning success.
  These tests may remain in an **unmerged draft spike** until the upstream
  extension is proven; never conceal them through `#[ignore]` or reduce CI.
- [ ] Add a public API compile probe that selects one of the viable solutions,
  including `Send`/`Sync`, connection ownership and rollback on failure.
- [ ] Define transaction state: no concurrent borrow of a failed CAS,
  no autocommit after ambiguous commit/rollback, and no swallowed primary
  error even when cleanup fails.
- [ ] On an isolated **real CUBRID** broker verify actual begin/commit/
  rollback SQL and driver operations, injected connection loss, result
  visibility across sessions and the exact server/driver/SeaORM versions.
- [ ] Verify the selected package using a **clean Cargo consumer** without
  workspace-only `[patch.crates-io]` overrides and with a supported Rust
  toolchain.
- [ ] Remove the `DbBackend::MySql` SQL-generation dependency or narrow
  features honestly before claiming **native** CUBRID ORM semantics.
- [ ] Obtain maintainer review of the selected extension and migration strategy
  and only then change production `src/proxy.rs`.

## Consequences

This is a release-level blocker, not a justification to remove existing
functionality without migration. Current mocked transaction tests merely
show delegation and can pass when a real commit failed. Safe work can proceed
**in parallel**: upstream API feasibility, fake-client RED cases, real broker
harness, Cargo packaging and CUBRID DDL/SQL design. The final public SeaORM
transaction contract is the only hard integration dependency.
