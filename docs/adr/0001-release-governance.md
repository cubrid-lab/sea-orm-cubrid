# ADR 0001: Release governance and compatibility contract

- **Status:** Proposed
- **Date:** 2026-11-07
- **Decision scope:** sea-orm-cubrid release and supported-consumer contract
- **Owner:** Repository maintainers (approval not yet recorded)

## Context

A tag, passing mock suite, or green job without executed database cases does not establish a ready release of sea-orm-cubrid. The release contract follows this repository's own responsibilities as the Rust SeaORM CUBRID adapter, including SeaORM/SeaQuery extension contracts, Cargo/MSRV, async error mapping, and CUBRID integration. External package versions cannot substitute for local compatibility evidence.

## Decision

Apply Cargo SemVer independently from cubrid-rs crates and SeaORM releases. Keep the current pre-1.0 status until CUBRID-native SQL generation without MySQL backend assumptions, correct generated-key handling, observable begin/commit/rollback errors, real CUBRID ORM tests and external cargo-consumer resolution pass. Record the supported Rust MSRV, SeaORM/SeaQuery feature and cubrid-tokio/cubrid-protocol versions. Align **tested stable compatibility generation** with Rust driver crates while permitting independent MINOR/PATCH releases. Root-workspace [patch.crates-io] does not establish that a published consumer resolves normally.

Applicable SemVer classification: a compatible bug/security fix is generally PATCH, an additive feature MINOR, and an incompatible public behavior or supported-environment change requires a reviewed breaking release (under current 0.x rules, commonly the next 0.y.0; at 1.x+ a MAJOR). Changes to SQL generation, data types, migrations or transactions can be compatibility-breaking without exported-type changes. A release candidate must have a reviewed version and notes, a final immutable commit, actual live CUBRID evidence when database behavior is claimed, and verified published artifacts. Keep omitted/failed checks visible; never invent approval or successful publication.

## Alternatives considered

- **Rejected:** Publish 1.0 after mocked proxy tests alone; mark driver 0.x requirement as tested stable; use numeric match with driver crates rather than explicit compatibility; declare the MySQL proxy a CUBRID compiler.
- **Rejected:** Making this package's releases contingent on matching another repository's version number or calendar.
- **Rejected:** Treat an issue, green mock suite, GitHub tag or draft changelog as sufficient release evidence.

## Consequences and trade-offs

The target 1.0 waits for upstream ORM extension feasibility, transaction semantics and true live DB integration; clear boundaries protect downstream users.

Maintainers must explicitly review the supported matrix and any runtime/API removal. Compatible fixes may be released independently when their own gates pass. Release automation can prepare PRs but must not bypass a maintainer review, exact candidate test or registry verification.

## Verification / acceptance criteria

- [ ] Maintainer reviews and accepts this ADR, links the approval and records acceptance date.
- [ ] Published README/release guide and dependency version declarations agree with the reviewed compatibility contract.
- [ ] A candidate-specific CI check identifies supported CUBRID server versions, tests run/passed/failed/skipped, runtime and driver/ORM versions, and actual artifact.
- [ ] Required release checks reject failed, missing, zero-case or skipped live tests, with scoped exceptions documented as unsupported/unverified—not counted as passes.
- [ ] User-visible breaking changes have accurate migration notes; correct author attribution in curated release notes comes from verified merged contributions.
- [ ] Existing immutable tags and published versions remain untouched by adopting this ADR.

## References

[release policy](../RELEASE_POLICY.md), [Cargo policy #23](https://github.com/cubrid-lab/sea-orm-cubrid/issues/23), [native backend #28](https://github.com/cubrid-lab/sea-orm-cubrid/issues/28), [live DB #31](https://github.com/cubrid-lab/sea-orm-cubrid/issues/31)

**This ADR does not update version numbers, create a release, authorize publishing, or claim any database test has passed.** Its status remains **Proposed** until explicit maintainer approval.
