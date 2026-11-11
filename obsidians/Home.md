---
tags:
  - index
---

# wslc-manager

Windows desktop app (Rust, Tauri 2) to manage WSL containers (WSLC). Status: early development. REQ-000 (walking skeleton), REQ-013 (CI and QA baseline) and REQ-014 (bun) are done and merged into `develop`. Next: REQ-001, then REQ-002 and REQ-008. Open the repository's `obsidians/` folder as an Obsidian vault.

## Start here
- [[ba-analysis]]: analysis, architecture, risks, phasing
- [[decision-log]]: D1 to D18 in one place
- [[unverified-items]]: what is still not proven
- [[qa/README|QA]]: test strategy, integration convention, definition of done
- [[qa/ci-and-toolchain|CI and toolchain]]: workflows, pins, caches, caveats, probe procedure
- [[pm/roadmap|Roadmap]]: estimates and critical path
- [[requirements/README|Requirements index]]: tickets, order of implementation (in `requirements/`)
- [[fixtures/README|Fixtures]]: real `wslc` and `wsl` output (WSL 3.0.5.0)
- [[fixtures/errors/README|Error fixtures]]: exit codes and `WSLC_E_*` codes

## Tickets (MVP first)
| Ticket | Priority | Status |
|---|---|---|
| [[REQ-000-walking-skeleton]] | Must | done |
| [[REQ-001-discover-runtimes]] | Must | todo (next) |
| [[REQ-002-detect-engine]] | Must | todo |
| [[REQ-003-list-containers]] | Must | todo |
| [[REQ-004-container-lifecycle]] | Must | todo |
| [[REQ-005-container-logs]] | Must | todo |
| [[REQ-006-live-updates]] | Must | todo |
| [[REQ-007-safe-destructive-actions]] | Must | todo |
| [[REQ-008-error-handling]] | Must | todo |
| [[REQ-009-wsl-update-check]] | Must | todo |
| [[REQ-010-sessions-panel]] | Should | todo |
| [[REQ-011-container-stats]] | Should | todo |
| [[REQ-012-release-and-signing-pipeline]] | Should (before first public release) | todo |
| [[REQ-013-ci-and-qa-baseline]] | Must (before REQ-001) | done |
| [[REQ-014-switch-package-manager-to-bun]] | Must (before REQ-001) | done |

Status here mirrors the `status` field in each ticket's frontmatter. Order of implementation: [[requirements/README|requirements index]].

## Evidence labels
Facts in the analysis carry a label: **[V]** verified on a real machine (WSL 3.0.5.0, 2026-11-11), **[R]** reported by a source or read from code but not run, **[U]** unverified or inferred.

## Working notes
- Project rules: git flow (`develop`, `feature/*`), `main` by pull request only (D16), bun as the only package manager (D17), MIT licence, English UI, Windows 11 only.
- Set a ticket's `status` in its frontmatter (`todo`, `doing`, `done`) and see the board in [[tickets-by-status]].
