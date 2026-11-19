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

Release Please reads the commit subjects on `main`. With the [required merge settings](#repository-settings) (squash merge only, commit title = PR title), the subject is the PR title, which `.github/workflows/pr-title.yml` checks: `type: description` or `type(scope): description`, with a type from `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`, `revert`.

| Commit on `main` | Next version while 0.x | From 1.0.0 |
| --- | --- | --- |
| `fix:`, `perf:`, `revert:` | patch (0.1.0 → 0.1.1) | patch |
| `feat:` | minor (0.1.0 → 0.2.0) | minor |
| `feat!:`, `fix!:` or a `BREAKING CHANGE:` footer | **minor** (0.1.0 → 0.2.0) | major |
| `docs:`, `style:`, `refactor:`, `test:`, `build:`, `ci:`, `chore:` | no release on their own, not listed in the CHANGELOG | same |

`bump-minor-pre-major: true` keeps a breaking change before 1.0 a minor bump, as Cargo's SemVer rules treat `0.y` minor versions as incompatible. Without it Release Please would turn the first breaking change into 1.0.0, while the [v1.0.0 milestone](../ROADMAP.md) still has open requirements. Choose 1.0.0 deliberately, for example with a `Release-As: 1.0.0` commit footer.

**Silently dropped commits.** Release Please's commit parser throws on some commit bodies, for example a line that starts with a nested call such as `where(eq(...`. Such a commit is left out of the version calculation and the CHANGELOG without an error. The blank squash message setting below (instead of the list of branch commits) makes this less likely; reviewers still compare the Release PR's entries with the PRs merged since the last release and add anything missing by hand.

## CHANGELOG ownership

**PRs do not edit `CHANGELOG.md`.** Release Please writes each version section from the Conventional Commit subjects merged since the last release and inserts it above `## [0.1.0]`. Describe user-visible and breaking changes in the PR title and description instead; type `BREAKING CHANGE: <what to do>` into the blank squash commit body while merging if a breaking change needs an explanation.

## Releasing

1. **Release PR.** After a releasable commit lands on `main`, the workflow opens `chore(main): release X.Y.Z` from the head branch `release-please--branches--main--components--sea-orm-cubrid`. It bumps `version` in `Cargo.toml`, `.release-please-manifest.json` and `CHANGELOG.md`. (`Cargo.lock` is not committed in this repository, so there is none to update.) A version is not released because the PR exists.
2. **CI on the Release PR.** `ci.yml` runs fmt, clippy, tests and coverage on every PR. **When the PR was created with `GITHUB_TOKEN`, no CI runs automatically** (GitHub does not trigger workflows from `GITHUB_TOKEN` events): close and reopen the PR so the checks run. Never merge with a failed or missing check.
3. **Release notes first.** Write the curated `docs/releases/vX.Y.Z.md` ([RELEASE_NOTES_QUALITY.md](RELEASE_NOTES_QUALITY.md)) in a separate `docs:` PR and merge it. This can regenerate the Release PR (when a new releasable commit lands or the UTC date changes).
4. **Review** the version and the generated CHANGELOG section against the merged PRs. Edits to the Release PR branch can be discarded when `main` moves (Release Please regenerates the branch when a new releasable commit lands or the UTC date changes), so make any CHANGELOG correction as the last step before merging.
5. **Merge** the Release PR (squash; the title stays `chore(main): release X.Y.Z`). Merging only changes `main`.
6. **Tag and release** the merge commit: `gh release create vX.Y.Z --target <merge-sha> --title "sea-orm-cubrid vX.Y.Z" --notes-file docs/releases/vX.Y.Z.md` (not as a draft). `sync-release-notes.yml` keeps the Release body equal to the reviewed note on later edits.
7. **Release PR label.** On the next run (any push to `main`, or **Run workflow** on "Prepare reviewed Release PR"), once `vX.Y.Z` has a non-draft GitHub Release, the workflow removes `autorelease: pending` from the merged `chore(main): release X.Y.Z` PR and adds `autorelease: tagged`. Release Please refuses to open the next Release PR while a merged one keeps `autorelease: pending`; if the automatic step ever fails, remove that label by hand: `gh pr edit <number> --remove-label "autorelease: pending"`.

The workflow refuses to propose a newer version until the version in `Cargo.toml` on `main` has a non-draft GitHub Release `vX.Y.Z`, so between steps 5 and 6 it logs a notice and does nothing; if it cannot read the GitHub Release (an API error rather than "not found"), it logs a warning and also does nothing. When the crate is published on crates.io, add a registry check to that step (as `drizzle-cubrid` and `cubrid-client` do for npm).

### The first Release PR (0.1.1)

The first Release PR will propose **0.1.1**. Its only releasable commit since `v0.1.0` is `aeb251a fix: bump MSRV to 1.75, fix rustfmt formatting`, and that bullet is misleading: the MSRV actually went from **1.70** at `v0.1.0` to **1.88** today, and the raise to 1.88 was made in `7b2ac8e ci: remove MSRV from test matrix, test only on stable`, a `ci:` commit the CHANGELOG hides. Before merging, as the last edit, either:

- rewrite the bullet to state the real change (`rust-version` 1.70 → 1.88), or
- if the MSRV raise is treated as breaking, land a commit whose body has a `Release-As: 0.2.0` footer so the PR proposes 0.2.0 instead, or
- leave the Release PR open until there is a real change to release.

Do not set `last-release-sha` or otherwise rewrite the release history to hide these commits.

## Repository settings

These are settings in the repository, not files.

- **Required merge settings (current values; keep them).** Settings → General → Pull Requests: squash merging only (`allow_squash_merge=true`, `allow_merge_commit=false`, `allow_rebase_merge=false`), with the squash commit title set to the **Pull request title** and the squash commit message **blank** (`squash_merge_commit_title=PR_TITLE`, `squash_merge_commit_message=BLANK`). The title is then always the checked PR title, and the body holds only what the person merging types in, such as a `BREAKING CHANGE:` or `Release-As:` footer, so Release Please reads the right type and does not trip over a list of branch commits. Do not re-enable merge commits or rebase merging: they bypass the PR-title check. Changing these settings back (for example to `COMMIT_OR_PR_TITLE`, where a single-commit PR keeps its commit subject instead of the PR title) breaks the assumptions of this process.
- **Recommended:** make the `Check PR title` check required in the branch protection or ruleset for `main`.
- **Settings → Actions → General → Workflow permissions:** "Allow GitHub Actions to create and approve pull requests" must stay enabled, or the workflow cannot open the Release PR.
- **Labels (one time):** this repository has no label sync, and neither autorelease label exists yet. Create both labels once:

  ```bash
  gh api repos/cubrid-lab/sea-orm-cubrid/labels -f name="autorelease: pending" -f color=ededed -f description="Release Please: Release PR not yet tagged"
  gh api repos/cubrid-lab/sea-orm-cubrid/labels -f name="autorelease: tagged" -f color=ededed -f description="Release Please: Release PR tagged and released"
  ```

  Release Please also creates `autorelease: pending` itself when it first labels a PR, so a "label already exists" error for it is harmless.
- **Optional `RELEASE_PLEASE_TOKEN` secret:** a fine-grained token limited to this repository (Contents and Pull requests: read and write). With it, the Release PR triggers CI automatically. Without it the workflow falls back to `GITHUB_TOKEN` and CI must be started by closing and reopening the PR (step 2). A GitHub App token minted per run is an alternative; do not store an expiring App token as a long-lived secret.
