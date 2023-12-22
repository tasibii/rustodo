use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Cell, Row, Table},
    Frame,
};

use crate::app::{App, InputMode};

pub fn render_list_task<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let focus_style = if matches!(app.input_mode, InputMode::Normal) {
        Style::default().fg(Color::LightGreen)
    } else {
        Style::default()
    };

    let selected_style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD);

    let widths = [
        Constraint::Percentage(10),
        Constraint::Percentage(50),
        Constraint::Percentage(30),
        Constraint::Percentage(10),
    ];

    let rows = app.items.iter().enumerate().map(|(index, todo)| {
        let cells = vec![
            Cell::from(index.to_string()),
            Cell::from(todo.task.clone()),
            Cell::from(todo.date.clone()),
            Cell::from(if todo.completed.clone() {
                "  ✅"
            } else {
                "  ❌"
            }),
        ];

        Row::new(cells)
            .style(Style::default().fg(Color::White))
            .bottom_margin(1)
    });

    // instantiate the table with the tasks provided in the task list
    let task_table = Table::new(rows)
        .header(
            Row::new(vec!["Index", "Name", "From Date", "Status"])
                .style(Style::default().add_modifier(Modifier::BOLD))
                .bottom_margin(1),
        )
        .block(
            Block::default()
                .style(focus_style)
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
