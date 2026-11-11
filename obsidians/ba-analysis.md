# BA Analysis: Rust desktop app on Windows to manage containers on WSL

Date: 2026-11-11 | Status: v6 (Windows 11 only, MIT, SignPath signing plan, error fixtures verified) | Repo state: no application code yet

Evidence labels
- [V] verified. Where the label comes from the fixtures it means "verified on WSL 3.0.5.0 / wslc 3.0.5.0, Windows 10.0.26300, 2026-11-11" (see `obsidians/fixtures/README.md`). Other [V] items were verified in search results I read.
- [R] reported (read from microsoft/WSL master by a helper agent, or a secondary source), not run by us.
- [U] unverified or inferred.

Tickets derived from this analysis are in `obsidians/requirements/` (index: `obsidians/requirements/README.md`).

## 1. Summary and decisions

Native WSL containers (WSLC) are a separate Microsoft runtime with `wslc.exe` and a C API. The C API has no list function for containers or sessions [R], while the `wslc.exe` CLI lists, streams events and logs, and gives stats snapshots with JSON output [V]. So the MVP uses `WslcCliConnector` only.

Decisions made by the user
- D1/D2 (revised, final): MVP scope is WSLC ONLY. `DockerCliConnector` and all Docker/Podman acceptance criteria are deferred to a later phase (Phase 2+). The `EngineConnector` trait, `Capabilities` and the engine-neutral domain stay, so Docker can be added later. No MVP effort goes to Docker fixtures, bollard or WSL distro discovery beyond what WSLC needs. (The earlier decision "both in MVP" is superseded.)
- D11: the app checks whether WSL is up to date, tells the user when it is not, asks whether to update, and runs `wsl --update` only after an explicit click. Latest-version source = option D: local capability check always, plus a once-a-day comparison with GitHub releases. The update banner and the daily check are ON by default, with a setting to turn the daily check off, plus "Remind me later" and "Don't ask for this version". The user accepts that `wsl --update` may need UAC and may stop distros/containers. The confirm dialog must still warn about both and list what may be affected, and nothing runs without the user's click.
- D12 = BA default: show the default session plus all sessions from `wslc system session list`, with the session name visible.
- D13 (new): UI language is English only for now. No i18n in the MVP, but all user-visible strings live in one place (single resource module) so i18n can be added later.

