# wslc-manager

A Windows desktop app, written in Rust, for managing Linux containers that run on **WSL containers (WSLC)**: list, start, stop, restart, logs, live status and resource stats, without opening a terminal.

> **Status: early development.** The walking skeleton (REQ-000), the CI and QA baseline (REQ-013) and the switch to bun (REQ-014) are done and merged into `develop`. The app today opens a window and shows one fake container through a fake connector; it does not talk to a real `wslc.exe` yet. **Next: REQ-001** (WSL/wslc availability, version, sessions), then REQ-002 and REQ-008. Order and ticket status: [`obsidians/requirements/README.md`](obsidians/requirements/README.md).

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

## Prerequisites

To run the app or the checks you need:

- **Windows 11.**
- **WSL with WSLC support** to use the real app later. Tested baseline: WSL **3.0.5.0** and `wslc` 3.0.5.0. The exact minimum version is unverified; the app checks capabilities at runtime. The app never installs or updates WSL silently: it tells you, and asks before running `wsl --update`. The current build and the checks do not need WSL.
- **Rust through [rustup](https://rustup.rs).** The compiler comes from `rust-toolchain.toml` (1.99.1, with `rustfmt` and `clippy`); the first `cargo` call downloads it (a few minutes).
- **[bun](https://bun.sh) 1.3.14**, the version in `packageManager` in `package.json`. CI installs the same one. Bun is the only package manager: `bun.lock` is the only lockfile.
- **Node 24** (`.nvmrc`, `engines` in `package.json`). Bun installs and builds, but Vitest runs on Node, so `bun run test` fails without a `node` on `PATH`.
- **WebView2 runtime** (Tauri on Windows). Windows 11 normally has it (TBC). The CI runner built and tested the Tauri crate without extra setup. Installer handling for end users belongs to REQ-012.

## Run the app

```powershell
bun install --frozen-lockfile
bunx tauri dev
```

`bunx tauri dev` starts Vite (`bun run dev`, `http://localhost:1420`) and the Tauri window. The REQ-014 test report did not run `tauri dev` (TBC on your machine); the debug build below was verified.

Debug build without an installer:

```powershell
bunx tauri build --debug --no-bundle
```

This runs `bun run build` first and writes `target\debug\aiyara-app.exe`. Installers are off (`bundle.active` is `false` in `src-tauri/tauri.conf.json`); the MSI/NSIS pipeline belongs to REQ-012.

## Local checks

The GitHub Actions workflow `ci` (job `ci-check`, on `windows-latest`) runs exactly these commands, in this order. Run them locally before opening a pull request. If you have an old `node_modules` from another package manager, delete it first.

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
bun install --frozen-lockfile
bun run build
bun run test
```

- `cargo test --workspace` runs every unit, integration and doc test of all workspace crates. Tests that need a real `wslc.exe` are `#[ignore]` and are listed as ignored.
- `bun run test` runs Vitest once (jsdom, no watch mode) and needs no Tauri, WebView2, Rust or WSL. It is the only test command. **Do not run `bun test`**: it is bun's own runner and ignores `vitest.config.ts`. A guard in `bunfig.toml` makes it fail with a message.
- To upgrade the Rust toolchain deliberately, edit the version in `rust-toolchain.toml` and fix any new lints in the same pull request. More in [`obsidians/qa/ci-and-toolchain.md`](obsidians/qa/ci-and-toolchain.md).

### CI jobs

| Workflow / job | Runs on | What it checks |
|---|---|---|
| `ci` / `ci-check` | `windows-latest` | The six commands above, on pull requests into `develop` and `main` and on pushes to `develop`, `feature/**`, `bugfix/**`, `release/**`, `hotfix/**`. Read-only token, no secrets, no release build |
| `branch-policy` / `branch-policy` | `ubuntu-latest` | A pull request into `main` must come from `develop`, `release/*` or `hotfix/*`. Pull requests into other branches pass |

The job names are the status-check names. Do not rename them without updating the `main` ruleset.

### Integration tests (real `wslc.exe`)

Tests that call the real `wslc.exe` live in `crates/engine/tests/wslc_integration.rs` (helpers in `crates/engine/tests/support/`). They are `#[ignore]` by default and never run in CI. To run them on your own machine:

```powershell
$env:WSLC_INTEGRATION = "1"
cargo test -p aiyara-engine --test wslc_integration -- --ignored --test-threads=1
Remove-Item Env:WSLC_INTEGRATION
```

Rules (details in [`obsidians/qa/qa-integration-convention.md`](obsidians/qa/qa-integration-convention.md)):

- Every resource a test creates is named `aiyara-test-<suffix>`. A test never starts, stops, removes or changes anything without that prefix.
- A drop guard removes what the test created, also when an assertion fails. A startup sweep removes leftovers with the prefix from crashed runs.
- Cleanup of leftovers by hand: `wslc container list --all` and remove only the entries whose name starts with `aiyara-test-`, for example `wslc container remove --force <name>`.

## Repository layout

| Path | Content |
|---|---|
| `crates/domain/` | `aiyara-domain`: engine-neutral domain model |
| `crates/engine/` | `aiyara-engine`: `EngineConnector` trait, `WslcCliConnector`, fake connector, process runner; unit, contract and integration tests under `tests/` |
| `src-tauri/` | `aiyara-app`: the Tauri 2 shell (Rust core, `tauri.conf.json`) |
| `src/` | Web UI (TypeScript, Vite), the English string module `strings.ts`, Vitest tests |
| `scripts/` | `refuse-bun-test.ts`, the guard behind `bun test` |
| `.github/workflows/` | `ci.yml`, `branch-policy.yml` |
| `obsidians/` | Docs vault: analysis, tickets, QA, plans, fixtures (see below) |

Architecture: a Tauri 2 desktop app (Rust core plus web UI). The core talks to the runtime through an `EngineConnector` trait with a `WslcCliConnector` first. Containers are identified by `(runtime, scope, container id)`, where the scope is a WSLC session. Details and the evidence behind each choice: [`obsidians/ba-analysis.md`](obsidians/ba-analysis.md).

## Documentation

| Document | Purpose |
|---|---|
| [`obsidians/ba-analysis.md`](obsidians/ba-analysis.md) | Business analysis, decisions, architecture, risks, phasing |
| [`obsidians/requirements/`](obsidians/requirements/README.md) | Tickets (REQ-xxx) with acceptance criteria, status and implementation order |
| [`obsidians/qa/`](obsidians/qa/README.md) | Test strategy, Definition of Done, integration convention, [CI and toolchain reference](obsidians/qa/ci-and-toolchain.md) |
| [`obsidians/decision-log.md`](obsidians/decision-log.md) | Decisions D1 to D18 |
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

```powershell
git flow feature start REQ-001-discover-runtimes
# ... work, commit ...
git flow feature finish REQ-001-discover-runtimes
```

### Merging into `main`: pull request only (D16)

`main` is protected by a GitHub ruleset ("main: pull request only"): nobody, including the owner, can push to `main` directly, delete it or force-push it. The only way in is a pull request, merged with a merge commit, with all review conversations resolved. `develop` and the topic branches are not protected, so `git flow feature finish` into `develop` still works locally.

The `branch-policy` job additionally fails a pull request into `main` whose head is not `develop`, `release/*` or `hotfix/*`. The `ci-check` and `branch-policy` jobs are not yet required status checks on the ruleset (owner step, planned after the first green pull request into `main`; TBC).

To release or hotfix:

1. `git flow release start <version>` (or `hotfix start`), then push the branch: `git push -u origin release/<version>`.
2. Open a pull request from that branch (or from `develop`) into `main` and merge it on GitHub.
3. Do **not** run `git flow release finish` / `hotfix finish` locally: it merges into `main` on your machine and the push would be rejected. After the PR is merged, tag the merge commit on `main`, then merge `main` back into `develop` and push.

## Contributing

Not open for contributions yet. Open an issue to discuss before sending a pull request.

## License

[MIT](LICENSE), Copyright (c) 2026 Vatthanachai W.

## Code signing policy

Planned, not in place: release binaries will be signed through the free [SignPath Foundation](https://signpath.org) programme once the project is eligible (ticket REQ-012, applied for after a first release exists). Until a signed release exists, installers are unsigned and Windows SmartScreen may warn. The full policy (team roles and privacy statement) will be published here before the first signed release.
