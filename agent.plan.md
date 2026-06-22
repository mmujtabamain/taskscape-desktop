# Taskscape — Radical UI Reinvention (Approved Plan)

> In-repo copy of the approved implementation plan (source:
> `~/.claude/plans/lovely-spinning-pie.md`). Progress is tracked in
> [agent.todo.md](agent.todo.md).

## Context

The user wants a **radical reinvention** of Taskscape's interface: rethink how
rounded surfaces should be, and reimagine every micro-interaction (hover, press,
focus, state changes) per component. The product's goals/use-cases are unchanged
(frictionless capture · calm list · attached context, for macOS power users).
Before any UI work, **all UI must be separated into a `ui/` tree** so it's easy
to iterate later.

Confirmed direction:
- **Motion:** custom animated components — a `ui/` layer where each interactive
  control owns its own eased hover/press/focus tweens, plus animated key moments.
- **Form:** *sharpened, not sharp* — **rounded corners throughout (no sharp/hard
  corners anywhere), no pills**. Sharpness comes from discipline, not edges: sharp/
  precise typography, **fill over outline** (a component with a background gets no
  border), restrained card use, and generous space.
- **Color/material:** full reinvention → **gray neutrals + a warm brown accent**.
  Two distinct materials:
  - **Mini window = frosted glass**, Spotlight-style: translucent + real blur,
    gray-toned, brown accent.
  - **Main window = solid** counterpart sharing the gray + brown identity.
- **Structure:** shared design system in `common/src/ui/`; screens move into each
  crate's `app/ui/`.

Feasibility verified against installed crates:
- Iced 0.14 `iced::animation::Animation` (Lilt easing); custom widgets own state
  in `widget::tree::State` and self-drive redraws via `Shell::request_redraw_at`
  inside `Widget::update`.
- Frosted blur = native `NSVisualEffectView` (`objc2-app-kit 0.3.2`, feature
  `"NSVisualEffectView"`, material `HUDWindow`/`FullScreenUI`, blending
  `BehindWindow`, state `Active`).
- Fonts/icons: Montserrat + Raleway (OFL) + Material Symbols Sharp (Apache-2.0),
  downloaded into `assets/fonts/`, embedded via `include_bytes!`. Retire Poppins,
  Inter, and `lucide-icons`.

## Design language — "Concrete & Bronze"

- **Color:** gray surfaces (dark bg `~#161719`→surface `~#1D1F22`→raised `~#26282C`;
  light bg `~#F2F3F4`→surface `~#FAFAFB`→raised `#FFFFFF`), separated by fill/tone
  not outlines. Bronze accent (`~#B5825A` dark / `~#8A5A36` light) — single signal
  (primary action, selection, focus ring, linked dot). Text off-white/secondary/
  muted, AA-checked.
- **Materials:** mini = frosted glass (NSVisualEffectView behind transparent Iced
  surface, ~16px corner, subtle edge stroke, follows ThemeMode); main = solid matte
  graphite, fill/tone separation, rounded radii.
- **Form (sharpened, soft-cornered):** no sharp corners, no pills. Radii `sm 8`,
  `md 10`, `lg 12`, `xl 16`. Sharpness via discipline: precise type + sharp icons;
  **fill over outline** (bg ⇒ no border; hairlines only where no fill or as a state
  ring); **card discipline** (flat rows, no nested cards).
- **Type & icons:** Raleway (display/headings/title), Montserrat (body/UI/labels);
  Material Symbols Sharp icons. Retire Poppins/Inter/Lucide.
- **Motion:** durations instant/120/180/250ms, ease-out-quint, no bounce. Per-widget
  redraw while animating. Hover lifts fill; press scale ~0.98; focus ring fades in.
  Task complete = checkbox fill + check draw + strike + fade. Key moments: mini
  open/close, theme cross-fade, task add/remove, modal in/out, sidebar width,
  linked-dot pulse. Reduce-motion toggle (config) collapses tweens.

## Architecture

- `common/src/ui/` replaces `thememanager/` + `widgets/`: `tokens.rs`, `theme.rs`,
  `motion.rs`, `components/` (`interactive.rs` cornerstone widget + button, text_input,
  checkbox, toggle, chip, dropdown, typography, icon, editable_title, metric,
  attachment, containers), `mod.rs` re-exports. Rework `utils/fonts.rs`.
- `main_src/src/app/ui/` (from `view/`): screens + app-state animations + `Tick` +
  `window::frames()` subscription + reduce-motion flag.
- `tray_src/src/app/ui/` (from `mini.rs` + `quit_confirm.rs`): native glass +
  open/close animation.
- Logic files untouched except for the reduce-motion config field.

## Phases
1. Foundation: fonts + icon font + `ui/{tokens,theme,motion}`; drop lucide.
2. Animated `interactive.rs` primitive.
3. Component toolkit; switch `lib.rs` to `ui`.
4. Main screens → `app/ui/`; app-state animations.
5. Tray mini + frosted glass.
6. Refresh impeccable docs (PRODUCT.md, DESIGN.md, sidecar), `.index/`, `CLAUDE.md`.

## Verification
`cargo check`/`cargo build` clean per phase; `./run-dev.sh` + `./run-dev.sh tray`.
User verifies visuals/feel (both themes, reduce-motion, AA, glass blur, sharpened
cues). Claude owns build/type errors only; no UI automation.

## Risks
Native blur (NSVisualEffectView below winit content view, transparent wgpu) is the
highest-risk bit — validated early, fallback to translucent fill. Custom Widget
impls intricate — contained by one reusable primitive. Large diff (~26 UI files +
app state). Git stays read-only on Claude's side.
