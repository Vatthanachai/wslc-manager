---
id: REQ-001
title: "WSL/wslc availability, version and sessions"
priority: Must
connector: wslc
status: todo
depends_on:
  - "[[REQ-000-walking-skeleton]]"
tags:
  - requirement
  - must
---

# REQ-001 WSL/wslc availability, version and sessions

- Priority: Must (US-1)
- Connector scope: WSLC (Docker/Podman and WSL distro discovery are deferred to a later phase)
- Dependencies: REQ-000
- User interface: yes. Design: [[REQ-001-002-008-ux]] (status `ready`, owner decisions approved 2026-11-11). Batch with [[REQ-002-detect-engine]] and [[REQ-008-error-handling]].

## User story
As a user, I want the app to tell me whether WSL and `wslc.exe` are available, which versions I have, and which WSLC sessions exist, so that I know whether the dashboard can work and what it will show.

## Decisions (owner, approved 2026-11-11, see the UX note)
- Layout: top tab strip, status bar, one Runtime panel (no left rail).
- This batch only DISPLAYS `wsl --install` and `wsl --update` as copyable text. The app never runs them, and no external link or browser handoff exists in this batch. The exact `wsl --install` text is [U] (standard command, not verified on this machine).
- Gate states: Available, WslNotInstalled, WslcMissing, WslTooOld, RuntimeUpdateNeeded, MissingComponents, Unavailable, all built now from one shared gate template. Evidence for RuntimeUpdateNeeded is only the token `WSLC_E_RUNTIME_UPDATE_NEEDED` [R, from source, not observed]. Evidence for MissingComponents is none [U]. Both are exercised by an injected fake runner, not by a real machine.
- Versions: `wslc version` (plain text `wslc <x.y.z.w>` [V]) is the single source for the wslc version and for the tested/untested decision (REQ-002). `wsl --version` (UTF-16LE [V]) is used only for the WSL version shown in the UI.
- WslTooOld: no version floor is hard-coded (REQ-002). If `wslc.exe` is missing and WSL is present, the state is `WslcMissing` and its copy says WSL "may be too old". `WslTooOld` is produced only when a runtime signal says so, and no such signal is known [U]. The token `WSLC_E_RUNTIME_UPDATE_NEEDED` gives `RuntimeUpdateNeeded`, not `WslTooOld`.
- Sessions: the app does not need to know which session is "default". Facts [V] from the owner's machine: `wslc system session list --verbose` printed one session with ID 1, Creator PID 40580, Display Name `wslc-cli-<user>`; `wslc info` lists the same; the wslc CLI resolves the default session from the caller's token and `--session` is a global option. The app lists every session from `session list` and shows its display name. A "default" badge is shown only if a reliable way to know which session the CLI uses when `--session` is omitted is found (dev spike [U]); otherwise there is no badge. The badge is optional in the Runtime panel.
- Zero sessions is a SUCCESS (empty state "No WSLC sessions found"), because wslc creates or opens the default session on demand [R, from source: OpenOrCreateDefaultSession]. The exact output of `session list` with 0 sessions is [U]; empty or unknown output must not crash the app.
- Window focus re-check while a gate is shown: ON, at most once per 5 s. No other automatic retry in this batch.
- Toast code is NOT written in this batch (REQ-004). Icons use the Segoe Fluent Icons system font (Windows 11 only, D14), not inline SVG.
- Dismissing the untested-version banner is remembered only for the running session, never across restarts.
- Facts marked [U] are unverified.

## Acceptance criteria

Availability and versions
- Given WSL is installed, when the app starts, then the WSL version from `wsl --version` is shown within 3 s. The output is UTF-16LE [V] and must be decoded without stray NUL characters (the displayed string contains no U+0000).
- Given `wslc.exe` is present, then its version from `wslc version` (`wslc 3.0.5.0`, plain text, exit 0 [V]) is shown and WSLC is marked Available.
- Given `wslc.exe` is not found and WSL is present, then the state is WslcMissing, shown as a gate with the title "wslc was not found", the body saying WSL "may be too old", the copyable text `wsl --update`, and a Retry button. The app does not crash.
- Given WSL is not installed, then the state is WslNotInstalled, shown as a gate with a copyable `wsl --install` text [U exact text] and a Retry button. There is no link and nothing is opened or run. The app does not crash.
- Given `wslc` reports the token `WSLC_E_RUNTIME_UPDATE_NEEDED` [R], then the state is RuntimeUpdateNeeded, shown as a gate with the update hint (`wsl --update` text) and Retry.
- Given a detection rule for missing components is found [U], then the state is MissingComponents, shown as a gate with an update-or-repair hint and Retry. Until a rule exists the gate is reachable only through the injected fake runner (test).
- Given none of the above and detection still fails (timeout of 5 s or an unexpected failure), then the state is Unavailable, shown as a gate with Retry and Details.
- Given availability states, then the result is exactly one of Available, WslNotInstalled, WslcMissing, WslTooOld, RuntimeUpdateNeeded, MissingComponents, Unavailable, and each has an English message from the single string resource module. WslTooOld is produced only by an explicit runtime signal [U] and never from a version number comparison in the UI.
- Given any gate is shown, then all gates use the same template (title, body, copyable command where one applies, Retry, Details) and the header chip says "Not available". Focus moves to the gate's primary button.
- Given a command text is shown, then "Copy command" puts exactly that text in the clipboard and the button text changes to "Copied" for 2 s.
- Given the app is running, then no process is started that runs `wsl --install` or `wsl --update` (the tester checks the code path and the fake runner call log).

