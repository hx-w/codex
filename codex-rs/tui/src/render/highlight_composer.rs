//! Opt-in composer text styling through the active TextMate theme.
//!
//! `codex.composer.input` supplies foreground and font style only. In particular,
//! the theme's global foreground/background are not defaults for this scope: a
//! theme without a matching rule leaves the terminal's input styling unchanged.
//! Reading the active theme on render also follows `/theme` preview and cancel.

use ratatui::style::Style;
use syntect::highlighting::FontStyle;
use syntect::highlighting::Highlighter;
use syntect::highlighting::Theme;
use syntect::parsing::Scope;

pub(crate) fn composer_input_style() -> Style {
    let theme = super::theme_lock()
        .read()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    style_for_theme(&theme)
}

fn style_for_theme(theme: &Theme) -> Style {
    let Ok(scope) = Scope::new("codex.composer.input") else {
        return Style::default();
    };
    let modifier = Highlighter::new(theme).style_mod_for_stack(&[scope]);
    let mut style = Style::default();
    if let Some(fg) = modifier.foreground.and_then(super::convert_syntect_color) {
        style = style.fg(fg);
    }
    if let Some(font_style) = modifier.font_style {
        if font_style.contains(FontStyle::BOLD) {
            style = style.bold();
        }
        if font_style.contains(FontStyle::ITALIC) {
            style = style.italic();
        }
        if font_style.contains(FontStyle::UNDERLINE) {
            style = style.underlined();
        }
    }
    style
}

#[cfg(test)]
#[path = "highlight_composer_tests.rs"]
mod tests;
