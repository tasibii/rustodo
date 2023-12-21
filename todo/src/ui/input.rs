use tui::{
    layout::Rect,
    style::{Color, Style}, 
    Frame, backend::Backend,
    widgets::{Block, Borders, BorderType, Paragraph},
};

use crate::app::{App, InputMode};

pub fn render_input<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    let input = Paragraph::new(app.input.value())
        .style(match app.mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("ADD & MODIFY")
                .border_type(BorderType::Rounded)
        );
    f.render_widget(input, area);
}