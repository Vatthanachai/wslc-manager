---
id: REQ-007
title: "Safe destructive actions"
priority: Must
connector: wslc
status: todo
depends_on:
  - "[[REQ-004-container-lifecycle]]"
tags:
  - requirement
  - must
---

# REQ-007 Safe destructive actions

- Priority: Must (US-7)
- Connector scope: WSLC (Docker deferred)
- Dependencies: REQ-004

## User story
As an operator, I want a confirmation before removing or killing a container so that I do not destroy something by accident.

## Acceptance criteria
- Given I choose Remove or Kill on a container, then a confirmation dialog shows the container name, session, and the exact action.
- Given the dialog, then the default focused button is Cancel, and the destructive button is labelled with the action name (not "OK").
- Given a Remove, then volumes are never deleted unless I tick an explicit "also delete volumes" option, which is off by default. The CLI option for this is not yet verified [U], so if it is not available the option is hidden and volumes are never touched.
- Given I confirm, then the command runs (`container remove` / `container kill` exist in the WSLC CLI [R], and are not yet tested [U]) and the list refreshes from `container list`.
- Given I cancel, then no command is run.
- Given a running container is removed, then the dialog states that it will be stopped first (or that removal is refused, depending on the CLI's behaviour, to be tested [U]).
- Given a prune action (if added later), then it must show reclaimable items first. Prune is out of scope here.

## Fixtures
- None for remove/kill yet (capture before implementing). `obsidians/fixtures/container-list.json` for the list refresh.

## Definition of done
- This ticket is done only when the checklist in [[qa/definition-of-done]] is met.

## Out of scope
- Prune, image/volume removal, undo, bulk delete.
