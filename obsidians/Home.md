---
tags:
  - index
---

# wslc-manager

Windows desktop app (Rust, Tauri 2) to manage WSL containers (WSLC). Status: planning, Phase 0. Open the repository's `obsidians/` folder as an Obsidian vault.

## Start here
- [[ba-analysis]]: analysis, architecture, risks, phasing
- [[decision-log]]: D1 to D15 in one place
- [[unverified-items]]: what is still not proven
- [[qa/README|QA]]: test strategy, integration convention, definition of done
- [[pm/roadmap|Roadmap]]: estimates and critical path
- [[requirements/README|Requirements index]]: tickets, order of implementation (in `requirements/`)
- [[fixtures/README|Fixtures]]: real `wslc` and `wsl` output (WSL 3.0.5.0)
- [[fixtures/errors/README|Error fixtures]]: exit codes and `WSLC_E_*` codes

## Tickets (MVP first)
| Ticket | Priority |
|---|---|
| [[REQ-000-walking-skeleton]] | Must |
| [[REQ-001-discover-runtimes]] | Must |
| [[REQ-002-detect-engine]] | Must |
| [[REQ-003-list-containers]] | Must |
| [[REQ-004-container-lifecycle]] | Must |
| [[REQ-005-container-logs]] | Must |
| [[REQ-006-live-updates]] | Must |
| [[REQ-007-safe-destructive-actions]] | Must |
| [[REQ-008-error-handling]] | Must |
| [[REQ-009-wsl-update-check]] | Must |
| [[REQ-010-sessions-panel]] | Should |
| [[REQ-011-container-stats]] | Should |
| [[REQ-012-release-and-signing-pipeline]] | Should (before first public release) |
| [[REQ-013-ci-and-qa-baseline]] | Must (before REQ-001) |
| [[REQ-014-switch-package-manager-to-bun]] | Must (before REQ-001) |

## Evidence labels
Facts in the analysis carry a label: **[V]** verified on a real machine (WSL 3.0.5.0, 2026-11-11), **[R]** reported by a source or read from code but not run, **[U]** unverified or inferred.

## Working notes
- Project rules: git flow (`develop`, `feature/*`), MIT licence, English UI, Windows 11 only.
- Set a ticket's `status` in its frontmatter (`todo`, `doing`, `done`) and see the board in [[tickets-by-status]].
