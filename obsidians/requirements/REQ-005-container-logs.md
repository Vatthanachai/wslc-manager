---
id: REQ-005
title: "View logs with follow and tail"
priority: Must
connector: wslc
status: todo
depends_on:
  - "[[REQ-003-list-containers]]"
tags:
  - requirement
  - must
---

# REQ-005 View logs with follow and tail

- Priority: Must (US-5)
- Connector scope: WSLC (Docker deferred)
- Dependencies: REQ-003

## User story
As a developer, I want to read and follow a container's logs so that I can debug without a terminal.

## Acceptance criteria
- Given a WSLC container, when I open Logs, then `wslc --session <name> container logs --tail 200 -t <name>` content is shown immediately [V that `--tail` and `-t` work].
- Given Follow is on, then `container logs --follow -t` streams new lines [V]. New lines appear within 1 s of being written (target, to be measured).
- Given lines carry an RFC3339Nano timestamp prefix such as `2026-11-11T07:09:54.459090506Z tick 11` [V], then the timestamp is parsed into its own column, and a line without a valid prefix is shown unchanged.
- Given the buffer exceeds 10 000 lines, then the oldest lines are dropped (ring buffer) and memory stays bounded.
- Given I close the Logs view or switch containers, then the underlying process is terminated.
- Given the container stops while following, then the stream ends and the view shows "container stopped" with a Retry action. Behaviour of the CLI at that moment is not captured yet [U].
- Given output encoding, then `wslc` output is decoded as plain text [V].

## Fixtures
- `obsidians/fixtures/container-logs-tail.txt`, `obsidians/fixtures/container-logs-follow-sample.txt` (strip CR; the files have CRLF line endings).

## Out of scope
- Search within logs, download/export, stdout/stderr separation, `--since`/`--until` filters.
