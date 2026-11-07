# ADR 0002: CUBRID correctness and failure-evidence boundaries

- **Status:** Proposed
- **Date:** 2026-11-07
- **Scope:** sea-orm-cubrid correctness obligations and review evidence

## Context

sea-orm-cubrid owns the Rust SeaORM/SeaQuery CUBRID adapter. This project's correctness decisions must cover SeaORM/SeaQuery extension feasibility, Tokio driver integration, SQL generation, error and value mapping, Cargo/MSRV, and actual CUBRID tests, including negative and failure-path behavior. Passing offline tests alone is not proof of database compatibility. The accepted public API, local implementation, and actual CUBRID server behavior are the basis for requirements and test evidence.

## Decision

SeaORM owns a native CUBRID SQL compilation decision and ORM results; cubrid-tokio owns transport. The currently used DbBackend::MySql proxy is **not** a CUBRID SQL compiler. A zero last_insert_id is not a verified server-generated key, and begin/commit/rollback errors cannot be silently discarded. Investigate actual upstream ProxyDatabaseTrait/SeaQuery extension possibilities, use async Rust transaction-bound connection ownership, preserve values and error results, and require real CUBRID SQL+DDL tests before stable claims.

Create targeted tests for this project's observable contracts. Record whether tests actually communicated with CUBRID, the server/broker and dependency versions, asserted SQL outcomes, and skipped cases. External test results never certify this package.

## Alternatives considered

- **Rejected:** Continue MySQL SQL as a permanent native proxy; silently accept begin/commit/rollback errors; invent generated ID=0; trust offline converter tests or a coverage badge as real DB validation.
- **Rejected:** Treating another package's API, implementation, or test results as this project's contract.
- **Rejected:** Marking an unsupported/unverified feature as absent simply because one server or connector failed in a particular test.

## Consequences and trade-offs

May require upstream SeaORM changes or reduced feature scope, with delayed 1.0 readiness; avoids unsupported transaction or schema guarantees. Ordinary documentation-only PRs need not run an expensive database matrix; **the final release candidate still must run all required real-CUBRID checks** for its promised compatibility.

## Verification / acceptance criteria

- [ ] Normal and failure-path tests cover the actual decision's affected behavior.
- [ ] No silent statement replay, fabricated database values, unknown-success transaction or stale-handle reuse for the supported public API.
- [ ] Real DB tests use an isolated ready-checked CUBRID instance, assert actual stored/returned values and report nonzero executed-case counts; skip/missing is not success.
- [ ] Independent JDBC (or verified official driver) comparisons evaluate meaning and type, not matching source implementation.
- [ ] Public docs, type declarations and compatibility claims are reconciled with observed behavior; unsupported operations fail clearly.
- [ ] Maintainer accepts this ADR and separately reviews release/code changes; neither this record nor a checklist fixes existing defects.

## References

[engineering checks](../ENGINEERING_INVARIANTS.md), [SeaORM native design #28](https://github.com/cubrid-lab/sea-orm-cubrid/issues/28), [transaction bug #32](https://github.com/cubrid-lab/sea-orm-cubrid/issues/32), [generated IDs #33](https://github.com/cubrid-lab/sea-orm-cubrid/issues/33)

**No claim of implementation completion is made.**
