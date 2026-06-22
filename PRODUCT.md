# Product

## Register

product

## Users

macOS power users who live in the menu bar and work keyboard-first. They want to
capture a task the instant it occurs — mid-flow, without context-switching to a
full app — and trust it to be there later. They value a native-feeling Mac app
(global hotkey, menu-bar mini window, light/dark, rounded popup) and reach for
Taskscape dozens of times a day for seconds at a time, occasionally opening the
full window to triage, attach context, or organize lists.

The job to be done: **get a thought out of my head and onto a list with zero
friction, then keep that list calm enough to actually look at.**

## Product Purpose

Taskscape is a macOS desktop task manager built for frictionless capture and calm
review. A global hotkey summons a compact mini window at the cursor; the full
window is the source of truth where tasks are triaged, organized into lists, and
enriched with attached files or screenshots. Success looks like: capture feels
instant and reliable, the list never feels like work to read, and relevant
material (files, screenshots) lives with the task instead of scattered elsewhere.

Three capabilities define it against a generic to-do app:

- **Frictionless quick capture** — global hotkey + menu-bar mini window; jot a
  task without leaving what you're doing.
- **A calm, focused list** — a warm, distraction-free home for tasks, the
  opposite of a busy productivity dashboard.
- **Context attached to tasks** — drop files and screenshots onto a task so the
  material that matters travels with it.

## Brand Personality

**Calm · quick · crafted.** The voice is quiet and confident — it never shouts,
never gamifies, never nags. Warmth is real but supporting: it comes from the
terracotta/amber palette and the Poppins + Inter typography, not from decoration
or personality-as-noise. The app should feel like a well-made native Mac tool
that respects attention — fast and effortless to capture into, deliberate and
precise in every detail, and restful to return to.

## Anti-references

- **Corporate SaaS dashboards** (Asana / Jira / Monday): dense, busy,
  enterprise-blue, productivity-as-labor. Taskscape is the antithesis.
- **Sterile gray minimalism** — cold flat gray/white generic Notion-clone looks
  with no warmth or character. Restraint here must still feel warm and alive.
- **Heavy / cluttered interfaces** — many controls, panels, and chrome competing
  for attention. If a control isn't earning its place, it doesn't ship.
- **Playful / gamified to-do apps** — confetti, mascots, streaks, bright primary
  colors. Calm confidence, not encouragement theatrics.

## Design Principles

1. **Capture beats organize.** The shortest path from thought to recorded task
   wins. Never insert a step between intent and capture; optimize the hot path
   (hotkey → type → Enter) above everything else.
2. **Calm by subtraction.** Every control earns its place or is removed. The
   list — not the chrome — is the subject. Reduce before you add.
3. **Warmth without noise.** Warmth lives in palette and type, never in
   decoration. The app may feel inviting; it must never feel busy, cute, or
   loud.
4. **Native craft.** Behave like a first-class Mac citizen. Corners, spacing,
   motion, focus behavior, and light/dark parity are deliberate, not incidental.
5. **Legible for everyone, both themes.** Readability is non-negotiable in light
   and dark alike. Hold the contrast and motion bar high (see below) rather than
   trading it for elegance.

## Accessibility & Inclusion

Target **WCAG 2.1 AA**, held deliberately rather than incidentally:

- Body text ≥ 4.5:1 contrast against its background; large/bold text ≥ 3:1 — in
  **both** the dark (browns/oranges) and light (tans/terracottas) themes.
- Honor the system reduced-motion preference; every animation needs a non-motion
  alternative (crossfade or instant).
- Keyboard-first throughout: the core capture and navigation flows must be fully
  operable without the mouse, with visible focus states.
- Don't rely on color alone to convey state (done/active/danger); pair it with
  shape, weight, or text.
