---
id: REQ-010
title: "Sessions panel (WSLC)"
priority: Should
connector: wslc
status: todo
depends_on:
  - "[[REQ-003-list-containers]]"
  - "[[REQ-007-safe-destructive-actions]]"
tags:
  - requirement
  - should
---

# REQ-010 Sessions panel (WSLC)

- Priority: Should (US-18)
- Connector scope: WSLC only
- Dependencies: REQ-003, REQ-007

## User story
As an operator, I want to see the WSLC sessions and be able to stop one so that I understand why containers appear or disappear and can clean up.

## Acceptance criteria
- Given WSLC is available, when the panel opens, then it runs `wslc system session list --verbose` and shows ID, Creator PID and Display Name per session [V].
- Given the output is a text table with no JSON option [V], then it is parsed from the header row, and the `[wslc] Found N session` line [V] must match the number of parsed rows, otherwise `ParseError` with raw output in the details.
- Given a display name contains spaces [U], then it is kept whole (split only the first two columns).
- Given zero sessions, then an empty state is shown (the format of the empty output is not captured [U]).
- Given the sessions list, then each session shows how many containers it holds (from `container list` with `--session`).
- Given I choose to terminate a session, then a confirmation dialog (REQ-007 rules) lists the containers that will stop. The command `wslc --session <name> system session terminate` exists [V: it answered with `WSLC_E_SESSION_NOT_FOUND` for a missing session, `obsidians/fixtures/errors/session-terminate-missing.txt`], but its success behaviour (exit code, what happens to the containers, what happens to the default session) is untested [U], so test it on a throwaway session before implementing. Given the session no longer exists at terminate time, then SessionNotFound is shown and the list is refreshed.
- Given a session other than the default one, then it is clearly labelled with its display name everywhere in the UI.
- Given sessions of other Windows users, then they are not shown, and the panel states that only the current user's sessions are visible [R].
- Multi-session behaviour is unverified [U] (only one session existed), so a test with two sessions is required before this ticket is finished.

## Fixtures
- `obsidians/fixtures/errors/session-terminate-missing.txt`.
- `obsidians/fixtures/session-list-verbose.txt` (one session, ID 1, Creator PID 40580, Display Name `wslc-cli-<user>`). Need: a two-session capture and an empty capture.

## Out of scope
- Creating sessions, renaming sessions, session persistence after exit/logoff/reboot (unverified [U]), C SDK sessions.
