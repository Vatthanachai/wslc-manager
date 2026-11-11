---
tags:
  - index
---

# Tickets by status

Needs the Dataview community plugin (install it in Obsidian, the vault does not enable it by default). Each ticket's `status` is `todo`, `doing` or `done`.

```dataview
TABLE priority, connector, status, depends_on AS "depends on"
FROM "requirements"
WHERE id
SORT status ASC, id ASC
```

Without Dataview, see the table in [[Home]] or the [[requirements/README|requirements index]].

Back to [[Home]].
