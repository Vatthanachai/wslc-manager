---
status: draft
tags: [design]
created: 2026-11-11
---

# Design system (wslc-manager)

Created with [[REQ-001-002-008-ux]] (tickets [[REQ-001-discover-runtimes]], [[REQ-002-detect-engine]], [[REQ-008-error-handling]]). Extend this file when a later ticket needs a new token or pattern, and do not fork it per ticket. Context: [[ba-analysis]], [[decision-log]] (D13 English only, D14 Windows 11 only).

Principles: dense, calm, Windows 11 (Fluent) look, follows the OS light/dark theme, keyboard first, no decoration, no remote assets, no inline scripts. State is never shown by colour alone (always icon shape plus text).

Evidence note: all contrast ratios below were computed by hand from the hex values (WCAG 2.x relative luminance), not with a tool. The tester should re-check them with a contrast checker in both themes. Fluent-like colours are chosen to be close to Windows 11 values, not copied from an official token list [U].

## 1. Colour tokens

Defined as CSS custom properties on `:root`, overridden under `@media (prefers-color-scheme: dark)`. No theme toggle in the app (follows the OS). Also add a `@media (forced-colors: active)` block that maps borders to `CanvasText` and keeps icons visible (see section 9).

```css
:root {
  color-scheme: light dark;

  /* surfaces */
  --color-bg:            #f3f3f3;  /* window background (behind panels) */
  --color-surface:       #ffffff;  /* tables, panels, popovers */
  --color-surface-alt:   #f9f9f9;  /* table header, status bar */
  --color-border:        #d1d1d1;  /* 1px lines; 3:1 on surface is not required for decorative lines */
  --color-border-strong: #8a8a8a;  /* control outlines (3.4:1 on white) */
  --color-hover:         #ececec;
  --color-selected:      #e5f1fb;

  /* text */
  --color-text:          #1b1b1b;  /* 16.6:1 on white, 15.0:1 on bg */
  --color-text-muted:    #5d5d5d;  /* 6.6:1 on white, 5.9:1 on bg */
  --color-text-on-accent:#ffffff;

  /* accent and focus */
  --color-accent:        #0067c0;  /* 5.7:1 on white */
  --color-accent-hover:  #005aa9;
  --color-focus:         #0067c0;  /* 2px ring, 1px gap in --color-surface */

  /* state: foreground (icon, 3px left border, text on tint) and tint (banner background) */
  --state-success-fg:    #0f7b0f;  /* 5.5:1 on white */
  --state-success-bg:    #dff6dd;
  --state-warning-fg:    #9d5d00;  /* 5.3:1 on white, 4.9:1 on its tint */
  --state-warning-bg:    #fff4ce;
  --state-error-fg:      #c42b1c;  /* 5.7:1 on white, 4.7:1 on its tint */
  --state-error-bg:      #fde7e9;
  --state-info-fg:       #0067c0;  /* 5.7:1 on white */
  --state-info-bg:       #e5f1fb;
  --state-neutral-fg:    #5d5d5d;
  --state-neutral-bg:    #f3f3f3;
}

@media (prefers-color-scheme: dark) {
  :root {
    --color-bg:            #202020;
    --color-surface:       #2b2b2b;
    --color-surface-alt:   #262626;
    --color-border:        #3d3d3d;
    --color-border-strong: #9a9a9a;  /* 4.5:1 on surface */
    --color-hover:         #353535;
    --color-selected:      #25384a;

    --color-text:          #ffffff;  /* 14.1:1 on surface */
    --color-text-muted:    #c5c5c5;  /* 8.2:1 on surface */
    --color-text-on-accent:#000000;

    --color-accent:        #60cdff;  /* 9.4:1 on surface */
    --color-accent-hover:  #7dd6ff;
    --color-focus:         #60cdff;

    --state-success-fg:    #6ccb5f;
    --state-success-bg:    #393d1b;
    --state-warning-fg:    #fce100;
    --state-warning-bg:    #433519;
    --state-error-fg:      #ff99a4;
    --state-error-bg:      #442726;
    --state-info-fg:       #60cdff;
    --state-info-bg:       #25384a;
    --state-neutral-fg:    #c5c5c5;
    --state-neutral-bg:    #2b2b2b;
  }
}
```

