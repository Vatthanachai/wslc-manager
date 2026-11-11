---
tags: [qa]
status: draft-v1
updated: 2026-11-11
---

# CI workflows and toolchain pins

One-page reference. Source of truth is the files named below, not this note: if they differ, the files win and this note needs a fix. Evidence: [[REQ-013-test-report]], [[REQ-014-test-report]]. Strategy: [[test-strategy]]. Checklist: [[definition-of-done]]. Decisions: D16 and D17 in [[decision-log]].

## 1. Workflows

Both live in `.github/workflows/`, both use `permissions: contents: read` at workflow level (none per job), no secrets, no `pull_request_target`, no path filters. Concurrency group is per `github.ref` with `cancel-in-progress`. A push run and a `pull_request` run of the same branch have different refs and do not cancel each other.

### `ci.yml` (workflow `ci`, job `ci-check`)

- Triggers: `pull_request` into `develop` and `main`; `push` to `develop`, `feature/**`, `bugfix/**`, `release/**`, `hotfix/**`.
- Runner `windows-latest`, `timeout-minutes: 45`, `CARGO_TERM_COLOR=always`.
- Steps in order:
  1. Check out (`persist-credentials: false`).
  2. Set up Node from `.nvmrc` (`package-manager-cache: false`, so no Node package cache).
  3. Set up Bun, `bun-version: 1.3.14`, `no-cache: true`.
  4. Cache Cargo (`Swatinem/rust-cache`).
  5. `cargo fmt --check`.
  6. `cargo clippy --workspace --all-targets -- -D warnings`.
  7. `cargo test --workspace`.
  8. Cache bun install (`actions/cache`).
  9. `bun install --frozen-lockfile`.
  10. `bun run build` (`tsc --noEmit && vite build`).
  11. `bun run test` (`vitest run`).
- The job name `ci-check` is the status-check context. Do not rename it without updating the `main` ruleset.

### `branch-policy.yml` (workflow `branch-policy`, job `branch-policy`)

- Triggers: `pull_request` into `develop` and `main`, types `opened`, `synchronize`, `reopened`, `edited`.
- Runner `ubuntu-latest`, `timeout-minutes: 5`.
- One step: reads `github.base_ref` and `github.head_ref` through `env:` (never interpolated into the script). Base other than `main`: pass. Base `main`: pass only if the head is `develop`, `release/*` or `hotfix/*` (exact, case-sensitive), otherwise exit 1 with a message naming the allowed heads.
- The job name `branch-policy` is the status-check context. Same rename rule.

### What `cargo test --workspace` covers

All workspace members (`aiyara-domain`, `aiyara-engine`, `aiyara-app`): unit tests, integration tests under `crates/*/tests/`, and doc tests. `--all-targets` would add nothing that matters here and excludes doc tests; a separate `cargo test --doc` is therefore not needed. `#[ignore]` tests (real `wslc.exe`, see [[qa-integration-convention]]) are listed as ignored and do not run.

## 2. Pinned actions

Each is pinned to a full commit SHA with a version comment. Every SHA was checked against its tag with `gh api repos/<owner>/<repo>/git/ref/tags/<tag>` in the REQ-013 and REQ-014 reports (an annotated tag must be dereferenced through `git/tags/<sha>`).

| Action | Version | SHA (first 8) |
|---|---|---|
| `actions/checkout` | v7.0.1 | `3d3c42e5` |
| `actions/setup-node` | v7.1.0 | `949feb24` |
| `oven-sh/setup-bun` | v2.2.0 | `0c5077e5` |
| `Swatinem/rust-cache` | v2.9.3 | `d1527c8f` |
| `actions/cache` | v6.1.0 | `55cc8345` |

## 3. Caches

| Cache | Path | Key | Notes |
|---|---|---|---|
| Cargo (`rust-cache`) | cargo registry and `target` as chosen by the action | built by the action: `v0-rust-ci-check-Windows_NT-x64-<hash>`, where the hash covers `Cargo.lock`, the `Cargo.toml` files and the Rust version | A change of `rust-toolchain.toml` changes the key; an older cache is then restored by prefix and saved again |
| bun install (`actions/cache`) | `~/.bun/install/cache` | `bun-${{ runner.os }}-${{ steps.setup-bun.outputs.bun-version }}-${{ hashFiles('bun.lock') }}` | Downloaded packages only, never `node_modules` and never the bun executable |
| Node package cache | none | none | disabled with `package-manager-cache: false` |

A cache miss still gives a green run. Caches are for checks only and must never be restored in a release build (REQ-012).

## 4. Toolchain pins

