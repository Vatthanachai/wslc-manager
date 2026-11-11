---
id: REQ-011
title: "Container stats"
priority: Should
connector: wslc
status: todo
depends_on:
  - "[[REQ-003-list-containers]]"
tags:
  - requirement
  - should
---

# REQ-011 Container stats

- Priority: Should (was US-9 in earlier analysis versions)
- Connector scope: WSLC (Docker deferred)
- Dependencies: REQ-003

## User story
As an operator, I want to see CPU and memory per container so that I can spot heavy containers.

## Acceptance criteria
- Given visible running WSLC containers, then the app polls `wslc --session <name> container stats --format json` every 2 s. The command returns one snapshot per call and does not stream [V].
- Given the output is JSON lines with BlockIO, CPUPerc, ID (64 chars), MemPerc, MemUsage, Name, NetIO, PIDs [V], then each object is matched to a list row by ID prefix or Name.
- Given values are preformatted strings such as `0.06%` and `1.672MiB / 15.43GiB` [V], then they are displayed as-is, and numeric values are derived only from the percent strings.
- Given a container is not visible or not running, then it is not polled.
- Given several sessions, then the polling concurrency is bounded and a slow session does not block the others.
- Given a stats call fails or times out (5 s), then the last values are kept, marked stale, and the error is shown once, not per tick.

## Fixtures
- `obsidians/fixtures/container-stats.json`.

## Definition of done
- This ticket is done only when the checklist in [[qa/definition-of-done]] is met.

## Out of scope
- Graphs and history, alerts, per-process stats, network or disk rates.
