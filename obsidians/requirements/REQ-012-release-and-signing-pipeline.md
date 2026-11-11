---
id: REQ-012
title: "Release and signing pipeline"
priority: Should
connector: n/a
status: todo
depends_on:
  - "[[REQ-000-walking-skeleton]]"
  - "[[REQ-009-wsl-update-check]]"
tags:
  - requirement
  - should
---

# REQ-012 Release and signing pipeline

- Priority: Should (post-MVP, but required before the first public distribution)
- Connector scope: not applicable (build, release and repository work)
- Dependencies: REQ-000 (a buildable app), REQ-009 (its privacy wording), MVP features complete enough to release

## Decisions (D14, D15)
- Windows 11 only (D14).
- Code signing through SignPath Foundation, the free OSS programme (D15). Licence MIT. Public GitHub repository.
- Facts about SignPath below are [R] (read from signpath.org/terms and docs.signpath.io by a helper, not independently verified). Items marked [U] are unverified.

## User story
As the maintainer, I want a reproducible, signed release pipeline on GitHub so that users can install a trusted build, and SignPath Foundation's conditions are met.

## SignPath constraints that drive this ticket [R]
- Eligibility: an OSI-approved licence for all components, no commercial dual licensing, no proprietary components (system libraries are fine), no malware or PUA, actively maintained, already released in the form to be signed, functionality described on the download page. SignPath performs an initial reputation/control check at its discretion (no numeric threshold), so a brand-new repository with no history may be rejected.
- Public text: a "Code signing policy" section or link on the home page and on the download/release pages, the line "Free code signing provided by SignPath.io, certificate by SignPath Foundation" with links, team roles (committers/reviewers and approvers), and a privacy policy or the standard statement "This program will not transfer any information to other networked systems unless specifically requested by the user or the person installing or operating it".
- Build: built verifiably from source, all workflow jobs on GitHub-hosted runners, `actions/upload-artifact` v4+ followed by `signpath/github-action-submit-signing-request@v3`, build settings only from source-controlled configuration, no manual overrides, no caching of earlier builds, enforced metadata on signed files (product name equals the project name, constant product version per build), manual approval for every release. The source review policy must cover CI configuration and build scripts.
- Team: MFA for SignPath and GitHub for all members, roles author, reviewer, approver.
- The publisher shown is SignPath Foundation, not the developer. SignPath may pause or revoke certificates, even retroactively.
- [U]: application steps and turnaround time, support for Tauri/NSIS/MSI and Rust binaries in the artifact configuration, how the publisher name appears in Windows prompts, acceptance of non-GitHub CI, the reputation threshold. SmartScreen reputation still builds over time even with a signed binary.

## Acceptance criteria

Repository hygiene
- Given the repository, then it contains an MIT `LICENSE` file, all dependencies are checked to be compatible with MIT and with the SignPath requirement of OSI-approved components (check done in CI or documented), and the repository is public on GitHub.
- Given the README, then it has a "Code signing policy" section (also linked from the release page text), the exact line "Free code signing provided by SignPath.io, certificate by SignPath Foundation" with links, the list of team members and their roles (author, reviewer, approver), and a note that MFA is required for all members.
- Given the README, then it has a privacy policy section stating that the program does not transfer information to other networked systems unless requested by the user or the person operating it, and discloses the one exception: if the daily WSL update check is enabled (the default, REQ-009), the app contacts the GitHub releases API of microsoft/WSL once a day without sending user or container data, and the user can turn it off in Settings. The wording must match REQ-009.
- Given the README and the release page, then the functionality is described (what the app does) so that the download page satisfies the eligibility rule.
- Given every maintainer account on GitHub and SignPath, then MFA is enabled.

Build pipeline
- Given a push of a release tag, when the GitHub Actions workflow runs, then every job runs on a GitHub-hosted runner (no self-hosted runners), the Tauri installer is built from source-controlled configuration only (lockfiles committed, pinned toolchain versions, no manual inputs that change the build), and no cache from an earlier build is restored into the release build.
- Given two builds of the same tag, then they produce installers with the same product name and the same product version, and the version comes from one source-controlled place. Bit-for-bit reproducibility is not claimed [U]. The requirement is verifiable build from source.
- Given the workflow produces the installer, then it uploads it with `actions/upload-artifact` v4 or newer, and the next step submits it with `signpath/github-action-submit-signing-request@v3` [R].
- Given the installer, then it provides uninstall (a Windows uninstaller entry), and the product name metadata equals the project name [R].
- Given the Tauri bundle formats (NSIS and/or MSI) and the Rust binary inside, then a test signing request proves that SignPath's artifact configuration can sign them [U until tested]. The signing configuration file lives in the repository.

Signing and approval
- Given a signing request is submitted, then it waits for manual approval by an approver before the signed artifact is released [R]. Nothing is published automatically after a build.
- Given the signed artifact is returned, then the workflow attaches the signed installer to the GitHub release and the release notes contain the Code signing policy link.
- Given the source review policy, then CI configuration and build scripts are included in the review scope and changes to them need a reviewer [R].

Release sequence
- Given no SignPath approval yet, then the first public release is an UNSIGNED installer, published with a README note that Windows SmartScreen will warn about it, to start building reputation and to meet SignPath's rule that the project must already be released in the form to be signed [R].
- Given at least one release exists and the project has some history, then the maintainer applies to SignPath Foundation. Application steps and turnaround are [U], and a rejection is possible [R], in which case unsigned releases continue and the application can be repeated later.
- Given SignPath approves the project, then the next release is signed through the pipeline, and the README keeps the Code signing policy section.
- Given SignPath pauses or revokes the certificate [R], then the pipeline can still publish an unsigned release and the README states the current signing status.

## Fixtures
- None. A sample SignPath artifact configuration and a successful test signing request log are to be added as evidence once available.

## Out of scope
- Paid code signing certificates, EV certificates, Microsoft Store distribution, winget or other package manager submission, auto-update of the app itself, non-GitHub CI.
