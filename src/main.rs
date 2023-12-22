use crossterm::event::KeyCode;
use std::io::{self, Result};
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use rustodo::app::{App, InputMode};
use rustodo::event::{Event, Events};
use rustodo::ui::input::render_input;
use rustodo::ui::date::render_tag_date;
use rustodo::ui::help::{render_help, Help};
use rustodo::ui::list_task::render_list_task;
use rustodo::ui::pinned_task::render_pinned_task;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::Terminal;

fn main() -> Result<()> {
    let mut app = App::new();
    let mut help = Help::new();

    app.state.select(Some(0));
    let events = Events::new();
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    let mut cur_screen = String::from("tasks");

    // Start the main loop.
    while app.running {
        // Render the user interface.
        terminal.draw(|frame| match cur_screen.as_str() {
            "help" => {
                let rects = Layout::default()
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .split(frame.size());
                render_help(frame, &mut help, rects[0]);
            }
            _ => {
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
                render_pinned_task(frame, &app, mini_chunks[0]);
                render_tag_date(frame, mini_chunks[1]);
                render_input(frame, &app, chunks[2]);
            }
        })?;

        match events
            .next()
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err.to_string()))?
        {
            Event::Input(input) => match app.input_mode {
                InputMode::Normal => handle_normal_mode_input(&mut app, &mut help, input, &mut cur_screen),
                InputMode::Adding => handle_adding_mode_input(&mut app, input),
                InputMode::Editing => handle_editing_mode_input(&mut app, input),
            },
            Event::Tick => {}
        };
    }
    Ok(())
}

fn handle_normal_mode_input(app: &mut App, help: &mut Help, input: KeyCode, cur_screen: &mut String) {
    match input {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.running = false;
        }
        KeyCode::Down | KeyCode::Char('j') => match cur_screen.as_str() {
            "help" => {
                help.next();
            }
            _ => {
                if app.items.len() > 0 {
                    app.next();
                }
            }
        }
        KeyCode::Up | KeyCode::Char('k') => match cur_screen.as_str() {
            "help" => {
                help.previous();
            }
            _ => {
                if app.items.len() > 0 {
                    app.previous();
                }
            }
        }
        KeyCode::Char('a') => {
            app.input_mode = InputMode::Adding;
        }
        KeyCode::Char('m') => {
            let len = app.items.len();
            if len > 0 {
            app.input_mode = InputMode::Editing;
            let index = app.state.selected().unwrap();
            app.input = app.items[index].task.to_string().into();
            }
        }
        KeyCode::Char('c') => {
            app.toggle_complete(app.state.selected().unwrap());
            app.sync();
        }
        KeyCode::Char('p') => {
            if app.items.len() > 0 {
                app.toggle_pin(app.state.selected().unwrap());
            }
        }
        KeyCode::Char('d') => {
            let len = app.items.len();
            if len > 0 {
                app.delete_task(app.state.selected().unwrap());
                if app.state.selected().unwrap() == len - 1 && len != 1 {
                    app.state.select(Some(len - 2));
                }
                app.sync();
            }
        }
        KeyCode::Char('h') => match cur_screen.as_str() {
            "help" => {
                *cur_screen = String::from("tasks");
            }
            _ => {
                *cur_screen = String::from("help");
            }
        },
        _ => {}
    }
}

fn handle_adding_mode_input(app: &mut App, input: KeyCode) {
    match input {
        // Handle editing mode input
        KeyCode::Char(c) => {
            app.insert(c);
        }
        KeyCode::Backspace => {
            app.remove();
        }
        KeyCode::Enter => {
            app.submit();
            app.sync();
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
        }
        _ => {}
    }
}

fn handle_editing_mode_input(app: &mut App, input: KeyCode) {
    match input {
        KeyCode::Char(c) => {
            app.insert(c);
        }
        KeyCode::Backspace => {
            app.remove();
        }
        KeyCode::Enter => {
            app.update(app.state.selected().unwrap());
            app.sync();
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
        }
        _ => {}
    }
}
