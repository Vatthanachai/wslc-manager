---
tags:
  - qa
  - testing
status: draft-v1
updated: 2026-11-11
---

# Test strategy

Context: [[ba-analysis]] (evidence labels [V]/[R]/[U], error mapping, performance targets), [[unverified-items]], [[decision-log]], [[REQ-013-ci-and-qa-baseline]], fixtures in `obsidians/fixtures/` (see its README and `errors/README.md`). Walking-skeleton results: [[REQ-000-test-report]].

Assumptions (marked A): CI is a GitHub-hosted `windows-latest` runner with no WSL and no `wslc.exe` (A, to be confirmed in REQ-013). The frontend runner is Vitest (REQ-013). The MVP is WSLC only (D1/D2), Windows 11 only (D14).

## 1. Levels and what belongs where

| Level | Scope | Where | CI | Local |
|---|---|---|---|---|
| L1 Rust unit | pure functions: parser helpers, error mapping, version compare, string table | `#[cfg(test)] mod tests` next to the code | yes | yes |
| L2 Parser/contract | real fixture files fed to parsers | `crates/*/tests/` | yes | yes |
| L3 Fake-runner connector | `WslcCliConnector` with an injected fake runner: args, exit codes, timeout, cancel, non-UTF-8 | `crates/*/tests/` | yes | yes |
| L4 Frontend unit | rendering of states and errors, string module, no network | Vitest (REQ-013) | yes | yes |
| L5 UI smoke | real window, real Tauri command path, FakeConnector | WebView2 remote debugging (section 3) | no (A) | yes |
| L6 Real wslc integration | real `wslc.exe` | `#[ignore]` tests, see [[qa-integration-convention]] | no | yes, opt-in |
| L7 Manual / clean machine | MSI install, upgrade, repair, uninstall, WebView2 absent, high DPI | VM, checklist in the ticket report | no | yes |
| L8 Performance | timings against targets | manual script, recorded in the report | no | yes |

Rules: a test sits at the lowest level that can prove the claim. No test depends on order, the network, a real WSL or the user's real containers outside L6.

## 2. Parser and contract tests (L2)

- Feed the real fixtures, not hand-made lookalikes. Fixtures carry CRLF and no BOM (checked 2026-11-11): tests must tolerate the trailing CR. BOM is covered only by synthetic input (BUG-001 history).
- JSON-lines commands (`container list`, `stats`, `image list`, `events`): one object per line, blank lines skipped. Empty stdout means an empty list, not an error [V].
- `container inspect` is a JSON array and has its own parser. `session list` is a text table with `[wslc] Found N session` as a cross-check.
- `wsl --version` / `wsl -l -v` fixtures are UTF-16LE (`*.utf16.txt`): decode with the separate decoder, never as UTF-8.
- Always add: unknown keys ignored, missing keys become `None`, unknown `State` becomes `Unknown(raw)`, garbage gives `ParseError` with raw output, very long and non-ASCII names, 10k+ rows.
- Error mapping tests use `fixtures/errors/*.txt`: exit code first, then the `WSLC_E_*` token; never assert on English message text. Every row of the mapping table in [[ba-analysis]] section 6 needs one test.

## 3. UI smoke via WebView2 remote debugging (L5)

Method used for REQ-000:
1. Build `bunx tauri build --debug --no-bundle`.
2. Set `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--remote-debugging-port=9222` and `WEBVIEW2_USER_DATA_FOLDER` to a fresh throwaway folder (never the user's profile).
3. Start the exe. `GET http://127.0.0.1:9222/json` lists the page; use CDP `Runtime.evaluate` to read the DOM and `Page.captureScreenshot` for evidence. DOM text is the authoritative evidence, a screenshot is supporting.
4. Always kill the app and the test-profile `msedgewebview2` processes afterwards (filter by the throwaway folder; leave other apps' WebView2 alone), then delete the folder. Verify 0 leftover processes.

Cover: window opens with the right title, a clear in-app error (not a blank screen) when WSL/wslc is missing or denied, and the states in the UX design under `obsidians/design/`. `tauri-driver` is optional, only if it proves practical (A).

## 4. Performance (L8)

Targets from [[ba-analysis]] section 10: container list under 1 s per scope, event delivered under 1 s, start/stop under 3 s excluding the container's own shutdown time, cold start under 2 s. Restart can take about 10 s when PID 1 ignores SIGTERM [V]: not a defect, but the UI must show progress.
Measure at least 10 runs, report median and worst, with machine, WSL and wslc version. Local only, never gated in CI (shared runners are noisy). A missed target is a Medium defect until the owner decides.

## 5. CI versus local

CI (GitHub-hosted Windows, no wslc): `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-targets` and `--doc`, `bun install --frozen-lockfile`, `bun run build`, `bun run test` (never `bun test`: it is bun's own runner and ignores `vitest.config.ts`). `#[ignore]` tests are listed but not run. Local only: L5 to L8. Changes to CI or caching need their own ticket ([[REQ-013-ci-and-qa-baseline]], [[REQ-012-release-and-signing-pipeline]]).

## 6. Fixture capture procedure

1. Use a throwaway container named `aiyara-test-*` ([[qa-integration-convention]]), never the user's own.
2. Record `wsl --version` and `wslc version` in the fixtures README header (date, WSL/wslc version, OS build).
3. Capture the raw bytes of stdout and stderr and the exit code. Strip nothing (no BOM removal, no CRLF fix, no trimming). For `wsl.exe`, capture the UTF-16LE bytes via `cmd /c ... > file`.
4. Replace only the Windows user name by `<user>` (also in session names and paths). Grep the result for the real user name and `C:\Users\`.
5. Name the file after command and condition (`container-stop-missing.txt`). Error files go under `fixtures/errors/` and hold COMMAND, EXIT, STDOUT, STDERR.
6. Update `fixtures/README.md` or `errors/README.md` (command table, verified/unverified lists), add the contract test, and move the fact from [[unverified-items]] into [[ba-analysis]] with a [V] label.
7. Remove the test resources. Re-capture after each WSL version the app claims to support; keep old fixtures per version, do not overwrite silently.

## 7. Unverified [U] items

Every [U] item a ticket depends on needs either a spike (fixture captured, [V] recorded) before the ticket is finished, or a clearly marked pending manual check in the ticket's test report, with the fallback behaviour (for example `CommandFailed{code, stderr}`) tested instead. The ticket's checklist in [[definition-of-done]] lists them. Current list: [[unverified-items]].

Back to [[README|QA index]].
