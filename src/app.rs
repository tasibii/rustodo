use crate::db::{connect, Todo};
use rusqlite::Connection;
use tui::widgets::TableState;
use tui_input::Input;
/// Application result type.

/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub state: TableState,
    pub connection: Connection,
    pub items: Vec<Todo>,
    pub current_task: String,
    pub input: Input,
    pub input_mode: InputMode,
}
#[derive(Debug)]
pub enum InputMode {
    Adding,
    Editing,
    Normal,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> App {
        let state = TableState::default();
        let connection = connect("database.sqlite").unwrap();
        let items = Todo::get(&connection).unwrap();
        let current_task = "".to_string();
        let input = Input::default();
        App {
            running: true,
            state,
            connection,
            items,
            current_task,
            input,
            input_mode: InputMode::Normal,
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

    pub fn sync(&mut self) {
        self.items = Todo::get(&self.connection).unwrap();
    }

    pub fn insert(&mut self, c: char) {
        self.input.handle(tui_input::InputRequest::InsertChar(c));
    }

    pub fn remove(&mut self) {
        self.input.handle(tui_input::InputRequest::DeletePrevChar);
    }

    pub fn toggle_complete(&mut self, index: usize) {
        let id = &self.items[index].id;
        let _ = Todo::toggle(&self.connection, *id).expect("Toggle complete failed");
    }

    pub fn toggle_pin(&mut self, index: usize) {
        if self.current_task == "".to_string() {
            self.current_task = self.items[index].task.to_string();
        } else {
            self.current_task = "".to_string();
        }
    }

    pub fn delete_task(&mut self, index: usize) {
        let id = &self.items[index].id;
        let _ = Todo::delete(&self.connection, *id).expect("Delete failed");
    }

    pub fn submit(&mut self) {
        let _ = Todo::new(&self.connection, self.input.value()).expect("Add failed");
        self.input = Input::default();
    }

    pub fn update(&mut self, index: usize) {
        let id = &self.items[index].id;
        let _ = Todo::update(&self.connection, *id, self.input.value()).expect("Update failed");
        self.input = Input::default();
    }
}
