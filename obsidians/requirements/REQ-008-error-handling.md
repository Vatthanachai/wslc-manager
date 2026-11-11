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
- User interface: yes (gate, banner, session strip, Details disclosure). Design: [[REQ-001-002-008-ux]] (status `ready`). Toast is specified there but its code is written with REQ-004.

## User story
As a user, I want clear messages and next actions when something is not available so that I am never left with a raw error or a frozen screen.

## Decisions (BA, after the owner approved the UX on 2026-11-11)
- Typed error kinds in this batch: WslMissing, WslcMissing, WslTooOld (reserved, produced only by a runtime signal [U]), RuntimeUpdateNeeded [R], MissingComponents [U], Timeout, ParseError, RuntimeUnavailable, UnsupportedOperation, SessionNotFound, ContainerNotFound, ImageNotFound, InternalError, CommandFailed. RuntimeUpdateNeeded and MissingComponents use the same gate template as the other availability gates (see REQ-001). The evidence for RuntimeUpdateNeeded is only the token `WSLC_E_RUNTIME_UPDATE_NEEDED` [R, not observed]; for MissingComponents there is none [U].
- PermissionDenied has no fixture and no detection rule, so it is NOT detected. Any such failure is shown as CommandFailed or InternalError with Details. The kind name stays reserved, marked [U] and non-blocking, and no UI copy for it is required in this batch.
- Error payload (new fields on `AppError`; REQ-000 has only `kind` plus raw/stderr in some variants). The backend returns, per error:
  - `kind`: one of the kinds above.
  - `code`: the `WSLC_E_*` token, or null.
  - `exit_code`: integer, or null (for example a spawn failure or a timeout).
  - `stderr` (raw output): text, truncated to a size cap of 16 KiB with a `truncated` flag. The UI shows and copies exactly what the payload holds.
  - `session`: session name, or null.
  - `command`: program and arguments as a list, shown only in Details. It contains no environment variables. It may contain container names and paths, so Details carries the privacy note.
  - for ParseError only: `expected_count` and `found_count`.
  - Nothing else is serialized to the UI (no environment, no full process info, no stack trace). Fields not needed for display are not added.
- Concurrency: per-session commands run concurrently with a limit of 4, each with its own timeout (5 s for reads). Each session's rows are shown as they arrive. A slow or failing session shows its own strip and never blocks the others.
- Stale data: when a refresh fails after a success, the last rows stay and the text "Showing the last known data." is shown under the error banner.
- No URL is opened in this batch, so no criterion requires an external link.
- Facts marked [U] are unverified.

