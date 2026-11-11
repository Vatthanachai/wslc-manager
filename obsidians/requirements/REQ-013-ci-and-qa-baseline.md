---
id: REQ-013
title: "CI and QA baseline"
priority: Must
connector: n/a
status: todo
depends_on:
  - "[[REQ-000-walking-skeleton]]"
tags:
  - requirement
  - must
---

# REQ-013 CI and QA baseline

- Priority: Must (it blocks the quality of every later ticket; do it before REQ-001)
- Connector scope: not applicable (test, build and repository work)
- Dependencies: REQ-000 (a buildable app with Rust tests and a TypeScript UI in `src/`)
- User interface: no new screens. Only existing `src/main.ts` rendering paths get tests, so the `ux` agent is not needed.

## Decisions and facts
- Windows 11 only (D14); CI runs on GitHub-hosted Windows runners because Tauri needs WebView2 and the MSVC toolchain.
- The main ruleset "main: pull request only" (id 27014559, D16) exists. Required status checks on it are added later, by the owner (see Follow-up).
- Caches are allowed for the CI checks of this ticket. REQ-012's release and signing pipeline must NOT restore caches into signed builds.
- Real `wslc.exe` is not available on GitHub runners [U, expected]. Real-wslc tests stay local and manual.
- The test strategy and Definition of Done are written separately by the `tester` agent under `obsidians/qa/`. This ticket references them and does not duplicate them. Exact file names are to be filled in once they exist (see open questions).
- Facts marked [U] are unverified.

## User story
As the maintainer, I want a written test strategy, a frontend test runner and an automatic CI check on every pull request so that regressions are caught before they reach `develop` or `main`, and every later ticket is judged by the same Definition of Done.

## Acceptance criteria

A. Test strategy and Definition of Done
- Given `obsidians/qa/`, then it contains the test strategy and the Definition of Done (written by `tester`), and this ticket links to both.
- Given any ticket REQ-001 and later, when it is written or updated, then its Definition of Done includes the qa Definition of Done by reference (a line linking to it), and a ticket without that line is not accepted by `tester`.
- Given the integration test convention, then real-`wslc.exe` tests are marked `#[ignore]` by default, and they run only when the environment variable `WSLC_INTEGRATION=1` is set (the exact name is fixed in the strategy document).
- Given an integration test runs, then every resource it creates has a name with the fixed prefix `aiyara-test-`, and the test removes what it created on success and on failure (cleanup also runs when an assertion fails).
- Given an integration test runs on a machine with existing containers or sessions, then it never starts, stops, removes or modifies any container or session whose name does not start with `aiyara-test-`. A test that needs such an action on a non-prefixed resource is rejected in review.
- Given `cargo test --workspace` runs without `WSLC_INTEGRATION`, then ignored integration tests are reported as ignored, not as failed or passed, and no `wslc.exe` process is started.

B. Frontend test runner
- Given `package.json`, then it has a Vitest test runner using jsdom or happy-dom (the choice is recorded in the PR), pinned by the lockfile, and `npm test` runs once and exits (no watch mode) with a non-zero exit code on failure.
- Given a clean checkout and `npm ci`, when `npm test` runs, then it passes without Tauri, WebView2, Rust or WSL installed (the Tauri IPC layer is replaced by a test double).
- Given `main.ts` renders a list containing the one fake container, when the test runs, then the table shows one row with that container's fields (name, status and the other fields the skeleton shows), and the column headers come from `src/strings.ts`.
- Given `main.ts` receives an empty list, when the test runs, then no data row is rendered and the empty-state text comes from `src/strings.ts`.
- Given `main.ts` receives an error from the command, when the test runs, then the error is shown through `textContent` only, using strings from `src/strings.ts`, and a test passes an error message containing `<img src=x onerror=alert(1)>` and asserts that no element was created from it and the text is shown literally.
- Given a deliberately broken assertion in one UI test, when `npm test` runs, then it exits non-zero (checked once by hand, noted in the PR).
- Given the UI test files, then no user-visible English literal is hard-coded in them where `src/strings.ts` provides it (tests import the strings).

