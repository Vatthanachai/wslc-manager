---
id: REQ-008
title: "Error handling and states"
priority: Must
connector: wslc
status: todo
depends_on:
  - "[[REQ-001-discover-runtimes]]"
tags:
  - requirement
  - must
---

# REQ-008 Error handling and states

- Priority: Must (US-8)
- Connector scope: WSLC (Docker deferred)
- Dependencies: REQ-001 (refined throughout)

## User story
As a user, I want clear messages and next actions when something is not available so that I am never left with a raw error or a frozen screen.

## Acceptance criteria
- Given any connector failure, then it is mapped to one typed error: WslMissing, WslcMissing, PermissionDenied, Timeout, ParseError, RuntimeUnavailable, UnsupportedOperation, SessionNotFound, ContainerNotFound, ImageNotFound, InternalError, CommandFailed.
- Given a command exits with code 0, then it is a success, even when nothing changed (for example `container stop` on an already stopped container [V], `container-start-missing.txt` shows the failure side).
- Given a command exits with code 1 [V: every failure exits 1] and stderr contains an `Error code:` line with `WSLC_E_CONTAINER_NOT_FOUND`, `WSLC_E_SESSION_NOT_FOUND` or `WSLC_E_IMAGE_NOT_FOUND` [V], then the error is ContainerNotFound, SessionNotFound or ImageNotFound respectively.
- Given an `Error code:` line with any other `WSLC_E_*` token, then the error is `CommandFailed` carrying that token, shown in the details expander.
- Given the error is decided, then only the exit code and the `WSLC_E_*` token are used. English message text is never matched, because messages may be localised on non-English Windows [U]. The user-facing message comes from the single English string resource module (D13).
- Given `container stats <missing>` prints only an empty line and the code line [V], then the same token mapping still applies and no message text is expected.
- Given `container inspect <missing>` exits 1 with `[]` on stdout and no `Error code:` line [V], then it maps to ContainerNotFound (exit 1 plus an empty `[]`), not to a parse failure.
- Given a usage error (bad `--format`, unknown option, unknown command: exit 1, usage text, no `Error code:` line [V]), then it maps to InternalError (a bug in our command line), and the raw stderr is available in the details.
- Given WSL or `wslc.exe` is missing or too old, then the UI shows that state with the matching action (install link, or the update flow of REQ-009) and a Retry action. How `wslc` missing or an old WSL looks to the app is unverified [U].
- Given a command fails, then the user sees a short message, and the raw stderr and exit code are in a details expander, never as the main text.
- Given a parse failure (for example `wslc system session list` output has an unexpected shape), then the error is `ParseError` and the raw output is available in the details.
- Given `wslc` JSON has unknown keys or missing keys, then unknown keys are ignored and missing keys become empty values, not a failure [U for other WSL versions].
- Given one session fails, then the others keep working and the failed one shows its own error state.
- Given a timeout (5 s for list/stats/inspect/session list, at least 30 s for stop/restart), then the process is killed and `Timeout` is shown.
- Given all user-visible error text, then it comes from the single English string resource module (D13).
- Given the app is offline from the network, then no feature except the optional update check (REQ-009) is affected.

## Fixtures
- `obsidians/fixtures/errors/container-stop-missing.txt`, `container-start-missing.txt`, `container-remove-missing.txt`, `container-logs-missing.txt`, `container-stats-missing.txt`, `container-inspect-missing.txt`, `container-list-bad-session.txt`, `session-terminate-missing.txt`, `container-run-missing-image.txt`, `image-pull-missing.txt`, `container-list-bad-format.txt`, `container-list-bad-option.txt`, `unknown-command.txt`, `wslc-version.txt` (each file holds COMMAND, EXIT, STDOUT, STDERR, see `obsidians/fixtures/errors/README.md`).
- Still to capture [U]: name collision (`run --name` twice), port already in use, remove of a running container without `--force`, `kill`, permission-denied cases, `wslc` missing or WSL too old, session list with 0 or 2+ sessions.

## Out of scope
- Crash reporting or telemetry, automatic repair of the engine, translating CLI messages.
