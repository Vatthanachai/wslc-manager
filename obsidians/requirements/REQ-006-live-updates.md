---
id: REQ-006
title: "Live status updates"
priority: Must
connector: wslc
status: todo
depends_on:
  - "[[REQ-003-list-containers]]"
tags:
  - requirement
  - must
---

# REQ-006 Live status updates

- Priority: Must (US-6)
- Connector scope: WSLC (Docker deferred)
- Dependencies: REQ-003

## User story
As a developer, I want the container list to update by itself so that I always see the current state.

## Acceptance criteria
- Given a WSLC session, then the app keeps one `wslc --session <name> system events --format json` process alive per session [V that it streams].
- Given an event with `Type` `container` and an `Action` such as `start`, `stop`, `die` or `kill` [V], then the affected container (matched by `id` or `Actor.ID` prefix) is refreshed through `container list`, and the UI updates within 1 s (target).
- Given events with `Type` `network` [V], then they are ignored for the container table (they have no `from`/`id`/`status` [V]).
- Given a restart, then the UI converges to Running after the final `start` event (sequence: kill, kill, disconnect, die, stop, connect, start [V]).
- Given the events process exits or fails, then the app reconnects with backoff, does a full `container list` after reconnecting, and shows a "reconnecting" indicator.
- Given events cannot be started (for example an unsupported version), then the app falls back to polling `container list` every 5 s at most.
- Given the app starts, then a full `container list` is always done first. Events are never the only source of truth.
- Given the app closes or a session disappears, then all event processes are terminated.

## Fixtures
- `obsidians/fixtures/events.jsonl`.

## Out of scope
- Desktop notifications, event history view, filtering events by type in the UI.
