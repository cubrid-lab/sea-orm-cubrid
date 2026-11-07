# Native CUBRID SeaORM integration — proposed architecture

**Status:** Feasibility and design proposal (2026-11-07). Current production behavior is **not** represented by this document. Parent issue [#28](https://github.com/cubrid-lab/sea-orm-cubrid/issues/28).

## Decision

The goal is a **Rust-idiomatic CUBRID backend** that makes correct CUBRID SQL and session/result behavior available to SeaORM users. MySQL SQL generation and MySQL's dialect assumptions are **not** a supported runtime mode for the target implementation.

As `pycubrid` is separate from `sqlalchemy-cubrid`, this crate should be separate from `cubrid-rs`: the driver implements CAS/connection APIs, the ORM layer owns query generation, type conversions, ORM results, schema and migration behavior. That shared idea does **not** mean reimplementing SQLAlchemy classes or copying Python's exception hierarchy in Rust.

## Current evidence and blockers

- `src/lib.rs` invokes `Database::connect_proxy(DbBackend::MySql, proxy)`. `ProxyDatabaseTrait` supplies execution hooks, not a native SQL compiler; the actual SQL is still assembled for MySQL.
- `src/proxy.rs` converts query rows and numeric values, but reports `last_insert_id: 0`. Zero is not proof of a server-generated ID.
- `ProxyDatabaseTrait` provides `begin/commit/rollback` callbacks without ordinary `Result` return values in the currently used contract. The implementation discards failures. This is a design blocker: the target must not report an unsuccessful transaction completion as success.
- `tokio::sync::Mutex` wraps one shared client; this requires careful review of transaction isolation, await cancellation and concurrency, rather than inferring thread safety from the mutex alone.
- README still describes a sync/`spawn_blocking` bridge although the checked-in `connect()` uses `cubrid-tokio::Client`.

## Proposed extension architecture

```text
SeaORM Entities / queries
         |
SeaORM + SeaQuery public extension points (upstream feasibility review)
         |
Native CUBRID SQL generation and query metadata (NOT DbBackend::MySql)
         |
CUBRID SeaORM session/proxy adapter
         |
cubrid-tokio / cubrid-protocol (Rust driver family)
         |
Broker / CAS / CUBRID
```

The exact implementation mechanism **is not yet approved**. Verify public SeaORM and SeaQuery extension mechanisms at the versions actually depended upon. Possible paths for comparative evaluation:

1. Contribute first-class CUBRID SQL backend support upstream to SeaQuery/SeaORM.
2. Use a documented supported public extension strategy if it supports both query construction **and** execution without a fake `DbBackend::MySql`.
3. Build an isolated CUBRID query adapter if upstream hooks are unavailable, with explicit feature scope and type-level interoperability boundaries.

Do **not** treat string rewriting of generated MySQL SQL as an independent compiler, nor silently emulate missing PostgreSQL/MySQL clauses on CUBRID.

## Rust/SeaORM-specific contracts

1. **Compile:** CUBRID identifiers, placeholders, SELECT/JOIN/relations, DML/defaults, update/delete, supported upsert, generated-key behavior and DDL from verifiable CUBRID rules.
2. **Execute:** use `cubrid-tokio` async execution with explicit error propagation, no blocking bridge when not required, safe ownership of a pooled connection across `Transaction`.
3. **Map:** preserve `Option<T>` nullability, 64-bit integers, precise numerics, timestamp semantics, native collections, bytes/LOBs and the intended SeaQuery `Value` mapping. Return explicit conversion errors rather than truncated or guessed values.
4. **Transactions:** resolve errors from start, commit, rollback and abandoned futures. If a trait cannot express those errors, require an upstream fix or alternative API path before promising transactional correctness.
5. **Concurrency:** decide whether a connection can serve concurrent tasks, and when locks, pool leases or transaction guards are appropriate. Prevent accidental cross-transaction execution.
6. **Migrations:** CUBRID catalog and DDL rules, independent of MySQL generation, destructive-change safeguards and failure/recovery documentation.

## Feasibility acceptance before implementation

- [ ] Review real SeaORM/SeaQuery versions and public extension API; prototype a typed simple SELECT **without** `DbBackend::MySql`.
- [ ] Demonstrate errors from query/start/commit/rollback are observable or document required upstream changes; no silent success.
- [ ] Demonstrate safe transaction scope and cancellation handling with `cubrid-tokio`.
- [ ] Define a supported and explicitly tested matrix of CUBRID, Rust MSRV, SeaORM, SeaQuery and driver crates.
- [ ] Real CUBRID tests for basic CRUD, relations, generated keys, errors, transaction semantics and migrations before 1.0 claims.
- [ ] Document feature limitations and an explicit staged migration path from current 0.x API.

## Versioning

Coordinate **tested major compatibility generations** with the `cubrid-rs` driver family but apply Cargo SemVer independently for this crate; minor and patch versions are never mechanically synchronized. The current `0.1.0` contract must not be labeled 1.0 before feasibility and integration evidence. Review published crates and the existing `[patch.crates-io]` configuration before making package changes.

Related: [version policy #23](https://github.com/cubrid-lab/sea-orm-cubrid/issues/23), [MSRV #26](https://github.com/cubrid-lab/sea-orm-cubrid/issues/26), [crate patching #1](https://github.com/cubrid-lab/sea-orm-cubrid/issues/1), [cubrid-rs](https://github.com/cubrid-lab/cubrid-rs).

**No runtime code, Cargo files, releases or currently advertised support are changed by this design-only proposal.**
