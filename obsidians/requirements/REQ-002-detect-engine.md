---
id: REQ-002
title: "WSLC capabilities by version"
priority: Must
connector: wslc
status: todo
depends_on:
  - "[[REQ-001-discover-runtimes]]"
tags:
  - requirement
  - must
---

# REQ-002 WSLC capabilities by version

- Priority: Must (US-2)
- Connector scope: WSLC (engine detection for Docker/Podman is deferred)
- Dependencies: REQ-001

## User story
As a user, I want the app to enable only the features my installed WSLC version supports so that I never click something that cannot work.

## Acceptance criteria
- Given the detected WSLC/WSL version is 3.0.5.0, then `Capabilities` equal the verified set: list, stats snapshot, inspect, events, logs follow, restart, `--session` [V].
- Given a version other than 3.0.5.0, then capabilities are limited to the verified set and an "untested version" notice is shown [U for other versions].
- Given a feature gate from source reading (host loopback needs >= 3.0.5, container network connect/disconnect needs >= 3.1.0 [R]), then features beyond the MVP are disabled below that version. Capabilities are never hard-coded as a version floor for the whole app (the floor is unverified [U]).
- Given a capability is false, then the related UI control is disabled with a tooltip, not hidden silently.
- Given the version cannot be determined within 5 s, then the app shows "could not detect" with a typed `Timeout` error and uses the verified set.
- Given the `EngineConnector` trait, then `Capabilities` is part of the trait so a future Docker connector can report a different set.

## Fixtures
- `obsidians/fixtures/README.md` (tested baseline 3.0.5.0). `obsidians/fixtures/errors/wslc-version.txt` (`wslc 3.0.5.0`), `obsidians/fixtures/wsl-version.utf16.txt`.

## Out of scope
- Docker/Podman engine detection, switching engines, Docker Desktop specifics (deferred).
