use std::error;
use tui::widgets::TableState;
use tui_input::Input;
/// Application result type.

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum InputMode {
    Normal,
    Editing,
}

/// Application.
#[derive(Debug)]
pub struct App {
    pub state: TableState,
    pub items:Vec<Vec<String>>,
    pub current_task: Vec<String>,
    pub input: Input,
    pub mode: InputMode,
    
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: TableState::default(),
            items: vec![vec![
                String::from("GANG"),
                String::from("GANG"),
                String::from("GANG"),
            ]],
            current_task: vec![
                String::from("Hello\n"),
                String::from("Heyyo!\n"),
                String::from("MEMES\n"),
            ],
            input: Input::default(),
            mode: InputMode::Normal,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i > self.items.len() - 1 {
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