C. GitHub Actions CI
- Given a pull request targeting `develop` or `main`, or a push to a branch matching `feature/**`, `bugfix/**`, `release/**` or `hotfix/**`, then the CI workflow runs. A push to `develop` or `main` directly does not need to trigger it.
- Given the workflow, then the check job runs on a GitHub-hosted Windows runner (`windows-latest` or a pinned Windows 11-class image [U, to be confirmed in the PR]) and runs, in order: `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `npm ci`, `npm run build`, `npm test`.
- Given a clean checkout of the branch, when the workflow runs, then every step passes.
- Given a deliberately introduced failure (a failing Rust test, a clippy warning, a formatting difference, a failing Vitest test, a TypeScript build error), when the workflow runs on that branch, then the job fails at the matching step. Each of the five is shown once (for example in a throwaway branch) and recorded in the PR.
- Given the workflow file, then it declares `permissions: contents: read` at workflow level and no job widens it, no secrets are used, and the `pull_request` trigger is used (not `pull_request_target`).
- Given the workflow file, then every third-party action is pinned to a full commit SHA [U: or at least a fixed major version, owner to decide, see open questions], with the version in a comment.
- Given Rust and npm caching is enabled, then the cache key includes the lockfiles (`Cargo.lock`, `package-lock.json`), a cache miss still gives a green build, and caches are not used for any release build (REQ-012).
- Given the check job, then its name is fixed in the workflow (for example `ci / check`; the final name is recorded in the PR and in this ticket) and is not changed afterwards without updating the main ruleset, because it will be used as a required status check.
- Given the branch policy job (can be a separate job and a separate workflow), when a pull request targets `main` and its head branch is `develop`, `release/*` or `hotfix/*`, then the job passes.
- Given the same job, when a pull request targets `main` and its head branch is anything else (for example `feature/x`), then the job fails with a message naming the allowed head branches.
- Given a pull request that targets `develop`, then the branch policy job passes or is skipped, and never blocks it.
- Given the branch policy job, then it has a stable name usable as a required status check, and the head branch name is read from the event context and not interpolated unquoted into a shell command (no script injection through branch names).

D. Documentation
- Given the README or a contributing note, then it states how to run the same checks locally (the exact commands above) and how to run the integration tests (`WSLC_INTEGRATION=1`, prefix, cleanup rule).

## Follow-up acceptance criterion (separate step, by the owner's decision, not automatic)
- Given the CI check job and the branch policy job have each run successfully at least once on GitHub, when the owner decides, then the owner adds both job names as required status checks on the "main: pull request only" ruleset (id 27014559) and records it in `obsidians/decision-log.md` (D16 update or a new decision). This step is not done by the pipeline and not before the checks exist, otherwise no PR could ever be merged into `main`. Whether to also require them on `develop` is an open question.

## Fixtures
- None. UI tests use the same single fake container the skeleton returns, or `obsidians/fixtures/container-list.json` mapped to the UI type.

## Edge cases
- A job name that changes (renamed job or matrix added) silently makes a required check never report, so PRs wait forever. Name changes must go through the ruleset.
- A required check that is skipped by path filters or a trigger rule stays "pending". The workflow must not use path filters for the required jobs.
- A feature branch with an open PR gets two runs (push and pull_request). Acceptable, or use concurrency groups with cancel-in-progress for the same ref [U, choice in PR].
- Pull request from a fork: has read-only token, no secrets, and no need for any in this ticket.
- `develop` to `main` PRs from `release/*` or `hotfix/*` branches with names containing odd characters or very long names.
- Formatting differences caused by line endings (CRLF versus LF) between local Windows and the runner. A `.gitattributes` or `rustfmt` setting may be needed [U].
- Vitest finds no test files: must fail, not pass silently (use the default behaviour, do not enable `passWithNoTests`).
- Integration tests left over after a crash: leftover `aiyara-test-*` resources must be identifiable and removable by a documented cleanup command, and a test must not fail because an earlier run left a resource with the same name.
- Tests that depend on the machine's real WSL state would break CI. Unit tests must not call `wsl.exe` or `wslc.exe`.

## Risks
- Cost and time of Windows runners: public repository minutes on GitHub-hosted runners are free for standard runners [U, check the current GitHub billing page], but Windows jobs are slower than Linux and the Rust build can be long. Mitigation: caching, and no hard time limit as a criterion, only a job `timeout-minutes` as a safety net.
- Cache poisoning: a cache written by a pull request run could be restored in a later trusted run. Mitigation: caches only for CI checks, key on lockfile hashes, no cache in release builds (REQ-012), `contents: read` only, and no secrets in these workflows [U: exact scoping rules of GitHub caches between PR and base branches to be checked].
- WebView2 on runners: the CI checks in this ticket do not start the app window, so no WebView2 is needed at runtime [U, `cargo test` and `clippy` for a Tauri crate still compile against the Tauri/WebView2 loader; confirm on the first run]. Any later test that opens a real window on a runner is expected to be flaky and is out of scope.
- npm 12 blocks dependency install scripts by default (for example esbuild, which Vite and Vitest use), and the REQ-000 dev saw this as a warning [U, exact behaviour and config key to be confirmed]. Risk: `npm ci` passes but the build or tests fail on CI because the esbuild binary is not set up. Mitigation: pin the Node and npm versions in CI, explicitly allow the install scripts that are needed (an allow-list in the project configuration, reviewed), and verify on a clean runner. Local and CI must use the same Node/npm versions.
- Unpinned or moving tags of actions (supply chain). Mitigation: pin versions, review updates (Dependabot for actions is optional, see open questions).
- Branch policy job can be bypassed by editing the workflow in the same PR. Accepted: the owner is the only maintainer and the ruleset has no bypass actors but also 0 required approvals (D16).
- Making the checks required before they have run once would block all merges into `main`. Mitigation: the follow-up is a separate, owner-decided step.

## Out of scope
- Release builds, installers and signing (REQ-012).
- Running real-`wslc` tests in CI (stay local and manual).
- Code coverage thresholds and coverage reports.
- Linux or macOS runners, self-hosted runners, end-to-end tests that open the app window.
- Dependency licence checks (REQ-012).

## Open questions
1. File names of the test strategy and Definition of Done under `obsidians/qa/` (from `tester`). The ticket links to them once they exist.
2. Pin actions to a full commit SHA, or to a major version tag? SHA is safer, tags are easier to maintain. Owner decides.
3. jsdom or happy-dom? Either meets the criteria. Default suggestion: jsdom, for the most complete DOM behaviour; happy-dom if speed matters.
4. Should the CI check also be required on `develop`, and should pushes to `develop` and `main` also run CI?
5. Which Node and npm versions are pinned (npm 12 is mentioned by the owner [U])? Where is the pin stored (`.nvmrc`, `engines`, workflow)?
6. Is `windows-latest` acceptable, or pin a specific Windows image so that a runner image change does not break CI unexpectedly?
7. Dependabot (or Renovate) for GitHub Actions and Cargo/npm updates: include now or later?
8. Final job names for the required checks (suggested: `ci / check` and `branch-policy / head-branch`).
