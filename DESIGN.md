---
name: Taskscape
description: A calm gray task manager with one warm bronze signal; a frosted-glass mini HUD.
colors:
  accent: "#B5825A"
  accent-hover: "#C69771"
  on-accent: "#1A1410"
  bg: "#161719"
  surface: "#1D1F22"
  raised: "#26282C"
  text: "#E7E8EA"
  text-dim: "#A2A7AE"
  text-muted: "#8A9098"
  hairline: "#FFFFFF1A"
  ring: "#B5825A8C"
  success: "#72B07D"
  danger: "#D67D67"
  warning: "#D9A445"
  scrim: "#0000008C"
  glass-tint: "#181A1D80"
  glass-edge: "#FFFFFF1F"
typography:
  display:
    fontFamily: "Raleway, sans-serif"
    fontSize: "32px"
    fontWeight: 700
    lineHeight: 1.1
    letterSpacing: "normal"
  heading:
    fontFamily: "Raleway, sans-serif"
    fontSize: "22px"
    fontWeight: 600
    lineHeight: 1.2
    letterSpacing: "normal"
  title:
    fontFamily: "Raleway, sans-serif"
    fontSize: "17px"
    fontWeight: 600
    lineHeight: 1.25
    letterSpacing: "normal"
  body:
    fontFamily: "Montserrat, sans-serif"
    fontSize: "14px"
    fontWeight: 400
    lineHeight: 1.4
    letterSpacing: "normal"
  label:
    fontFamily: "Montserrat, sans-serif"
    fontSize: "12px"
    fontWeight: 500
    lineHeight: 1.3
    letterSpacing: "normal"
rounded:
  sm: "8px"
  md: "10px"
  lg: "12px"
  xl: "16px"
spacing:
  xs: "4px"
  sm: "6px"
  md: "8px"
  lg: "12px"
  xl: "16px"
  xxl: "24px"
components:
  button-primary:
    backgroundColor: "{colors.accent}"
    textColor: "{colors.on-accent}"
    rounded: "{rounded.md}"
    padding: "10px 14px"
  button-primary-hover:
    backgroundColor: "{colors.accent-hover}"
    textColor: "{colors.on-accent}"
    rounded: "{rounded.md}"
  button-ghost:
    backgroundColor: "#E7E8EA0D"
    textColor: "{colors.text}"
    rounded: "{rounded.md}"
    padding: "10px 14px"
  button-plain:
    backgroundColor: "transparent"
    textColor: "{colors.text}"
    rounded: "{rounded.md}"
    padding: "6px 7px"
  input:
    backgroundColor: "{colors.raised}"
    textColor: "{colors.text}"
    rounded: "{rounded.md}"
    padding: "12px 14px"
  chip-accent:
    backgroundColor: "{colors.accent}"
    textColor: "{colors.on-accent}"
    rounded: "{rounded.sm}"
    padding: "5px 9px"
  chip-neutral:
    backgroundColor: "{colors.raised}"
    textColor: "{colors.text-dim}"
    rounded: "{rounded.sm}"
    padding: "5px 9px"
  card-modal:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.text}"
    rounded: "{rounded.xl}"
    padding: "24px"
  glass-shell:
    backgroundColor: "{colors.glass-tint}"
    textColor: "{colors.text}"
    rounded: "{rounded.xl}"
---

# Design System: Taskscape

## 1. Overview

**Creative North Star: "Concrete & Bronze"**

Taskscape is a calm gray field with a single warm bronze signal. The surface is a
quiet, machined neutral — cool graphite in dark, pale concrete in light — and the
one accent, a caramel bronze, is the tool you reach for: the primary action, the
current selection, the focus ring, the live "linked" dot. Its rarity is what makes
it read as *the* action. Two materials carry the product: the menu-bar **mini
window is a frosted-glass HUD** (Spotlight-like — translucent, blurred, floating),
while the **main window is the solid matte workbench** you sit at. Same identity,
different state of matter.

The system is **sharpened, not sharp**. Every corner is rounded (8–16px) — there
are no hard edges and no pills — yet the whole thing reads precise and crafted.
That sharpness comes from discipline, not geometry: crisp Raleway/Montserrat type,
sharp-cornered Material Symbols icons, **fill over outline** (a surface with a
background never also carries a border), and restraint (one accent, generous space,
no decorative chrome). Depth comes from a tonal ladder (`bg → surface → raised`),
not from heavy shadows.

