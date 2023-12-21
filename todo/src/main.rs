use termion::event::Key;
use todo::app::App;
use todo::event::{Event, Events};
use todo::ui::date::render_tag_date;
use todo::ui::input::render_input;
use todo::ui::list_task::render_list_task;
use todo::ui::current_task::render_current_task;
use std::io::{Result, self};
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Layout, Direction};
use tui::Terminal;

fn main() -> Result<()> {
    // Create an application.
    let mut app = App::new();
    let events = Events::new();
    // Terminal initialization for UI
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // Start the main loop.
    loop {
        // Render the user interface.
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(15),
                        Constraint::Percentage(70),
                        Constraint::Percentage(15),
                    ]
                    .as_ref(),
                )
                .split(frame.size());
            let mini_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
                .split(chunks[0]);
            // draw_gauge(&mut f, &app, chunks[2]);
            render_list_task(frame, &mut app, chunks[1]);
            render_current_task(frame, &app, mini_chunks[0]);
            render_tag_date(frame, mini_chunks[1]);
            render_input(frame, &app, chunks[2]);
        })?;


        match events.next().map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err.to_string()))? {
            Event::Input(input) => match input {
                // Exit application on `ESC` or `q`
                Key::Esc | Key::Char('q') => {
                    break;
                }
                // Exit application on `Ctrl-C`
                Key::Down | Key::Char('j') => {
                    app.next();
                },

                Key::Up | Key::Char('k') => {
                    app.previous();
                },

                Key::Char('a') => {
                    app.
                }
                _ => {}
            },
            Event::Tick => {},
        };
    }
    Ok(())
}
