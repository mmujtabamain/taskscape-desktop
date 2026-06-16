# Theming & widget toolkit

All UI in both binaries is styled through `common::thememanager` and built from
the `common::widgets` `t_*` toolkit. **Don't hardcode colors in the binaries** —
add or reuse a style factory here so dark/light both work.

## Theme layers

```
ThemeMode (Dark | Light)                         thememanager/theme_mode.rs   (serde, persisted in config)
   │
   ├─ app_theme(mode) -> iced::Theme              thememanager/palette.rs      (used by each app's theme() callback)
   └─ tokens(mode)    -> AppPalette               thememanager/palette.rs      (the named colors)
                              │
        primitives ──────────┘                    thememanager/helpers.rs      (color, with_alpha, mix, border, shadow, background_gradient)
                              │
        per-element style factories (consume mode → iced style fn):
          button_style(mode, kind)                thememanager/button.rs       (kind = ButtonKind, button_kind.rs)
          *_container(mode)                        thememanager/container.rs    (shell, mini_shell, panel_alt, empty_state, modal_backdrop, modal_card, sidebar)
          pick_list_style(mode)                    thememanager/pick_list.rs
          text_input_style(mode)                   thememanager/text_input.rs
```

`AppPalette` holds the named colors (backgrounds, panels, borders, text
variants, accent, shadow). Both themes use a warm palette (dark = browns/oranges,
light = tans/terracottas). All factories are re-exported from
[thememanager/mod.rs](../common/src/thememanager/mod.rs).

### Adding/altering a look

- New color → add to `AppPalette` + set it for both modes in `tokens()` (palette.rs).
- New element style → add a `*_style(mode)` / `*_container(mode)` factory next to
  its peers and re-export it from `mod.rs`.
- New button look → add a `ButtonKind` variant (button_kind.rs) + a branch in
  `button_style` (button.rs).

## Widget toolkit (`common::widgets`, `t_*`)

Composable Iced `Element` builders that already apply the theme + fonts. Re-exported
from [widgets/mod.rs](../common/src/widgets/mod.rs).

| Helper                                  | File                | Use for                                        |
| --------------------------------------- | ------------------- | ---------------------------------------------- |
| `t_heading` / `t_body` / `t_caption`    | t_typography.rs     | Text (Poppins SemiBold / Inter Regular)        |
| `t_button`                              | t_button.rs         | Labeled button (optional icon + `ButtonKind`)  |
| `t_icon_button` / `t_icon_button_ghost` | t_icon_button.rs    | Icon-only button (bordered / borderless)       |
| `t_input_box`                           | t_input_box.rs      | Themed text field (placeholder, submit)        |
| `t_dropdown`                            | t_dropdown.rs       | Themed select                                  |
| `t_editable_title`                      | t_editable_title.rs | Inline-editable large title (`TITLE_INPUT_ID`) |
| `t_metric_card`                         | t_metric_card.rs    | Value + label card                             |
| `t_small_chip`                          | t_small_chip.rs     | Small badge/chip (accent or neutral)           |
| `lucide_icon`                           | lucide_icon.rs      | A Lucide glyph as text                         |

Debug helpers in `widgets/mod.rs`: `t_debug_outline()` and the `DebugWidget`
trait (`.debug()` / `.debug_colored()`) to outline layout while developing.

## Fonts

Embedded TTF bytes + `Font` builders in
[utils/fonts.rs](../common/src/utils/fonts.rs): `inter_regular()`,
`poppins_semibold()`. Registered in each app's `run()` along with the Lucide
font; sources under [assets/fonts/](../assets/fonts/) (Inter, Poppins).