## Acceptance criteria
- Given any connector failure, then it is mapped to one typed error from the list in Decisions. The set of kinds is fixed in one place and every kind has a presentation and a message key.
- Given a command exits with code 0, then it is a success, even when nothing changed (for example `container stop` on an already stopped container [V], `container-start-missing.txt` shows the failure side). No error UI of any kind is shown.
- Given a command exits with code 1 [V: every failure exits 1] and stderr contains an `Error code:` line with `WSLC_E_CONTAINER_NOT_FOUND`, `WSLC_E_SESSION_NOT_FOUND` or `WSLC_E_IMAGE_NOT_FOUND` [V], then the error is ContainerNotFound, SessionNotFound or ImageNotFound respectively.
- Given an `Error code:` line with `WSLC_E_RUNTIME_UPDATE_NEEDED` [R], then the error is RuntimeUpdateNeeded, shown as a gate.
- Given an `Error code:` line with any other `WSLC_E_*` token, then the error is `CommandFailed` carrying that token in `code`, shown in the Details disclosure.
- Given the error is decided, then only the exit code and the `WSLC_E_*` token are used. English message text is never matched, because messages may be localised on non-English Windows [U]. The user-facing message comes from the single English string resource module (D13). The UI never reads stderr to decide anything.
- Given `container stats <missing>` prints only an empty line and the code line [V], then the same token mapping still applies, no message text is expected, and Details shows "(empty)" for the missing message.
- Given `container inspect <missing>` exits 1 with `[]` on stdout and no `Error code:` line [V], then it maps to ContainerNotFound (exit 1 plus an empty `[]`), not to a parse failure, and the "Error code" row is omitted in Details.
- Given a usage error (bad `--format`, unknown option, unknown command: exit 1, usage text, no `Error code:` line [V]), then it maps to InternalError (a bug in our command line), and the raw stderr is available in the Details.
- Given WSL or `wslc.exe` is missing, or a runtime-update signal is received, then the UI shows the matching gate with the copyable command text (`wsl --install` or `wsl --update`, [U] exact text, never run by the app) and a Retry action. How `wslc` missing or an old WSL looks to the app is unverified [U].
- Given a command fails, then the user sees a short message, and the raw stderr, exit code and other fields are in a Details disclosure (closed by default), never as the main text.
- Given Details is opened, then it shows only the rows that exist from: Error type, Error code, Exit code, Session, Command (program and arguments), and "Output from wslc" in a `<pre>` (shows "(empty)" when stderr is empty, and a "truncated" marker when `truncated` is true). Under it is the note "Details can include your Windows user name and file paths. Review them before sharing." The message above Details contains none of these values.
- Given the "Copy details" button is pressed, then a plain-text block of the visible Details content is copied to the clipboard and the button shows "Details copied" for 2 s (announced through a status live region).
- Given the error payload is serialized to the UI, then it contains only the fields listed in Decisions. A test asserts that no other field (for example environment variables) is present.
- Given a parse failure (for example `wslc system session list` output has an unexpected shape or the count line does not match), then the error is `ParseError`, the raw output is in the Details, and for the session list the Details also show the expected and found counts. The banner has a Retry that re-runs session discovery only.
- Given the error scope, then the presentation follows one rule: the whole app cannot use WSLC = gate; a whole data set (session discovery, list for all sessions) = banner above the content; one session = session strip; one user action = toast (code arrives with REQ-004); a bug in our own command = banner with Copy details emphasised.
- Given `wslc` JSON has unknown keys or missing keys, then unknown keys are ignored and missing keys become empty values (an empty cell), not a failure [U for other WSL versions].
- Given several sessions and one of them fails or is slow, then the others keep working, rows of each session appear as that session answers (at most 4 session commands run at the same time), and the failed one shows its own strip with a Retry that re-lists only that session. Given 5 sessions where one never answers, then the other four show rows within their own timeouts and the stuck one shows a Timeout strip after 5 s.
- Given a refresh fails after an earlier success, then the last rows stay visible, an error banner appears above them with "Showing the last known data.", and the rows are not cleared.
- Given a timeout (5 s for list/stats/inspect/session list, at least 30 s for stop/restart), then the process is killed and `Timeout` is shown, with the timeout seconds in Details.
- Given Retry is pressed while its request is in flight, then the click is ignored and the button shows a busy state ("Retrying...", "Still working..." after 3 s).
- Given all user-visible error text, then it comes from the single English string resource module (D13). An unknown error kind falls back to "Something went wrong." with Details.
- Given the app is offline from the network, then no feature except the optional update check (REQ-009) is affected, and no offline message is shown.

## Fixtures
- `obsidians/fixtures/errors/container-stop-missing.txt`, `container-start-missing.txt`, `container-remove-missing.txt`, `container-logs-missing.txt`, `container-stats-missing.txt`, `container-inspect-missing.txt`, `container-list-bad-session.txt`, `session-terminate-missing.txt`, `container-run-missing-image.txt`, `image-pull-missing.txt`, `container-list-bad-format.txt`, `container-list-bad-option.txt`, `unknown-command.txt`, `wslc-version.txt` (each file holds COMMAND, EXIT, STDOUT, STDERR, see `obsidians/fixtures/errors/README.md`).
- Still to capture [U]: name collision (`run --name` twice), port already in use, remove of a running container without `--force`, `kill`, `wslc` missing or WSL too old, session list with 0 or 2+ sessions, a real `WSLC_E_RUNTIME_UPDATE_NEEDED` output [R]. Permission-denied is no longer a capture target for this batch (not detected).
- RuntimeUpdateNeeded, MissingComponents, multi-session timeouts and the 16 KiB truncation use an injected fake runner or synthetic input.

## Spikes and manual checks still needed
- Real `wslc` missing, WSL missing, WSL too old and runtime-update-needed outputs (see REQ-001 spikes) [U]/[R].
- Whether a failure caused by missing rights can be told from CommandFailed by exit code or token [U]; until then it is not detected.
- Real behaviour of 2+ sessions with one slow session and the concurrency limit of 4 [U].
- Tracked in [[unverified-items]].

## Definition of done
- This ticket is done only when the checklist in [[qa/definition-of-done]] is met.

## Out of scope
- Crash reporting or telemetry, automatic repair of the engine, translating CLI messages, detecting permission-denied, opening external links, the toast component (REQ-004).
