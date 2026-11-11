---
ticket: REQ-013
status: partial
tags:
  - test-report
---

# REQ-013 test report: CI and QA baseline

- Ticket: [[REQ-013-ci-and-qa-baseline]]. Plan: [[REQ-013-plan]]. Definition of done applied: [[qa/definition-of-done]] (and [[qa/qa-integration-convention]], [[qa/test-strategy]]). Facts: [[ba-analysis]].
- Branch: `feature/REQ-013-ci-and-qa-baseline`, head `9a76266` plus this report and one new test file. Date: 2026-11-11. OS: Windows 11 (10.0.26300), local Rust 1.97.0, Node v24.21.0, npm 12.2.1, wslc 3.0.5.0.

## 1. Summary

The implementation works. Local checks all pass, the three GitHub runs behave as claimed (first run red at clippy, second and third green), every action SHA matches its tag, the branch-policy script gives the right answer on 34 cases without executing any branch name, and the ignored integration example runs no process by default and is read-only when enabled. The UI tests test the real `main.ts`, and the XSS test fails when `main.ts` uses `innerHTML` for list data.

Two things hold the verdict at **partial** (not pass):

- BUG-001 (Medium): the ticket file itself does not meet criteria A1 and C8 (no links to the QA documents, job names not recorded, open questions 1 and 8 unanswered, status still `todo`).
- Criterion A2 is not met for the existing tickets: none of REQ-001 to REQ-012 carries the Definition-of-Done reference line (finding F1, a process item for `ba`, not a code defect).

Three further Low defects and an added mutation-hardening test are listed below. No production code was changed (`git diff` over `src/` and `crates/` is empty apart from the new test file).

## 2. Criterion matrix

Legend: met / not met / pending (needs a step that does not exist yet, such as the PR or an owner action).

