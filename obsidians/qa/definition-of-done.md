---
tags:
  - qa
  - process
status: draft-v1
updated: 2026-11-11
---

# Definition of done, severity, templates

Applies to every ticket in [[README|requirements index]] before `git flow feature finish`. Strategy: [[test-strategy]]. Facts: [[ba-analysis]]. Decisions: [[decision-log]].

## 1. Checklist

- [ ] Each acceptance criterion is mapped to a test (name in the report) or to a recorded manual check (who, when, machine, result).
- [ ] `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` and `cargo test --workspace` pass. These are the commands CI runs. `--workspace` covers every crate's unit, integration and doc tests; `#[ignore]` tests are listed as ignored, not run. No separate `--all-targets` or `--doc` run is needed (details: [[ci-and-toolchain]]).
- [ ] `bun install --frozen-lockfile`, `bun run build` and `bun run test` pass (frontend runner from [[REQ-013-ci-and-qa-baseline]], package manager from [[REQ-014-switch-package-manager-to-bun]]; Node 24 is needed because Vitest runs on Node). Never `bun test`.
- [ ] Tester report `obsidians/plans/REQ-NNN-test-report.md` exists with verdict pass and no open defect above Low. Each open Low defect is listed with an owner decision (accept, fix later with ticket id).
- [ ] Every new [V] fact has a fixture (captured per [[test-strategy]] section 6) and a contract test, and is recorded in [[ba-analysis]].
- [ ] Unverified [U] items the ticket depends on are listed in the report and in [[unverified-items]], each with a spike result or a marked pending manual check.
- [ ] Ticket status, [[README|requirements index]] and related docs updated.
- [ ] No personal data in code, docs, tests or fixtures: no real Windows user name, no absolute user paths (`C:\Users\...`, `Z:\...`). Fixtures use `<user>`.
- [ ] UI tickets: screens checked against `obsidians/design/` (each screen/state in the design has a test or a screenshot/DOM check; deviations listed).
- [ ] UI text lives only in the string module (D13). No network calls from the frontend. No `#[ignore]` without a reason string.
- [ ] No caching or CI changes unless the ticket asks for them.
- [ ] Changes reach `main` by pull request only (D16: merge commit, conversations resolved, no direct push). Feature work merges into `develop`.

## 2. Severity scale

| Severity | Meaning | Examples for this app |
|---|---|---|
| Critical | data loss, security hole, app unusable for everyone | remove or `wsl --update` runs without a click; command built from a shell string allows injection; crash at start; blank window with no message |
| High | main flow broken or destructive action without safeguard | destructive action without confirmation or with default focus on the destructive button; start/stop acts on the wrong container or session; list shows stale or wrong state; update dialog hides the UAC/stop warning |
| Medium | feature degraded, workaround exists, or a target missed | list over 1 s; wrong error class shown (for example not-found shown as generic failure); error matched on English text; button not disabled while busy; resource leak (orphan process after closing logs) |
| Low | cosmetic or rare, no data risk | label typo, alignment, BOM-only parse failure with real output being plain text (BUG-001), missing tooltip |

Rule: Critical, High and Medium block the ticket. Low may ship with an owner decision.

## 3. Defect report template

```
### BUG-NNN <short title> (OPEN/FIXED/WONTFIX)
- Ticket: REQ-NNN   Severity: <Critical|High|Medium|Low>   Found: <date>
- Environment: Windows build, WSL/wslc version, app build (commit), fake or real wslc
- Steps to reproduce: 1. ... 2. ...
- Expected:
- Actual (include exit code, stdout, stderr, screenshot or DOM text):
- Cause (if known; do not fix production code in QA):
- Test: file and test name (ignored with the BUG id until fixed)
- Owner decision (for Low): accept / fix in REQ-NNN
```

## 4. Test report template

File `obsidians/plans/REQ-NNN-test-report.md`, frontmatter `ticket`, `status` (pass/fail), `tags: [test-report]`. Model: [[REQ-000-test-report]].

1. Title, requirement link, [[ba-analysis]] link, branch, date, OS.
2. Summary: what works, defects found, verdict.
3. Test matrix: table Criterion / Test / Result (every acceptance criterion, plus parser, security, hygiene rows).
4. Commands run: exact commands with real counts (passed, failed, ignored), clippy, fmt, bun, app launch evidence.
5. Defects: per the template above, with re-verification notes.
6. Notes for later tickets and observations (not defects).
7. Unverified: what was not tested and why ([U] items, real wslc, MSI, VM, high DPI).
8. Design check (UI tickets): screen vs `obsidians/design/`, deviations.
9. Verdict: pass or fail, with open items.

Never mark a case pass without running it. Report real numbers.

Back to [[README|QA index]].
