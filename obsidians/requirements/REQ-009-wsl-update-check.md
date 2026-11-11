---
id: REQ-009
title: "WSL update check (decision D11)"
priority: Must
connector: wslc
status: todo
depends_on:
  - "[[REQ-001-discover-runtimes]]"
tags:
  - requirement
  - must
---

# REQ-009 WSL update check (decision D11)

- Priority: Must
- Connector scope: WSLC (it concerns WSL itself, which WSLC needs; a later Docker connector would share it)
- Dependencies: REQ-001

## User story
As a user, I want the app to tell me when WSL is not up to date and ask whether to update, so that I get WSL fixes and features without anything being changed behind my back.

## Decisions (D11, final)
- Latest-version source = option D: a local capability check always runs (is a needed feature missing for the detected version, see REQ-002), plus a once-a-day comparison of `wsl --version` with the latest release of microsoft/WSL on GitHub.
- The update banner and the daily GitHub check are ON by default. A setting turns the daily check off. "Remind me later" and "Don't ask for this version" are kept.
- The user accepts that `wsl --update` may need administrator permission (UAC) and may stop distros and containers. The confirm dialog must still warn about both and list what may be affected. Nothing runs without the user's click.
- No supported dry-run for `wsl --update` is known [U]. How the GitHub release tag relates to the installed version string and to the channel `wsl --update` installs (Store/MSIX vs GitHub), and pre-release filtering, are unverified [U] and must be checked in implementation.

## Acceptance criteria
- Given the app starts, when WSL is detected, then the installed version comes from `wsl --version` (UTF-16LE output [V]) and is shown in the About/Runtime panel.
- Given the daily check setting is on (default), when the app starts and the last check is older than 24 h, then the latest stable release of microsoft/WSL is fetched from GitHub and compared with the installed version. The check runs at most once per 24 h.
- Given the check finds a newer version, then a non-blocking banner shows installed and latest versions with buttons "Update now", "Remind me later" and "Don't ask for this version".
- Given I turn the daily check off in Settings, then no network request is made for the check, and the local capability warning (REQ-002) still appears when a feature is unavailable.
- Given I click "Update now", then a confirmation dialog states the exact command (`wsl --update`), warns that Windows may ask for administrator permission (UAC) and that running distros and containers may stop, and lists what may be affected: running WSLC sessions and their containers, and running distros if they can be detected cheaply (for example `wsl -l --running` [U]). Only the explicit "Update" button runs the command.
- Given I click Cancel, "Remind me later" or close the dialog, then no command runs, and "Remind me later" snoozes the banner for 24 h.
- Given "Don't ask for this version" is chosen, then the banner does not return for that version but returns for a newer one.
- Given the update runs, then progress is shown, and on completion the versions are re-detected and the result (success, or exit code and stderr on failure) is shown. If UAC is declined, then the result says so and nothing else changes.
- Given the daily check is on, then the privacy text (README, `obsidians/requirements/REQ-012-release-and-signing-pipeline.md`) discloses it: once a day the app sends a request to the GitHub releases API of microsoft/WSL, with no user data other than what an HTTPS request inherently reveals (IP address, User-Agent). The app must not send machine or container information in that request. The off switch removes the request completely.
- Given the GitHub check is unreachable, rate limited or fails, then nothing alarming is shown, the failure is logged, and the app keeps working.
- Given the user has never clicked "Update", then `wsl --update` is never run: no auto-update, no background update, no update at install time.
- Given the detected version is below what a needed feature requires (for example the per-feature gates 3.0.5 and 3.1.0 [R]), then the relevant feature is disabled with an explanation, independently of the banner.
- Given a version different from the tested 3.0.5.0, then the "untested version" notice from REQ-001 is shown.
- Given all banner and dialog text, then it comes from the single English string resource module (D13).

## Fixtures
- `obsidians/fixtures/wsl-version.utf16.txt` (UTF-16LE). `obsidians/fixtures/README.md` records the baseline 3.0.5.0 and the encoding.
- A sample GitHub releases API response must be captured before implementing the comparison.

## Out of scope
- Updating Windows itself, updating Docker or distros' packages, installing WSL from scratch, rollback, managing WSL pre-release channels.
