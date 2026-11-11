---
ticket: REQ-014
status: partial
tags:
  - test-report
---

# REQ-014 test report: switch the frontend package manager to bun

- Ticket: [[REQ-014-switch-package-manager-to-bun]]. Plan: [[REQ-014-plan]]. Definition of done applied: [[qa/definition-of-done]] (also [[qa/test-strategy]], [[qa/qa-integration-convention]]). Earlier evidence redone under bun: [[REQ-013-test-report]]. Facts: [[ba-analysis]].
- Branch: `feature/REQ-014-switch-package-manager-to-bun`, head `2ce4036` (plus this report). Date: 2026-11-11. OS: Windows 11 (10.0.26300). Local: bun 1.3.14, Node v24.21.0, Rust 1.99.1 (from `rust-toolchain.toml`).

## 1. Summary

The migration works and the claims of the dev agent and of T11 hold. Every local check passes from a clean state and in a fresh clone, the four XSS mutations fail exactly the tests that were reported, `bun test` is refused, the debug build never calls npm and the window renders the fake container row, the Node-removal finding reproduces, and every action pin matches its tag. The GitHub runs I looked up (the passing run, all five probes, the policy probes, and a later run with a bun cache hit) show what the plan records. No production code is touched by the branch.

Verdict is **partial**, not pass, for one reason: BUG-001 (Medium), the ticket file itself is not finished (status `todo`, open questions unanswered, decisions and job names not recorded in the ticket). It is the same kind of defect as BUG-001 in the REQ-013 report, which was handled as blocking, so I apply the same rule. Two Low items (BUG-002, BUG-003) and the PR-only items are listed below. There is no code defect.

## 2. Criterion matrix

Legend: met / not met / pending (needs a pull request, a second machine or an owner action).