- D14: Windows 10 is NOT supported. Windows 11 only. (The fixtures' "Windows 10.0.26300" is the OS version string of the Windows 11 test machine, not a Windows 10 target.)
- D15: code signing via SignPath Foundation (free OSS programme). Licence is MIT, and the repo will be public on GitHub. The release and signing pipeline is ticket REQ-012 (Should, post-MVP, required before public distribution). Facts about SignPath below are [R] (read from signpath.org/terms and docs.signpath.io by a helper), details marked [U] are unverified.

Nothing else is open for the MVP except the timing of the Docker phase (section 11).

Stack recommendation (unchanged): Tauri 2 (Rust core + web UI). Docker side is deferred and untested on a real machine [U], so Docker-related facts below remain [U] or [R].

## 2. Problem, personas, goals, non-goals

Problem: developers running containers on WSL have no unified Windows GUI. They use `wsl -d X docker ...` or `wslc ...` in a terminal.

Personas: P1 developer (one runtime, few containers), P2 operator (several distros or both runtimes, wants resources and safe cleanup), P3 WSLC adopter (uses `wslc.exe` instead of Docker).

Goals (MVP): see all WSLC containers across sessions with live status, run lifecycle actions safely, read logs, minimal setup. Later: the same for Docker/Podman in distros, never exposing the Docker socket on the network.

Non-goals (MVP): Docker/Podman support, WSL distro discovery, remote hosts, Kubernetes, Swarm, image build, registry management, managing WSL distros, Linux/macOS, C API (W2) integration.

## 3. Decisions

| # | Question | Status |
|---|----------|--------|
| D1/D2 | What "container on WSL" means | Decided: MVP = WSLC only. Docker/Podman deferred (Phase 2+) behind the same trait |
| D3 | Number of distros | Deferred with Docker |
| D4 | GUI / TUI / CLI | Default: GUI (Tauri) |
| D5 | MVP scope | Default: list/start/stop/restart/logs/stats |
| D6 | local or remote | Default: local only |
| D7 | Auto-start a stopped distro | Default: ask once, remember per distro |
| D8 | Permissions (docker group, rootless Podman) | Default: assume access, clear error otherwise |
| D9 | Distribution | Default: NSIS/MSI via the Tauri bundler |
| D10 | Minimum Windows | Decided (with D14): Windows 11 only |
| D11 | WSL update policy | Decided: check, tell, ask, `wsl --update` only after a click. Source of latest = option D. Banner and daily check on by default, with an off setting. UAC and possible stop of distros/containers accepted, with a warning |
| D12 | Which WSLC sessions are shown | Decided: BA default |
| D13 | UI language | Decided: English only, strings in one place, no i18n in MVP |
| D14 | Windows 10 support | Decided: not supported, Windows 11 only |
| D15 | Code signing | Decided: SignPath Foundation (free OSS programme), MIT licence, public GitHub repo. Applied after a first release exists (REQ-012) |

## 4. Verified WSLC CLI facts (WSL 3.0.5.0, 2026-11-11) [V]

Source: `obsidians/fixtures/` (files named per row).

| Area | Fact | Fixture |
|------|------|---------|
| Session option | `--session` is a global option on every command | README.md |
| Session list | Path is `wslc system session list` (not `wslc session list`). Table output only, no `--format`. `--verbose` adds a line `[wslc] Found N session`. Columns ID, Creator PID, Display Name | session-list-verbose.txt |
| Container list | `container list --all --format json` prints JSON lines (one object per line), not an array. With no containers it prints nothing (exit 0), not `[]` | container-list.json |
| Container list keys | PascalCase: Command, CreatedAt, HealthStatus, ID (12 chars), Image, Labels, LocalVolumes, Mounts, Names, Networks, Platform{architecture,os}, Ports, RunningFor, Size, State, Status. All values are strings except `Platform`. Observed values: State `running`, Status `Up 3 seconds`, RunningFor `3 seconds ago`, Ports `""`, Mounts `""`, Networks `bridge`, Size `0B`, LocalVolumes `"0"`, HealthStatus `""`. `CreatedAt` looks like `2026-11-11 14:09:43 +0700 GMT+7`. `Labels` is a single string (the sample contains a `com.microsoft.wsl.container.metadata={...}` entry) | container-list.json |
| Stats | `container stats --format json` prints one object per container (JSON lines): BlockIO, CPUPerc, ID (full 64 chars), MemPerc, MemUsage, Name, NetIO, PIDs (number). Values are preformatted strings (for example `0.06%`, `1.672MiB / 15.43GiB`). One snapshot per call, no streaming | container-stats.json |
| Inspect | `container inspect` prints a pretty-printed JSON array (Docker-style, first key `Config`) | container-inspect.json |
| Images | `image list --format json` prints JSON lines: Containers, CreatedAt, CreatedSince, Digest, ID, Repository, SharedSize, Size, Tag, UniqueSize | image-list.json |
| Events | `system events --format json` streams (events arrived while the command was still running). JSON lines with Action, Actor{Attributes,ID}, Type (`container`, `network`), `from`, `id`, `scope`, `status`, `time`, `timeNano`. Network events have no `from`/`id`/`status`. A restart produced: kill (signal 15), kill (signal 9), network disconnect, die (exitCode 137), stop, network connect, start | events.jsonl |
| Logs | `container logs --follow` streams. `-t` prefixes an RFC3339Nano timestamp (for example `2026-11-11T07:09:54.459090506Z tick 11`). `--tail N` works | container-logs-tail.txt, container-logs-follow-sample.txt |
| Timing | `container restart` took about 10 s because PID 1 (`sh`) ignored SIGTERM and wslc killed it after the default timeout | events.jsonl |
| Encoding | `wsl --version` and `wsl -l` output is UTF-16LE. `wslc` output is plain text. Fixture files carry BOM and CRLF from PowerShell, so tests must strip both | README.md |

Still unverified [U]
- Whether containers created by the C SDK appear in the CLI, and the reverse.
- Session persistence after the creating process exits, after logoff and after reboot.
- Behaviour with several sessions (only one existed), including whether a default session is created implicitly on a clean machine.
- JSON field names and shapes on other WSL versions. The floor version for `wslc.exe` is unverified. 3.0.1 was a GA claim from a summary [U], and 3.0.5.0 is the tested baseline.
- `State` values other than `running` and `exited`, `HealthStatus` values, and the format of multiple networks or multiple port mappings.
- Remaining failure cases (see "Phase 0 results" below): name collision, port already in use, remove of a running container without `--force`, `kill`, `wslc` missing or WSL too old, session list with 0 or 2+ sessions, permission-denied cases, and the GitHub releases API sample.
- Whether `wslc` has a `--format` for `system session list` (it does not [V]) or a JSON alternative (none observed).
- The whole Docker/Podman side (no fixtures yet).

## 5. Integration options

### Docker/Podman in distro (DEFERRED, not in MVP)
Kept for the later phase. (a) `wsl.exe -d X -e docker ...` CLI is the MVP connector for Docker/Podman [R, not yet tested on a real machine]. (b1) Docker API on TCP 2375: reject by default (unauthenticated, root-equivalent). (b2) npiperelay + socat: reject. (b3) `docker system dial-stdio` via `wsl.exe -e`: [U], spike later. (c) Mirrored networking: do not depend on it, sources conflict.

### WSLC
| Option | Role | Verdict |
|--------|------|---------|
| W1 spawn `wslc.exe` | List, events, logs, stats, lifecycle, all verified on 3.0.5.0 for the listed commands [V] | Primary connector, in MVP |
| W2 C API `wslcsdk.dll` | No list or enumerate function, ANSI/wide string mix, opaque struct sizes tied to SDK version, extra DLL, preview status unverified [R]. Visibility of CLI-created containers from the SDK is unverified [U] | Out of MVP. Optional later for control operations only |
| W3 C#/WinRT | Needs a non-Rust backend | Reject |

Rust crates: third-party WSLC crates exist (`wslc`, `wslc-sys`, `wslcsdk`, `wslcsdk-sys`) [R], unaudited. They are not needed for the MVP. If W2 is revisited, avoid the crate name clash (use `aiyara-wslc*`). `bollard` 0.21.0 [V] is not needed for the CLI connectors. Common crates: `tokio`, `serde`/`serde_json`, `tracing`, `thiserror`. UI: Tauri 2 (alternatives egui, iced, slint, ratatui as in earlier versions, from secondary sources and a 2023 benchmark).

## 6. Architecture

```
UI (Tauri webview)  <-> Tauri commands/events (typed)
App layer (use cases, runtime-agnostic)
Domain (engine-neutral): Runtime, Scope, ContainerRef, Container, Image, Stats, Event, Capabilities, AppError
EngineConnector trait: scopes, list, start, stop, restart, remove, logs_stream, events_stream, stats_snapshot, capabilities
  WslcCliConnector   (wslc.exe --session S ...)          MVP
  DockerCliConnector (wsl.exe -d X -e docker|podman)     DEFERRED (Phase 2+)
Infrastructure: WSL/wslc availability + version check, shared Process runner, Event hub, Config, tracing, single string resource module (English only)
```

### Domain model fields (from the verified JSON)

`Container` is engine-neutral. Raw preformatted strings from the CLI are kept as display strings, and only fields we can parse reliably get typed forms.

| Domain field | WSLC source (`container list`) | Notes |
|--------------|-------------------------------|-------|
| `runtime` | constant `Wslc` | `Docker`/`Podman` for the Docker connector |
| `scope` | session id + display name (WSLC) or distro name (Docker) | `ContainerRef = (runtime, scope, id)` |
| `id` | `ID` (12 chars in list, 64 chars in stats and events) | Match list/stats/events by prefix, not equality |
| `name` | `Names` | String, multiple names format unverified [U] |
| `image` | `Image` | |
| `state` | `State` | Observed only `running` [V]. Map unknown values to `Unknown(raw)` and keep the raw string |
| `status_text` | `Status` | Display only (`Up 3 seconds`). Never parse it for logic |
| `running_for` | `RunningFor` | Display only |
| `created_at` | `CreatedAt` | Format `2026-11-11 14:09:43 +0700 GMT+7` [V]. Parse the date, time and numeric offset, ignore the trailing `GMT+7` label, and keep the raw string on parse failure |
| `ports` | `Ports` | Empty string observed [V]. Non-empty format unverified [U], so keep as a string |
| `networks` | `Networks` | String (`bridge`). Multiple-network format unverified [U] |
| `mounts`, `local_volumes`, `size` | `Mounts`, `LocalVolumes`, `Size` | Strings (`LocalVolumes` is `"0"`) |
| `labels` | `Labels` | One string, comma-separated format unverified [U]. Hide the internal `com.microsoft.wsl.container.metadata` entry in the default UI |
| `health` | `HealthStatus` | Empty string means none |
| `platform` | `Platform{architecture,os}` | The only nested object |
| `command` | `Command` | Truncated by the CLI (contains an ellipsis) |

`Stats` (from `container stats`): `cpu_percent_text` (`CPUPerc`), `mem_usage_text` (`MemUsage`), `mem_percent_text` (`MemPerc`), `net_io_text`, `block_io_text`, `pids` (number). Values are strings like `0.06%` and `1.672MiB / 15.43GiB`. A typed numeric value is derived by parsing the percent strings only, and the raw string is always kept.

`Event`: `kind` (`Type`: container or network), `action` (`Action`), `container_id` (`id` or `Actor.ID`), `attributes` (`Actor.Attributes`, including `name`, `image`, `signal`, `exitCode`), `time_nano` (`timeNano`). Network events carry no `from`/`id`/`status`, so those fields are optional.

### WslcCliConnector design

Parsing rules
- Strip BOM and CRLF in tests (fixtures carry them). Production output of `wslc` is plain text [V], so decode as UTF-8.
- `wsl.exe` output (`--version`, `-l`) is UTF-16LE [V]. Use a separate decoder.
- JSON lines: split on newlines, skip blank lines, parse each line as one object. Empty output means an empty list, not an error [V].
- `container inspect` is a JSON array [V], so use a different parser from the JSON-lines commands.
- Unknown keys are ignored and missing keys become `None`, because the schema on other WSL versions is unverified [U].
- Every command is called with `--session <name>` [V that it is a global option]. When the session is the default one, the flag may be omitted [U].

Commands used (all verified on 3.0.5.0 [V]): `container list --all --format json`, `container stats --format json [<name>...]`, `container inspect <name>`, `container logs --follow --tail N -t <name>`, `system events --format json`, `container restart/stop/start`, `image list --format json`, `system session list --verbose`. `container remove`/`kill` exist [R] and are untested.

Session discovery: `wslc system session list --verbose` is a table with no JSON [V]. Parse the `[wslc] Found N session` line as a cross-check, skip the header row, and split the remaining rows into ID, Creator PID and Display Name. Display names may contain spaces [U], so split on the first two whitespace runs only. Add a fixture-based test, and treat a parse failure as `ParseError` with the raw output attached.

Live updates
- Run one `system events --format json` process per session, kept alive while the app is open. On each event for a container, refresh that container through `list`. On process exit, reconnect with backoff and do a full `list`.
- Always do a full `list` at startup and after every reconnect. Events alone are not a source of truth.

Stats: poll `container stats` (one snapshot per call, no streaming [V]) every 2 s, only for visible running containers, and with a concurrency limit.

Logs: `container logs --follow --tail 200 -t` streams [V]. Parse the RFC3339Nano timestamp prefix, keep a ring buffer (10k lines), and kill the process when the view closes.

Timeouts
- `list`, `stats`, `inspect`, `session list`: 5 s.
- `start`: 15 s [U].
- `stop`/`restart`: at least 30 s. A restart took about 10 s in the fixture [V] because PID 1 ignored SIGTERM, and the UI shows progress rather than a frozen button.
- Streaming processes have no total timeout but an idle/health check and cancellation.

Error mapping (verified on WSL 3.0.5.0, fixtures in `obsidians/fixtures/errors/`): decide by exit code first (any failure is exit 1 [V], success 0), then by the `WSLC_E_*` token found on the `Error code:` line of stderr. Never match English text, because messages may be localised on non-English Windows [U].

| Condition | Signal | AppError |
|-----------|--------|----------|
| Exit 0 | none | success (even for `stop` on an already stopped container [V]) |
| `WSLC_E_CONTAINER_NOT_FOUND` | `Error code:` line (`container-stop-missing.txt`, `container-start-missing.txt`, `container-remove-missing.txt`, `container-logs-missing.txt`, `container-stats-missing.txt`) | ContainerNotFound |
| `WSLC_E_SESSION_NOT_FOUND` | `Error code:` line (`container-list-bad-session.txt`, `session-terminate-missing.txt`) | SessionNotFound |
| `WSLC_E_IMAGE_NOT_FOUND` | `Error code:` line (`container-run-missing-image.txt`, `image-pull-missing.txt`) | ImageNotFound |
| `container inspect <missing>` | exit 1, stdout `[]`, stderr message, no `Error code:` (`container-inspect-missing.txt`) | ContainerNotFound (treat exit 1 plus empty `[]` as not found, and do not parse the English message) |
| Usage error (bad `--format`, unknown option, unknown command) | exit 1, message plus usage text, no `Error code:` (`container-list-bad-format.txt`, `container-list-bad-option.txt`, `unknown-command.txt`) | InternalError (a bug in our command line, not shown as a user error) |
| Exit 1 with any other `WSLC_E_*` token | token | `CommandFailed{code, stderr}` (the code is kept for display, for example `WSLC_E_RUNTIME_UPDATE_NEEDED` is known from source [R] but not observed) |
| Exit 1 with no token and no known usage pattern | none | `CommandFailed{stderr}` |
| Timeout | our timer | Timeout |

Note `container stats <missing>` prints an empty line and only the `Error code:` line, with no human message [V], so the UI message always comes from our string table, not from stderr.

### DockerCliConnector (deferred, Phase 2+)
No MVP effort. Same shape: `wsl.exe -d X -e docker ps --all --format json` (JSON lines in Docker, array in Podman [R]), `docker events --format json` (streaming), `docker logs -f`, `docker stats --no-stream`. Not yet tested on a real machine [U], so a Docker fixture capture becomes a prerequisite of the Docker phase, not of the MVP.

## 7. Non-functional requirements

- Security: never expose the Docker socket on TCP by default. Build commands from argument arrays, never shell strings. Mask env vars in inspect output. No telemetry. Never run `wsl --update` without an explicit yes (D11). Sessions belong to the current Windows user [R], so other users' sessions are invisible.
- Versions: do not hard-code a floor. Detect at runtime. The tested baseline is WSL 3.0.5.0 / wslc 3.0.5.0 [V]. The `wslc.exe` GA floor is unverified (3.0.1 is a summary claim [U]). Per-feature gates from source reading [R]: host loopback needs >= 3.0.5, container network connect/disconnect >= 3.1.0. 2.9.3 was the public preview and is historical only. `Capabilities` are set per detected version. On versions other than 3.0.5.0, show a notice that the app is untested there [U].
- Availability states: Available, WslNotInstalled, WslcMissing (no `wslc.exe`), WslTooOld, Unavailable.
- Localisation: English only in the MVP. All user-visible strings live in one resource module so i18n can be added later.
- Performance: cold start under 2 s, list refresh under 1 s per runtime/session (to be measured), one events process per scope, stats only for visible rows, bounded concurrency for per-session fan-out.
- WSL specifics: encoding rules in section 6. Stopped-distro handling (D7) and path translation belong to the deferred Docker phase.
- Reliability: timeouts per command class, cancellation, reconnect with backoff, typed errors (WslMissing, DistroStopped, EngineMissing, PermissionDenied, Timeout, ParseError, RuntimeUnavailable, UnsupportedOperation, SessionNotFound, CommandFailed).
- Compatibility: contract tests per connector using the fixtures in `obsidians/fixtures/`, captured per WSL version.
- Installer: must not bundle or install WSL. The update flow is in US-9 (REQ-009).

## 8. User stories (index)

Full tickets are in `obsidians/requirements/`.

| Story | Ticket | Priority |
|-------|--------|----------|
| Walking skeleton (enabler) | REQ-000 | Must |
| US-1 WSL/wslc availability, version, sessions | REQ-001 | Must |
| US-2 WSLC capabilities by version | REQ-002 | Must |
| US-3 List containers | REQ-003 | Must |
| US-4 Lifecycle | REQ-004 | Must |
| US-5 Logs | REQ-005 | Must |
| US-6 Live updates | REQ-006 | Must |
| US-7 Safe destructive actions | REQ-007 | Must |
| US-8 Error handling | REQ-008 | Must |
| US-9 WSL update check (D11) | REQ-009 | Must |
| US-18 Sessions panel | REQ-010 | Should |
| Stats | REQ-011 | Should |
| Release and signing pipeline | REQ-012 | Should (post-MVP, before first public release) |

Other stories (stats US-9 of v3, images/volumes, exec, settings, Compose, embedded terminal, Run image form, distro management) stay as Should/Could and will be ticketed later. Note: the v3 "US-9 stats" is now REQ-011 (Should, to be written), to avoid clashing with the new update-check story.

### D11 WSL update-check flow (REQ-009)

Goal: tell the user when WSL is not up to date and ask whether to update, running `wsl --update` only after a yes.

Decided: option D. Honest status: we do not know a supported dry-run for `wsl --update` [U], so the GitHub comparison is the only "latest" source, and its details (release channel vs Store/MSIX, pre-release filtering) are [U] until verified in implementation. Options considered:

| Option | Idea | Pros | Cons |
|--------|------|------|------|
| A | Query the GitHub releases API for microsoft/WSL and compare to `wsl --version` | Simple, shows a concrete latest version | Needs network, rate limits, pre-release vs stable filtering, the Store/MSIX channel may differ from GitHub tags |
| B | Run `wsl --update` only on user request, with no "latest" check at all (a manual "Update WSL" button) | No unverified version source, no network call | Does not "check", so does not meet D11 as stated |
| C | Check for the minimum version the app needs (derived from detected `Capabilities`) and prompt only when a needed feature is missing | No external source, fully local | Does not notify about newer versions in general |
| D (chosen) | Combine C (local, always) and A (once a day, ON by default, with a setting to turn it off) | Honest and cheap | Needs network access for the daily check |

Show the C result as a warning always, and A as an informational banner (on by default). UX and consent rules:
- Banner (not a modal) says: "A newer WSL version is available (installed 3.0.5.0, latest X). Update now?" with buttons "Update now", "Remind me later", "Don't ask for this version".
- "Update now" opens a confirmation dialog that warns about both accepted risks: `wsl --update` may need administrator permission (UAC) and may stop distros and containers (both [U], accepted by the user). It states the command and lists what may be affected (running WSLC sessions and their containers, and running distros if cheaply detectable, for example `wsl -l --running` [U]). Only an explicit "Update" button runs it.
- A setting turns the daily GitHub check off. No auto-update, no background update, no update at install time. "Remind me later" snoozes for 24 h [default, adjustable]. The decision per version is stored in settings.
- After the update, re-detect versions and show the result, including failures (exit code and stderr).
- The check never blocks the app. When the version source is unreachable, the app shows nothing alarming and logs it.

## 9. Risks

| Risk | Impact | Mitigation |
|------|--------|-----------|
| Session scoping: containers in other sessions are invisible unless found through `system session list` (no all-sessions option [R]) | High | `scopes()` fan-out, show session names |
| Multi-session behaviour untested (only one session existed [U]) | Medium | Phase 0 follow-up with two sessions |
| Per-user COM server: other Windows users' sessions invisible [R] | Medium | State it in the UI and docs |
| `system session list` is table-only, so parsing is text-based [V] | Medium | Fixture tests, `ParseError` with raw output, use `--verbose` count as a cross-check |
| CLI JSON schema drift on other WSL versions [U] (casing verified only on 3.0.5.0 [V]) | High | Tolerant parsing, contract tests with fixtures per version, "untested version" notice |
| Values are preformatted strings (`Status`, `RunningFor`, `CPUPerc`, `MemUsage`), not stable for logic | Medium | Display-only fields, parse only what is needed, keep raw strings |
| `CreatedAt` format is unusual (`... +0700 GMT+7`) | Low | Tolerant parser, raw fallback |
| Only `State=running` observed, other values unknown [U] | Medium | `Unknown(raw)` state, capture fixtures for stopped/exited/created |
| Empty `container list` output is empty text, not `[]` [V] | Low | Treat empty output as an empty list |
| Restart can take about 10 s [V] | Low | Generous timeouts, progress indicator |
| Error text may be localised on non-English Windows [U] | Medium | Map by exit code and `WSLC_E_*` token only, never by English text |
| Some failure cases still unverified (name collision, port in use, remove running without `--force`, `kill`) [U] | Medium | Capture before finishing REQ-004/REQ-007, fall back to `CommandFailed` |
| SignPath may reject a brand-new project (initial reputation/control check, no numeric threshold [R]) | Medium | Release unsigned first, apply after a release exists and the project has some history, keep the pipeline ready (REQ-012) |
| Unsigned first releases trigger SmartScreen warnings; SmartScreen reputation still builds over time even with a signed binary [R] | Medium | Tell users in the README, release signed builds as soon as SignPath approves |
| SignPath requires builds verifiable from source on GitHub-hosted runners, no manual overrides, no cached earlier builds, manual approval per release [R] | Medium | Pipeline design in REQ-012, no local release builds |
| SignPath shows "SignPath Foundation" as publisher, and may pause or revoke certificates even retroactively [R] | Low | Document it, keep an unsigned fallback and uninstall support |
| Support for Tauri bundles (NSIS/MSI) and Rust binaries in SignPath artifact configuration unverified [U] | Medium | Verify early in REQ-012 with a test signing request |
| Daily GitHub check must be disclosed in the privacy text, otherwise the SignPath privacy statement is violated [R] | Medium | Privacy text in REQ-012 matches REQ-009, off switch required |
| C SDK vs CLI visibility and session persistence after exit/logoff/reboot unverified [U] | Low for MVP | Out of MVP, since the MVP uses the CLI only |
| Product depends on a new Microsoft runtime only (WSLC-only MVP): smaller audience, API/CLI may change [U] | Medium | Engine-neutral trait kept so Docker can be added, version gate, contract tests |
| Docker/Podman deferred and untested [U] | Low for MVP | Not in MVP. Capture Docker fixtures at the start of the Docker phase |
| WSL update flow runs a command that may need UAC and may stop distros and containers (accepted by the user) | High | Dialog warns about both and lists what may be affected, nothing runs without a click (D11) |
| Daily GitHub check: the GitHub release may not match the channel `wsl --update` installs [U], rate limits, no network | Medium | Failures are silent and logged, setting to turn it off, local check C always runs |
| Update banner on by default may annoy | Low | "Remind me later", "Don't ask for this version", off setting |
| Third-party Rust crates (if W2 revisited) unaudited [R] | Low | Not used in MVP |
| English-only strings scattered in code make later i18n costly | Low | One resource module from the start |
| Scope creep to a Portainer clone | High | MoSCoW, review each phase |

## 10. Phasing

- Phase 0 results (done on WSL 3.0.5.0 / wslc 3.0.5.0, 2026-11-11, fixtures in `obsidians/fixtures/` and `obsidians/fixtures/errors/`, labelled [V]):
  - Every failure exits with code 1, success is 0. Errors go to stderr and stdout stays empty, with one exception: `container inspect <missing>` prints `[]` to stdout (`container-inspect-missing.txt`).
  - Domain errors end with an `Error code: WSLC_E_*` line followed by a "filing a new issue" hint. Seen: `WSLC_E_CONTAINER_NOT_FOUND`, `WSLC_E_SESSION_NOT_FOUND`, `WSLC_E_IMAGE_NOT_FOUND`. Usage errors (bad `--format`, unknown option, unknown command) have no code. `container stats <missing>` prints only an empty line and the code (`container-stats-missing.txt`).
  - `container stop` on an already stopped container exits 0 and prints the name (idempotent). `container stats` on a stopped container exits 0 with zeroed values and `PIDs` 0. `container logs` on an exited container works.
  - For an exited container (`container-list-exited-ports.json`): `State` is `exited`, `Status` is `Exited (3) 3 seconds ago`, the `Ports` string is empty even with a published port, and the mapping lives only inside the `Labels` metadata JSON (`com.microsoft.wsl.container.metadata`, with `Ports[]` of BindingAddress, ContainerPort, Family, HostPort, Protocol, VmPort). `Labels` is one comma-joined string, not an object.
  - `wslc version` prints `wslc 3.0.5.0` (plain text, `wslc-version.txt`). `wsl --version` and `wsl -l -v` are UTF-16LE (`wsl-version.utf16.txt`, `wsl-list-verbose.utf16.txt`). `wslc --session X system session terminate` exists as a command (it failed with SESSION_NOT_FOUND for a missing session, `session-terminate-missing.txt`), its success behaviour is untested.
  - Still unverified failure cases [U]: name collision (`run --name` twice), port already in use, remove of a running container without `--force`, `kill`, `wslc` missing or WSL too old (not testable without uninstalling), session list with 0 or 2+ sessions, permission-denied cases, the GitHub releases API sample, and localised stderr on non-English Windows.
- Phase 0 (remaining spike, WSLC only): captured `wsl -l --running` (optional); a two-session test; fixtures for the unverified failure cases above; fixtures for other `State` values and multiple networks or port mappings; GitHub releases API check for the latest WSL version (tags, pre-releases, relation to the installed 3.0.5.0 version string); clean-machine install check; persistence after exit/logoff/reboot (only needed for W2 and the sessions panel); C SDK vs CLI visibility (only if W2 is revisited). Go/no-go measures: list under 1 s, event delivered under 1 s, start/stop under 3 s excluding the container's own shutdown time.
- Phase 1 (MVP): REQ-000 to REQ-009, WSLC only (`WslcCliConnector`).
- Release gate (after MVP, before the first public release): REQ-012 release and signing pipeline.
- Phase 2: REQ-010 sessions panel, REQ-011 stats, then the Docker/Podman phase start (Docker fixtures, `DockerCliConnector`, distro discovery, D3/D7/D8 revisited).
- Phase 3: images/volumes, exec, settings, Could items, i18n if wanted.

## 11. Open questions for the user

Decided since v4: latest-version source (option D), banner and daily check on by default, UAC/stop risk accepted, WSLC-only MVP, English-only UI, Windows 11 only (D14), SignPath Foundation signing and MIT licence (D15).

Still open:
1. When the Docker phase starts (after MVP, or after Phase 2)?
2. Who fills the SignPath team roles (author, reviewer, approver) for a possibly one-person project, since SignPath expects these roles [R] and whether one person can hold several is unverified [U]?

## Sources

- `obsidians/fixtures/README.md`, `obsidians/fixtures/errors/README.md` and the fixture files listed there (verified on WSL 3.0.5.0, 2026-11-11)
- SignPath Foundation terms (signpath.org/terms) and docs (docs.signpath.io) [R, read by a helper, not independently verified]
- [WSL containers (Microsoft Learn)](https://learn.microsoft.com/windows/wsl/wsl-container) [R, via helper]
- [WSL container API reference (wsl.dev)](https://wsl.dev/api-reference/) [R, via helper]
- microsoft/WSL master source (`wslcsdk.h`, `src/windows/wslc/commands/`) [R, read by a helper agent, not run]
- [Microsoft.WSL.Containers on NuGet](https://www.nuget.org/packages/Microsoft.WSL.Containers)
- [WSL 2.9.3 release notes mirror (preview)](https://newreleases.io/project/github/microsoft/WSL/release/2.9.3)
- [Windows Developer Blog: GA](https://blogs.windows.com/windowsdeveloper/2026/09/29/wsl-containers-now-generally-available/)
- [bollard 0.21.0 README](https://docs.rs/crate/bollard/0.21.0/source/README.md)
- [Rust GUI frameworks comparison](https://www.pistack.xyz/posts/2026-08-25-rust-gui-frameworks-egui-iced-slint-comparison/)
