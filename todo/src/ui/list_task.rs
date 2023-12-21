use tui::{
    layout::{ Rect, Constraint},
    style::{Color, Style, Modifier},
    widgets::{Block, BorderType, Borders, Row, Table, Cell},
    Frame, backend::Backend,
};

use crate::app::App;

pub fn render_list_task<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let selected_style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD);
    let normal_style = Style::default().fg(Color::White);
    let widths = [
        Constraint::Percentage(20),
        Constraint::Percentage(50),
        Constraint::Percentage(30),
    ];
    
    let rows = app.items.iter().map(|i| {
        let cells = i.iter().map(|c| {
            let x = c.clone();
            Cell::from(x)
        });
        Row::new(cells).style(normal_style)
    });

    // instantiate the table with the tasks provided in the task list
    let task_table = Table::new(rows)
        .header(
            Row::new(vec!["Tag", "Name", "Due Date"])
                .style(Style::default().add_modifier(Modifier::BOLD))
                .bottom_margin(1),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("ALL TASKS")
                .border_type(BorderType::Rounded),
        )
        .highlight_symbol(">> ")
        .highlight_style(selected_style)
        .widths(&widths)
        .column_spacing(1);

    f.render_stateful_widget(task_table, area, &mut app.state);
}