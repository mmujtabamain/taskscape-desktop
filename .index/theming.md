# UI design system & component toolkit (`common::ui`)

All UI in both binaries is built from the **`common::ui`** layer and styled through
`ui::theme` + `ui::tokens`. **Don't hardcode colors or sizes in the binaries** — add
or reuse a token / style factory so dark/light both work and the form stays
consistent. Identity: "Concrete & Bronze" (calm gray field, one warm bronze accent;
sharpened-but-rounded; fill-over-outline). Full spec: [../DESIGN.md](../DESIGN.md).

## Layers

```
ThemeMode (Dark | Light)                       ui/theme.rs   (serde, persisted in config)
   ├─ app_theme(mode) -> iced::Theme            ui/theme.rs   (each app's theme() callback)
   ├─ palette(mode)   -> Palette                ui/theme.rs   (the named gray+bronze colors)
   └─ color/with_alpha/mix/border/shadow         ui/theme.rs   (primitives)

ui/tokens.rs   radius {sm 8, md 10, lg 12, xl 16} · space {xs..xxl} · text sizes · HAIRLINE_WIDTH
ui/motion.rs   EASING (ease-out-cubic) · QUICK/PRESS/BASE/SLOW · reduce_motion() gate · progress()
```

`Palette` holds the named colors: `bg / surface / raised` (the tonal ladder),
`text / text_dim / text_muted`, `accent / accent_hover / on_accent / ring`,
`hairline`, `success / danger / warning`, and `scrim`. Both themes are gray with a
bronze accent.

## Component toolkit — `ui/components/`

Composable Iced builders that already apply theme + fonts + motion. Re-exported via
`ui/components/mod.rs` and surfaced at `common::ui::*`.

| Helper / type                          | File              | Use for                                            |
| -------------------------------------- | ----------------- | -------------------------------------------------- |
| `Interactive` (custom `Widget`), `Surface`, `SurfaceStyle` | interactive.rs | **Cornerstone**: animated hover/press fill+lift; everything builds on it |
| `t_button` + `ButtonKind` / `surface_style` | button.rs    | Labeled button (Primary/Ghost/Icon/Plain)          |
| `t_icon_button` / `t_icon_button_ghost`| icon_button.rs    | Icon-only button (filled / borderless)             |
| `t_input_box` / `text_input_style`     | input.rs          | Filled text field (no rest border; focus ring)     |
| `t_dropdown` / `pick_list_style`       | dropdown.rs       | Filled select                                      |
| `t_checkbox`                           | checkbox.rs       | Animated box → bronze fill + check                 |
| `t_toggle`                             | toggle.rs         | Rounded-rect toggle (not a pill)                   |
| `t_small_chip` / `t_attachment_chip`   | chip.rs           | Static badge / interactive attachment chip         |
| `t_heading`/`t_display`/`t_body`/`t_caption` | typography.rs| Text (Raleway display, Montserrat body)            |
| `t_editable_title` (`TITLE_INPUT_ID`)  | editable_title.rs | Inline-editable 32px Raleway title                 |
| `t_metric`                             | metric.rs         | Flat value+label readout (not a card)              |
| `icon` + `Icon`                        | icon.rs           | A Material Symbols Sharp glyph                      |
| `shell`/`mini_shell`/`surface`/`raised`/`bar`/`divider`/`sidebar`/`modal_backdrop`/`modal_card` | containers.rs | Container styles (`shell` = full-bleed solid main window; `mini_shell` = rounded solid mini/popover) |

### Adding/altering a look
- New color → add to `Palette` + set both modes in `palette()` (theme.rs).
- New geometry/size → add to `tokens.rs` (don't hardcode in screens).
- New control look → a new `SurfaceStyle` ramp consumed by `Interactive` (interactive.rs).
- New container → a `*_container`-style factory in containers.rs, re-exported.

### Key rules (enforced by convention)
- **Fill over outline**: a component with a background gets **no** border; hairlines
  only where there's no fill, plus the focus/selection `ring`.
- **No sharp corners, no pills**: rounded radii only; sharpness is in type + icons.
- **One bronze**: the accent marks a single primary affordance per view.
- **Reduce motion**: `motion::reduce_motion()` (set from config) collapses tweens to
  instant; `Interactive` reads it.

## Fonts & icons

Embedded TTF bytes + `Font` builders in [../common/src/utils/fonts.rs](../common/src/utils/fonts.rs):
`montserrat_regular/medium/semibold`, `raleway_medium/semibold/bold`, `icon_font`.
`REGISTERED_FONT_BYTES` is the list each app registers in `run()`. Icon glyphs are a
28-glyph **subset** of Material Symbols Sharp; add icons by editing
`assets/fonts/MaterialSymbols/used-icons.txt` + running `regen-subset.sh`, then adding
a variant in `ui/components/icon.rs`. Sources under
[../assets/fonts/](../assets/fonts/) (Montserrat, Raleway, MaterialSymbols).

## Window chrome (both windows)

Both windows are solid matte (no blur/vibrancy):

- **Mini window** — a borderless, transparent HUD with corners clipped via CALayer
  (`tray::round_window` in [../tray_src/src/app/tray.rs](../tray_src/src/app/tray.rs)).
  The window stays transparent only so the clipped corners read as transparent;
  `mini_shell` paints the opaque rounded fill on top.
- **Main window** — opaque; `chrome::apply` in
  [../main_src/src/app/chrome.rs](../main_src/src/app/chrome.rs) makes the system title
  bar transparent + full-size content view (native traffic lights kept, custom title bar
  drawn in [../main_src/src/app/ui/titlebar.rs](../main_src/src/app/ui/titlebar.rs)).
  `shell` lays the full-bleed solid fill on top; native frame supplies the rounded
  corners + shadow.
