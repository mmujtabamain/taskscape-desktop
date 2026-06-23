# Agent TODO — UI Reinvention ("Concrete & Bronze")

Living progress tracker for the redesign. Plan: [agent.plan.md](agent.plan.md).
`[x]` done, `[~]` in progress, `[ ]` pending.

## Phase 1 — Foundation ✅
- [x] Montserrat + Raleway (OFL) static weights in `assets/fonts/`
- [x] Material Symbols Sharp full font (dev) + codepoints; subset → `MaterialSymbolsSharp-subset.ttf` (4.4KB, 28 glyphs) + `regen-subset.sh` + `used-icons.txt`
- [x] `common/src/utils/fonts.rs` — Montserrat/Raleway/MS-Sharp builders + `REGISTERED_FONT_BYTES`
- [x] `common/src/ui/{tokens,theme,motion}.rs` — gray+bronze palette, radii, spacing, type, motion
- [x] `common/src/ui/components/icon.rs` — MS-Sharp glyph map (28 icons)

## Phase 2 — Animated `interactive` primitive ✅
- [x] Custom `Widget` with hover/press `Animation` in `tree::State`, self-driven redraws, fill+ring+lift

## Phase 3 — Component toolkit ✅
- [x] button, icon_button, text_input, checkbox, toggle, chip (+attachment), dropdown, typography, editable_title, metric, containers (incl. glass mini-shell)

## Phase 4 — Main window screens ✅
- [x] `view/` → `app/ui/`; all screens on `common::ui`; hardcoded values → tokens
- [x] Reduce-motion: config field + Settings toggle + `motion::set_reduce_motion`
- [~] App-state choreography (theme cross-fade, modal/sidebar tweens) **DEFERRED** —
  per-widget hover/press motion (interactive) is the core ask and ships; app-state
  frames-driven tweens need a redraw loop and are left as a follow-up (kept instant).

## Phase 5 — Tray mini + frosted glass ✅
- [x] `mini.rs` → `app/ui/{mini,quit_confirm}.rs`; on `common::ui`
- [x] Native `NSVisualEffectView` frost (`tray::frost_window`, BehindWindow/HUDWindow), wired on mini + confirm open; `"NSVisualEffectView"`+`"NSGraphics"` features
- [x] Fonts registered; reduce-motion from config

## Phase 5b — Remove legacy ✅
- [x] Deleted `common/src/widgets` + `common/src/thememanager`; `ThemeMode` now in `ui::theme`
- [x] Dropped `lucide-icons` from all crates; retired Inter/Poppins builders
- [x] Enabled iced `advanced` feature on common (custom Widget). **Full `cargo build` clean.**

## Phase 6 — Docs ✅
- [x] `PRODUCT.md` — personality/anti-refs reconciled with gray+bronze
- [x] `DESIGN.md` + `.impeccable/design.json` — rewritten for Concrete & Bronze
- [x] `.index/{theming,where-to-fix,common,main,tray,README,architecture,glossary}.md`
- [x] `CLAUDE.md` + `common/Cargo.toml` comment

## Frost fix (post-build)
- [x] **No-frost bug:** iced cleared the surface with the *opaque* theme background
  (`theme::Style.background_color` default) → solid dark over the vibrancy. Fixed by
  adding `.style()` to both daemons returning `background_color: Color::TRANSPARENT`
  (mini + main). View-hierarchy frost technique (`frost_window`/`chrome::apply`) was
  already correct; the opaque clear was masking it.
- Debug `eprintln!("[frost]"/"[chrome]"…)` left in `tray.rs`/`chrome.rs` for this
  round — remove once the blur is confirmed.

## DONE — full `cargo build` clean. Hand off to user for visual verification.

## Notes / follow-ups
- Inter/Poppins TTFs still on disk under `assets/fonts/` (unused) — safe to `git rm`.
- App-state animation choreography deferred (see Phase 4). To enable: add a
  `window::frames()`-driven `Tick` + per-moment `Animation`s in app state.
- Add icons later: edit `assets/fonts/MaterialSymbols/used-icons.txt` + run
  `regen-subset.sh` (+ a variant in `ui/components/icon.rs`). No internet needed.
- Verify (user): both themes, reduce-motion, AA contrast, the frosted mini window
  over a real desktop, hover/press across all controls.
