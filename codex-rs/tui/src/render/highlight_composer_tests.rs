use super::*;
use crate::terminal_palette::rgb_color;
use pretty_assertions::assert_eq;
use std::io::Cursor;
use syntect::highlighting::ThemeSet;

#[test]
fn custom_theme_composer_scope_keeps_only_text_style() {
    let mut source = Cursor::new(
        r##"<?xml version="1.0" encoding="UTF-8"?>
<plist version="1.0"><dict><key>settings</key><array>
<dict><key>settings</key><dict>
<key>foreground</key><string>#FFFFFF</string>
<key>background</key><string>#000000</string>
</dict></dict>
<dict><key>scope</key><string>codex.composer.input</string>
<key>settings</key><dict>
<key>foreground</key><string>#305F72</string>
<key>background</key><string>#FF0000</string>
<key>fontStyle</key><string>bold italic underline</string>
</dict></dict></array></dict></plist>"##,
    );
    let theme = ThemeSet::load_from_reader(&mut source).expect("custom theme");
    assert_eq!(
        style_for_theme(&theme),
        Style::default()
            .fg(rgb_color((48, 95, 114)))
            .bold()
            .italic()
            .underlined()
    );
    let mut unscoped = theme;
    unscoped.scopes.clear();
    assert_eq!(style_for_theme(&unscoped), Style::default());
}