| # | Criterion | Result | Evidence |
|---|---|---|---|
| A1 | `bun.lock` committed, `package-lock.json` deleted, no other lockfile | met | `git ls-files \| grep -i lock` gives `Cargo.lock` and `bun.lock` only; `package-lock.json`, `yarn.lock`, `pnpm-lock.yaml`, `bun.lockb` do not exist. `bun.lock` is text (`lockfileVersion` 1), 0 CR bytes in a fresh clone |
| A2 | `packageManager: bun@1.3.14`, CI installs exactly that version | met | `package.json:6`; `ci.yml:51` `bun-version: 1.3.14`; CI log "bun install v1.3.14 (0d9b296a)". No `.bun-version` file. The version is written in two places by hand (comment at `ci.yml:45` says so); the action does not read `packageManager` (not used, see section 7) |
| A3 | Node pin decision recorded with a reason | met in files, "recorded in the PR" pending | `.nvmrc` 24, `engines.node` `>=24 <25`, `setup-node` kept with a reason in the `ci.yml:38` comment, README, ticket and D17. Reproduced: without Node on PATH, `bun run test` fails (section 3, step 5). No PR exists yet |
| A4 | `trustedDependencies` absent unless proven needed | met | `grep -c trustedDependencies package.json` = 0; `bun pm untrusted`: "Found 0 untrusted dependencies with scripts"; `bun pm default-trusted` lists `esbuild`, `@bazel/esbuild`, `@netlify/esbuild` |
| B1 | Script names unchanged, `test` is `vitest run` | met | `package.json:7-12`. Vitest runs on Node: a temporary test printed `process.versions.bun` = `undefined`, `node` = 24.21.0 (temporary file deleted) |
| B2 | Hooks are `bun run dev` / `bun run build`, no npm/npx in `tauri.conf.json` | met | `tauri.conf.json:8-9`; `git grep -i -E "npm\|npx"` over `src-tauri/tauri.conf.json` finds nothing |
| B3 | `bun test` does not silently run another suite, guard documented | met | `bunfig.toml` `[test] preload` loads `scripts/refuse-bun-test.ts`. `bun test` printed "error: Do not use `bun test` ... Run `bun run test`.", "0 pass 2 fail 2 errors", exit 1. `bun run test` unaffected (17 pass). README states `bun run test` is the only test command. The guard is a thrown preload error, so the output is noisy, but it fails loudly |
| B4 | `bunx tauri build --debug --no-bundle` works on a clean checkout without npm | met | Log line 2: "Running beforeBuildCommand `bun run build`"; grep for `npm` in the log: none; exit 0, exe rebuilt (`target/debug/aiyara-app.exe`, 16:03). Also exit 0 with Node removed from PATH. Done in the working tree, not on a machine without Rust |
| B5 | Window renders the fake container row (WebView2 remote debugging) | met | CDP `Runtime.evaluate`: title "Aiyara Container Dashboard", DOM text `Containers / Name Image State Status Session / aiyara-spike alpine:latest Running Up 3 seconds Fake session`, 1 `tbody tr`. Afterwards 0 `aiyara-app`, 0 test-profile `msedgewebview2`, profile folder deleted |
| C1 | Clean state: frozen install, build, test all exit 0, files unchanged | met | After `rm -rf node_modules dist`: install exit 0 ("65 packages installed"), `git status` shows no change to `bun.lock` or `package.json`; build exit 0; test exit 0. Repeated in a fresh clone |
| C2 | Frozen install fails when `package.json` and `bun.lock` disagree | met (see observation O1) | In a scratch clone, adding a dependency `left-pad` to `package.json`: "error: lockfile had changes, but lockfile is frozen", exit 1 |
| C3 | Same 17 test names under bun as under npm | met, npm side not independent | Verbose run: 17 names, 2 files. A diff against `npm-names.txt` (the dev's saved npm-era list, in the scratchpad, made at 15:36) is empty. I did not re-run npm, so the npm list is the dev's evidence |
| C4 | XSS mutations M1 to M4 fail the same tests under bun | met | Section 3. Restored with `cmp` against a backup; `git diff --stat -- src crates` empty |
| C5 | Broken assertion exits non-zero | met | M4 above gave a failing run, exit non-zero, and GitHub probe `feature/REQ-014-probe-vitest` (run `42990688078`) failed exactly at `bun run test` |
| C6 | No test files found means failure | met by reading and one check | `vitest.config.ts` has no `passWithNoTests`; `bun run test nonexistent-pattern` ends "No test files found" and `error: script "test" exited with code 1`. A run with an empty `include` was not tried |
| D1 | `setup-bun` pinned to full SHA with version comment, same version as `packageManager`, SHA = tag commit | met | `ci.yml:49` `0c5077e5...857d6 # v2.2.0`; `gh api repos/oven-sh/setup-bun/git/ref/tags/v2.2.0` = commit `0c5077e5...` (lightweight tag). `no-cache: true` shown in the log |
| D2 | Step order, only the npm steps changed | met | `ci.yml` steps: checkout, Node, Bun, Cargo cache, fmt, clippy, test, bun cache, `bun install --frozen-lockfile`, `bun run build`, `bun run test`. The diff against the merge base touches only the Node cache lines, the Bun step, the bun cache step and the three renamed steps |
| D3 | Bun cache: global install cache, key has OS, bun version, hash of `bun.lock`; miss is green; no release use | met | `ci.yml:73-77` path `~/.bun/install/cache`, key `bun-${{ runner.os }}-${{ steps.setup-bun.outputs.bun-version }}-${{ hashFiles('bun.lock') }}`; log key `bun-Windows-1.3.14-3b36c8...`. Run `42989902391`: "Cache not found", job green. Run `42991633495` (head `2ce4036`): "Cache hit", job green. `actions/cache` `55cc8345...c52a9` = commit of tag v6.1.0. No `node_modules` and no executable is cached. No release workflow exists. The comment says never restore it in a release build |
| D4 | Names, permissions, trigger, secrets, no path filters | met | `name: ci-check` (`ci.yml:29`) and `branch-policy` (`branch-policy.yml:21`), both unchanged (`git diff develop...HEAD --stat -- .github` lists only `ci.yml`); GitHub reports the job as `ci-check`. `permissions: contents: read` at workflow level (`ci.yml:16-17`), none at job level. `pull_request`, no `pull_request_target`, no `secrets`, no `paths:` (Grep over `.github`: matches only comments). `${{ }}` occurs only at `ci.yml:20` (concurrency group) and `ci.yml:77` (cache key), never inside a `run:` script; `persist-credentials: false` kept |
| D5 | Clean checkout passes on the Windows runner | met | Run `42989902391` (`e566d2e`): all steps success, bun 1.3.14, "65 packages installed [4.11s]". Run `42991633495` (`2ce4036`): success |
| D6 | Five deliberate-failure probes fail at the matching step | met | Looked up with `gh run view`: `42990669219` fails at `cargo test`; `42990674781` at `cargo clippy`; `42990682507` at `cargo fmt --check`; `42990688078` at `bun run test`; `42990694697` at `bun run build`. Probe branches are gone: the remote has only `develop`, `feature/REQ-014-switch-package-manager-to-bun`, `main`; no open PRs. The deletion is recorded in [[REQ-014-plan]] |
| D7 | `branch-policy` fails for feature into `main`, passes for `develop` and `release/*` | met, with the caveat | `42990716363` FAILED at "Check head branch for pull requests into main"; `42990723811` (feature into `develop`) success; `42990729954` (`release/0.0.1-probe`) success. Runs `42990716363` and `42990723811` share head SHA `76a2c8d`, so the shared-head-SHA caveat (documented in [[REQ-013-plan]] and in [[REQ-014-plan]] T11) is still real and still documented. `branch-policy.yml` is unchanged |
| E1 | README "Local checks" has bun commands; prerequisites list bun, rustup, Node with the reason | met | `README.md` "Local checks": bun 1.3.14, rustup with `rust-toolchain.toml`, Node 24 "Bun installs and builds, but Vitest still runs on Node, so `bun run test` fails without a `node` on `PATH`"; seven commands in CI order |
| E2 | Fresh clone with only bun, Rust (and Node) passes by README alone | met on this machine only | Cloned the local repo into the scratchpad, ran the README commands: fmt 0, clippy 0, `cargo test --workspace` 0 (9 suites ok), `bun install --frozen-lockfile` 0, build 0, test 0 (17), `git status` clean after install. Same user profile and same machine, so "second environment or clean profile" is not done (pending) |
| E3 | No `npm`/`npx` in living files | met | `git grep -n -i -E "npm\|npx"` over README.md, package.json, .nvmrc, bunfig.toml, src-tauri/tauri.conf.json, .github, obsidians/qa, scripts: no output |
| E4 | DoD checklist line reads `bun run build` and `bun run test` pass | met | `obsidians/qa/definition-of-done.md:17` |

Other checks asked for:

| Item | Result |
|---|---|
| Production code unchanged | met: `git diff develop...HEAD --stat -- src crates src-tauri/src` is empty. (With the two-dot form `develop..HEAD` the diff also shows files changed by later commits on `develop`; the three-dot form is the branch's own change.) |
| Personal data and absolute paths in changed files | none found in the added lines of the branch diff (grep for `C:\Users`, `Z:\`, user name, scratchpad). The owner's public name appears in `README.md:122` (MIT line) and the GitHub URL in `obsidians/decision-log.md:30`; both pre-date this branch and are public owner data |
| esbuild correction | true as written: `bun pm default-trusted` lists esbuild; `bun pm untrusted` reports 0. D17 (`decision-log.md:27`) and ticket line 25 state "default-trusted, postinstall runs, `trustedDependencies` stays empty". Ticket line 39 still carries "[U, confirm]" for esbuild; it is now confirmed (cosmetic, see BUG-001) |
| Shared-head-SHA caveat documented | yes, [[REQ-014-plan]] T11 results |
| Design check | no UI change, nothing to compare in `obsidians/design/`; the window evidence is B5 |

## 3. Commands run (real output, abridged)

| Command | Result |
|---|---|
| `bun --version` | 1.3.14 |
| `rm -rf node_modules dist`, `bun install --frozen-lockfile` | exit 0, 65 packages; `bun.lock` and `package.json` unchanged |
| `bun run build` | exit 0 (`tsc --noEmit`, vite 7.3.8, 6 modules) |
| `bun run test` | exit 0, 2 files, 17 passed |
| `cargo fmt --check` | exit 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit 0 |
| `cargo test --workspace` | exit 0, 61 passed, 1 ignored |
| `cargo test --workspace --all-targets` and `--doc` (DoD wording) | exit 0 both; unit/integration suites 0, 9, 8, 32, 10 passed with 1 ignored; doc tests 1 + 1 passed |
| `bun test` | refused: "Do not use `bun test` ... Run `bun run test`.", exit 1 |
| `bunx tauri build --debug --no-bundle` | exit 0, "Running beforeBuildCommand `bun run build`", no npm in the log |
| Node removed from PATH (PowerShell, this call only): `bun install --frozen-lockfile` | exit 0 |
| same: `bun run build` | exit 0 |
| same: `bunx tauri build --debug --no-bundle` | exit 0, hook ran `bun run build` |
| same: `bun run test` | exit 1: "Error: [vitest-pool]: Failed to start forks worker ... Caused by: TypeError: 'addEventListener' called on an object that is not a valid instance of EventTarget" (`jsdom/lib/generated/idl/EventTarget.js:108`), "Test Files no tests", exit 1 |

Mutation checks under bun (one at a time on `src/main.ts`, restored each time):

| Id | Temporary change | Failing tests |
|---|---|---|
| M1 | `cell()` uses `el.innerHTML = text` (line 46) | 2: `main.test.ts` "shows container data from the engine literally, never as markup" and `main.dom-safety.test.ts` "does not use innerHTML ... when rendering a list" |
| M3 | state cell built with `innerHTML` (line 71) | the same 2 tests |
| M4 | swap two column headers (line 64) | 1: "renders one row with the fields of the one fake container" |
| M2 | `root.innerHTML = errorText(e)` (line 86) | 1: `main.dom-safety.test.ts` "does not use innerHTML, outerHTML or insertAdjacentHTML on the error path" (the 14 older tests all pass, as REQ-013 documented) |

After the last mutation `cmp` against the backup was identical and `git diff --stat -- src crates` was empty.

GitHub (read-only `gh`): passing run `42989902391` on `e566d2e`, all 12 steps success, check name `ci-check`; setup-bun log "Downloading a new version of Bun ... bun-v1.3.14", "1.3.14+0d9b296af". Later run `42991633495` on `2ce4036`: success, Cargo cache hit, bun cache hit.

Action pins checked with `gh api repos/<o>/<r>/git/ref/tags/<tag>`:

| Action | Tag | Tag object | Result |
|---|---|---|---|
| oven-sh/setup-bun | v2.2.0 | commit `0c5077e5...` | MATCH |
| actions/cache | v6.1.0 | commit `55cc8345...` | MATCH |
| actions/checkout | v7.0.1 | commit `3d3c42e5...` | MATCH |
| actions/setup-node | v7.1.0 | commit `949feb24...` | MATCH |
| Swatinem/rust-cache | v2.9.3 | annotated tag `2201cc58...`, dereferenced with `git/tags/<sha>` to commit `d1527c8f...` | MATCH |

## 4. Defects

### BUG-001 Ticket file is not finished (OPEN)
- Ticket: REQ-014   Severity: Medium   Found: 2026-11-11
- Environment: branch head `2ce4036`, docs only.
- Steps: open `obsidians/requirements/REQ-014-switch-package-manager-to-bun.md`; read the frontmatter, the "Open questions" section and lines 25 to 39.
- Expected: [[qa/definition-of-done]] "Ticket status ... updated"; the ticket's own criteria say the Node decision and the bun version are recorded; [[REQ-014-plan]] "Decisions taken" answers the open questions and the ticket should say so (same fix as BUG-001 of [[REQ-013-test-report]]).
- Actual: `status: todo`; all six open questions are still open although the plan decided them and the work is done; line 39 still says "esbuild is not expected to need one [U, confirm]" although confirmed; the job names `ci-check` and `branch-policy` are not recorded as unchanged; [[REQ-014-plan]] frontmatter still says `status: ready` and T12 is unchecked (T12 is this report, so the owner or `main` can tick it).
- Cause: documentation step not done.
- Owner decision: not applicable (Medium blocks). Fix: answer the open questions from the plan, drop the "[U, confirm]" on esbuild, set the ticket status, add a line recording the unchanged job names. No code needed. Re-verification is a read of those lines.

### BUG-002 Merge conflict with `develop` in `obsidians/decision-log.md` (OPEN)
- Ticket: REQ-014   Severity: Low   Found: 2026-11-11
- Environment: `develop` at `a78e9da` (two commits ahead of the merge base `495e589`: `c1a0c4e`, `a78e9da`, docs for the UX batch), branch head `2ce4036`.
- Steps: `git merge-tree --write-tree develop HEAD`.
- Expected: the branch merges cleanly into `develop`.
- Actual: reports a conflict in `obsidians/decision-log.md` (stages 1, 2, 3 listed; the branch edits the D17 row, `develop` added D18). No other file conflicts. I did not merge anything.
- Owner decision (Low): resolve when finishing the feature (keep both the reworded D17 and the new D18 rows), or merge `develop` into the branch first.

### BUG-003 `bun test` guard produces a noisy failure (OPEN, cosmetic)
- Ticket: REQ-014   Severity: Low   Found: 2026-11-11
- Steps: run `bun test` in the repo.
- Expected: a clear refusal.
- Actual: the refusal text is shown, but as "Unhandled error between tests", followed by a stack excerpt and "0 pass, 2 fail, 2 errors". Exit code 1, so the criterion is met; only the presentation is rough, and "2 fail" counts the two test files.
- Owner decision (Low): accept (documented in the README and DoD) or leave as is.

## 5. Observations (not defects)

- O1: `bun install --frozen-lockfile` accepted a range-only edit (`"typescript": "^5"` to `"^5.8"`), exit 0 "no changes", because the locked 5.9.3 satisfies it. A new dependency is rejected (exit 1). The criterion (non-zero on disagreement) holds for a real mismatch; a loosened or tightened range that the lock still satisfies passes.
- O2: The bun version appears in `package.json:6` and `ci.yml:51` and nothing checks that they stay equal (a comment says so). The ticket accepted this (open question 2).
- O3: README and CI run `cargo test --workspace`; the DoD wording is `--all-targets` plus `--doc`. I ran both, both pass. The same wording gap existed in REQ-013.
- O4: `package.json` `engines.node` is not enforced by bun, as the ticket expected; the Node requirement is visible only through the README and the Vitest error.
- O5: Dev's `bun.lock` resolves vite 7.3.8, vitest 5.1.0, jsdom 31.0.0, typescript 5.9.3, `@tauri-apps/cli` and `api` 2.12.4. I did not compare with the old `package-lock.json` (the scratchpad has an `old-versions.json` from the dev; not independently checked).

## 6. Coverage gaps

No new tests were added: the existing 17 tests plus the recorded mutations cover the behaviour this ticket can change, and the guard and CI changes are checked by command and by GitHub runs.

## 7. Unverified

- `setup-bun` reading `packageManager` through `bun-version-file`: not used here, not tested.
- Cache scoping between PR runs and the base branch (a cache saved on a feature branch is visible to that branch and, by GitHub's rule, to PRs targeting it; not examined). Poisoning controls rest on `contents: read` and no secrets.
- Behaviour on a second machine or a clean Windows user profile (E2): the fresh clone ran on this machine and under this user, with the same global bun cache and rustup state.
- The `edited` trigger of `branch-policy` (REQ-013 BUG-004) when a PR base is changed: `branch-policy.yml` was not changed by this branch and no PR was created.
- Pull requests from forks whose head is named `develop`: the policy compares names only (REQ-013 note).
- Whether esbuild's `postinstall` actually executed on this machine (it is default-trusted and `bun pm untrusted` is empty; I did not inspect a log of it).
- The 17-name comparison against npm uses the dev's saved npm list; I did not re-run npm.
- The C6 variant with a truly empty test set (empty `include`); only the filter-no-match case was run.
- Node removal on the GitHub runner: the experiment was local. The plan keeps Node in CI, which is the safe side.
- "Recorded in the PR" items (bun version, Node decision, grep result, hand-checked mismatch and broken assertion): no PR exists yet.
- A machine without Rust or WebView2 installed, an MSI, and the `beforeDevCommand` path in other shells (PowerShell was used for the PATH test, Git Bash for the rest; `tauri dev` was not run).

## 8. Verdict

**Partial.** All acceptance criteria that can be checked before a pull request are met, and no code defect was found. Blocking: BUG-001 (Medium, documentation of the ticket). Open Low: BUG-002, BUG-003. After BUG-001 is fixed and the conflict is handled at merge, the verdict becomes pass without re-running the technical checks (no code or workflow change is needed); a re-read of the ticket lines is enough.

Owner decisions:

| Id | Severity | Decision needed |
|---|---|---|
| BUG-001 | Medium | Fix the ticket text and status (blocks) |
| BUG-002 | Low | How to resolve the `decision-log.md` conflict: merge `develop` first, or resolve at finish |
| BUG-003 | Low | Accept the noisy `bun test` refusal |
| O2 | n/a | Accept two hand-edited places for the bun version, or add a CI check later |
| Pending | n/a | Do the "recorded in the PR" items when the pull request is opened; a second-machine fresh clone is optional |
