# Agent TODO — UI Reinvention ("Concrete & Bronze")

Living progress tracker for the redesign. Plan: [agent.plan.md](agent.plan.md).
Updated as work proceeds; `[x]` done, `[~]` in progress, `[ ]` pending. Notes
inline.

## Phase 1 — Foundation (fonts, icons, ui tokens/theme/motion)
- [x] Download Montserrat + Raleway (OFL) static weights into `assets/fonts/`
- [x] Download Material Symbols Sharp (full, dev resource) + codepoints into `assets/fonts/`
- [x] Subset icon font → `MaterialSymbolsSharp-subset.ttf` (4.4KB, 28 glyphs) + `regen-subset.sh` + `used-icons.txt`
- [~] Rework `common/src/utils/fonts.rs`: ADD Montserrat/Raleway/MS-Sharp builders (keep Inter/Poppins until screens migrate)
- [ ] `common/src/ui/tokens.rs` — palette roles, radii, spacing, type sizes/weights, motion presets
- [ ] `common/src/ui/theme.rs` — `app_theme`/`tokens` (gray+bronze), `ThemeMode`, color helpers
- [ ] `common/src/ui/motion.rs` — durations/easings, tween helper, reduce-motion gate
- [ ] `common/src/ui/components/icon.rs` — MS-Sharp glyph map (~28 icons)
- [ ] `cargo check`

**Notes:**
- **Migration ordering (keep build green):** build new `common::ui` ALONGSIDE old
  `widgets`/`thememanager`; old code keeps compiling. Switch screen imports in
  Phases 4–5, then delete old `widgets`+`thememanager`, drop `lucide-icons` dep, and
  remove Inter/Poppins font builders/bytes. So "remove lucide / switch lib.rs" lands
  at the END of Phase 5, not Phase 3.
- Fontsource static weights have **no typographic family** — each weight is its own
  family ("Montserrat", "Montserrat Medium", "Montserrat SemiBold", "Raleway",
  "Raleway Medium", "Raleway SemiBold"). `fonts.rs` selects by exact family name.
- Icon font family name = `Material Symbols Sharp`. Add new icons by editing
  `used-icons.txt` + running `regen-subset.sh` (needs fonttools venv; no internet).
- Inter/Poppins TTFs left in `assets/fonts/` for now (unused once migrated); not
  deleting tracked files — you can `git rm` them later.

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
