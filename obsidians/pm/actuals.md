---
status: active
updated: 2026-11-11
tags: [pm]
---

# Actuals vs estimates

Plan and ranges: [[roadmap]]. Labels: **measured** (agent run clock, git, test report), **assumed**, **not recorded**.

## Data quality notes
- Git timestamps are commit times, not run times. Several commits share one second (14:37:39 for the two feature commits, 14:41:07 for the tests and the test report), so they were committed together after the work. Use them only for the calendar span.
- Only REQ-000 has data. One data point is not a trend, do not extrapolate from it.
- Human review and decision time has not been measured for any stage.

## REQ-000 walking skeleton

Estimate: none was made before the work (this log starts after the fact). The ranges in [[roadmap]] are therefore not validated against anything yet.

Git span (measured, +0700, 2026-11-11): initial commit `6e92a54` 14:25:09, vault docs `d2b88d6` 14:29:27, gitattributes `9e31d3c` 14:30:32, core `18aaa4a` and app `7f4ffa7` 14:37:39, QA tests `7bec032` and `e3d6128` and test report `5b333a6` 14:41:07. Calendar span first commit to test report: about 16 min. The BA analysis and ticket writing precede the first docs commit and are not separable from git.

| Ticket | Stage | Estimated range | Actual | Variance | Lesson |
|---|---|---|---|---|---|
| [[REQ-000-walking-skeleton]] | BA ticket / analysis | none made | initial analysis about 2 min, revisions and ticket batches 2 to 3.5 min each, about 4 min for 12 tickets (measured, agent clock) | n/a | BA batches are cheap, the review of them was done by the owner inside the same session (time not recorded) |
| [[REQ-000-walking-skeleton]] | PM estimate | none made | this note set, created 2026-11-11 after the feature was built | n/a | run PM before dev next time, so variance can be computed |
| [[REQ-000-walking-skeleton]] | UX design | none made | not run (skeleton table only) | n/a | |
| [[REQ-000-walking-skeleton]] | Plan and tasks | none made | not recorded | n/a | record plan time from REQ-001 on |
| [[REQ-000-walking-skeleton]] | Dev | none made | about 6.5 min (390 s), ~68k tokens, 45 tool calls, first Tauri/Cargo build of 226 crates included (measured) | n/a | first build is a one-off cost, later tickets should not repeat it unless a fresh worktree is used |
| [[REQ-000-walking-skeleton]] | Tester | none made | about 3.1 min (188 s), ~61k tokens. 46 tests passed, 2 ignored. 1 defect BUG-001 (low, BOM) (measured, [[REQ-000-test-report]]) | n/a | tester produced 355 plus 168 lines of extra tests and found an edge case the dev claimed to handle: a tester pass is worth keeping |
| [[REQ-000-walking-skeleton]] | Dev fix BUG-001 | none made | 0.5 min (28 s) (measured) | n/a | a one-line fix costs seconds of agent time; the cost is in the loop, not the fix |
| [[REQ-000-walking-skeleton]] | Tester re-verification | none made | about 0.9 min (55 s) (measured), verdict pass | n/a | retest is cheap |
| [[REQ-000-walking-skeleton]] | Owner review and `git flow feature finish` | none made | done, merged into `develop` (stated by owner, 2026-11-11); human time not recorded | n/a | start timing owner review in minutes per stage |

REQ-000 total agent time (measured): 6.5 + 3.1 + 0.5 + 0.9 = about 11 min across dev, test, fix and retest, status DONE. Owner capacity is 10 h/week (stated by owner, 2026-11-11).

Observations that affect later estimates (measured or from the report):
- Test and dev were separate runs of similar length, so the tester stage is about half of dev time (3.1 vs 6.5 min), small sample.
- A tester-found defect created a fix loop at the first feature: keep one fix loop in every estimate.
- The real process runner and real `wslc` interaction do not exist yet (fake runner only), so the next tickets may take longer per task than REQ-000.

## Other tickets (rows reserved)

Fill the estimate columns from [[roadmap]] before the ticket starts, and the actual columns when it finishes. Use stage rows: BA, PM, UX, Plan, Dev, Tester, Fix loops, Owner review (hours), Finish.

| Ticket | Stage | Estimated range (agent min / owner h) | Actual | Variance | Lesson |
|---|---|---|---|---|---|
| [[REQ-001-discover-runtimes]] | all | 25 to 70 min / 1.0 to 2.0 h | | | |
| [[REQ-002-detect-engine]] | all | 15 to 45 min / 0.5 to 1.0 h | | | |
| [[REQ-008-error-handling]] | all | 25 to 80 min / 1.0 to 2.0 h | | | |
| [[REQ-003-list-containers]] | all | 35 to 110 min / 1.5 to 3.0 h | | | |
| [[REQ-004-container-lifecycle]] | all | 25 to 70 min / 1.0 to 2.0 h | | | |
| [[REQ-007-safe-destructive-actions]] | all | 20 to 70 min / 1.0 to 2.0 h | | | |
| [[REQ-006-live-updates]] | all | 30 to 100 min / 0.75 to 1.5 h | | | |
| [[REQ-005-container-logs]] | all | 25 to 85 min / 1.0 to 2.0 h | | | |
| [[REQ-009-wsl-update-check]] | all | 35 to 110 min / 1.5 to 3.0 h | | | |
| [[REQ-010-sessions-panel]] | all | 25 to 80 min / 1.0 to 2.5 h | | | |
| [[REQ-011-container-stats]] | all | 20 to 60 min / 0.5 to 1.0 h | | | |
| [[REQ-012-release-and-signing-pipeline]] | all | 40 to 150 min / 2.0 to 5.0 h, plus SignPath wait (unknown) | | | |

## Re-estimation rule
After each finished ticket, compare agent minutes and owner hours with the range. If two consecutive tickets land outside the range in the same direction, rescale the remaining ranges in [[roadmap]] and note the date here.
