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

## Availability and error UI (REQ-001, REQ-002, REQ-008), added 2026-11-11
- `wslc system session list` output with 0 sessions (is it `[wslc] Found 0 sessions` with no table?) and with 2+ sessions (count line wording). 0 sessions is decided to be a success [R from source: OpenOrCreateDefaultSession].
- How to identify the session the CLI uses when `--session` is omitted (optional "default" badge). Known [V]: one session ID 1, PID 40580, `wslc-cli-<user>`; the CLI resolves the default from the caller's token.
- A real `WSLC_E_RUNTIME_UPDATE_NEEDED` output [R, token only], any real signal for WslTooOld, and any rule for MissingComponents [U, no evidence].
- How a missing `wslc.exe` and a missing WSL look to the app (spawn error, exit code).
- `wsl --version` timing on a cold machine against the 3 s target.
- Exact text of `wsl --install` (displayed only, never run by the app in this batch).
- Whether a permission-denied failure can be told from CommandFailed (not detected for now).
- Window focus event behaviour in WebView2 (used for the gate re-check, at most once per 5 s).
- Behaviour with a wslc version other than 3.0.5.0, and whether the wslc and WSL versions can differ.
- Real 2+ session behaviour with one slow session and the concurrency limit of 4.
- Tickets: [[REQ-001-discover-runtimes]], [[REQ-002-detect-engine]], [[REQ-008-error-handling]]. Design: [[REQ-001-002-008-ux]].

## Update check
- A sample GitHub releases API response, and how the release tag relates to what `wsl --update` installs. Ticket: [[REQ-009-wsl-update-check]].

## Release and signing
- SignPath: application steps and turnaround, Tauri/NSIS/MSI signing support, publisher name shown by Windows, roles for a one-person team, reputation threshold. Ticket: [[REQ-012-release-and-signing-pipeline]].

## Deferred, not ticketed
Docker/Podman connector, images and volumes, exec, settings beyond the update check, i18n.

Back to [[Home]].
