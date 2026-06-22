---
name: Taskscape
description: A warm, calm macOS task manager built for frictionless capture.
colors:
  accent: "#FF7C5D"
  accent-text: "#21110E"
  bg-top: "#4A301D"
  bg-bottom: "#2B1C16"
  panel-alt: "#3C2D26"
  panel-raised: "#2B211B"
  border: "#C59A742E"
  border-strong: "#F2A17580"
  text-primary: "#F5E7D4"
  text-secondary: "#C9B199"
  text-muted: "#9B7F6C"
  shadow: "#08050459"
  success: "#81B27D"
  danger: "#E27F67"
  warning: "#E4B262"
typography:
  display:
    fontFamily: "Poppins, -apple-system, sans-serif"
    fontSize: "40px"
    fontWeight: 600
    lineHeight: 1.1
    letterSpacing: "normal"
  headline:
    fontFamily: "Poppins, -apple-system, sans-serif"
    fontSize: "26px"
    fontWeight: 600
    lineHeight: 1.2
    letterSpacing: "normal"
  title:
    fontFamily: "Poppins, -apple-system, sans-serif"
    fontSize: "20px"
    fontWeight: 600
    lineHeight: 1.25
    letterSpacing: "normal"
  body:
    fontFamily: "Inter, -apple-system, sans-serif"
    fontSize: "15px"
    fontWeight: 400
    lineHeight: 1.3
    letterSpacing: "normal"
  label:
    fontFamily: "Inter, -apple-system, sans-serif"
    fontSize: "12px"
    fontWeight: 400
    lineHeight: 1.3
    letterSpacing: "normal"
rounded:
  square: "0px"
  sm: "10px"
  md: "12px"
  lg: "14px"
  xl: "16px"
spacing:
  xs: "4px"
  sm: "6px"
  md: "8px"
  lg: "12px"
  xl: "20px"
components:
  button-primary:
    backgroundColor: "{colors.accent}"
    textColor: "{colors.accent-text}"
    rounded: "{rounded.lg}"
    padding: "10px 14px"
  button-primary-hover:
    backgroundColor: "#FF876A"
    textColor: "{colors.accent-text}"
    rounded: "{rounded.lg}"
  button-ghost:
    backgroundColor: "#2B211B59"
    textColor: "{colors.text-primary}"
    rounded: "{rounded.lg}"
    padding: "10px 14px"
  button-icon:
    backgroundColor: "{colors.panel-raised}"
    textColor: "{colors.text-primary}"
    rounded: "{rounded.md}"
    padding: "10px 12px"
  button-plain:
    backgroundColor: "transparent"
    textColor: "{colors.text-primary}"
    rounded: "{rounded.md}"
    padding: "6px 7px"
  input:
    backgroundColor: "{colors.panel-raised}"
    textColor: "{colors.text-primary}"
    rounded: "{rounded.md}"
    padding: "12px 14px"
  chip-accent:
    backgroundColor: "{colors.panel-alt}"
    textColor: "{colors.accent-text}"
    rounded: "{rounded.xl}"
    padding: "6px 10px"
  chip-neutral:
    backgroundColor: "{colors.panel-raised}"
    textColor: "{colors.text-secondary}"
    rounded: "{rounded.lg}"
    padding: "6px 10px"
  card-panel:
    backgroundColor: "{colors.panel-alt}"
    textColor: "{colors.text-primary}"
    rounded: "{rounded.sm}"
    padding: "12px"
  card-modal:
    backgroundColor: "{colors.panel-alt}"
    textColor: "{colors.text-primary}"
    rounded: "{rounded.xl}"
    padding: "20px"
---

# Design System: Taskscape

## 1. Overview

**Creative North Star: "The Considered Desk"**

Taskscape is an uncluttered, deliberately arranged personal desk. Nothing extra
sits on it; everything that is there is there on purpose and within easy reach.
The surface is warm — terracotta, amber, toasted browns — so it feels lived-in
and calm rather than clinical, but the warmth never tips into decoration. A
single coral accent is the one tool you reach for most, and its rarity is what
makes it read as "the action." This is a tool you keep open all day and never
have to brace yourself to look at.

