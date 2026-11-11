---
id: REQ-004
title: "Start, stop, restart"
priority: Must
connector: wslc
status: todo
depends_on:
  - "[[REQ-003-list-containers]]"
  - "[[REQ-008-error-handling]]"
tags:
  - requirement
  - must
---

# REQ-004 Start, stop, restart

- Priority: Must (US-4)
- Connector scope: WSLC (Docker deferred)
- Dependencies: REQ-003, REQ-008

## User story
As a developer, I want to start, stop and restart a container from the UI so that I do not need a terminal.

## Acceptance criteria
- Given a running WSLC container, when I click Stop, then `wslc --session <name> container stop <name>` runs and the row becomes not running after the command finishes, without a manual refresh.
- Given a running WSLC container, when I click Restart, then the button shows a progress state and the UI does not freeze. A restart took about 10 s on the tested machine when PID 1 ignored SIGTERM [V], so the timeout for stop and restart is at least 30 s.
- Given Stop on a container that is already stopped, then the command exits 0 and prints the name [V, idempotent], and the app treats it as success without an error message.
- Given Stop or Start on a container that does not exist, then the command exits 1 with `WSLC_E_CONTAINER_NOT_FOUND` [V, `container-stop-missing.txt`, `container-start-missing.txt`], and the app shows ContainerNotFound and refreshes the list (REQ-008 mapping).
- Given a stopped WSLC container, when I click Start, then it runs `container start` and the row becomes running.
- Given a command fails or times out, then a typed error and the stderr text are shown, and the row state is refreshed from `container list` (not guessed).
- Given an action is in progress for a container, then its action buttons are disabled to prevent double clicks.
- Given a runtime that lacks an action (per `Capabilities`), then the button is disabled with a tooltip.

## Fixtures
- `obsidians/fixtures/events.jsonl` (the event sequence that a restart produces: kill 15, kill 9, network disconnect, die exitCode 137, stop, network connect, start) for verifying that the UI converges to the right state.
- `obsidians/fixtures/errors/container-stop-missing.txt`, `obsidians/fixtures/errors/container-start-missing.txt`, `obsidians/fixtures/container-list-exited-ports.json` (the exited state to start from).
- Still unverified [U]: Start/Stop permission-denied cases, restart of an exited container, port already in use at Start, name collision.

## Out of scope
- Kill, remove (REQ-007), bulk actions, custom stop timeout setting.
