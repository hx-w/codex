# Headers, primary, and secondary text

- **Headers:** Use `bold`. For markdown with various header levels, leave in the `#` signs.
- **Primary text:** Default.
- **Secondary text:** Use `dim`.

# Foreground colors

- **Default:** Most of the time, just use the default foreground color. `reset` can help get it back.
- **User input tips, selection, and status indicators:** Use ANSI `cyan`.
- **Success and additions:** Use ANSI `green`.
- **Errors, failures and deletions:** Use ANSI `red`.
- **Codex:** Use ANSI `magenta`.

# Avoid

- Avoid custom colors because there's no guarantee that they'll contrast well or look good in various terminal color themes. (`shimmer.rs` is an exception that works well because we take the default colors and just adjust their levels.)
- Avoid ANSI `black` & `white` as foreground colors because the default terminal theme color will do a better job. (Use `reset` if you need to in order to get those.) The exception is if you need contrast rendering over a manually colored background.
- Avoid ANSI `blue` and `yellow` because for now the style guide doesn't use them. Prefer a foreground color mentioned above.

(There are some rules to try to catch this in `clippy.toml`.)

# Explicit composer text overrides

Custom `.tmTheme` files may style `codex.composer.input` with `foreground` and
`fontStyle` (`bold`, `italic`, `underline`). Only matching scope settings apply;
the theme's global foreground and background do not become composer defaults.
Apply this base style to editable glyphs before element and search highlights.
Keep unoccupied cells, placeholders, masked input, prompts, and footers unchanged.
Embedded `ComposerInput` owners can override the base style with `set_text_style`
and return to the active theme with `reset_text_style`; owners request redraws.
