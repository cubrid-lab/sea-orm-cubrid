# Release process

This crate uses [Release Please](https://github.com/googleapis/release-please) (release type **rust**) to **propose** each release in a reviewed pull request. Release Please never tags, creates a GitHub Release or publishes to crates.io: those stay manual maintainer steps. `sea-orm-cubrid` is not published on crates.io yet (it depends on unpublished `cubrid-rs` crates through `[patch.crates-io]`, see [ROADMAP.md](../ROADMAP.md)), so the git tag and GitHub Release are the release record.

## Files

| File | Role |
| --- | --- |
| `.github/workflows/release-please.yml` | On every push to `main` (or a manual run), creates or updates the Release PR. PR-only: `skip-github-release: true`. |
| `release-please-config.json` | One crate at the repository root, release type `rust`, tags `vX.Y.Z` (no component), `bump-minor-pre-major: true`. |
| `.release-please-manifest.json` | The last released version. It started at `0.1.0`, the `v0.1.0` tag and GitHub Release; historical tags are never recreated. |
| `.github/workflows/pr-title.yml` | Fails a PR whose title is not a Conventional Commits title. |

## How versions are chosen

PRs are squash-merged, so the PR title becomes the commit subject that Release Please reads. `.github/workflows/pr-title.yml` requires `type: description` or `type(scope): description`, with a type from `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`, `revert`.

| Commit on `main` | Next version while 0.x | From 1.0.0 |
| --- | --- | --- |
| `fix:` | patch (0.1.0 → 0.1.1) | patch |
| `feat:` | minor (0.1.0 → 0.2.0) | minor |
| `feat!:`, `fix!:` or a `BREAKING CHANGE:` footer | **minor** (0.1.0 → 0.2.0) | major |
| `docs:`, `test:`, `ci:`, `chore:` and the rest | no release on their own | same |

`bump-minor-pre-major: true` keeps a breaking change before 1.0 a minor bump, as Cargo's SemVer rules treat `0.y` minor versions as incompatible. Without it Release Please would turn the first breaking change into 1.0.0, while the [v1.0.0 milestone](../ROADMAP.md) still has open requirements. Choose 1.0.0 deliberately, for example with a `Release-As: 1.0.0` commit footer.

## CHANGELOG ownership

**PRs do not edit `CHANGELOG.md`.** Release Please writes each version section from the Conventional Commit titles merged since the last release and inserts it above `## [0.1.0]`. Describe user-visible and breaking changes in the PR title and description instead; put `BREAKING CHANGE: <what to do>` in the squash commit body when a breaking change needs an explanation.

## Releasing

1. **Release PR.** After a releasable commit lands on `main`, the workflow opens `chore(main): release X.Y.Z`. It bumps `version` in `Cargo.toml`, `.release-please-manifest.json` and `CHANGELOG.md`. (`Cargo.lock` is not committed in this repository, so there is none to update.) A version is not released because the PR exists.
2. **CI on the Release PR.** `ci.yml` runs fmt, clippy, tests and coverage on every PR. **When the PR was created with `GITHUB_TOKEN`, no CI runs automatically** (GitHub does not trigger workflows from `GITHUB_TOKEN` events): close and reopen the PR so the checks run. Never merge with a failed or missing check.
3. **Review** the version and the generated CHANGELOG section against the merged commits, and write the curated `docs/releases/vX.Y.Z.md` ([RELEASE_NOTES_QUALITY.md](RELEASE_NOTES_QUALITY.md)) in a separate `docs:` PR. Edits to the Release PR branch are discarded whenever `main` moves, so make any CHANGELOG correction as the last step before merging.
4. **Merge** the Release PR (squash; the title stays `chore(main): release X.Y.Z`). Merging only changes `main`.
5. **Tag and release** the merge commit: `gh release create vX.Y.Z --target <merge-sha> --title "sea-orm-cubrid vX.Y.Z" --notes-file docs/releases/vX.Y.Z.md`. `sync-release-notes.yml` keeps the Release body equal to the reviewed note on later edits.
6. **Mark the Release PR as released.** Release Please labels its PR `autorelease: pending` and refuses to open the next Release PR while a merged PR keeps that label: `gh pr edit <number> --remove-label "autorelease: pending" --add-label "autorelease: tagged"`.

The workflow refuses to propose a newer version until the version in `Cargo.toml` on `main` has a GitHub Release `vX.Y.Z`, so between steps 4 and 5 it logs a notice and does nothing. When the crate is published on crates.io, add a registry check to that step (as `drizzle-cubrid` and `cubrid-client` do for npm).

## Repository settings

- **Settings → Actions → General → Workflow permissions:** "Allow GitHub Actions to create and approve pull requests" must stay enabled, or the workflow cannot open the Release PR.
- **Optional `RELEASE_PLEASE_TOKEN` secret:** a fine-grained token limited to this repository (Contents and Pull requests: read and write). With it, the Release PR triggers CI automatically. Without it the workflow falls back to `GITHUB_TOKEN` and CI must be started by hand (step 2). A GitHub App token minted per run is an alternative; do not store an expiring App token as a long-lived secret.