Sessions
- Given WSLC is available, then sessions are discovered with `wslc system session list --verbose` [V]: the table is parsed into ID, Creator PID and Display Name, and the `[wslc] Found N session` line [V] must match the parsed row count, otherwise `ParseError` with raw output and expected vs found counts in the details.
- Given the output lists one or more sessions, then every session is shown with its Display Name (and ID and PID in the Runtime panel), and the status bar shows the count.
- Given the output lists zero sessions or is empty, then the state is still Available and the main region shows the empty state "No WSLC sessions found". No error is shown and the app does not crash [U exact output of `session list` with 0 sessions].
- Given the output is in a shape the parser does not know, then the result is `ParseError` (banner, Retry, raw output in Details), not a crash and not an empty state.
- Given a way to identify the session used without `--session` is found [U], then that session shows a "default" badge. Given no such way, then no session shows a badge and no criterion fails.
- Given a version of wslc other than the tested 3.0.5.0, then a non-blocking "untested version" notice is shown [U for other versions]. See REQ-002 for the decision source.

Loading and app shell
- Given the app starts, then nothing extra is shown during the first 150 ms. After 150 ms with detection still running, a loading panel shows three lines "Checking WSL...", "Checking wslc...", "Looking for sessions...", the current one with a spinner. At 3 s the muted line "Still checking. This can take a few seconds." appears. At 5 s a step that has not finished ends with a typed `Timeout`.
- Given the shell, then it has the regions header (with nav tabs and the Runtime chip), notices, main and status bar, and one tab "Containers". No disabled "coming soon" tab exists.
- Given the status bar when Available, then it shows availability, the wslc version (with "(untested)" next to it when untested), the WSL version and the session count, in that priority when the width is short (checked at 800 x 600).
- Given the Runtime chip is activated or Ctrl+I is pressed, then the Runtime panel opens with status, WSL and wslc versions, sessions and capabilities (REQ-002); pressing Ctrl+I again or Esc closes it and focus returns to the chip.
- Given F5 or Ctrl+R is pressed, then detection and discovery re-run once (same as Retry). Given a run is already in flight, then the key is ignored. The webview's own page reload does not happen.
- Given F6 (and Shift+F6) is pressed, then focus cycles through the header, notices, main and status bar regions. Every shortcut also has a visible button or control.
- Given the untested-version banner is dismissed, then it stays hidden until the app is restarted, the "(untested)" marker in the status bar and Runtime panel stays, and nothing is written to disk.
- Given a gate is shown and the window regains focus, then detection re-runs automatically, but at most once per 5 s and never while a detection is already running. Given the state is Available (no gate), then window focus triggers nothing.
- Given any icon is shown (Segoe Fluent Icons font), then it is `aria-hidden` and paired with visible text; if the font glyph is missing, the meaning is still clear from the text.
- Given any user-visible text, then it comes from `src/strings.ts` (no literals in code or tests where the string exists), and all dynamic values are set through `textContent`.

## Fixtures
- `obsidians/fixtures/session-list-verbose.txt` (one session, [V]). A two-session and an empty-session capture are still needed [U].
- `obsidians/fixtures/wsl-version.utf16.txt` (UTF-16LE), `obsidians/fixtures/errors/wslc-version.txt`. `obsidians/fixtures/wsl-list-verbose.utf16.txt` is available but only needed by the deferred distro discovery.
- Gate states for WslNotInstalled, WslcMissing, RuntimeUpdateNeeded, MissingComponents and Unavailable use an injected fake runner (no real fixture exists, [U]).

## Spikes and manual checks still needed (before or during implementation)
- `wslc system session list` output with 0 sessions and with 2+ sessions (count line wording, singular and plural) [U].
- How `wslc.exe` missing looks to the app (spawn error code) and how WSL missing looks (`wsl.exe` absent or a stub) [U]; not testable here without uninstalling.
- Any real signal for WslTooOld and for MissingComponents [U]; a real `WSLC_E_RUNTIME_UPDATE_NEEDED` run [R].
- `wsl --version` timing on a cold machine against the 3 s target [U].
- Identification of the session the CLI uses when `--session` is omitted (for the optional "default" badge) [U].
- Exact text of `wsl --install` and whether it behaves as expected (only displayed, never run) [U].
- Behaviour of the window focus event in WebView2 (fires on activation of the window) [U].
- Tracked in [[unverified-items]].

## Definition of done
- This ticket is done only when the checklist in [[qa/definition-of-done]] is met.

## Out of scope
- WSL distro discovery (`wsl -l -v`), Docker/Podman detection, starting distros, hiding `docker-desktop` distros (all deferred with Docker), running or automating WSL install or update (REQ-009 decides any flow that runs a command), opening external links, toast component (REQ-004), persisted settings.