| # | Criterion | Result | Evidence |
|---|---|---|---|
| A1 | `obsidians/qa/` has strategy and DoD, and the ticket links to both | **not met** | Both files exist (`obsidians/qa/test-strategy.md`, `definition-of-done.md`, plus `qa-integration-convention.md`). The ticket has no link to either: lines 26 and 102 still say "file names to be filled in". `README.md:92` links them, the ticket does not. See BUG-001 |
| A2 | Later tickets include the DoD reference line, `tester` rejects those without | **not met for existing tickets** | The rule exists (`definition-of-done.md`). Grep for "definition-of-done" in `obsidians/requirements/`: REQ-001 to REQ-012 all lack it (list in F1). Only REQ-013 and the index mention it |
| A3 | Real-wslc tests are `#[ignore]`, run only with `WSLC_INTEGRATION=1` | met | `wslc_integration.rs:34` `#[ignore = "needs real wslc.exe, ..."]`, runtime guard `support/mod.rs:40` `require_integration()`; reason string pinned by test `ignore_reason_matches_the_convention` |
| A4 | Created resources use `aiyara-test-`, removed on success and failure | met at helper level | `unique_name` (mod.rs:55), `Guard` with `Drop` (mod.rs:262), tests `guard_removes_tracked_resources_in_reverse_order`, `guard_cleans_up_when_the_test_panics`. No test creates a resource yet, so the create-then-clean path with real wslc is untested (see section 7) |
| A5 | Never touches non-prefixed resources | met at helper level | `remove_container` refuses (mod.rs:199), `Guard::track` asserts (mod.rs:247), sweep filters through `owned_names`. Mutation checks R1 to R3 below all make a test fail |
| A6 | `cargo test --workspace` without the env var: ignored reported as ignored, no `wslc.exe` started | met | Output "ignored, needs real wslc.exe ...", 1 ignored. Process trace below: a fake `wslc.exe` first on PATH is never started by the default run or by `--ignored` without the env var, and is started (`version`, `container list --all --format json`) with the env var |
| B1 | Vitest with jsdom, pinned by lockfile, `npm test` one-shot, non-zero on failure | met | `package.json` `"test": "vitest run"`, `vitest.config.ts` (jsdom, no `passWithNoTests`), vitest 5.1.0 in lockfile. Non-zero on failure shown by the Vitest probe run on GitHub (fails at `npm test`). "Choice recorded in the PR": pending (no PR yet; recorded in [[REQ-013-plan]] decision 3) |
| B2 | Passes without Tauri, WebView2, Rust or WSL (IPC replaced by a double) | met by reading and CI, not on a bare machine | `main.test.ts:5-6` `vi.mock("@tauri-apps/api/core")`. Ran green on the runner. Not run on a machine without Rust installed |
| B3 | One container: one row with its fields, headers from `strings.ts` | met | `main.test.ts:49` compares header cells to `strings.columns` and the five cells to the fixture. Mutation M4 (swap two headers) makes it fail |
| B4 | Empty list: no data row, empty-state text from `strings.ts` | met | `main.test.ts:89` |
| B5 | Error via `textContent` only, strings from `strings.ts`, XSS payload test | met, with a gap (BUG-003) | `main.test.ts:140,151` assert no `img` element and literal strings. See section 4 for the mutation result |
| B6 | Deliberately broken assertion exits non-zero | met | Mutation M4 gave a failing run; GitHub probe `feature/REQ-013-probe-vitest` failed exactly at `npm test`. "Noted in the PR": pending |
| B7 | Tests import strings, no hard-coded UI English | met | `main.test.ts` uses `strings.*` for every UI text; the remaining literals are fixture data (`aiyara-spike`, `Up 3 seconds`, `paused`) |
| C1 | Triggers | met | `ci.yml:4-12`: `pull_request` to develop and main, push to `feature/**`, `bugfix/**`, `release/**`, `hotfix/**`, `develop`. Runs seen for push (feature and release probes) and `pull_request` (probe PRs) |
| C2 | Windows runner, steps in order | met | `ci.yml:30` `windows-latest`; steps fmt, clippy, test, `npm ci`, `npm run build`, `npm test` at lines 53-73 after checkout, Node, rustup components, cache |
| C3 | Clean checkout: every step passes | met | Run `42984255053` (`3f861d3`) and `42986101835` (`9a76266`): all 6 check steps success |
| C4 | Five deliberate failures fail at the matching step | met | Looked up independently with `gh run view`: rusttest fails at `cargo test`, clippy at `cargo clippy`, fmt at `cargo fmt --check`, vitest at `npm test`, tsbuild at `npm run build` (runs 42985019245, 42985023674, 42985029367, 42985037245, 42985042921). "Recorded in the PR": pending |
| C5 | `permissions: contents: read`, no widening, no secrets, `pull_request` | met | `ci.yml:14-15`, `branch-policy.yml:9-10`; no job-level `permissions`; grep for `pull_request_target`, `secrets`, `GITHUB_TOKEN`, `paths` finds only comments; `persist-credentials: false` on checkout |
| C6 | Actions pinned to full SHA with version comment | met | Three actions, all 40-char SHAs; each verified with `gh api repos/<o>/<r>/git/ref/tags/<tag>`: checkout v7.0.1 and setup-node v7.1.0 are lightweight tags (commit), rust-cache v2.9.3 is an annotated tag and dereferences to `d1527c8f...`. All three MATCH |
| C7 | Cache keys on lockfiles, miss still green, no cache in release builds | met | Run 2: "No cache found" for Cargo, job green. Run 3: Cargo "full match: true" (501 MB), npm "Cache hit" (24 MB). rust-cache key `v0-rust-ci-check-Windows_NT-x64-<hash>` lists `Cargo.lock` and the three `Cargo.toml` files plus Rust version 1.99.1; npm key `node-cache-Windows-x64-npm-<hash of package-lock.json>`. No release workflow exists yet (REQ-012) |
| C8 | Check job name fixed, recorded in the PR and in the ticket | **partly** | `name: ci-check` (`ci.yml:28`), reported by GitHub as `ci-check` (`gh api .../check-runs`). Not recorded in the ticket (BUG-001) |
| C9 | Branch policy: develop, release/*, hotfix/* into main pass | met | Local table (section 3) and real PR run 4 (`release/0.0.0-probe` into main, success) |
| C10 | Any other head into main fails with a message naming the allowed heads | met | Local table; real PR run `42985085685` failed at the check step with "Pull requests into main must come from develop, release/* or hotfix/*." |
| C11 | PR into develop passes or is skipped | met | Real run `42985092869` success; local `base=develop` cases exit 0 |
| C12 | Stable name; head branch via env, no unquoted interpolation | met, one edge (BUG-004) | `branch-policy.yml:26-27` env, `"$HEAD_REF"` quoted in `case`. Grep for `${{` finds only concurrency groups and the two env lines, none inside a `run:` block. Injection payloads tested, nothing executed |
| D | README documents local checks and integration tests | met | `README.md:77-106`. Commands run as written: all exit 0. Integration command run with `WSLC_INTEGRATION=1` (without `--test-threads=1`, which does not change a one-test run) |

## 3. Commands run (real output, abridged)

Local, branch head plus the new test file:

| Command | Result |
|---|---|
| `cargo fmt --check` | exit 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit 0 |
| `cargo test --workspace` | exit 0. 61 passed, 0 failed, 1 ignored (connectors 9, qa_hygiene 8, qa_parse_edge 32, wslc_integration 10 + 1 ignored, two doc/unit binaries 1 each). The ignored one is `list_is_read_only_and_parses` |
| `npm ci` | exit 0, 65 packages, 0 vulnerabilities, warning "install-scripts: esbuild@0.28.2 postinstall blocked, not covered by allowScripts" |
| `npm run build` | exit 0 (`tsc --noEmit` then `vite build`, 6 modules) |
| `npm test` | exit 0. Before my test: 1 file, 14 tests. After: 2 files, 17 tests |

Process trace (fake `wslc.exe` that appends its arguments to a marker file, placed first on PATH):

- `cargo test -p aiyara-engine --test wslc_integration` (no env): 10 passed, 1 ignored, marker file not created.
- Same with `-- --ignored` and no `WSLC_INTEGRATION`: the test returns early ("skipped"), marker file not created.
- Positive control, `WSLC_INTEGRATION=1 ... --ignored`: marker holds `["wslc.exe", "version"]` and `["wslc.exe", "container", "list", "--all", "--format", "json"]`, nothing else.

Real read-only run: `WSLC_INTEGRATION=1 cargo test -p aiyara-engine --test wslc_integration -- --ignored --nocapture` printed `wslc 3.0.5.0` and `WSL version: 3.0.5.0`, test ok. `wslc container list --all --format json` before and after: both empty output (0 bytes), `cmp` identical. This evidence is weak because the machine has no containers (see section 7); the code path itself only issues `version`, `wsl --version` and the list (read-only).

GitHub (read-only `gh`):

- Runs on the branch: `42983631759` (`0cf462d`) failure at `cargo clippy`, later steps skipped; `42984255053` (`3f861d3`) success, 5 min cold; `42986101835` (`9a76266`) success, 1.5 min with warm caches.
- Runner toolchain in the logs: Node v24.22.2, npm 11.19.1, Rust 1.99.1. Local: Node 24.21.0, npm 12.2.1, Rust 1.97.0.
- Tauri crate compiled and tested on the runner without extra WebView2 setup: `cargo clippy` and `cargo test --workspace` passed in runs 2 and 3. The risk "[U] Tauri crate on the runner" is now verified.
- `npm ci` on the runner prints the same esbuild install-scripts warning (text "not yet covered by allowScripts", npm 11.19.1). `npm run build` and `npm test` pass anyway.
- Ruleset 27014559 "main: pull request only", enforcement active, rules `deletion`, `non_fast_forward`, `pull_request`; no required status checks yet.

Branch-policy script (the `run:` body extracted from the YAML, run with `BASE_REF` and `HEAD_REF` set): 34 cases, all as expected, exit codes shown. Highlights:

| base | head | exit | note |
|---|---|---|---|
| main | `develop`, `release/1.0`, `hotfix/urgent`, `release/1.0/rc1`, unicode `hotfix/...` | 0 | allowed |
| main | `feature/x`, `bugfix/x`, `main`, empty | 1 | message names the allowed heads |
| main | `release/a b`, `hotfix/../x`, `release/`, `release/` plus 5000 chars | 0 | pattern match only (git itself would refuse `..` and a trailing slash) |
| main | `HOTFIX/x`, `Develop`, `develop/x`, `release`, `hotfix`, `refs/heads/release/x`, `-n`, `*`, `release*`, `develop ` , ` develop` | 1 | case-sensitive, exact |
| main | `release/a<LF>b` | 0 | |
| main | `feature/x<LF>release/y` | 1 | |
| main | `feature/$(touch pwned)`, `release/$(touch pwned)`, ``release/`id`;x`` | 1, 0, 0 | file `pwned` never created: no command injection |
| develop | `feature/x`, `develop`, empty; empty base | 0 | "nothing to enforce" |

Mutation checks (all restored; `git diff --stat -- src crates` empty afterwards, `cmp` against a backup identical):

| Id | Change (temporary) | Result |
|---|---|---|
| M1 | `cell()` uses `innerHTML` instead of `textContent` | `main.test.ts` "shows container data ... never as markup" FAILS (13 of 14 pass). The XSS test is meaningful for list data |
| M3 | state cell built with `innerHTML` | same test FAILS |
| M4 | swap two column headers | "renders one row with the fields" FAILS |
| M2 | error path `root.innerHTML = errorText(e)` | **all 14 original tests PASS (survivor)**, see BUG-003. With the added file `main.dom-safety.test.ts` it FAILS |
| R1 | `remove_container` prefix check disabled (fake wslc on PATH) | `remove_container_refuses_names_without_the_prefix_before_any_process` FAILS and the fake recorded `container remove --force aiyara-spike`, which proves the test would have started a process |
| R2 | `Guard::track` assert removed | `guard_refuses_to_track_a_resource_without_the_prefix` FAILS |
| R3 | `is_test_resource` always true | 3 tests FAIL |

Test added by QA: `src/main.dom-safety.test.ts` (3 tests): traps `innerHTML`, `outerHTML` and `insertAdjacentHTML` while `main.ts` renders a list and an error, and a self-test that the trap fires. Passes on the clean tree (17 of 17), fails under M1 and M2.

## 4. Defects

### BUG-001 Ticket does not link the QA documents or record the job names (OPEN)
- Ticket: REQ-013   Severity: Medium   Found: 2026-11-11
- Environment: branch head `9a76266`, docs only.
- Steps: open `obsidians/requirements/REQ-013-ci-and-qa-baseline.md`, search for `qa/`, `test-strategy`, `definition-of-done`, `ci-check`, `branch-policy`.
- Expected: criterion A1 "this ticket links to both"; C8 "the final name is recorded in the PR and in this ticket"; [[REQ-013-plan]] T10 says job names are written into the ticket; open questions 1 and 8 answered; DoD checklist "ticket status ... updated".
- Actual: no wikilink to the QA documents (lines 26 and 102 still say names are to be filled in), no `ci-check` or `branch-policy` anywhere in the ticket, open questions 2 to 8 unanswered although the plan decided them, `status: todo`.
- Cause: documentation step not done by dev or the main assistant.
- Owner decision: not applicable (not Low). Fix before finishing the ticket: link `[[qa/test-strategy]]`, `[[qa/definition-of-done]]`, `[[qa/qa-integration-convention]]`, record `ci-check` and `branch-policy`, close the open questions, set status.

### BUG-002 Local and CI toolchains differ (OPEN)
- Ticket: REQ-013   Severity: Low   Found: 2026-11-11
- Environment: local Node 24.21.0, npm 12.2.1, Rust 1.97.0 versus runner Node 24.22.2, npm 11.19.1, Rust 1.99.1.
- Steps: compare `node -v`, `npm -v`, `rustc -V` with the setup logs of run `42986101835`.
- Expected: ticket risk section "Local and CI must use the same Node/npm versions".
- Actual: only the Node major is pinned (`.nvmrc` 24, `engines` `>=24 <25`); npm and Rust are not. The Rust drift already broke CI once (clippy lint `chunks_exact_to_as_chunks`, fixed in `3f861d3`). The esbuild "install-scripts" warning appears with both npm versions and no `allowScripts` allow-list is configured; build and tests pass without it.
- Owner decision (Low): accept, or pin Rust with `rust-toolchain.toml` (pending decision recorded in the plan) and decide whether to pin npm and to add an `allowScripts` entry for esbuild.

### BUG-003 Error-path XSS test cannot detect `innerHTML` (OPEN, mitigated by the added test)
- Ticket: REQ-013   Severity: Low   Found: 2026-11-11
- Steps: change `root.textContent = errorText(e);` to `root.innerHTML = errorText(e);` in `src/main.ts` and run `npm test`.
- Expected: criterion B5 "error shown through `textContent` only" is enforced by a test.
- Actual: 14 of 14 pass. `main.ts` never shows the error message, only fixed strings from `strings.ts`, so a DOM-content check cannot tell the two apart; the XSS tests at `main.test.ts:140,151` pass for any implementation that ignores the message. Production code is safe today.
- Test: `src/main.dom-safety.test.ts` (added by QA) traps the HTML-parsing APIs and fails under the mutation.
- Owner decision: accept the added test (committed with this report).

### BUG-004 `branch-policy` is not re-run when a PR's base branch is changed (OPEN, reasoned, not reproduced)
- Ticket: REQ-013   Severity: Low   Found: 2026-11-11
- Steps (not executed, GitHub writes are not allowed): open a PR from `feature/x` into `develop` (check passes), then change its base to `main`.
- Expected: `branch-policy` fails for the new base.
- Actual (expected from GitHub's documented default activity types `opened`, `synchronize`, `reopened`; a base change is an `edited` event): no new run, so the earlier green `branch-policy` on the same head SHA may satisfy a required check. This is the same family as the shared-head-SHA caveat in [[REQ-013-plan]]. `branch-policy.yml:6-7` has no `types:` list.
- Cause: missing `types: [opened, synchronize, reopened, edited]`.
- Owner decision (Low): accept (the rule is a guard, not a security boundary; sole maintainer) or add `edited`; verify with a throwaway PR if added.

## 5. Findings and notes (not defects of this ticket)

- **F1 (process): tickets lacking the DoD reference line (criterion A2).** All twelve: REQ-001-discover-runtimes, REQ-002-detect-engine, REQ-003-list-containers, REQ-004-container-lifecycle, REQ-005-container-logs, REQ-006-live-updates, REQ-007-safe-destructive-actions, REQ-008-error-handling, REQ-009-wsl-update-check, REQ-010-sessions-panel, REQ-011-container-stats, REQ-012-release-and-signing-pipeline. Per the rule, `tester` will not accept them until a line such as "Definition of done: includes [[qa/definition-of-done]]" is added. Tickets were not edited by QA. REQ-000 also lacks it (older than the rule).
- A fork PR whose head branch is called `develop` or `release/x` passes `branch-policy` (only the branch name is compared, not the head repository). Acceptable for a guard that is not a security boundary; note for the owner.
- Concurrency group uses `github.ref`, so a push run (`refs/heads/...`) and a `pull_request` run (`refs/pull/N/merge`) do not cancel each other; two runs per PR is the accepted trade-off in the ticket.
- The shared-head-SHA caveat from the plan (two PRs from one head to `develop` and `main`) stands. Not retested.
- Hygiene: no Windows user name and no `C:\Users` or `Z:\` paths in workflows, tests, README or `obsidians/qa/` (the strings appear only as the denylist in `qa_hygiene.rs`, as placeholders `<user>` and in the DoD text). The owner's GitHub login and copyright name appear in `LICENSE`, `README.md:114`, `obsidians/decision-log.md:30` and `obsidians/plans/REQ-013-plan.md:25`; this is public owner data, not a Windows user name, flagged only for awareness.
- `.gitignore` covers `target`, `node_modules`, `dist`. `.gitattributes` enforces LF, and `cargo fmt --check` passes on the runner. `git status` shows only others' files (`obsidians/design/`, `obsidians/.obsidian/app.json`, `appearance.json`, `Untitled.base`), which must not be committed.

## 6. Design check

No UI change in this ticket; no screens to compare with `obsidians/design/`.

## 7. Unverified and coverage gaps

- Real create-and-clean path: no test creates a container yet, so the sweep (`sweep()`), real `wslc container remove --force` and the drop guard against a real wslc are untested. The `--force` flag and the `Names` JSON key are [U] in the convention. Only the pure helpers are covered.
- Real-wslc run was on a machine with no containers, so "before and after identical" proves little; the read-only property rests on the code (only `version`, `wsl --version`, `container list`).
- A bare machine without Rust, WebView2 or WSL was not used for `npm test` (B2); the CI runner has Rust installed.
- Cache scoping between PR runs and base branches ([U] in the ticket) was not examined; only hit and miss on one branch were seen. Cache poisoning controls rest on `contents: read` and no secrets.
- BUG-004 is reasoned from GitHub's documentation, not reproduced.
- Not checked: whether `windows-latest` image changes will alter results; Dependabot is out of scope.
- The `wsl.exe` half of the fake-PATH trace was not captured (the system `wsl.exe` was found first); the `wslc.exe` half is the one the criterion names.
- PR-based items (jsdom choice, broken-assertion note, five probes, job names recorded "in the PR") are pending because no PR for this branch exists.

## 8. Verdict

**Partial.** Code, tests and workflows meet criteria A3 to A6, B1 to B7, C1 to C7, C9 to C12 and D, with evidence above. Blocking before the ticket can be closed: BUG-001 (ticket text) and A2 (add the DoD line to REQ-001 to REQ-012, owner or `ba`). Low items BUG-002, BUG-003 (test added), BUG-004 need an owner decision. Owner decisions still open: pin the Rust toolchain; add `ci-check` and `branch-policy` as required checks on ruleset 27014559 after this ticket merges (not before the checks have run on a PR into `main`); optionally pin npm and add `edited` to the branch-policy trigger.
