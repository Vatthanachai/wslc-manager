---
tags:
  - decisions
---

# Decision log

Source of truth for details: [[ba-analysis]] section 3. This page is the quick view. Dates are when the user decided (2026-11-11 unless noted).

| # | Decision | Status | Related |
|---|---|---|---|
| D1/D2 | MVP supports WSLC only. Docker/Podman is deferred to Phase 2+ behind the same `EngineConnector` trait | Decided | [[REQ-000-walking-skeleton]], [[REQ-001-discover-runtimes]] |
| D3 | Number of distros | Deferred with Docker | |
| D4 | GUI (Tauri 2) | BA default | [[REQ-000-walking-skeleton]] |
| D5 | MVP scope: list, start, stop, restart, logs, stats | BA default | [[REQ-003-list-containers]], [[REQ-004-container-lifecycle]], [[REQ-005-container-logs]], [[REQ-011-container-stats]] |
| D6 | Local only, no remote hosts | BA default | |
| D7 | Auto-start a stopped distro: ask once, remember | BA default, moot until Docker | |
| D8 | Permissions: assume access, clear error otherwise | BA default | [[REQ-008-error-handling]] |
| D9 | Installer: NSIS/MSI via the Tauri bundler | BA default | [[REQ-012-release-and-signing-pipeline]] |
| D10 | Minimum Windows: 11 | Decided (with D14) | |
| D11 | WSL update policy: check, tell the user, ask, run `wsl --update` only after a click. Source of "latest" is option D (local capability check always plus a once-a-day GitHub releases comparison). Banner and daily check on by default, with an off setting. UAC and a possible stop of distros and containers are accepted but must be warned about | Decided | [[REQ-009-wsl-update-check]] |
| D12 | Show the default WSLC session plus all sessions from `wslc system session list`, with the session name visible | Decided (BA default) | [[REQ-010-sessions-panel]] |
| D13 | UI language: English only, strings in one resource module, no i18n in MVP | Decided | [[REQ-000-walking-skeleton]] |
| D14 | Windows 10 is not supported | Decided | |
| D15 | Code signing via SignPath Foundation, MIT licence, public GitHub repo. Apply after a first release exists | Decided | [[REQ-012-release-and-signing-pipeline]] |
| D16 | Changes reach `main` only through a pull request (GitHub ruleset "main: pull request only", id 27014559, created 2026-11-11: PR required, merge commits only, conversations resolved, no direct push, no deletion, no force-push, no bypass actors, 0 required approvals because the project has one owner) | Decided | [[REQ-012-release-and-signing-pipeline]] |
| D17 | Frontend package manager is bun, replacing npm (owner decision 2026-11-11). Done as REQ-014 after REQ-013 closed with npm. Removes the npm 11/12 mismatch between CI and local and the npm install-scripts warning for esbuild (esbuild is on bun's default-trusted list, so its postinstall runs and `trustedDependencies` stays empty). Node 24 stays a prerequisite because Vitest does not run without it (verified by experiment) | Decided | [[REQ-014-switch-package-manager-to-bun]] |
| D18 | UX batch 1 review (2026-11-11): seven design decisions approved (top tabs, status bar, one Runtime panel; focus re-check at most once per 5 s while a gate shows; install/update commands shown as copyable text only; toast code with REQ-004; Segoe Fluent Icons; shortcuts F5/Ctrl+R, Ctrl+I, F6; gates for RuntimeUpdateNeeded and MissingComponents built now). Also accepted: stderr capped at 16 KiB in the error payload; no install-guide link until a separate ticket; no "default" session badge unless a way to know is found; the MissingComponents gate is built although it can only be tested through a fake runner | Decided | [[REQ-001-002-008-ux]], [[REQ-001-discover-runtimes]], [[REQ-002-detect-engine]], [[REQ-008-error-handling]] |

## Repository
- Remote: `https://github.com/Vatthanachai/wslc-manager.git` (public; `main` is PR-only, see D16).
- Branching: git flow (`main`, `develop`, `feature/*`, `bugfix/*`, `release/*`, `hotfix/*`).

Back to [[Home]].
