---
ticket: REQ-013
status: ready
tags:
  - plan
  - tasks
---

# REQ-013 plan and tasks: CI and QA baseline

Ticket: [[REQ-013-ci-and-qa-baseline]]. Background: [[REQ-000-test-report]], [[decision-log]] (D16), [[ba-analysis]]. QA documents: [[qa/README|QA index]] (written in parallel by `tester`; link their final file names in the PR).

No UI change, so no UX design is needed. Branch: `feature/REQ-013-ci-and-qa-baseline` (git flow).

## Decisions taken in this plan (answers to the ticket's open questions, change them before dev starts if you disagree)

| # | Question | Decision | Why |
|---|---|---|---|
| 2 | Pin actions | Full commit SHA with the version in a comment. `dev` looks each SHA up with `gh api repos/<owner>/<repo>/git/ref/tags/<tag>` and checks it is the tag's commit (dereference annotated tags) | supply-chain safety, one-time cost |
| 3 | jsdom or happy-dom | jsdom | most complete DOM, speed is irrelevant for three tests |
| 4 | Required checks and triggers | CI runs on PRs to `develop`/`main` and on pushes to `feature/**`, `bugfix/**`, `release/**`, `hotfix/**` and `develop`. Required status check on `main` only, added later by the owner. Not on `develop` | `develop` stays frictionless for `git flow feature finish` |
| 5 | Node/npm pin | Node 24 (matches the machine: v24.x) in `.nvmrc`; `actions/setup-node` reads it. npm version is whatever Node 24 ships on the runner unless the install-scripts problem forces a pin; record the versions in the PR | one source of truth |
| 6 | Runner image | `windows-latest` | Windows 11 target; pin later if an image change ever breaks CI |
| 7 | Dependabot | Later (separate ticket) | out of scope |
| 8 | Job names | Workflow `ci`, job `name: ci-check`; workflow `branch-policy`, job `name: branch-policy`. The status-check context GitHub reports is the job name, so `dev` confirms the exact strings from the first run (`gh api repos/Vatthanachai/wslc-manager/commits/<sha>/check-runs --jq '.check_runs[].name'`) and writes them into the ticket | avoids a required check that never reports |

## Approach

1. **QA documents** come from `tester` ([[qa/README|obsidians/qa/]]); this ticket only links them (criteria A). The `#[ignore]` + `WSLC_INTEGRATION=1` convention is implemented as a small, shared helper in the `aiyara-engine` tests with one example test that is ignored by default (so `cargo test` proves it is reported as ignored and starts no `wslc.exe`). The helper owns the `aiyara-test-` prefix, the startup sweep and the drop-guard cleanup. The example test must be safe: it only lists containers (read-only) unless a later ticket needs more.
2. **Frontend tests**: add `vitest` and `jsdom` as devDependencies, a `vitest.config.ts` (environment jsdom, no `passWithNoTests`), `npm test` = `vitest run`. Tests live in `src/` as `*.test.ts`. Mock `@tauri-apps/api/core` `invoke` with `vi.mock`. If `main.ts` cannot be tested without side effects at import, refactor minimally (export a `render(...)` function, keep behaviour identical) and say so in the PR. Tests import `src/strings.ts` for expected text; one XSS test uses `<img src=x onerror=alert(1)>`.
3. **CI**: `.github/workflows/ci.yml` and `.github/workflows/branch-policy.yml`, `permissions: contents: read`, `pull_request` (never `pull_request_target`), pinned SHAs, `timeout-minutes` as a safety net, concurrency group per ref with cancel-in-progress, cache for Cargo (`Swatinem/rust-cache` or `actions/cache` keyed on `Cargo.lock`) and npm (`actions/setup-node` cache on `package-lock.json`), no secrets, no path filters. Steps in the ticket's order. `branch-policy` reads `github.base_ref` / `github.head_ref` through `env:` variables and compares in the shell with quoted variables (no `${{ }}` interpolation inside `run:` scripts).
4. **Docs**: README section "Local checks" with the exact commands and the integration-test instructions (`WSLC_INTEGRATION=1`, prefix, cleanup, how to remove leftovers).
5. **Verification** (needs GitHub, so the main assistant pushes; `dev` does not push): push the feature branch, watch the run with `gh run watch`, then probe failures on throwaway branches `feature/REQ-013-probe-<n>` (one each for failing Rust test, clippy warning, fmt difference, failing Vitest test, TypeScript build error, plus a PR-style check for branch-policy using a draft PR from a non-allowed head into `main` that is closed unmerged), then delete the probe branches. Results go into the PR/test report.

## Risks to watch (from the ticket)
npm 12 blocking install scripts (esbuild) · Tauri crate compiling on the runner without WebView2 SDK [U] · CRLF vs LF in `cargo fmt --check` (`.gitattributes` already sets `eol=lf`; add `rustfmt.toml` only if needed) · job names drifting from the ruleset · cache scoping between PR and base [U].

