---
id: REQ-003
title: "List containers"
priority: Must
connector: wslc
status: todo
depends_on:
  - "[[REQ-002-detect-engine]]"
  - "[[REQ-008-error-handling]]"
tags:
  - requirement
  - must
---

# REQ-003 List containers

- Priority: Must (US-3)
- Connector scope: WSLC (Docker deferred)
- Dependencies: REQ-002, REQ-008 (typed errors)

## User story
As a developer, I want one table of containers across WSLC sessions so that I can see everything at a glance.

## Acceptance criteria
- Given a WSLC session with containers, when the list loads, then it runs `wslc --session <name> container list --all --format json` [V] and shows name (`Names`), image (`Image`), state (`State`), status text (`Status`), ports (`Ports`), created (`CreatedAt`) and session name. The row model keeps `runtime` and `scope` fields (engine-neutral), but no runtime badge is shown while only WSLC exists.
- Given the WSLC output is JSON lines (one object per line, not an array) [V], when parsed, then each non-blank line becomes one container.
- Given the WSLC output is empty (no containers, exit 0) [V], then the table shows an empty state, not an error.
- Given `CreatedAt` is `2026-11-11 14:09:43 +0700 GMT+7` [V], then it is shown as a local date/time. Given it cannot be parsed, then the raw string is shown.
- Given `State` is `running` [V], then the row shows Running. Given `State` is `exited` with `Status` `Exited (3) 3 seconds ago` [V, `container-list-exited-ports.json`], then the row shows Exited and offers Start. Given any other value (not yet observed [U]), then the row shows the raw value and no action buttons are disabled by mistake.
- Given the `Ports` string is empty even when a port is published [V], then the ports cell is NOT taken from `Ports`. It is built from the `Ports[]` array inside the JSON stored under `com.microsoft.wsl.container.metadata` in the `Labels` string (fields BindingAddress, ContainerPort, Family, HostPort, Protocol, VmPort [V]), shown for example as `127.0.0.1:18080->80`. The meaning of `Protocol` 6 (assumed TCP) and `Family` 2 (assumed IPv4) is inferred [U], so unknown values are shown raw.
- Given `Labels` is one comma-joined string, not an object [V], then it is split into user labels (for example `app=aiyara`, `tier=test`) and the metadata entry. The metadata value itself contains commas, so the splitter must parse the metadata JSON as one unit (find `com.microsoft.wsl.container.metadata=`, then read the balanced JSON object) before splitting the rest on commas. A label value containing a comma is a [U] edge case.
- Given the metadata JSON is missing, malformed or has no `Ports`, then the ports cell is empty and the row is still shown (no error).
- Given `Status` and `RunningFor` are preformatted strings [V], then they are displayed only and never parsed for logic.
- Given the internal label `com.microsoft.wsl.container.metadata` [V], then it is hidden from the default labels display.
- Given several WSLC sessions (multi-session behaviour unverified [U]), then containers from each session found by REQ-010's session discovery are merged and labelled by session. Given a session fails, then the others still show and the failure is shown on that session only.
- Given 50 containers, then the table renders in under 2 s and can be filtered by name, state and session.
- IDs from list (12 chars) and from stats/events (64 chars) are matched by prefix [V].

## Fixtures
- `obsidians/fixtures/container-list-exited-ports.json` (exited container with a published port and user labels, the Ports-from-Labels case).
- `obsidians/fixtures/errors/container-list-bad-session.txt` (failure case for an unknown session), `container-list-bad-format.txt`.
- `obsidians/fixtures/container-list.json` (note the fixture has CRLF and no BOM, and is a single line; strip CR in tests and cover BOM synthetically. Add a synthetic empty-output case and a multi-line case in tests).

## Definition of done
- This ticket is done only when the checklist in [[qa/definition-of-done]] is met.

## Out of scope
- Docker/Podman containers and distro filters (deferred), stats columns (REQ-011), inspect detail view, images and volumes lists, sorting persistence.