It explicitly rejects the **corporate SaaS dashboard** (dense, enterprise-blue,
productivity-as-labor), **sterile gray minimalism** (gray that reads cold, flat,
and lifeless — ours is warmed by the bronze and the craft), **visual clutter**, and
**gamification**. Calm confidence is the register; the tool disappears into the
task.

**Key Characteristics:**

- Calm gray surfaces, one warm bronze signal used sparingly as "the action."
- Two materials: a frosted-glass mini HUD vs. a solid matte main window.
- Sharpened, soft-cornered: rounded everywhere (no sharp corners, no pills).
- Fill over outline; separation by tone, not borders.
- Animated micro-interactions on every control (hover/press), reduce-motion aware.

## 2. Colors

A cool gray ladder under a single caramel-bronze accent, shipped in two themes that
share one identity. The frontmatter carries the **dark** theme as canonical; the
light theme below is its equal.

### Primary

- **Caramel Bronze** (dark `#B5825A` / light `#8A5A36`): The sole accent and the
  app's signature. Reserved for the primary action, current selection/open list,
  input focus ring, and the live link indicator. Never a large wash, never
  decorative. **On-Accent Ink** (dark `#1A1410` / light `#FCF8F4`) sits on bronze
  fills.

### Neutral

The gray ladder does the structural work (dark → light per role).

- **Background** (dark `#161719` / light `#F2F3F4`): The window base.
- **Surface** (dark `#1D1F22` / light `#FAFAFB`): Mid layer — panels, sidebar,
  list area, modal cards, status bar.
- **Raised** (dark `#26282C` / light `#FFFFFF`): Top layer — inputs, dropdowns,
  resting controls, chips.
- **Text** (dark `#E7E8EA` / light `#181B1F`): Primary text.
- **Text Dim** (dark `#A2A7AE` / light `#555B62`): Supporting copy.
- **Text Muted** (dark `#8A9098` / light `#6E747C`): Quietest labels — kept above AA.
- **Hairline** (white @10% / black @12%): The *only* sanctioned line — used where
  there is no fill (e.g. the header divider), never on a filled surface.
- **Ring** (accent @ ~50%): Focus / selection cue.

### Tertiary (semantic state)

- **Success** (`#72B07D` / `#3E7D4E`), **Danger** (`#D67D67` / `#B8543F`),
  **Warning** (`#D9A445` / `#8A6012`) — state only, never brand color.

### Glass (mini window only)

- **Glass Tint** (dark `#181A1D` @50% / light white @55%) and **Glass Edge**
  (white @12% / black @12%) ride on top of the native vibrancy view to define the
  floating HUD against the desktop.

### Named Rules

**The One Bronze Rule.** Bronze appears on a *single* primary affordance per view.
Two bronze things on screen means one is wrong. All other hierarchy is the gray
ladder + type weight.

**The Fill-Over-Outline Rule.** A component with a background color gets **no
border**. Separation is the fill and its tonal step. Hairlines exist only where
there is no fill; a border otherwise is forbidden.

**The Dual-Theme Parity Rule.** Every color exists in both themes at full quality;
a change to a token changes both values.

## 3. Typography

**Display / Heading Font:** Raleway (Medium 500 / SemiBold 600 / Bold 700).
**Body / UI Font:** Montserrat (Regular 400 / Medium 500 / SemiBold 600).

**Character:** Two geometric sans set on a deliberate size/weight contrast — large,
airy Raleway headings against compact, even Montserrat UI. The pairing reads
intentional because the roles never blur: Raleway speaks for structure, Montserrat
for content and controls.

### Hierarchy

- **Display** (Raleway Bold, 32px, ~1.1): The editable list title — the one large moment.
- **Heading** (Raleway SemiBold, 22px): Screen/section headings, empty-state titles.
- **Title** (Raleway SemiBold, 17px): Modal titles, sub-headers.
- **Body** (Montserrat Regular, 14px): Task text, button labels, input values.
- **Label** (Montserrat Medium, 12px): Captions, chips, metric counts.

### Named Rules

**The Two-Voice Rule.** Raleway = structure; Montserrat = content/controls. Never
swap them.

**The Sharp-Marks Rule.** Sharpness lives in the marks — crisp type, sharp-cornered
icons — not in the (always rounded) surfaces. No tracked all-caps eyebrows.

