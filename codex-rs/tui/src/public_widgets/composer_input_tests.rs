use super::*;
use crossterm::event::KeyCode;
use pretty_assertions::assert_eq;
use ratatui::style::Color;

#[test]
fn changing_and_resetting_text_style_preserves_submission() {
    let mut input = ComposerInput::new();
    input.handle_paste("你好 👩‍💻\nsecond line".into());
    let area = Rect::new(
        /*x*/ 0, /*y*/ 0, /*width*/ 30, /*height*/ 6,
    );
    let mut original = Buffer::empty(area);
    input.render_ref(area, &mut original);
    let cursor = input.cursor_pos(area);
    let mut snapshots = Vec::new();
    for style in [
        Style::default().fg(Color::Green).italic().underlined(),
        Style::default().fg(Color::Magenta).bold(),
    ] {
        input.set_text_style(style);
        let mut styled = Buffer::empty(area);
        input.render_ref(area, &mut styled);
        assert_eq!(input.cursor_pos(area), cursor);
        snapshots.push(format!("{styled:?}"));
    }
    insta::assert_snapshot!(snapshots.join("\n\n"));
    input.reset_text_style();
    let mut reset = Buffer::empty(area);
    input.render_ref(area, &mut reset);
    assert_eq!(reset, original);
    let ComposerAction::Submitted(text) = input.input(KeyCode::Enter.into()) else {
        panic!("expected submission");
    };
    assert_eq!(text, "你好 👩‍💻\nsecond line");
    let mut empty = Buffer::empty(area);
    input.render_ref(area, &mut empty);
    input.set_text_style(Style::default().fg(Color::Green).bold());
    let mut placeholder = Buffer::empty(area);
    input.render_ref(area, &mut placeholder);
    assert_eq!(placeholder, empty);
}
