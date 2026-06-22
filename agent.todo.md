# Agent TODO — UI Reinvention ("Concrete & Bronze")

Living progress tracker for the redesign. Plan: [agent.plan.md](agent.plan.md).
Updated as work proceeds; `[x]` done, `[~]` in progress, `[ ]` pending. Notes
inline.

## Phase 1 — Foundation (fonts, icons, ui tokens/theme/motion)
- [~] Download Montserrat + Raleway (OFL) static weights into `assets/fonts/`
- [ ] Download Material Symbols Sharp icon font into `assets/fonts/`
- [ ] Rework `common/src/utils/fonts.rs`: embed Montserrat/Raleway/MS-Sharp; drop Poppins/Inter
- [ ] `common/src/ui/tokens.rs` — palette roles, radii, spacing, type sizes/weights, motion presets
- [ ] `common/src/ui/theme.rs` — `app_theme`/`tokens` (gray+bronze), `ThemeMode`, color helpers
- [ ] `common/src/ui/motion.rs` — durations/easings, tween helper, reduce-motion gate
- [ ] `common/src/ui/components/icon.rs` — MS-Sharp glyph map (~28 icons); remove `lucide-icons` from 3 crates
- [ ] `cargo check`

**Notes:**
- _(none yet)_

## Phase 2 — Animated `interactive.rs` primitive
- [ ] Custom `Widget` with hover/press/focus `Animation` in `tree::State`, self-driven redraws
- [ ] Prove behind one button; `cargo check`

**Notes:**
- _(none yet)_

## Phase 3 — Component toolkit
- [ ] Port button, text_input, checkbox, toggle, chip, dropdown, typography, editable_title, metric, attachment
- [ ] `containers.rs` (shell/panel/modal/sidebar + glass mini-shell)
- [ ] Switch `common/src/lib.rs` to `ui`; keep API names stable; `cargo build`

**Notes:**
- _(none yet)_

## Phase 4 — Main window screens
- [ ] Move `view/` → `app/ui/`; update `app/mod.rs`
- [ ] Centralize hardcoded radii/spacing/sizes into tokens
- [ ] App-state animations (theme cross-fade, modal, sidebar) + `Tick` + gated `window::frames()`
- [ ] Reduce-motion config field + Settings toggle
- [ ] `cargo build` + `./run-dev.sh`

**Notes:**
- _(none yet)_

## Phase 5 — Tray mini + frosted glass
- [ ] Move `mini.rs` → `app/ui/mini.rs`; split `quit_confirm.rs`
- [ ] Add `"NSVisualEffectView"` feature; insert vibrancy behind Iced surface (validate FIRST)
- [ ] Transparent mini-shell background; rounded-corner clip; edge stroke
- [ ] Mini open/close animation + frames subscription
- [ ] `cargo build` + `./run-dev.sh tray`

**Notes:**
- _(none yet)_

## Phase 6 — Docs
- [ ] Refresh `PRODUCT.md` (personality/anti-ref language; reconcile gray vs "sterile gray")
- [ ] Rewrite `DESIGN.md` + `.impeccable/design.json` for the new identity
- [ ] Update `.index/{theming,where-to-fix,common,main,tray,README}.md`
- [ ] Update `CLAUDE.md` (new `ui/` layout; retired Poppins/Inter/Lucide)

**Notes:**
- _(none yet)_
