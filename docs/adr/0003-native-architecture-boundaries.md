# ADR 0003: Native CUBRID architecture boundaries

- **Status:** Proposed
- **Date:** 2026-11-07
- **Prerequisite:** Approval and merge of this repository's release policy ADR in its earlier release-policy PR
- **Decision scope:** Rust SeaORM/SeaQuery native SQL and error propagation

## Context

The current SeaORM proxy delegates SQL execution to cubrid-tokio, but `Database::connect_proxy(DbBackend::MySql, ...)` produces MySQL SQL, not CUBRID-native SQL. The proxy implementation drops begin/commit/rollback errors and substitutes last_insert_id=0. The available public extension traits may constrain independent SQL construction and error reporting.

The CUBRID Python driver and SQLAlchemy dialect provide engineering lessons about protocol, SQL and transaction correctness, **not** implementations or cross-language public APIs to copy.

## Decision

Target a CUBRID SQL backend and result/transaction adapter that uses Rust and SeaORM idioms, not MySQL SQL generation as a permanent runtime dependency. Before implementation, prototype and verify SeaORM/SeaQuery **public** extension APIs. If native compilation or transaction error propagation cannot be represented safely, document a required upstream API change or limit the supported feature set; do not mask failure or fabricated IDs. cubrid-tokio remains responsible for transport, while the ORM owns SQL, type mapping and schema/migration semantics.

## Alternatives considered

Treat ProxyDatabaseTrait alone as a native SQL compiler (rejected); rewrite MySQL SQL strings (rejected); log but discard failed transactions (rejected); return zero as though CUBRID generated it (rejected); immediately fork SeaORM without feasibility comparison (deferred).

## Consequences and trade-offs

May require upstream changes, a narrower 0.x API and postponed 1.0 release. Improves correctness of transaction and data outcomes and prevents false native compatibility claims.

## Verification / acceptance criteria

Compile and execute a typed query without DbBackend::MySql, capture errors for begin/commit/rollback, verify CUBRID generated-key behavior, implement safe Tokio connection isolation, test real DB and external Cargo consumer resolution. Keep the existing 0.x behavior and migration path explicit.

- [ ] Approve release/compatibility policy ADR 0001 **before** accepting this architecture ADR.
- [ ] The feasibility evidence and exact supported versions are recorded; unknown or unsupported behavior is not reported as implemented.
- [ ] Separate implementation issues, regression tests and release documentation agree with this decision.
- [ ] A maintainer reviews the decision and records acceptance explicitly; draft status alone never authorizes 1.0.

## References

[native SeaORM design](../CUBRID_NATIVE_SEAORM_DESIGN.md), [architecture #28](https://github.com/cubrid-lab/sea-orm-cubrid/issues/28), [transaction bug #32](https://github.com/cubrid-lab/sea-orm-cubrid/issues/32), [generated ID #33](https://github.com/cubrid-lab/sea-orm-cubrid/issues/33), [release policy PR #30](https://github.com/cubrid-lab/sea-orm-cubrid/pull/30)

**This ADR is an architecture proposal, not a runtime implementation, migration completion, CI pass or approved release.**
