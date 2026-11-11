---
id: REQ-014
title: "Switch the frontend package manager from npm to bun"
priority: Must
connector: n/a
status: done
depends_on:
  - "[[REQ-013-ci-and-qa-baseline]]"
tags:
  - requirement
  - must
---

# REQ-014 Switch the frontend package manager from npm to bun

- Priority: Must (it changes the toolchain every later ticket uses, so do it before REQ-001)
- Connector scope: not applicable (toolchain, build and CI work)
- Dependencies: REQ-013 (CI and QA baseline). REQ-013 is closed with npm first, and this ticket then migrates it.
- User interface: no new screens and no UI change, so the `ux` agent is not needed. The only UI evidence is that the existing window still renders the fake container row.

## Decisions and facts
- Owner decision (2026-11-11): the frontend package manager changes from npm to bun.
- Facts checked in the repository on 2026-11-11 [V]: `package.json` has scripts `dev` (`vite`), `build` (`tsc --noEmit && vite build`), `tauri` (`tauri`), `test` (`vitest run`); dependency `@tauri-apps/api`; devDependencies `@tauri-apps/cli`, `jsdom`, `typescript`, `vite`, `vitest`; `engines.node` is `>=24 <25`; there is no `packageManager` field. `.nvmrc` (Node 24), `package-lock.json`, `.gitattributes` (`* text=auto eol=lf`), `.github/workflows/ci.yml` and `branch-policy.yml` exist. npm commands appear in `README.md`, `src-tauri/tauri.conf.json`, `.github/workflows/ci.yml`, `obsidians/qa/test-strategy.md` and `obsidians/qa/definition-of-done.md`.
- Facts given by the owner and not re-checked here [U]: bun 1.3.14 is installed locally; `tauri.conf.json` uses `npm run dev` and `npm run build` as `beforeDevCommand` and `beforeBuildCommand`; there are 17 UI tests in `src/main.test.ts` and `src/main.dom-safety.test.ts`; CI uses the npm shipped with Node 24 (npm 11.x) and local is npm 12.x.
- Bun removes the esbuild install-scripts warning left open in REQ-013. Correction found during implementation (2026-11-11): esbuild is on bun's default-trusted list (`bun pm default-trusted`), so its `postinstall` does run; bun does not run lifecycle scripts of other, untrusted dependencies. `trustedDependencies` stays absent. Vitest does not run without Node (verified), so Node 24, `.nvmrc`, `engines.node` and `setup-node` stay.
- Historical documents stay as written: [[REQ-013-plan]], [[REQ-013-test-report]], [[REQ-000-test-report]] and the REQ-013 ticket describe npm because that is what was true when they were written. They are evidence, not living docs, and are NOT updated by this ticket. A short note at the top of each is allowed but not required.
- Living docs that must be updated: `README.md` ("Local checks"), [[qa/test-strategy]], [[qa/definition-of-done]], and [[qa/qa-integration-convention]] if it gives commands.
- Result record (2026-11-11): the job names `ci-check` (workflow `ci`) and `branch-policy` did NOT change. The new or changed pinned actions are `oven-sh/setup-bun` v2.2.0 and `actions/cache` v6.1.0, both pinned by full commit SHA with the version in a comment and checked against their tags. There is no `trustedDependencies` in `package.json`. Bun version 1.3.14 is written in `package.json` (`packageManager`) and in `ci.yml`.
- Where the evidence is: the criteria below say "recorded in the PR", but no pull request is opened for a feature branch (pull requests are only opened from `develop` to `main`, D16). The items are therefore recorded in [[REQ-014-plan]], section "T11 results" (CI run, five probes, policy probes, deletion of probe branches and PRs), and in [[REQ-014-test-report]] (bun version, Node decision, grep result, mismatch and broken-assertion checks, mutation checks, window check).
- Facts marked [U] are unverified.

## User story
As the maintainer, I want the whole project (local, `tauri.conf.json` hooks, CI, docs) to use bun with one pinned version and one lockfile, so that every later ticket builds and tests with the same toolchain, installs are fast and deterministic, and dependency install scripts do not run unless explicitly trusted.

## Acceptance criteria

