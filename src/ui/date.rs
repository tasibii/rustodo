use chrono::{Datelike, Local};
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn render_tag_date<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    const MTWRFSU: [&'static str; 7] = ["MON", "TUE", "WED", "THU", "FRI", "SAT", "SUN"];
    let current_time = Local::now();
    let clock = format!(
        "{} {}",
        MTWRFSU[current_time.weekday().number_from_monday() as usize].to_string(),
        current_time.format("%d/%m/%Y - %H:%M:%S").to_string()
    );
    let content = Spans::from(Span::styled(
        clock,
        Style::default().add_modifier(Modifier::BOLD),
    ));

    let dt = Paragraph::new(content.clone())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Datetime")
                .border_type(BorderType::Rounded),
        )
        .alignment(Alignment::Center);
    f.render_widget(dt, area);
}
