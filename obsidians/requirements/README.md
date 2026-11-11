# Requirements index

Source analysis: `../ba-analysis.md` (v5). Fixtures: `../fixtures/` (captured on WSL 3.0.5.0 / wslc 3.0.5.0, 2026-11-11).

Decisions in force
- MVP scope is WSLC only. Docker/Podman (DockerCliConnector, distro discovery, bollard, Docker fixtures) is deferred to a later phase. The `EngineConnector` trait and the engine-neutral domain stay so Docker can be added later.
- D11: check WSL version (local capability check always, plus a once-a-day GitHub releases comparison, on by default, with an off setting), tell the user, ask, run `wsl --update` only after a click. UAC and possible stop of distros/containers are accepted but must be warned about and listed.
- D12: show the default WSLC session plus all sessions from `wslc system session list`, with the session name visible.
- D13: UI is English only for now. No i18n in the MVP, but all user-visible strings live in one resource module.
- D14: Windows 11 only, Windows 10 is not supported.
- D15: code signing via SignPath Foundation (free OSS programme), MIT licence, public GitHub repo (REQ-012).

| ID | Title | Priority | Connector scope | Depends on |
|----|-------|----------|-----------------|------------|
| [REQ-000](REQ-000-walking-skeleton.md) | Walking skeleton | Must | WSLC (fake connector) | none |
| [REQ-001](REQ-001-discover-runtimes.md) | WSL/wslc availability, version, sessions | Must | WSLC | REQ-000 |
| [REQ-002](REQ-002-detect-engine.md) | WSLC capabilities by version | Must | WSLC | REQ-001 |
| [REQ-003](REQ-003-list-containers.md) | List containers | Must | WSLC | REQ-002, REQ-008 |
| [REQ-004](REQ-004-container-lifecycle.md) | Start, stop, restart | Must | WSLC | REQ-003 |
| [REQ-005](REQ-005-container-logs.md) | View logs with follow and tail | Must | WSLC | REQ-003 |
| [REQ-006](REQ-006-live-updates.md) | Live status updates | Must | WSLC | REQ-003 |
| [REQ-007](REQ-007-safe-destructive-actions.md) | Safe destructive actions | Must | WSLC | REQ-004 |
| [REQ-008](REQ-008-error-handling.md) | Error handling and states | Must | WSLC | REQ-001 (refined throughout) |
| [REQ-009](REQ-009-wsl-update-check.md) | WSL update check (D11) | Must | WSLC (concerns WSL itself) | REQ-001, REQ-002 |
| [REQ-010](REQ-010-sessions-panel.md) | Sessions panel | Should | WSLC | REQ-003, REQ-007 |
| [REQ-011](REQ-011-container-stats.md) | Container stats | Should | WSLC | REQ-003 |
| [REQ-012](REQ-012-release-and-signing-pipeline.md) | Release and signing pipeline (SignPath, MIT, public repo) | Should (post-MVP, before first public release) | n/a (build and release) | REQ-000, REQ-009 |
| [REQ-013](REQ-013-ci-and-qa-baseline.md) | CI and QA baseline (test strategy, Vitest, GitHub Actions) | Must (before REQ-001) | n/a (test and CI) | REQ-000 |

## Suggested implementation order (walking skeleton first)

1. REQ-000: Tauri shell + Rust core + `EngineConnector` trait + fake connector end to end, with the English string resource module in place.
1b. REQ-013: CI and QA baseline (qa strategy and Definition of Done, Vitest, GitHub Actions checks), before any other ticket. Making its checks required on the `main` ruleset is a later owner step after the first successful run.
2. REQ-001 and REQ-002: availability, versions (UTF-16LE decoding for `wsl.exe`, plain text for `wslc.exe`), sessions, capabilities.
3. REQ-008 (typed errors) together with REQ-003 (list), using the existing fixtures.
4. REQ-004, then REQ-007.
5. REQ-006 (events with polling fallback), then REQ-005 (logs).
6. REQ-009 (update check), independent after REQ-001 and REQ-002, so it can run in parallel with steps 3 to 5.
7. Release gate: REQ-012, after the MVP and before the first public release. Order inside it: repository hygiene (LICENSE, README with Code signing policy and privacy text), CI workflow, unsigned first release, then the SignPath application once a release exists.
8. Should items: REQ-010, REQ-011 (can run in parallel with REQ-012).

## Deferred (not ticketed yet)
Docker/Podman connector and its tickets (distro discovery, engine detection, list/lifecycle/logs/events/stats for Docker), images/volumes, exec, settings beyond the update check, i18n. Also not ticketed: real-`wslc` tests in CI (no wslc on GitHub runners, they stay local with `WSLC_INTEGRATION=1`), code coverage thresholds, and the owner step of adding the REQ-013 jobs as required checks on the `main` ruleset.

## Phase 0 results (verified, see `../fixtures/errors/README.md`)
Captured: `wsl --version` and `wsl -l -v` (UTF-16LE), `wslc version`, failure fixtures for missing container/session/image and usage errors, an exited container with a published port. Used by REQ-001, REQ-002, REQ-003, REQ-004, REQ-008, REQ-010.

## Still unverified before or during implementation (WSLC only)
- Failure cases: name collision, port already in use, remove of a running container without `--force`, `kill`, permission-denied, `wslc` missing or WSL too old (REQ-004, REQ-007, REQ-008).
- Session list with 0 or 2+ sessions, and success behaviour of `system session terminate` (REQ-001, REQ-010).
- Other `State` values, multiple networks and multiple port mappings (REQ-003).
- A sample GitHub releases API response for the latest-WSL comparison (REQ-009).
- SignPath: application steps, Tauri/NSIS/MSI signing support, one-person team roles (REQ-012).

## Ticket conventions

Each ticket lists id, priority (MoSCoW), user story, Given/When/Then criteria, dependencies, connector scope, fixtures, and out-of-scope notes. Evidence labels [V]/[R]/[U] follow `../ba-analysis.md`: [V] means verified on WSL 3.0.5.0, 2026-11-11.
