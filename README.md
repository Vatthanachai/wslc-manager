# wslc-manager

A Windows desktop app, written in Rust, for managing Linux containers that run on **WSL containers (WSLC)**: list, start, stop, restart, logs, live status and resource stats, without opening a terminal.

> **Status: planning / Phase 0.** There is no application code yet. This repository currently holds the requirements, the design analysis and real `wslc` output captured as test fixtures.

## Goals

- See all WSLC containers (across sessions) in one place, with live status.
- Run lifecycle actions safely (confirmation before destructive actions, clear errors).
- Zero configuration; nothing is exposed on the network.
- Stay engine-neutral internally so Docker/Podman inside WSL distros can be added later.

## Scope

| | |
|---|---|
| **MVP** | WSLC only, driven through `wslc.exe` (`--format json`, `system events`, `logs --follow`, stats polling) |
| **Later** | Docker/Podman inside WSL distros, optional C SDK (`wslcsdk.dll`) connector, Compose |
| **Platform** | Windows 11 only (Windows 10 is not supported) |
| **UI language** | English only for now |

## Requirements

- Windows 11
- WSL with WSLC support. Tested baseline: WSL **3.0.5.0** and `wslc` 3.0.5.0. The exact minimum version is unverified; the app checks capabilities at runtime.
- The app never installs or updates WSL silently. It checks whether WSL is up to date, tells you if not, and asks before running `wsl --update`.

## Planned architecture

Tauri 2 desktop app (Rust core + web UI). The Rust core talks to the runtime through an `EngineConnector` trait with a `WslcCliConnector` first. The domain model is engine-neutral, with containers identified by `(runtime, scope, container id)`, where the scope is a WSLC session.

Details and the evidence behind each choice are in [`obsidians/ba-analysis.md`](obsidians/ba-analysis.md).

## Documentation

| Document | Purpose |
|---|---|
| [`obsidians/ba-analysis.md`](obsidians/ba-analysis.md) | Business analysis, decisions, architecture, risks, phasing |
| [`obsidians/requirements/`](obsidians/requirements/README.md) | Tickets (REQ-xxx) with acceptance criteria and implementation order |
| [`obsidians/fixtures/`](obsidians/fixtures/README.md) | Real `wslc` / `wsl` output (JSON, events, logs) captured on WSL 3.0.5.0 |
| [`obsidians/fixtures/errors/`](obsidians/fixtures/errors/README.md) | Real error output (exit codes, `WSLC_E_*` codes) |

Facts in the analysis are labelled by evidence: verified on a real machine, reported by a source, or unverified.

The `obsidians/` folder is also an [Obsidian](https://obsidian.md) vault: open it with "Open folder as vault" and start from `Home.md` (decision log, unverified items, ticket board with the optional Dataview plugin).

## Development workflow

This repository uses **git flow** (`git-flow-next`):

| Branch | Purpose |
|---|---|
| `main` | Released code only |
| `develop` | Integration branch |
| `feature/*`, `bugfix/*` | Branch from and merge into `develop` |
| `release/*`, `hotfix/*` | Branch from `develop` / `main` and merge back into both |

```bash
git flow feature start REQ-000-walking-skeleton
# ... work, commit ...
git flow feature finish REQ-000-walking-skeleton
```

Work follows the order in [`obsidians/requirements/README.md`](obsidians/requirements/README.md), starting with the walking skeleton.

## Contributing

Not open for contributions yet. Open an issue to discuss before sending a pull request.

## License

[MIT](LICENSE), Copyright (c) 2026 Vatthanachai W.

## Code signing policy

Planned: release binaries will be signed through the free [SignPath Foundation](https://signpath.org) programme once the project is eligible. Until a signed release exists, installers are unsigned and Windows SmartScreen may warn. The full policy (team roles and privacy statement) will be published here before the first signed release.