Rules
- Body text and banner text always use `--color-text` (or muted for secondary lines). The state colour is used for the icon, the 3px left border and (optionally) the title, never as the only carrier of meaning.
- Primary button: `--color-accent` background with `--color-text-on-accent` text (white on #0067c0 is 5.7:1, black on #60cdff is 12:1+).
- Text on state tints (title in state colour) stays at or above 4.5:1 in the values above. Do not put muted text on a tint.

## 2. Spacing, size and shape

4 px base. Tokens: `--space-1: 2px`, `--space-2: 4px`, `--space-3: 8px`, `--space-4: 12px`, `--space-5: 16px`, `--space-6: 24px`, `--space-7: 32px`.

| Item | Value |
|---|---|
| Header height | 40 px |
| Status bar height | 24 px (min 28 px if text zoom is above 100%, use `min-height`) |
| Table row height (REQ-003 reuse) | 32 px (dense), min click target 24 x 24 px |
| Banner min height | 40 px single line, grows with wrapped text |
| Buttons | height 28 px, horizontal padding 12 px, radius 4 px |
| Panel/popover radius | 8 px, border 1 px `--color-border`, shadow `0 8px 24px rgb(0 0 0 / 0.18)` |
| Control radius | 4 px |
| Page padding | 16 px (12 px under 900 px width) |
| Min window | 800 x 600 |

## 3. Typography

- Font stack: `"Segoe UI Variable", "Segoe UI", system-ui, sans-serif`. Mono (details, versions, commands): `"Cascadia Mono", Consolas, ui-monospace, monospace`. No web fonts.
- Sizes: caption 12 px, body 13 px (dense app default), subtitle 14 px semibold, panel title 16 px semibold, gate title 20 px semibold. Line height 1.4. Weight 400 body, 600 for titles and table headers.
- `font-variant-numeric: tabular-nums` for versions, counts, PIDs.
- Do not set `user-select: none` on error text and details: users copy them.

## 4. Icons

Single icon source: the Windows 11 system font **Segoe Fluent Icons** (installed on every Windows 11, D14, so no remote asset and no CSP issue). Render as `<span class="icon" aria-hidden="true">` with a character, always next to a text label or inside a button that has an `aria-label`. Codepoints are from memory and must be verified on the target machine [U]. If a glyph is missing, the paired text still carries the meaning.

| Purpose | Glyph name | Codepoint [U verify] | Shape cue (so it is not colour only) |
|---|---|---|---|
| Info / untested notice | Info | E946 | circle with i |
| Warning | Warning | E7BA | triangle |
| Error | ErrorBadge | EA39 | filled circle with x |
| Success / available | Completed | E930 | circle with check |
| Busy | Sync | E895 | circular arrows (rotates, static under reduced motion) |
| Retry / refresh | Refresh | E72C | arrow in circle |
| Copy | Copy | E8C8 | two sheets |
| Close / dismiss | Cancel | E711 | x |
| Expand | ChevronRight / ChevronDown | E76C / E70D | chevron |
| Runtime / settings (reserved) | Settings | E713 | gear |

Container state glyphs (defined now so REQ-003 stays consistent; REQ-003 owns the table):

| State | Glyph | Colour token | Label (strings) |
|---|---|---|---|
| running | Play (E768) | success | Running |
| exited | Stop (E71A) | neutral | Exited |
| unknown raw value | Help (E897) | warning | Unknown (raw) |
| transitioning (start/stop/restart in progress) | Sync (E895) | info | Starting, Stopping, Restarting (REQ-004 owns the copy) |

Paused and other states are [U] (only `running` and `exited` were observed [V]), so they fall into "unknown" with the raw value shown.

## 5. Components

### 5.1 Status chip (header, runtime state)
Icon + short word, no background fill except a 1 px border in the state colour. Example: `[check] Available`, `[x] Not available`, `[triangle] Untested version`. It is a button that opens the runtime panel (5.8).

### 5.2 Status bar
Passive, one line, `--color-surface-alt`, top border. Items separated by a 1 px divider and 12 px padding. Left cluster: availability icon + text, wslc version, WSL version, sessions summary. Right cluster: reserved slot for the live-update indicator (REQ-006). Container is `role="status"` `aria-live="polite"` and only changes text when the state changes (do not re-render each second). Items drop out by priority below 900 px (see wireframes): first the WSL version label shortens to the number, then the session names collapse to the count.

### 5.3 Banner (inline, persistent, non-modal)
Anatomy: 3 px left border in the state colour, tint background, icon, text block (bold title, one or two lines of message), then actions right-aligned: primary action (Retry), optional secondary (for example Copy command), Details toggle, Dismiss (only when dismissible). Details open inline below the message (native `<details>`).

| Severity | Icon | Role | Dismissible | Auto-clears |
|---|---|---|---|---|
| error | ErrorBadge | `role="alert"` | no (clears when the condition is resolved) | yes, on a successful retry |
| warning | Warning | `role="status"` | no while the condition holds | yes |
| info | Info | `role="status"` | yes (for the session, not remembered) | no |

Stack order in the notice region (top to bottom): update banner slot (REQ-009), errors, warnings, info. Maximum 3 visible, the rest collapse into one row "N more notices" that expands. Banners push content down with a 150 ms height transition (none under reduced motion).

### 5.4 Session strip (inline per-session error, list scope)
A banner variant (error or warning severity) that sits directly above the container table, one per failing session, naming the session in the title. Used so that one failing session never hides the others (REQ-008). Same anatomy as a banner, plus a leading session-name badge (monospace, truncated with a tooltip).

### 5.5 Gate panel (blocking, fills the main region)
Centered block (max width 560 px, left-aligned text), used when the app cannot work at all. Anatomy: 32 px icon, 20 px title, one-paragraph cause (max two sentences), "What you can do" list or a single primary action, secondary actions, Details expander. No modal and no overlay: the header and status bar stay usable. Keyboard focus moves to the primary action on first render of a gate state.

### 5.6 Toast (transient or persistent, for results of one user action)
Bottom-right, above the status bar, 360 px wide, max 3 stacked (older ones collapse). Success: auto-dismiss after 5 s, paused while hovered or focused, `role="status"`. Error and warning: no auto-dismiss, `role="alert"` for error, always have a Dismiss button, optional Details. Esc dismisses the focused toast. Toasts are never the only place an error is recorded when the user cannot retry from elsewhere (see error matrix in [[REQ-001-002-008-ux]]).

### 5.7 Empty state
Centered in the region, 32 px neutral icon, 14 px semibold title, one sentence of explanation, optional single action. No illustration. Distinct from error: neutral colour, no alert role.

### 5.8 Runtime panel (non-modal popover)
360 px wide, anchored under the header runtime chip, `role="dialog"` with `aria-label`, not modal (no focus trap beyond returning focus on Esc). Closes on Esc or outside click, focus returns to the chip. Sections with 12 px titles: Status, Versions, Sessions, Capabilities, then a footer button.

### 5.9 Details disclosure
Native `<details><summary>Details</summary>...</details>`. Content is a `<dl>` of key/value rows in mono 12 px plus a bordered `<pre>` for raw output (max height 160 px, scrolls, wraps), then a "Copy details" button. Closed by default. Raw text is inserted with `textContent` only (never `innerHTML`).

### 5.10 Disabled control with reason
Use `aria-disabled="true"` (not the `disabled` attribute) so the control stays focusable and its reason is reachable by keyboard. Dim to 45% opacity plus a diagonal-free, plain style (no strikethrough). Click and Enter do nothing. A custom tooltip shows the reason on hover (after 400 ms) and immediately on keyboard focus, Esc closes it, and the same text is wired with `aria-describedby`. The native `title` attribute is not enough (not keyboard reachable). Reason text patterns: `Requires WSL {version} or later. You have {installed}.` and `WSLC is not available. See Runtime.`

### 5.11 Busy button
During a retry or long action: button `aria-disabled="true"` `aria-busy="true"`, label changes to the "-ing" form (Retry becomes Retrying...), small spinner replaces the icon. After 3 s of waiting append "Still working..." in muted text next to it. Repeat clicks are ignored.

### 5.12 Loading panel (startup)
Shown only if detection takes longer than 150 ms (avoids flicker). Three step lines, the current one with a spinner, completed ones with a check, pending ones muted.

## 6. Layout regions (shell)

```
+---------------------------------------------------------------+
| header (40)   brand | nav tabs ............ runtime chip       |
+---------------------------------------------------------------+
| notices region: [update banner slot REQ-009] [system notices] |
+---------------------------------------------------------------+
| main region:  gate panel  OR  [list notices] + container list |
|                                           [toast region ->]   |
+---------------------------------------------------------------+
| status bar (24)   availability | versions | sessions ... slot  |
+---------------------------------------------------------------+
```

The main region scrolls, header and status bar do not. Never scroll the whole window.

## 7. Motion

- Spinner: 1 s linear rotation. Banner/toast/popover entry: 150 ms fade plus 4 px translate, ease-out. Nothing else animates.
- `@media (prefers-reduced-motion: reduce)`: no rotation (the Sync glyph is static and the label already says "Retrying..."), no translate, fades shortened to 0 ms, height transitions removed.
- Nothing flashes more than once per second, no auto-scrolling.

## 8. Focus and keyboard (global)

- Visible focus ring on every interactive element: `outline: 2px solid var(--color-focus); outline-offset: 2px;` Never remove the outline.
- Tab order follows visual order. Landmarks: `header`, `nav`, notices `section`, `main`, status `footer`. First tab stop is a "Skip to main content" link (visible on focus).
- Global shortcuts are listed per ticket and registered once in one place. Registered so far: see [[REQ-001-002-008-ux]] section 6.

## 9. Accessibility baseline

- Text 4.5:1, large text and UI icons/borders 3:1. Verified by hand for the tokens above (re-check with a tool).
- Never colour only: every state has an icon with a distinct shape and a word.
- Respect `prefers-reduced-motion`, `prefers-color-scheme`, and `forced-colors: active` (use `CanvasText`, `Canvas`, `Highlight` system colours for borders, focus ring and icons; state meaning is carried by the icon glyph and text anyway).
- Text must reflow at 200% zoom down to 800 px wide without horizontal scrolling of the shell (tables may scroll inside their own region).
- Screen reader names: icon-only buttons have `aria-label` from strings.

## 10. Copy rules

- Sentence case, no exclamation marks, no blame. First sentence says what happened, second says what to do. Max two sentences in a message; everything else goes into Details.
- Never put exit codes, `WSLC_E_*` tokens, stderr or paths in the primary message.
- Buttons are verbs: Retry, Refresh, Copy details, Dismiss.
- All strings live in `src/strings.ts` (D13). Templates use `{name}` placeholders, filled by one small formatting helper.