## Tasks (in order, tick them off; report deviations)

- [x] T1 `.nvmrc` (Node 24), `engines` in `package.json`; record `node -v` / `npm -v`. Local `npm ci` still works. (B: pinned by lockfile)
- [x] T2 Add `vitest` + `jsdom`, `vitest.config.ts`, `npm test` script (`vitest run`). Confirm the esbuild install-script situation on a clean `npm ci` (delete `node_modules` first) and document any allow-list needed. (B)
- [x] T3 UI tests for `main.ts`: table with the one fake container (fields + headers from `strings.ts`), empty list (no data row, empty-state text from `strings.ts`), error path via `textContent` with the XSS payload (no element created, text literal). Refactor only if unavoidable. (B, criteria 1 to 7)
- [x] T4 Prove a broken assertion fails `npm test` (non-zero exit), once, and note it. (B)
- [x] T5 Rust integration-test helper + convention in `crates/engine/tests/` (ignored by default, `WSLC_INTEGRATION=1` guard, `aiyara-test-` prefix, sweep, drop-guard cleanup, never touches non-prefixed resources), one read-only example test. `cargo test --workspace` reports it as ignored and spawns no process. (A)
- [x] T6 `ci.yml` (job `ci-check`): fmt, clippy `-D warnings`, test, `npm ci`, `npm run build`, `npm test`; triggers, permissions, pinned SHAs (verified), caches, timeout, concurrency. (C)
- [x] T7 `branch-policy.yml` (job `branch-policy`): passes for head `develop`, `release/*`, `hotfix/*` into `main`; fails with a message naming them for any other head into `main`; passes or is skipped for PRs into `develop`; injection-safe. Test the shell logic locally with a small table of branch names (including odd characters and long names). (C)
- [x] T8 README "Local checks" + integration-test instructions; link the QA documents. (D, A)
- [x] T9 Run all local checks (`cargo fmt --check`, clippy, `cargo test --workspace`, `npm ci && npm run build && npm test`) and report real output. Commit in small conventional commits ending with the Co-Authored-By line. Do not push.
- [x] T10 (main assistant) push the feature branch, confirm the CI run is green, run the five deliberate-failure probes and the branch-policy probes, record job names in the ticket, delete probe branches.
- [ ] T11 (`tester`) verify against the ticket's criteria and the QA Definition of Done; report in `obsidians/plans/REQ-013-test-report.md`.
- [ ] T12 (owner, separate step) after both jobs ran green at least once: decide whether to add `ci-check` and `branch-policy` as required status checks on ruleset 27014559 and record it in [[decision-log]].

## Definition of done
Ticket acceptance criteria A to D met or explicitly marked pending; `tester` verdict pass; QA Definition of Done ([[qa/README|obsidians/qa/]]) satisfied; CI green on the feature branch; no personal data in workflow files or docs; PR-only rule for `main` respected (D16).

## T10 results (GitHub, 2026-11-11)

- First run on `0cf462d` FAILED at `cargo clippy`: the runner has Rust 1.99, whose clippy adds `chunks_exact_to_as_chunks`; local Rust is 1.97. Fixed in `3f861d3` (`.chunks(2).filter(len == 2)`; `as_chunks` was not used because the workspace declares `rust-version = 1.80`). The second run on `3f861d3` passed every step (`ci-check`, success). Lesson: the Rust toolchain is not pinned, so CI and local can diverge. Decision pending: pin it with `rust-toolchain.toml` (owner).
- Status-check names reported by GitHub (these are the strings to use in the ruleset): `ci-check` and `branch-policy`.
- Deliberate-failure probes on throwaway branches, each failed at the matching step: failing Rust test -> `cargo test`; clippy warning -> `cargo clippy`; formatting difference -> `cargo fmt --check`; failing Vitest test -> `npm test`; TypeScript error -> `npm run build`. Probe branches and PRs were deleted/closed unmerged.
- `branch-policy` probes through real PRs: feature -> `main` FAILED with the message "Pull requests into main must come from develop, release/* or hotfix/*."; feature -> `develop` passed; `release/*` -> `main` passed.
- Caveat found: when two PRs share the same head SHA (e.g. one into `develop` and one into `main`), the check runs of both land on the same commit and both show in each PR's status rollup (one FAILURE, one SUCCESS). GitHub's required-check evaluation for such a case depends on which run is latest, so a passing run could mask a failing one. The risk only exists for a head branch with PRs into both bases, which git flow does not normally do (feature -> `develop` only; `develop`/`release/*`/`hotfix/*` -> `main`). Mitigation if it matters: never open a PR into `main` from a `feature/*` branch (the check is meant to prevent it) and keep the rule as a guard, not a security boundary.
- Cache: first run built from cold; second run time not compared here [U].
