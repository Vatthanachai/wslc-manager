# wslc fixtures (Phase 0 spike)

Captured 2026-11-11 on WSL 3.0.5.0 / wslc 3.0.5.0, Windows 10.0.26300, session `wslc-cli-<user>` (the Windows user name is replaced by `<user>` in all fixtures).
Container: `alpine:latest` running a 1-second tick loop (`aiyara-spike`, removed after capture).
Files were written on Windows. Checked on 2026-11-11: none has a UTF-8 BOM, all carry CRLF line endings, and the two `wsl-*.utf16.txt` files are UTF-16LE without a BOM. Tests must strip the trailing CR. BOM handling is covered only by synthetic tests (real `wslc` output is plain text).

| File | Command |
|------|---------|
| container-list.json | `wslc container list --all --format json` |
| container-stats.json | `wslc container stats --format json aiyara-spike` |
| container-inspect.json | `wslc container inspect aiyara-spike` |
| image-list.json | `wslc image list --format json` |
| container-logs-tail.txt | `wslc container logs --tail 3 -t aiyara-spike` |
| container-logs-follow-sample.txt | last 3 lines of `wslc container logs --follow -t aiyara-spike` (4 s) |
| events.jsonl | `wslc system events --format json` while running `wslc container restart` |
| session-list-verbose.txt | `wslc system session list --verbose` |

## Verified behaviour (observed, not inferred)

- `--session` is a global option on every command.
- `container list --format json` prints **one JSON object per line** (JSON lines, Docker style), not an array. With no containers it prints **nothing** (exit 0), not `[]`.
- `container list` JSON keys (PascalCase, all values strings except `Platform`): Command, CreatedAt, HealthStatus, ID (12 chars), Image, Labels, LocalVolumes, Mounts, Names, Networks, Platform{architecture,os}, Ports, RunningFor, Size, State (`running`), Status. `CreatedAt` looks like `2026-11-11 14:09:43 +0700 GMT+7`.
- `container stats --format json`: one object per container: BlockIO, CPUPerc, ID (full 64 chars), MemPerc, MemUsage, Name, NetIO, PIDs (number). Values are preformatted strings ("0.06%", "1.672MiB / 15.43GiB").
- `container inspect` prints a pretty-printed JSON **array** (Docker-style inspect; first key Config).
- `image list --format json`: JSON lines with Containers, CreatedAt, CreatedSince, Digest, ID, Repository, SharedSize, Size, Tag, UniqueSize.
- `system events --format json` **streams** (events arrived while the command was still running). JSON lines with Action, Actor{Attributes,ID}, Type (container, network), from, id, scope, status, time, timeNano. A restart produced kill (signal 15), kill (signal 9), network disconnect, die (exitCode 137), stop, network connect, start.
- `container logs --follow` **streams**; `-t` prefixes an RFC3339Nano timestamp.
- `container restart` took about 10 s here because `sh` as PID 1 ignores SIGTERM and wslc killed it after the default timeout. Stop/restart timeouts must be generous in the UI.
- `system session list` has only `--verbose` (no `--format`); table output with a `[wslc] Found N session` line when verbose. Path is `wslc system session list`, not `wslc session list`.
- `wsl --version` and `wsl -l` output is UTF-16LE; `wslc` output is plain text.

## Still unverified

- Whether containers created by the C SDK appear in the CLI, and vice versa.
- Session persistence after the creating process exits, after logoff and reboot.
- Behaviour with several sessions (only one session existed).
- JSON field casing/shape on other WSL versions.
