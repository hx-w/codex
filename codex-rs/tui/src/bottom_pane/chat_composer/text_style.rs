//! Render-only base styling. Elements and search highlights still render above
//! this style; placeholders, masked input, prompts and footers keep their styles.

use super::ChatComposer;
use ratatui::style::Style;

impl ChatComposer {
    pub(super) fn input_text_style(&self) -> Style {
        if self.draft.textarea.is_empty() {
            Style::default()
        } else {
            self.text_style
                .unwrap_or_else(crate::render::highlight::composer_input_style)
        }
    }
}
