---
id: REQ-000
title: "Walking skeleton"
priority: Must
connector: wslc
status: done
depends_on: []
tags:
  - requirement
  - must
---

# REQ-000 Walking skeleton

- Priority: Must (technical enabler)
- Connector scope: WSLC (uses a fake connector behind the engine-neutral `EngineConnector` trait)
- Dependencies: none

## User story
As a developer on the team, I want a minimal end-to-end slice (UI -> Tauri command -> Rust core -> `EngineConnector` -> UI) so that build, IPC, testing and packaging are proven before real features are added.

## Acceptance criteria
- Given a clean checkout on Windows 11, when `cargo build` and the frontend build run, then the app builds without errors and starts a window.
- Given the app is running, when the window loads, then it shows one hard-coded container returned by a `FakeConnector` through the real `EngineConnector` trait and Tauri command path.
- Given the domain crate, when unit tests run, then a test using `FakeConnector` passes without any WSL installed (CI-friendly).
- Given a process runner abstraction exists, when a test injects a fake runner returning canned stdout, then connector code can be tested without `wsl.exe` or `wslc.exe`.

- Given any user-visible text in the skeleton, then it comes from one English string resource module (no i18n yet, D13).

## Fixtures
- `obsidians/fixtures/container-list.json` may be used as the canned output for the fake runner.

## Out of scope
- Real WSL discovery, real container actions, installer packaging beyond a dev build, UI styling.
