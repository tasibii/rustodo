use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::{App, InputMode};

pub fn render_input<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    let style = if matches!(app.input_mode, InputMode::Editing | InputMode::Adding) {
        Style::default().fg(Color::LightGreen)
    } else {
        Style::default()
    };
    let input = Paragraph::new(app.input.value()).style(style).block(
        Block::default()
            .borders(Borders::ALL)
            .title("ADD & MODIFY")
            .border_type(BorderType::Rounded),
    );
    f.render_widget(input, area);
}