## 4. Elevation

**Flat-by-default with rationed lift.** Depth is normally the tonal ladder
(`bg → surface → raised`); most surfaces have no shadow. Shadows are scarce and
warm-neutral: the **modal card** lifts above its scrim, and the **sidebar** casts a
soft shadow onto the content area. Interactive controls convey depth through their
animated fill and a 1px hover lift, not shadow.

The mini window is a special case: its depth is *physical* — a real native
`NSVisualEffectView` blur behind a transparent surface.

### Named Rules

**The Earned-Shadow Rule.** A shadow only where a surface genuinely floats (modal,
sidebar edge, the glass HUD). Everything else separates by tone.

## 5. Components

Every control is built from the `common::ui` toolkit and styled through `theme` +
`tokens`. The animated `interactive` widget is the cornerstone: it owns its own
hover/press animation in widget state and tweens the fill (and a 1px lift) on an
ease-out-quint curve (~120ms), self-driving redraws — so micro-interactions never
touch app state. All motion collapses to instant under **Reduce motion**.

### Buttons (radius 10px)
- **Primary:** bronze fill, On-Accent ink; hover brightens (`#C69771`) + 1px lift; press settles.
- **Ghost / Icon:** a faint text-tinted fill (~5%) that strengthens on hover; no border.
- **Plain:** transparent until a faint hover tint — for actions nested in a row.

### Inputs / Dropdown (radius 10px)
- Filled (`raised`), **no rest border**; focus brings a bronze **ring**; hover lifts the fill slightly. Placeholder in `text-muted`.

### Checkbox / Toggle
- **Checkbox:** an outlined box (hairline ring) that becomes a bronze fill with a sharp check when set; hover/press animated.
- **Toggle:** a rounded-rect track (not a pill); knob left (off) / right (on); fill goes bronze when on.

### Chips (radius 8px)
- Filled, no border. **Accent** (bronze fill, ink text) or **neutral** (`raised`, dim text). Attachment chips add a thumbnail/icon + a remove (×) and animate on hover.

### List rows / Task rows
- **Flat** — transparent at rest, a faint fill on hover (not stacked cards). The current list / open row uses a bronze-tinted fill. **No nested cards.**

### Cards / Containers (radius 12–16px)
- Reserved for genuinely distinct surfaces (the **modal card**). Filled, no border, soft shadow above the scrim. The sidebar and panels are filled surfaces separated by tone.

### Mini Window (signature, frosted glass)
- A transparent window with a native `NSVisualEffectView` (BehindWindow / HUDWindow material) behind the Iced content; `glass_shell` lays a faint tint + a 1px defining edge on top, clipped to a 16px radius (CALayer), no drop shadow. The Spotlight-style capture HUD.

### Editable Title (signature)
- A 32px Raleway Bold title that flips in place between display text and a borderless inline field, with a bronze underline while editing.

## 6. Do's and Don'ts

### Do:
- **Do** reserve bronze for one primary action per view (the One Bronze Rule); build the rest from the gray ladder and type weight.
- **Do** separate filled surfaces by tone, never by a border (the Fill-Over-Outline Rule); use a hairline only where there is no fill.
- **Do** round every corner (8–16px) and let sharpness come from crisp type + sharp icons + restraint.
- **Do** keep both themes AA-legible (4.5:1 body, 3:1 large) — including bronze-on-surface and text on the frosted glass.
- **Do** route every style through `common::ui`; build controls on the `interactive` widget so hover/press are animated and reduce-motion aware.
- **Do** keep capture frictionless: hotkey → type → Enter and the mini window stay ceremony-free.

### Don't:
- **Don't** build toward a **corporate SaaS dashboard** — no dense data-grid chrome, no enterprise blue.
- **Don't** ship **sterile gray** — gray is the calm ground, but the bronze signal + crafted detail must keep it alive, never cold/flat/lifeless.
- **Don't** use **sharp/hard corners or pills** — everything is softly rounded; sharpness is in the marks, not the geometry.
- **Don't** put a border on a filled component, or stack/nest cards (no chip-in-card-in-panel; task rows are flat).
- **Don't** **gamify** or **clutter** — no confetti/streaks/mascots, no controls competing for attention.
- **Don't** add a second bronze element, a decorative gradient, or a shadow "to make it pop" — reach for the tonal ladder.
