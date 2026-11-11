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
- User interface: yes (Runtime panel capability rows, untested marker). Design: [[REQ-001-002-008-ux]] (status `ready`).

## User story
As a user, I want the app to enable only the features my installed WSLC version supports so that I never click something that cannot work.

## Decisions (owner approved the UX on 2026-11-11; BA decisions on the open questions)
- Version source: the wslc version from `wslc version` (plain text `wslc <x.y.z.w>` [V]) drives capability decisions and the "untested" decision. `wsl --version` (UTF-16LE [V]) is shown in the UI as the WSL version only and never drives capabilities.
- No version floor is hard-coded for the whole app (the floor is unverified [U]). A missing `wslc.exe` is a gate state (REQ-001), not a capability result.
- In this batch no MVP control can be disabled by a version gate (the MVP set is fixed). REQ-002 is satisfied by the Runtime panel data plus the disabled-with-reason pattern; the visual check of a gated control's tooltip is deferred to the first ticket that has such a control.
- Capability rows are data-driven: one row per capability key the backend reports, label from `src/strings.ts` keyed by capability id, raw id as the fallback for an unknown key.
- Facts marked [U] are unverified.

## Acceptance criteria
- Given the detected wslc version is 3.0.5.0, then `Capabilities` equal the verified set: list, stats snapshot, inspect, events, logs follow, restart, `--session` [V], and the Runtime panel shows exactly these seven rows, all "Available", in a "Verified" column.
- Given a wslc version other than 3.0.5.0, then capabilities are limited to the verified set, the column says "Assumed", and the "untested version" notice is shown [U for other versions]. The untested decision compares the wslc version only, and the WSL version does not change it.
- Given a feature gate from source reading (host loopback needs >= 3.0.5, container network connect/disconnect needs >= 3.1.0 [R]), then features beyond the MVP are disabled below that version. Capabilities are never hard-coded as a version floor for the whole app (the floor is unverified [U]). The UI never compares version numbers itself, it reads capability booleans.
- Given a capability is false, then the related UI control is disabled with a tooltip (reason from `src/strings.ts`, shown on hover and on keyboard focus, closed by Esc), not hidden silently. The panel row shows "Unavailable" as text.
- Given WSLC is not Available (a gate is shown), then every container action control added later is disabled with the reason "WSLC is not available. See Runtime." (pattern defined here, no such control exists yet).
- Given the wslc version cannot be determined within 5 s, then the app shows a warning "Could not detect the WSL version" with a typed `Timeout` error, the status bar says "Version unknown", the panel shows "Unknown", only the verified set is used, and Retry re-runs detection. The details show Error type `Timeout` and the timeout seconds.
- Given the version output has an unexpected shape (not `wslc <digits and dots>`), then the result is `ParseError` with the raw output in the details, the verified set is used, and the version is shown as "Unknown".
- Given the Runtime panel is open and Retry detection is pressed, then versions, sessions and capabilities are re-read and the panel updates without closing.
- Given the `EngineConnector` trait, then `Capabilities` is part of the trait so a future Docker connector can report a different set.

## Fixtures
- `obsidians/fixtures/README.md` (tested baseline 3.0.5.0). `obsidians/fixtures/errors/wslc-version.txt` (`wslc 3.0.5.0`), `obsidians/fixtures/wsl-version.utf16.txt`.
- A version other than 3.0.5.0 and the timeout case use an injected fake runner (no real capture, [U]).

## Spikes and manual checks still needed
- Whether the wslc version and the WSL version can differ in practice (both read 3.0.5.0 now) [U].
- Behaviour with a real untested version (no second version is installable here) [U].
- Tracked in [[unverified-items]].

## Definition of done
- This ticket is done only when the checklist in [[qa/definition-of-done]] is met.

## Out of scope
- Docker/Podman engine detection, switching engines, Docker Desktop specifics (deferred).
