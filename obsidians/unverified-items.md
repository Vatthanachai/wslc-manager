---
tags:
  - risks
---

# Unverified items

Things not yet proven on a real machine. Update this page when a spike or a ticket settles one, and move the fact into [[ba-analysis]] with a **[V]** label. Original list: [[requirements/README|requirements index]], "Still unverified".

## Behaviour of wslc (WSL 3.0.5.0)
- Failure cases: name collision, port already in use, remove of a running container without `--force`, `kill`, permission-denied. Tickets: [[REQ-004-container-lifecycle]], [[REQ-007-safe-destructive-actions]], [[REQ-008-error-handling]].
- `wslc` missing or WSL too old (not testable here without uninstalling). Tickets: [[REQ-001-discover-runtimes]], [[REQ-009-wsl-update-check]].
- Session list with 0 or 2+ sessions, and the success behaviour of `system session terminate`. Tickets: [[REQ-001-discover-runtimes]], [[REQ-010-sessions-panel]].
- Other `State` values, several networks, several port mappings. Ticket: [[REQ-003-list-containers]].
- Stderr text on non-English Windows (match `WSLC_E_*` tokens only).
- Whether containers created through the C SDK appear in the CLI, and whether sessions persist after logoff or reboot.
- Minimum WSL version for `wslc.exe` (3.0.1 is from a release-notes summary).

## Update check
- A sample GitHub releases API response, and how the release tag relates to what `wsl --update` installs. Ticket: [[REQ-009-wsl-update-check]].

## Release and signing
- SignPath: application steps and turnaround, Tauri/NSIS/MSI signing support, publisher name shown by Windows, roles for a one-person team, reputation threshold. Ticket: [[REQ-012-release-and-signing-pipeline]].

## Deferred, not ticketed
Docker/Podman connector, images and volumes, exec, settings beyond the update check, i18n.

Back to [[Home]].
