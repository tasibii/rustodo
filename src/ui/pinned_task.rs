use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

// Draw the current task that has been selected.
pub fn render_pinned_task<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    let current = Spans::from(Span::styled(
        &app.current_task,
        Style::default()
            .bg(Color::LightGreen)
            .add_modifier(Modifier::BOLD),
    ));
    let task_paragraph = Paragraph::new(current.clone())
        .block(
            Block::default()
                .title("PINNED TASK")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(task_paragraph, area);
}