A. Lockfile and version pinning
- Given the repository after this ticket, then `bun.lock` exists and is committed, `package-lock.json` is deleted, and no other lockfile exists (`package-lock.json`, `yarn.lock`, `pnpm-lock.yaml`, `bun.lockb`).
- Given `package.json`, then it has `"packageManager": "bun@1.3.14"` (or the version current when the work is done; the version is recorded in the PR), and CI installs exactly that version. If a `.bun-version` file is also used, it holds the same version (the choice is an open question; one source of truth is preferred).
- Given the Node pin, then the decision is recorded in the PR: if Vite, Vitest or the Tauri CLI still run on Node, `.nvmrc` and `engines.node` stay, with a stated reason; if everything runs on bun, they are removed with a stated reason. A pin is not removed without a reason.
- Given `trustedDependencies` in `package.json`, then it is absent or empty unless a dependency demonstrably fails without its install script. If an entry is added, the PR names the package and the failure it prevents. esbuild does not need one: it is on bun's default-trusted list [V, `bun pm default-trusted`; `bun pm untrusted` reports 0]. Result: no `trustedDependencies` exists.

B. Scripts and Tauri hooks
- Given `package.json`, then the script names `dev`, `build`, `tauri` and `test` are unchanged and `test` is still `vitest run` (Vitest, not bun's own runner).
- Given `src-tauri/tauri.conf.json`, then `beforeDevCommand` is `bun run dev` and `beforeBuildCommand` is `bun run build`, and no `npm` or `npx` string remains in it.
- Given a contributor types `bun test` by mistake, then it does not silently run a different, possibly passing, test suite. One guard is chosen and documented in the PR: for example a `bunfig.toml` `[test]` setting that makes `bun test` fail or point to `bun run test`, or a documented rule in README "Local checks" if no technical guard works [U, what bunfig can do is to be confirmed]. The README states `bun run test` is the only test command.
- Given `bunx tauri build --debug --no-bundle` (or `bun run tauri build --debug --no-bundle`) runs on a clean checkout with Rust and bun installed, then it produces the debug exe without calling npm.
- Given the debug exe from that build starts, then the window renders the fake container row. Evidence is taken with the WebView2 remote-debugging method from [[qa/test-strategy]] and recorded in the test report.

C. Local checks
- Given a clean checkout (no `node_modules`, no `target`), when `bun install --frozen-lockfile`, `bun run build` and `bun run test` run in this order, then all three exit with code 0 and `bun install --frozen-lockfile` changes neither `bun.lock` nor `package.json`.
- Given `bun install --frozen-lockfile` runs when `package.json` and `bun.lock` disagree, then it exits non-zero (checked once by hand and noted in the PR).
- Given the 17 UI tests, then under bun the same 17 test names pass as under npm. The list of test names from the last npm run (REQ-013 evidence) and from the bun run are saved and compared, and any difference (missing, extra, renamed, skipped) is explained or fixed.
- Given the mutation checks of the XSS tests documented in [[REQ-013-test-report]], when each mutation is applied again under bun, then the same tests fail as documented there, and pass again when the mutation is reverted.
- Given a deliberately broken assertion in one UI test, when `bun run test` runs, then it exits non-zero (checked once by hand).
- Given no test files are found, then `bun run test` fails, not passes (the REQ-013 rule still holds; `passWithNoTests` stays off).

D. GitHub Actions CI
- Given `.github/workflows/ci.yml`, then the `actions/setup-node` step and `npm ci` are replaced by `oven-sh/setup-bun` pinned to a full commit SHA with the version in a comment, using the same bun version as `packageManager`, and each SHA is checked to be the commit of its tag (annotated tags dereferenced, looked up with `gh api` in the plan stage). The same pinning rules as REQ-013 apply.
- Given the check job, then it runs in order: `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `bun install --frozen-lockfile`, `bun run build`, `bun run test`. Only the three npm steps change. If Node is kept (see A), the workflow still sets it up from `.nvmrc` and the reason is in the PR.
- Given bun caching is enabled, then the cache is bun's global install cache and its key includes the hash of `bun.lock` (and the OS and bun version), a cache miss still gives a green build, and no cache is used for release builds (REQ-012).
- Given the workflows, then the job names `ci-check` (workflow `ci`) and `branch-policy` (workflow `branch-policy`) are unchanged, no matrix or rename is introduced, `permissions: contents: read` stays at workflow level, no job widens it, no secret is used, and the `pull_request` trigger is kept. `branch-policy.yml` is not changed unless it uses npm (it is not expected to).
- Given a clean checkout of the branch, when the workflow runs, then every step passes on the GitHub-hosted Windows runner.
- Given the five deliberate-failure probes from REQ-013 (a failing Rust test, a clippy warning, a formatting difference, a failing Vitest test, a TypeScript build error), when each runs on a throwaway branch, then the job fails at the matching step (the last two now at `bun run test` and `bun run build`). Each probe branch and PR is deleted afterwards and the deletion is recorded.
- Given a throwaway pull request into `main` from `feature/x` and one from `develop`, then `branch-policy` fails and passes respectively, as in REQ-013 (it is expected to be unaffected). Throwaway branches and PRs are deleted.

E. Documentation and cleanliness
- Given `README.md`, then "Local checks" gives the bun commands, and a prerequisites list names: bun (the pinned version), Rust installed through rustup (the toolchain comes from `rust-toolchain.toml`), and Node only if the decision in A keeps it.
- Given a fresh clone on a Windows 11 machine that has only bun and Rust (and Node only if required), then following the README alone, the build and the tests pass. This is checked once on a clean clone, ideally by a second environment or a clean user profile [U].
- Given the living files (`README.md`, `package.json`, `src-tauri/tauri.conf.json`, `.github/`, `obsidians/qa/test-strategy.md`, `obsidians/qa/definition-of-done.md`, `obsidians/qa/qa-integration-convention.md`), when searched for `npm` and `npx` (case-insensitive), then no command remains. Allowed exceptions: this ticket, historical documents (listed in Decisions and facts), and a deliberate "was npm" explanation. The grep result is recorded in the PR.
- Given `obsidians/qa/definition-of-done.md`, then its checklist line reads `bun run build` and `bun run test` pass.

## Fixtures
- None. The UI tests use the same fake container as before. The saved list of the 17 test names (npm and bun runs) is test evidence and is stored with the test report, not as a fixture.

## Edge cases
- Windows shell and path differences: Tauri runs `beforeDevCommand` and `beforeBuildCommand` through a shell, and `bun` must be on `PATH` for that shell (PowerShell, cmd, Git Bash, CI). A `bun` installed per user may not be on `PATH` for a service or IDE launching `tauri`. Check on a fresh terminal.
- `bun install` and lifecycle scripts: a dependency whose `postinstall` is needed (the Tauri CLI downloads or selects a platform binary through an optional dependency package) must still work when scripts are not run. Verify `bunx tauri --version` and the debug build on a clean `node_modules`.
- Tauri CLI binary: `@tauri-apps/cli` ships platform binaries as optional dependencies; bun must pick the Windows x64 one, also on CI. A frozen lockfile created on one OS must include it for the runner [U].
- Vitest and jsdom under bun: `vitest run` started by `bun run` still executes on whichever runtime the `vitest` bin resolves to (Node by default through its shebang, unless `bun --bun` is used) [U]. Behaviour of jsdom and `textContent` assertions can differ between Node and bun, so the XSS tests are re-verified. Decide and document whether `bun run` or `bun --bun run` is used, and keep local and CI the same.
- `bun test` versus `bun run test`: `bun test` runs bun's own runner, ignores `vitest.config.ts` and the jsdom environment, and may fail or find nothing, or worse, pass without the intended checks. See the guard in B.
- `bun.lock` format: bun 1.2 and later writes a text `bun.lock` (older versions wrote binary `bun.lockb`) [U, confirm for 1.3.14]. Only the text file is committed. A different bun version may rewrite it, which is a reason to pin the version.
- CRLF versus LF: `.gitattributes` has `* text=auto eol=lf`, so `bun.lock` should be LF in the repository and on checkout. If bun on Windows writes CRLF, `--frozen-lockfile` may report a change or git may show a diff. Check after a clean clone and add an explicit `bun.lock text eol=lf` line if needed.
- `engines` field: bun may warn about or ignore `engines.node` [U]. Do not rely on it as a guard.
- `.nvmrc` and `actions/setup-node`: if Node is removed from CI, any step that needs the `node` binary (a Vite plugin, a script with a node shebang) fails at run time only. The clean-runner run catches this.
- Existing `node_modules` from npm on a developer machine: delete it before the first `bun install` (documented in the PR, not an acceptance criterion).
- The setup-bun action may resolve `latest` if no version is given. The version must always be explicit.

## Risks
- Bun version drift between local, CI and contributors. Mitigation: `packageManager` (and/or `.bun-version`) plus the same version in the setup action, and a single place to bump it.
- Supply chain of `oven-sh/setup-bun` and of the bun binary it downloads. Mitigation: full commit SHA pin with the version comment (REQ-013 rules), `contents: read`, no secrets, and a tag-to-SHA check.
- The evidence from REQ-013 (test report, mutation checks, five probes, cache behaviour) is npm-based and must be redone under bun. Treat this ticket as not done until it is.
- Bun is younger than npm, and some behaviour (jsdom, native binaries, Windows paths) may differ from Node. Mitigation: the comparison of test names, the mutation checks and the debug build with the window check.
- Contributors may need a second runtime. If Node stays for Vite or the Tauri CLI, a contributor needs bun, Node and Rust. Mitigation: state prerequisites in the README, and prefer running everything on one runtime if it works.
- Making the CI jobs required on `main` happens after REQ-013 (owner step). A renamed job here would break that, so the names are frozen.
- REQ-012 (release and signing pipeline, not built yet) must use bun as well, and must not restore any cache into a signed build. Record this in REQ-012 when it is planned, not by editing it now.
- Documents diverging: if a living doc keeps an npm command, a contributor follows it and creates `package-lock.json` as a second lockfile. Mitigation: the grep criterion and the no-second-lockfile criterion (a CI check for stray lockfiles is a could-have, not required).

### Accepted findings of the test report (Low, none blocks)
- BUG-003: `bun test` is refused by the `bunfig.toml` preload `scripts/refuse-bun-test.ts`, but bun prints it as "Unhandled error between tests" with "2 fail" (the two test files) and then exits 1. The refusal text is clear and the exit code is non-zero, so it is accepted as is.
- The bun version is written by hand in two places, `package.json` (`packageManager`) and `ci.yml` (`bun-version`), and nothing checks that they match (a comment in `ci.yml` says so; `setup-bun` is not configured to read `packageManager`). A CI check that compares them is a could-have, not part of this ticket.
- `bun install --frozen-lockfile` fails for a new dependency but does NOT fail for a range edit that the old lock still satisfies (for example `^5` to `^5.8` with 5.9.3 locked). The criterion ("exits non-zero when `package.json` and `bun.lock` disagree") was checked with a real mismatch only.
- Definition-of-done wording versus the real commands: `obsidians/qa/definition-of-done.md` line 16 asks for `cargo test --workspace --all-targets` and `cargo test --doc`, while the README and `ci.yml` run `cargo test --workspace`. This does differ (the same gap existed in REQ-013). The tester ran both forms and both pass. This ticket does not edit the DoD; the documentor will align the DoD wording and the README/CI.
- BUG-002 (a merge conflict in `obsidians/decision-log.md` between this branch and `develop`) is a merge-time item for the owner and is not a text defect of this ticket.

## Rollback
- Revert the migration commit (or merge) and restore `package-lock.json`, `.nvmrc`/`engines` if they were removed, the npm hooks in `tauri.conf.json` and the npm steps in `ci.yml`. Because the scripts keep their names and no dependency is added or changed, nothing else needs to change. Re-run `npm ci`, `npm run build` and `npm test` to confirm.

## Definition of done
- This ticket is done only when the checklist in [[qa/definition-of-done]] is met.

## Out of scope
- Changing frameworks or bundlers (Vite, Vitest, TypeScript stay), adding or upgrading dependencies (the lockfile is generated from the current ranges; if resolution picks newer versions, record them), using bun's own test runner, bun as the runtime for the app itself.
- The release and signing pipeline (REQ-012; its workflow must also use bun, without caches).
- Dependabot (a separate, later ticket).
- Making the CI jobs required status checks on `main` (the owner step from REQ-013).

## Open questions
All answered (decisions in [[REQ-014-plan]], verified by experiment and in [[REQ-014-test-report]]):
1. Node pin: KEPT. `.nvmrc` (24), `engines.node` (`>=24 <25`) and `setup-node` in CI stay, because Vitest does not run without Node (with no `node` on `PATH`, `bun run test` fails with an EventTarget error in the Vitest worker; `bun --bun` also fails). Bun still does the install, build and Tauri CLI work.
2. Version pin source: `packageManager` in `package.json` only (`bun@1.3.14`), plus the same value in `ci.yml` with a comment pointing to it. No `.bun-version` file.
3. `trustedDependencies`: stays absent (empty). esbuild is on bun's default-trusted list, so it needs no entry.
4. Runtime for Vitest: Node, through `bun run test` (the `vitest` bin's own shebang). This keeps the REQ-013 evidence valid. `bun --bun run test` was tried for information and also fails, and it is not the CI path.
5. `bun test` guard: technical guard. `bunfig.toml` has a `[test]` `preload` of `scripts/refuse-bun-test.ts`, which throws a message telling the user to run `bun run test`, so `bun test` exits 1. The README also states that `bun run test` is the only test command. Presentation is noisy (see accepted findings).
6. Stray second lockfile CI check: not in this ticket (could-have).
