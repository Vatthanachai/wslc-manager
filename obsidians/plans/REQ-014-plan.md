---
ticket: REQ-014
status: ready
tags:
  - plan
  - tasks
---

# REQ-014 plan and tasks: switch the package manager to bun

Ticket: [[REQ-014-switch-package-manager-to-bun]]. Builds on [[REQ-013-plan]] and [[REQ-013-test-report]] (npm-based evidence that must be redone). QA: [[qa/definition-of-done]], [[qa/test-strategy]]. No UI change, so no UX step. Branch: `feature/REQ-014-switch-package-manager-to-bun` (git flow).

## Decisions taken in this plan (answers to the ticket's open questions; change them before dev starts if you disagree)

| # | Question | Decision | Why |
|---|---|---|---|
| 1 | Node pin | Keep `.nvmrc` and `engines.node` for now; `dev` tests removing Node locally and on CI in a throwaway branch and reports. Remove the pin only if everything (Vite build, Vitest, Tauri CLI, debug build) runs without a `node` binary on PATH, with the reason in the PR. If removed, CI no longer needs `setup-node` | do not remove a pin without proof |
| 2 | Version pin source | `packageManager: "bun@<version>"` only; `oven-sh/setup-bun` gets the same version with a comment pointing to `package.json`. Use the version installed locally (1.3.14) unless the lookup shows it is unavailable or the runner cannot install it | one place to bump |
| 3 | `trustedDependencies` | Absent. Add only for a proven failure | bun blocks dependency scripts by default, which is the point |
| 4 | Runtime for Vitest | Default (`bun run test` -> the `vitest` bin as resolved: Node via shebang if Node is present). `dev` records which runtime actually ran and compares one run with `bun --bun run test` for information only, not as the CI path | keep the REQ-013 evidence valid |
| 5 | Guard against `bun test` | `dev` checks what `bunfig.toml` can do (e.g. `[test]` options such as a preload or root that makes `bun test` fail loudly); if nothing reliable exists, document the rule in README and in the test strategy | decided on facts |
| 6 | Stray-lockfile CI check | Not in this ticket | could-have |

## Approach

1. **Lockfile and pin**: remove `package-lock.json` and `node_modules`, run `bun install` (text `bun.lock`; if bun writes the binary `bun.lockb` on 1.3.14, record it and stop to ask), commit `bun.lock`, add `packageManager`. Check `.gitattributes` handling (LF) after a fresh clone; add `bun.lock text eol=lf` only if needed. Record resolved versions that differ from the old lockfile (vitest, jsdom, vite, typescript, @tauri-apps/*), and whether any dependency's install script was blocked.
2. **Hooks and scripts**: `tauri.conf.json` -> `bun run dev` / `bun run build`; script names unchanged; keep `test` = `vitest run`; guard `bun test` per decision 5.
3. **CI**: `ci.yml`: replace the npm steps with `oven-sh/setup-bun` (pinned to a full SHA, verified against the tag with `gh api`, annotated tags dereferenced), `bun install --frozen-lockfile`, `bun run build`, `bun run test`; bun install cache keyed on `bun.lock` + OS + bun version (use the action's cache feature if it exists at the pinned version, otherwise `actions/cache` pinned by SHA on the bun cache dir; check with `gh api` what the action supports and keep the cache out of anything release-like); job names, triggers, permissions unchanged; keep `setup-node` only if decision 1 keeps Node.
4. **Docs**: README "Local checks" and prerequisites, `qa/test-strategy.md`, `qa/definition-of-done.md`, `qa/qa-integration-convention.md` (if it gives commands). Do not rewrite historical documents (REQ-013 plan/report, REQ-000 test report, REQ-013 ticket); the grep for `npm`/`npx` must only match the allowed exceptions.
5. **Evidence** (needs real runs): compare the 17 test names under npm (from the last npm run, e.g. `git stash`-free: use `npm`-era evidence from the REQ-013 report or re-run it once from a scratch worktree) and under bun; re-apply the XSS mutations M1, M3, M4 and M2 from [[REQ-013-test-report]] under bun and confirm the same tests fail, then revert; broken-assertion check; `bunx tauri build --debug --no-bundle` plus the WebView2 DOM check (use the method in [[qa/test-strategy]], throwaway profile, kill the app afterwards); `bun install --frozen-lockfile` failing on a deliberate mismatch.
6. **GitHub verification** (needs GitHub, so the main assistant pushes; `dev` does not push): push the feature branch, confirm `ci-check` is green, run the five deliberate-failure probes and the `branch-policy` probes exactly like REQ-013 (throwaway branches and PRs, deleted afterwards), confirm the check names are still `ci-check` and `branch-policy`.

## Risks to watch
Tauri CLI optional platform binary under bun on the runner [U] · bun and `beforeDevCommand` PATH on Windows shells · `bun.lock` CRLF vs LF with `eol=lf` · Vitest/jsdom behaviour differences · the `setup-bun` action's cache input name and default version behaviour [U, read its README/action.yml at the pinned SHA] · job names must not change (they are going to be required checks).

## Tasks (in order, tick them off; report deviations)

- [ ] T1 Record the baseline before changing anything: `bun --version`, `node -v`, `npm -v`, the 17 test names from `npm test --reporter=verbose` (save the list to a scratch file, not the repo), `git status` clean except other agents' files.
- [ ] T2 Migrate lockfile: delete `package-lock.json` and `node_modules`, `bun install`, commit `bun.lock`, add `packageManager`, handle LF if needed, note resolved version differences and blocked scripts. (A)
- [ ] T3 `tauri.conf.json` hooks and `bun test` guard (decision 5). (B)
- [ ] T4 Local checks from a clean state: `bun install --frozen-lockfile`, `bun run build`, `bun run test`; deliberate mismatch check; broken assertion check; no-test-files check; compare test names with T1. (C)
- [ ] T5 Mutation checks M1 to M4 under bun, revert, `git diff` empty afterwards. (C)
- [ ] T6 `bunx tauri build --debug --no-bundle` and the window check with WebView2 remote debugging; kill the app, verify no leftover processes. (B)
- [ ] T7 Node removal experiment (decision 1): result recorded; keep or remove with reason. (A)
- [ ] T8 `ci.yml` with `setup-bun` (SHA verified), caching, order of steps, names unchanged. Validate YAML locally. (D)
- [ ] T9 Docs updates and the grep for `npm`/`npx` in living files; fresh-clone check in a scratch directory (`git clone` the local repo, follow the README only). (E)
- [ ] T10 Commit in small conventional commits ending with the Co-Authored-By line; do not push.
- [ ] T11 (main assistant) push the branch, confirm CI green, run the five probes and the policy probes, delete probe branches/PRs, record in this plan.
- [ ] T12 (`tester`) verify against the ticket and the QA Definition of Done; report in `obsidians/plans/REQ-014-test-report.md`.

## Definition of done
Ticket criteria A to E met or explicitly pending; `tester` verdict pass; [[qa/definition-of-done]] met; CI green with the same job names; no `npm`/`npx` command in living files; no second lockfile; no personal data in any file; PR-only rule for `main` respected (D16).
