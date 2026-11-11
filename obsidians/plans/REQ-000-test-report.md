---
ticket: REQ-000
status: partial
tags: [test-report]
---

# REQ-000 test report

Requirement: [[REQ-000-walking-skeleton]]. Context: [[ba-analysis]]. Branch `feature/REQ-000-walking-skeleton`, tested 2026-11-11 on Windows 11 (10.0.26300).

## Summary

The skeleton builds, lints and runs. All five acceptance criteria are met in substance. The window renders the one FakeConnector container through the real Tauri command path (confirmed via WebView2 DevTools protocol and a screenshot). One defect found: a UTF-8 BOM in stdout makes `parse_container_list` fail (BUG-001, Low/Medium). Two QA tests that expose it are `#[ignore]`d with the bug id so the suite stays green. Verdict: partial (pass with one open defect).

## Test matrix

| Criterion | Test | Result |
|---|---|---|
| AC1 build + window starts | `cargo test --workspace` compiles all crates; `npm run build`; `npx tauri build --debug --no-bundle`; app launched, window title "Aiyara Container Dashboard" | pass |
| AC2 one FakeConnector container via Tauri command | CDP `Runtime.evaluate` on the live WebView2 page, screenshot; `fake_connector_is_object_safe_and_echoes_scope`, dev `fake_connector_returns_one_container` | pass (DOM: heading Containers, 1 tbody row: aiyara-spike / alpine:latest / Running / Up 3 seconds / Fake session) |
| AC3 FakeConnector unit test, no WSL | dev `connectors.rs`, QA `fake_connector_is_object_safe_and_echoes_scope` | pass |
| AC4 injected fake runner | dev `wslc_connector_*`; QA args as separate args (spaces, quotes, `; rm -rf /`, empty, `--format`, 100k chars), non-zero/negative exit, runner Timeout propagation, non-UTF-8 and UTF-16LE stdout (no panic) | pass |
| AC5 strings only in strings.ts | QA `ui_text_only_in_strings_module` (heuristic scan of main.ts, index.html title empty) | pass |
| Parser: JSON lines, blank/CRLF, empty = empty list, unknown keys, missing keys, garbage -> ParseError, unknown state raw | dev tests plus QA: both real fixtures (running, exited + ports/labels), trailing line without newline, JSON array -> ParseError, scalar/null lines, wrong-typed value, null values, `{}`, duplicate keys (no panic), case-sensitive state, 1 MB name, non-ASCII, 20,000 rows, unicode escapes | pass |
| Parser: BOM | `utf8_bom_prefixed_output_is_accepted`, `bom_only_is_empty_list` | FAIL, ignored (BUG-001) |
| Security: strict CSP, empty capabilities, no network, forbid(unsafe_code) | QA `qa_hygiene.rs` (csp_is_strict, capabilities_grant_no_permissions, frontend_has_no_network_calls_or_unsafe_dom, no_network_crates_in_dependencies, forbid_unsafe_...) | pass |
| Hygiene | `gitignore_covers_build_output`, `no_personal_paths_in_sources_and_fixtures`, manual grep | pass |

## Commands run

- `cargo test --workspace`: 46 passed, 0 failed, 2 ignored (BUG-001), including 1 doc test per crate that has one.
- `cargo clippy --workspace --all-targets -- -D warnings`: no issues.
- `cargo fmt --check`: clean.
- `npm run build` (`tsc --noEmit && vite build`): ok, 6 modules, dist/index.html + 2.48 kB JS.
- `npx tauri build --debug --no-bundle`: ok, `target/debug/aiyara-app.exe` (12.3 MB).
- Launch with `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--remote-debugging-port=9222` and a throwaway `WEBVIEW2_USER_DATA_FOLDER`; `GET http://127.0.0.1:9222/json` gave page `http://tauri.localhost/` titled "Aiyara Container Dashboard"; `Runtime.evaluate` returned the table text above (rows=1). App process then killed; 0 `aiyara-app` and 0 test-profile `msedgewebview2` processes remained (unrelated WebView2 processes of other apps were left untouched).

## Defects

### BUG-001 UTF-8 BOM in stdout makes the parser fail
- Severity: Low (real `wslc` output is plain text without BOM per fixtures README; Medium if output is ever redirected through PowerShell or file capture, as the fixtures were).
- Steps: `parse_container_list("\u{feff}{\"ID\":\"a\"}\r\n", &scope)`; also `"\u{feff}"` alone.
- Expected: one container / empty list (the dev states BOM is handled; the fixtures README says to strip BOM and CRLF).
- Actual: `Err(ParseError { raw })`. Cause: `str::trim` does not strip U+FEFF, and `String::from_utf8_lossy` keeps it.
- Tests: `crates/engine/tests/qa_parse_edge.rs` (`utf8_bom_prefixed_output_is_accepted`, `bom_only_is_empty_list`), currently `#[ignore = "BUG-001 ..."]`. Remove the ignore when dev fixes it.

## Observations (not defects)

- Non-UTF-8 bytes are replaced lossily (U+FFFD), no panic; UTF-16LE stdout gives ParseError (acceptable for `wslc`, relevant later for `wsl.exe`).
- A single malformed line fails the whole list, and `raw` holds the entire stdout.
- `container-list.json` in the repo has no BOM or CRLF as stored, although the fixtures README says it does; the dev test's BOM stripping is therefore never exercised by real data.
- `wslc-info.txt` contains `C:\Users\<user>\...` with the user name already replaced by a placeholder; no real personal path found. Commit author metadata is the repo owner's own.
- `ui_text_only_in_strings_module` is a heuristic, not a proof.

## Unverified

- No frontend unit test runner exists, so `main.ts` error rendering (parseError, etc.) was not exercised; only the happy path was seen live.
- Not tested: real `wslc.exe`, timeouts of a real process runner (none exists yet), MSI/bundle, clean-VM install, WebView2-absent behavior, high-DPI. The screenshot was taken with the window overlapping other windows; DOM content is the authoritative evidence.
- Clean checkout build was not repeated from scratch (incremental build used).

## Verdict

Partial: functionally pass for REQ-000 acceptance criteria; BUG-001 open (BOM handling) and two tests ignored for it.
