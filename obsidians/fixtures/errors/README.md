# wslc error fixtures

Captured 2026-11-11 on WSL 3.0.5.0 / wslc 3.0.5.0 (session `wslc-cli-<user>`). Each `*.txt` holds COMMAND, EXIT, STDOUT and STDERR (UTF-8, no BOM, CRLF line endings). Related fixtures one level up: `wsl-version.utf16.txt`, `wsl-list-verbose.utf16.txt` (raw `cmd /c` redirect, UTF-16LE, no BOM), `container-list-exited-ports.json`.

## Verified (observed)

- Every failure exits with code **1**; success is 0. Errors go to **stderr**, stdout stays empty, with one exception: `container inspect <missing>` prints `[]` to stdout, a message to stderr, exit 1.
- Domain errors end with `Error code: WSLC_E_*` on its own line, followed by a "filing a new issue" hint line. Seen: `WSLC_E_CONTAINER_NOT_FOUND`, `WSLC_E_SESSION_NOT_FOUND`, `WSLC_E_IMAGE_NOT_FOUND`. Parse the `WSLC_E_` token from stderr; do not match the English text.
- `container stats <missing>` prints an empty line, then only the `Error code:` line (no human message).
- `container inspect <missing>` has **no** `Error code:` line (message only).
- Usage errors (bad `--format`, unknown option, unknown command) print the message plus usage text to stderr, exit 1, and have **no** `Error code:` line.
- Bad `--session NAME` -> `Session not found: '<name>'` + `WSLC_E_SESSION_NOT_FOUND`.
- Missing image on `run` also prints `Unable to find image '<x>' locally` before the pull error; on `image pull` and `run` the error code is `WSLC_E_IMAGE_NOT_FOUND` (pull access denied text).
- `wslc version` -> `wslc 3.0.5.0` (exit 0, plain text).
- `wsl -l -v` and `wsl --version` output is **UTF-16LE** (the `-l -v` file starts with bytes `32 0 32 0`, i.e. two spaces; BOM handling differs by how it is captured). Distros on this machine: `kali-linux` (default, Stopped, v2), `docker-desktop` (Stopped, v2).

## Exited-state fixture (`../container-list-exited-ports.json`)

Container `alpine` with `sh -c 'echo done; exit 3'`, `-p 18080:80`, labels `app=aiyara`, `tier=test`:
- `State` = `exited`, `Status` = `Exited (3) 3 seconds ago`.
- `Ports` string is **empty** even with a published port; the mapping lives only inside the `Labels` string under `com.microsoft.wsl.container.metadata` (JSON with `Ports[]`: BindingAddress, ContainerPort, Family, HostPort, Protocol, VmPort). Labels are one comma-joined string, not an object.
- `container logs` on an exited container works (exit 0, prints the output).
- `container stop` on an already-stopped container exits **0** and prints the name (idempotent).
- `container stats` on a stopped container exits 0 with zeroed values (`PIDs` 0), not an error.

## Still unverified

- Start/stop of a container the user lacks permission for, name collision (`run --name` twice), port already in use, remove of a running container without `--force`, `kill`.
- Behaviour when `wslc.exe` is missing or WSL is too old (not testable on this machine without uninstalling).
- stderr text on non-English Windows (messages appear localized; match `WSLC_E_*` only).
- Session list with 0 or 2+ sessions.