Density is low and rhythm is gentle. Depth comes from **tonal layering** — a
ladder of warm surfaces (`bg → panel-alt → panel-raised`) — not from heavy
shadows or outlines. Shadows are rationed: they appear only where a surface
genuinely lifts off the page (the primary action, a modal, the sidebar's edge).
Corners are softly rounded (10–16px) so the app feels friendly and native to
macOS, while the sidebar stays square to read as architecture rather than a
floating card.

The system explicitly rejects the **corporate SaaS dashboard** (dense,
enterprise-blue, productivity-as-labor), **sterile gray minimalism** (cold, flat,
characterless), **visual clutter** (controls competing for attention), and
**gamification** (confetti, streaks, encouragement theatrics). Calm confidence is
the register. The design earns its keep by getting out of the way.

**Key Characteristics:**

- Warm, tonal, low-contrast surfaces; warmth carried by palette and type, never decoration.
- One coral accent, used sparingly, as the single "this is the action" signal.
- Tonal layering over shadows; depth is rationed and meaningful.
- Soft, friendly radii (10–16px); the square sidebar is the deliberate exception.
- Full dark/light parity — both themes are first-class, and both must stay legible (WCAG AA).

## 2. Colors

A warm earthen palette: toasted browns and ambers under a single coral accent,
shipped in two fully-developed themes that share one identity. The frontmatter
above carries the **dark** theme as the canonical reference; the light theme
below is its equal, not an afterthought.

### Primary

- **Ember Coral** (dark `#FF7C5D` / light `#F26E53`): The sole accent and the
  app's signature. Reserved for the primary action (the filled button), focus
  rings on inputs, the mini window's hairline border, and selection highlights.
  It is never used as a large fill or a decorative wash — its scarcity is the
  point. On it sits **Accent Ink** (dark `#21110E` / light `#FEF8F1`) for label
  text at full contrast.

### Neutral

The warm-neutral ladder does all the structural work. Listed dark → light per role.

- **Backgrounds** — a top-to-bottom gradient, not a flat fill. Dark: `#4A301D` →
  `#2B1C16`. Light: `#F6E9D8` → `#F1E5D4`. The window body is always this
  gentle warm gradient (angle ≈ 0.25rad), with a blended midpoint at 42%.
- **Panel (alt)** (dark `#3C2D26` / light `#F7F0E3`): The mid surface — sidebar,
  metric cards, modal cards, accent chips. The first step up from the body.
- **Panel (raised)** (dark `#2B211B` / light `#FBF7EF`): The top surface —
  inputs, dropdowns, icon buttons, neutral chips. Reads as the most "tactile,
  pick-up-able" layer.
- **Text Primary** (dark `#F5E7D4` / light `#311D15`): Body and headings.
- **Text Secondary** (dark `#C9B199` / light `#7A6153`): Supporting copy, dropdown handles, chip labels.
- **Text Muted** (dark `#9B7F6C` / light `#9B8475`): Placeholders, metric-card captions, the quietest labels. **Handle with care — see the Contrast Floor Rule.**
- **Border** (dark `#C59A74` @18% / light `#8E634E` @18%): Hairline dividers and resting borders; a warm tint of the surface, never a hard gray line.
- **Border Strong** (dark `#F2A175` @50% / light `#E68B6B` @42%): Hover/active borders and modal edges — a warmed-up version of the resting border.
- **Shadow** (dark `#080504` @35% / light `#6B4D3C` @10%): A warm-brown shadow, never neutral black, so lift still feels part of the palette.

### Tertiary (semantic state)

Used only to signal state, never as brand color.

- **Success Sage** (dark `#81B27D` / light `#7EA274`): Completion / positive state.
- **Danger Clay** (dark `#E27F67` / light `#D16B59`): Destructive actions, errors. Note its closeness to the accent — never let danger read as the primary action.
- **Warning Amber** (dark `#E4B262` / light `#D29B43`): Cautions.

### Named Rules

**The One Ember Rule.** Ember Coral appears on a *single* primary affordance per
view. If two things on screen are coral, one of them is wrong. Everything else
earns hierarchy through the neutral ladder and type weight, not more accent.

**The Dual-Theme Parity Rule.** Every color exists in both themes and both must
ship at full quality. Never tune one theme and let the other degrade. A change to
a token is a change to *both* of its values.

**The Warm-Shadow Rule.** Shadows and borders are tinted toward the surface's own
hue (warm brown/amber), never neutral gray or pure black. A gray line on this
palette looks broken.

## 3. Typography

**Display / Heading Font:** Poppins SemiBold (weight 600), with `-apple-system` fallback.
**Body Font:** Inter Regular (weight 400), with `-apple-system` fallback.

**Character:** A clean geometric-humanist pairing. Poppins gives headings a
friendly, rounded confidence that echoes the soft corners; Inter keeps body and
UI text quiet, neutral, and exceptionally legible at small sizes. The contrast
lives on the weight/role axis (semibold display vs. regular body), not on
fighting personalities — they share a calm temperament.

### Hierarchy

- **Display** (Poppins SemiBold, 40px, ~1.1): The editable list title only — the one large, confident moment on the page.
- **Headline** (Poppins SemiBold, 26px, ~1.2): Primary screen and section headings, empty-state titles.
- **Title** (Poppins SemiBold, 17–22px): Modal titles, sub-section headers, secondary headings.
- **Body** (Inter Regular, 14–16px, ~1.3): Task text, button labels (16px), input values, descriptions. 15px is the default.
- **Label** (Inter Regular, 11–13px): Captions, chips, metric counts, the quietest metadata.

### Named Rules

**The Two-Voice Rule.** Poppins speaks for structure (titles, headings); Inter
speaks for content and controls. Don't set body copy in Poppins or headings in
Inter — the role of a piece of text is legible from its font alone.

**The Quiet-Type Rule.** Size and weight carry hierarchy; tracking and case do
not. No all-caps tracked eyebrows, no letter-spacing gymnastics. Headings stay at
normal tracking.

## 4. Elevation

Taskscape is **flat-by-default with rationed lift.** Depth is normally conveyed
by the tonal ladder — body gradient → `panel-alt` → `panel-raised` — so most
surfaces have *no* shadow at all. Shadows are a deliberate, scarce signal that a
surface has genuinely left the page, and they are always warm-tinted.

### Shadow Vocabulary

- **Action lift** (`offset 0, blur 14px`, warm shadow token): The primary
  (coral) button only. A soft halo that says "press me," removed entirely on `:active`.
- **Modal lift** (`offset 0 8px, blur 28px`): The centered modal card, floating clearly above its dimmed backdrop.
- **Sidebar edge** (`offset 3px 0, blur 12px`, cast rightward): The sidebar casts onto the content area so it reads as a raised plane beside the workspace, not a seam.

### Named Rules

**The Earned-Shadow Rule.** A surface gets a shadow only if it physically lifts:
the primary action, a modal, the sidebar edge. List rows, cards, chips, inputs,
and panels stay flat and separate themselves by tone. If you're adding a shadow
to make something "pop," use the tonal ladder instead.

## 5. Components

Every component is built from the `common::widgets` `t_*` toolkit and styled
exclusively through `common::thememanager` factories — colors are never
hardcoded at the call site, so dark/light parity is automatic.

### Buttons

- **Shape:** Softly rounded — primary & ghost at 14px (`{rounded.lg}`), icon &
  plain at 12px (`{rounded.md}`). Internal padding `10px 14px` for labeled
  buttons.
- **Primary:** Coral fill (`{colors.accent}`) with Accent-Ink text and the
  *Action lift* shadow. Hover lightens the fill ~8% toward white (`#FF876A`);
  press drops the shadow.
- **Ghost:** Translucent raised panel (35%) with a 1px resting border and
  primary text. Hover blends the fill toward `panel-alt` and warms the border to
  *border-strong*. The quiet secondary action.
- **Icon (bordered):** Solid `panel-raised` fill, 1px border, `10px 12px`
  padding; optional count beside the glyph. Hover warms fill + border together.
- **Plain (borderless):** Transparent, no border, `6px 7px` padding. Only a faint
  hover tint (`text-primary` @6%, deepening to @12% on press) so it can sit
  inside an already-styled row without painting a redundant box. The default for
  in-row actions (per-task paperclip, etc.).

### Chips

- **Accent chip:** `panel-alt` fill, 16px radius, Accent-Ink label, `6px 10px` padding.
- **Neutral chip:** `panel-raised` fill, 14px radius, Text-Secondary label.
- Both use 1px resting borders and 13px Inter labels. Used for attachment chips and small badges.

### Cards / Containers

- **Corner Style:** 10px (`panel-alt` panels, mini window), 12px (empty state), 16px (modal card).
- **Background:** The tonal ladder — `panel-alt` for cards/sidebar/modals, `panel-raised` (often at reduced alpha) for the quietest surfaces.
- **Shadow Strategy:** Flat by default; only the modal card and sidebar carry shadows (see Elevation).
- **Border:** Hairline `border` at rest; `border-strong` (often alpha-reduced) on lifted surfaces like the empty state and modal card.
- **Internal Padding:** 12px (metric cards), 20px (modal cards).

### Inputs / Fields

- **Style:** `panel-raised` background, 12px radius, 1px `border`, `12px 14px` padding, 16px Inter value text.
- **Focus:** Border shifts to full **Ember Coral** (`{colors.accent}`); selection highlight is the accent at 28% alpha.
- **Hover:** Border warms to `border-strong`.
- **Placeholder:** `text-muted` — must still clear the contrast floor (see Do's and Don'ts).
- **Disabled:** Background drops to 50% alpha, value text falls to `text-muted`.

### Dropdown (pick list)

- Matches the input: `panel-raised` fill, 12px radius, 1px border. Hover and open
  states warm the border to `border-strong`. Handle glyph in `text-secondary`.

### Editable Title (signature)

- A 40px Poppins SemiBold list title that flips in place between display text and
  a borderless inline text field (`TITLE_INPUT_ID`). The single largest, most
  confident piece of type in the app, and the primary way a list is renamed —
  capture-first, no modal.

### Mini Window (signature)

- The hotkey-summoned capture popup. A transparent window clipped to a 10px
  radius with a **1.5px Ember-Coral hairline border at 45% alpha** — the one
  place the accent draws an outline, marking the floating surface against the
  desktop behind it. No drop shadow (native CALayer clip); compact spacing
  throughout. This is the frictionless-capture surface and must open, focus, and
  accept a task with zero ceremony.

## 6. Do's and Don'ts

### Do:

- **Do** reserve Ember Coral for one primary action per view (the One Ember Rule). Build all other hierarchy from the neutral ladder and Poppins/Inter weight.
- **Do** convey depth with tonal layering (`bg → panel-alt → panel-raised`) before reaching for a shadow. Shadows are earned (the Earned-Shadow Rule), not decorative.
- **Do** keep both themes at full quality and verify contrast in *each*. Body/UI text must clear **4.5:1**, large/bold text **3:1**, in dark and light alike (the Dual-Theme Parity Rule).
- **Do** route every style through `common::thememanager` factories and build UI from the `t_*` toolkit. Add a style factory rather than hardcoding a color in a binary.
- **Do** tint borders and shadows toward the surface's warm hue (the Warm-Shadow Rule).
- **Do** keep capture frictionless: the hotkey → type → Enter path and the mini window must stay fast, focused, and ceremony-free.
- **Do** honor the system reduced-motion preference; every animation needs a crossfade or instant fallback.

### Don't:

- **Don't** build toward a **corporate SaaS dashboard** — no dense data-grid chrome, no enterprise blue, no productivity-as-labor heaviness. Coral and warm browns are the identity; blue is not in the palette.
- **Don't** drift into **sterile gray minimalism** — no cold flat gray/white surfaces, no neutral-black shadows, no hard gray hairlines. Warmth is non-negotiable.
- **Don't** **clutter** the surface — every control earns its place or is removed (calm by subtraction). No stacked panels or competing actions.
- **Don't** **gamify** — no confetti, streaks, mascots, badges, or bright primary colors. Calm confidence, not encouragement theatrics.
- **Don't** let `text-muted` (`#9B7F6C` / `#9B8475`) sit on a low-contrast surface for anything that must be read — placeholders included. If contrast is even close to 4.5:1, bump toward `text-secondary` or `text-primary`. **The Contrast Floor Rule: light gray "for elegance" is forbidden.**
- **Don't** add a second coral element, a decorative gradient wash, or a shadow "to make it pop." Reach for the tonal ladder instead.
- **Don't** use all-caps tracked eyebrows or letter-spacing tricks for hierarchy; use size and weight (the Quiet-Type Rule).