| Tool | Pin | Where |
|---|---|---|
| Rust | 1.99.1, profile `minimal`, components `rustfmt`, `clippy` | `rust-toolchain.toml` (rustup installs it on the first `cargo` or `rustc` call; CI has no separate Rust step) |
| Rust version declared by the workspace | `rust-version = "1.80"` (declared; not tested as an MSRV, CI uses 1.99.1) | workspace `Cargo.toml` |
| bun | 1.3.14 | `packageManager` in `package.json` and `bun-version` in `ci.yml` (two places) |
| Node | 24 | `.nvmrc`; `engines.node` is `>=24 <25` (bun does not enforce `engines`) |

Why Node stays: Vitest does not start without Node (its worker fails with an `EventTarget` error, also under `bun --bun`), verified by experiment in REQ-014 (D17).

## 5. Known caveats

- **Runner Rust was newer than local.** The first CI run failed at `cargo clippy` because the runner had Rust 1.99, whose clippy added the lint `chunks_exact_to_as_chunks`, while the local compiler was 1.97. That is why `rust-toolchain.toml` pins the compiler.
- **bun version in two places.** `packageManager` and `ci.yml` must be equal. A comment in `ci.yml` says so; nothing checks it. `setup-bun` reading `packageManager` was not tried (unverified).
- **Shared head SHA masks `branch-policy` failures.** If two pull requests share one head (one into `develop`, one into `main`), both runs land on the same commit and each PR shows both a failure and a success. A required check could then be satisfied by the wrong run. Never open a PR into `main` from a `feature/*` branch; treat the rule as a guard, not a security boundary.
- **Fork PR named `develop` passes.** `branch-policy` compares branch names only, not the head repository.
- **`edited` trigger unverified.** `edited` was added so that a base change re-runs the check. Whether GitHub really re-runs it after a base change was not verified (no probe PR for that).
- **`bun test` is refused.** A preload in `bunfig.toml` makes it fail loudly, with noisy output (REQ-014 BUG-003, accepted). Use `bun run test`.
- **`bun install --frozen-lockfile` accepts a range edit** that the lock still satisfies (REQ-014 O1). It fails on a new dependency.
- **`ci-check` and `branch-policy` are required status checks on the `main` ruleset** (D19, 2026-11-11, `strict` off, both bound to the GitHub Actions app id 15368). If a job is renamed, removed or skipped by a trigger or path rule, the required check never reports and pull requests into `main` wait forever: change the ruleset first. The required checks do not apply to `develop` or to topic branches. Recorded in [[decision-log]].
- Cache scoping between PR runs and the base branch was not examined.

## 6. Bump a pin deliberately

Always in its own pull request, never mixed with a feature.

- **Rust:** edit `channel` in `rust-toolchain.toml`, run the full local check list, fix new lints in the same PR. Expect one cold Cargo cache run in CI.
- **bun:** edit `packageManager` in `package.json` and `bun-version` in `ci.yml` together. Run `bun install --frozen-lockfile` locally, and confirm `bun.lock` did not change unexpectedly. Check `bun test` is still refused.
- **Node:** edit `.nvmrc` and `engines.node` together, and the README prerequisite. Run `bun run test`.
- **An action:** look up the new tag's commit with `gh api repos/<owner>/<repo>/git/ref/tags/<tag>` (dereference annotated tags), put the full SHA in the workflow with the version comment, update the table in section 2.
- Never rename the jobs `ci-check` and `branch-policy`.

## 7. Probe procedure (throwaway branches)

Used in REQ-013 and REQ-014 to prove that CI fails where it should. Run it again after a change to a workflow, a pin or a job. The main assistant does the pushes; nothing here is pushed from a work branch that matters.

1. Push the real feature branch. Confirm `ci-check` is green and the check name is exactly `ci-check` (`gh api repos/<owner>/<repo>/commits/<sha>/check-runs --jq '.check_runs[].name'`).
2. Create five probe branches from that head, each with one deliberate fault, push each, and read the failing step with `gh run view`:
   - [ ] failing Rust test, expect failure at `cargo test`
   - [ ] clippy warning, expect failure at `cargo clippy`
   - [ ] formatting difference, expect failure at `cargo fmt --check`
   - [ ] failing Vitest test, expect failure at `bun run test`
   - [ ] TypeScript error, expect failure at `bun run build`
3. Open three throwaway pull requests for `branch-policy`, use a draft or close each one unmerged:
   - [ ] `feature/*` probe into `main`: expect FAILURE with the message naming the allowed heads
   - [ ] `feature/*` probe into `develop`: expect success
   - [ ] `release/<version>-probe` into `main`: expect success
4. Note the shared-head-SHA effect if two PRs use one head (each shows a failure and a success).
5. Close every PR unmerged. Delete every probe branch, remote and local.
6. Check nothing is left: the remote has only the expected branches (`develop`, `main`, the real feature branch) and no open PRs.
7. Record run ids, the failing step of each probe and the deletion in the ticket plan (as in [[REQ-014-plan]]).

Back to [[README|QA index]].
