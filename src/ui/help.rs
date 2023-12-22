use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Cell, Row, Table, TableState},
    Frame,
};


pub struct Help<'a> {
    state: TableState,
    items: Vec<Vec<&'a str>>,
}

impl<'a> Help<'a> {
    pub fn new() -> Help<'a> {
        Help {
            state: TableState::default(),
            items: vec![
                vec!["", "k", "scroll up in ALL TASKS table"],
                vec!["", "j", "scroll down in ALL TASKS table"],
                vec!["", "a", "add a new task"],
                vec!["", "d", "delete a new task"],
                vec!["", "c", "toggle complete selected task"],
                vec!["", "m", "edit & modify selected task"],
                vec!["", "p", "toggle pin selected task"],
                vec!["", "q | ESC", "quit"],
                vec!["", "h", "toggle help menu"],
            ],
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub fn render_help<B>(f: &mut Frame<B>, help: &mut Help, area: Rect)
where
    B: Backend,
{
    let selected_style = Style::default()
        .bg(Color::White)
        .fg(Color::Black)
        .add_modifier(Modifier::BOLD);
    let normal_style = Style::default().fg(Color::White);
    let widths = [Constraint::Percentage(20), Constraint::Percentage(30), Constraint::Percentage(50)];
    let rows = help.items.iter().map(|i| {
        let cells = i.iter().map(|c| Cell::from(*c));
        // cells.pop;
        Row::new(cells).style(normal_style).bottom_margin(1)
    });

    // instantiate the table with the tasks provided in the task list
    let table = Table::new(rows)
        .header(
            Row::new(vec!["", "KEY", "DESCRIPTION"])
                .style(Style::default().add_modifier(Modifier::BOLD))
                .bottom_margin(1),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("HELP TABLE")
                .border_type(BorderType::Rounded),
        )
        .highlight_style(selected_style)
        .highlight_symbol(" ")
        .widths(&widths);

    f.render_stateful_widget(table, area, &mut help.state);
}
