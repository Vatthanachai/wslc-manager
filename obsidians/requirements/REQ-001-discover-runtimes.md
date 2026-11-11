---
id: REQ-001
title: "WSL/wslc availability, version and sessions"
priority: Must
connector: wslc
status: todo
depends_on:
  - "[[REQ-000-walking-skeleton]]"
tags:
  - requirement
  - must
---

# REQ-001 WSL/wslc availability, version and sessions

- Priority: Must (US-1)
- Connector scope: WSLC (Docker/Podman and WSL distro discovery are deferred to a later phase)
- Dependencies: REQ-000

## User story
As a user, I want the app to tell me whether WSL and `wslc.exe` are available, which versions I have, and which WSLC sessions exist, so that I know whether the dashboard can work and what it will show.

## Acceptance criteria
- Given WSL is installed, when the app starts, then the WSL version from `wsl --version` is shown within 3 s. The output is UTF-16LE [V] and must be decoded without stray NUL characters.
- Given `wslc.exe` is present, then its version is shown (`wslc version` prints `wslc 3.0.5.0` as plain text, exit 0 [V]) and WSLC is marked Available.
- Given `wslc.exe` is not found, then WSLC is marked "not available" with a hint (and a pointer to the update flow in REQ-009 when the cause may be an old WSL). The app does not crash.
- Given WSL is not installed, then the app shows a message with an install link and does not crash.
- Given WSLC is available, then sessions are discovered with `wslc system session list --verbose` [V]: the table is parsed into ID, Creator PID and Display Name, and the `[wslc] Found N session` line [V] must match the parsed row count, otherwise `ParseError` with raw output in the details.
- Given the default session, then it is shown as "default" alongside named sessions (D12). Whether a default session exists on a clean machine is unverified [U].
- Given a detected version other than the tested 3.0.5.0, then a non-blocking "untested version" notice is shown [U for other versions].
- Given availability states, then the result is one of Available, WslNotInstalled, WslcMissing, WslTooOld, Unavailable, and each has an English message from the single string resource module.

## Fixtures
- `obsidians/fixtures/session-list-verbose.txt` (one session). A two-session and an empty-session capture are still needed.
- `obsidians/fixtures/wsl-version.utf16.txt` (UTF-16LE), `obsidians/fixtures/errors/wslc-version.txt`. `obsidians/fixtures/wsl-list-verbose.utf16.txt` is available but only needed by the deferred distro discovery.
- Still unverified [U]: behaviour when `wslc.exe` is missing or WSL is too old (not testable without uninstalling).

## Out of scope
- WSL distro discovery (`wsl -l -v`), Docker/Podman detection, starting distros, hiding `docker-desktop` distros (all deferred with Docker), installing or updating WSL (REQ-009).
