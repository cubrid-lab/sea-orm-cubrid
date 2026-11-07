# Design sign-off and TDD acceptance plan

> **Status: Not signed off / design under review.** This plan captures *expected* test behavior, not tests already written or run. The repository release policy and ADR 0001/0002/0004 are proposed in the earlier policy PR. The native architecture ADR 0003 (for this repository, if applicable) is proposed in this design draft. No implementation, CI success, approved public API or live DB outcome is implied.

## Project setup prerequisite

**ADR 0005 and the development setup guide** are proposed in the earlier release-policy PR; check [the setup documentation](https://github.com/cubrid-lab/sea-orm-cubrid/blob/docs/release-policy-before-native-design/docs/DEVELOPMENT_SETUP.md) and the manifest/CI before checking these boxes.

- [ ] **Clean checkout verified:** language-native runtime/compiler and tooling are selected with a documented developer versus published consumer support distinction.
- [ ] **Dependency resolution verified:** exact lockfile/Go module/Cargo graph installs without undocumented replacement, incompatible version or incorrect import path.
- [ ] **Read-only setup checks executed:** record **actual** build/type/lint/unit results, not command examples; all known missing config/scripts have explicit blockers.
- [ ] **Real DB harness decided:** identify a disposable real CUBRID service and concrete test selection, or mark missing harness as blocked. A Docker healthcheck, fake broker or zero/fully skipped suite is not evidence.
- [ ] **External package consumer planned:** npm packed ESM/CJS/TypeScript, Go module imports or Cargo crate package resolution (as appropriate), with release preflight separate from initial setup.
- [ ] **Repository developer guidance consistent:** README, CONTRIBUTING, AGENTS, tooling and CI refer to commands that actually exist and do not silently raise the supported runtime floor.
- [ ] **Maintainer approval recorded** for ADR 0005 and any associated module-path, MSRV, dependency or CI changes.

These are **not completed** merely because the files are written. If setup is blocked, fix or explicitly scope it in a separate reviewed task before claiming the design is ready for implementation.

## Design-first gate

Before implementation changes are treated as approved, establish each of the following in reviewed project documentation and link the evidence. Check only when genuinely resolved, not merely because this template exists.

- [ ] **Boundaries and interfaces:** owned layer, public API, language-native patterns, cross-repository dependency, version and compatibility envelopes.
- [ ] **Protocol/SQL contract:** CUBRID-specific wire/query semantics, supported and unsupported behaviors, parameter/result mapping, resource lifetimes.
- [ ] **State and failure model:** connection/session/transaction/statement states, cancellation, retries, ambiguous outcomes and cleanup.
- [ ] **Data types and precision:** NULL, signed integers, exact numeric, text, date/time, collections, binary/LOB and errors.
- [ ] **Schema/migration contract (where applicable):** CUBRID DDL, introspection, reversible behavior, destructive change guardrails.
- [ ] **Security and operations:** credentials/log redaction, timeout limits, resource bounds, cancellation behavior and diagnostics.
- [ ] **Verification design:** red/green cases (below), real CUBRID versions, non-vacuous failure gate, independent JDBC semantic comparison when appropriate.
- [ ] **Release readiness:** published package/module/crate and supported consumer matrix, reviewed release notes, stable 1.0 exit conditions.
- [ ] **Upstream feasibility:** independently verify third-party ORM public extension APIs and unsupported methods before committing to architecture.
- [ ] **Review approval:** maintainers explicitly approve the language-native architecture ADR, test contract and open blockers; no speculative sign-off.

The PR/issue must distinguish **decided**, **proposed**, **blocked**, and **requires live verification**. If a significant feasibility constraint prevents the proposed direction, update the ADR and its alternatives before implementing a workaround.

## TDD acceptance cases

For each feature or bug: first run a targeted test that fails for the intended reason (**Red**), make the smallest implementation pass (**Green**), refactor under the same tests, and verify live database behavior before claiming compatibility. A red failure log may be summarized truthfully in the PR but need not be committed. The final PR must have green required checks.

| Test contract (proposed) | Expected observable outcome | First test layer | TDD starting point |
| --- | --- | --- | --- |
| Upstream extension feasibility | A typed SELECT compiles into independently generated CUBRID SQL without DbBackend::MySql | compile-only public API spike + real SQL smoke | if public hook unavailable record blocked, do not fake a green test |
| Transaction error propagation | Begin, commit and rollback failures reach actual SeaORM caller, not swallowed by proxy trait | fault-injection async tests + live rollback | red for ignored driver error, require upstream design if trait cannot return it |
| Generated metadata | Missing lastInsertID is explicit; never substitute constant zero as a server result | fake driver results + live INSERT variants | red for dummy 0 |
| Concurrency ownership | Each transaction owns its physical connection; another task cannot accidentally join or commit it | Tokio multiclient tests + live CUBRID | red cross-task visibility/rollback |
| Migrations and value fidelity | CUBRID DDL and types match SQL grammar/catalog; fake MySQL clauses are not allowed | typed SeaQuery value unit + live ORM and DDL | red SQL/NULL/decimal mismatch before fix |

## Project boundary

**Role:** Rust SeaORM/SeaQuery native SQL backend and proxy.

`cubrid-tokio` owns transport, SeaORM/SeaQuery integration owns query compilation and ORM result/transaction types. The current MySQL-backed compiler and proxy trait are not proof of native CUBRID feasibility.

## Decisions still blocking full sign-off

#28 upstream SeaORM/SeaQuery extension decision, #32 ignored tx errors, #33 constant insert ID, #31 live database suite, #1 crates.io publish patches. Do not mark design complete until the upstream trait strategy is demonstrated.

These are outstanding verifications, not proof of failure. Existing code and currently declared support are not silently reclassified as unsupported merely because a test is absent.

## Release and implementation sequencing

1. Approve the repository release policy and [TDD ADR 0004](https://github.com/cubrid-lab/sea-orm-cubrid/blob/docs/release-policy-before-native-design/docs/adr/0004-test-first-development.md) in the release-policy PR.
2. Resolve the above design/feasibility blockers with evidence, review [SeaORM design proposal](CUBRID_NATIVE_SEAORM_DESIGN.md), and explicitly accept the architecture/contract ADR when applicable.
3. Implement each scoped issue with test-first evidence, one observable contract at a time. No implementation-wide PR before essential extension feasibility.
4. Run real CUBRID and downstream consumer tests on the final release candidate; document exact versions and test counts; do not publish based on mocks alone.

**Tracked design PR:** #29. This file is a **sign-off checklist**, not a signed-off design.
