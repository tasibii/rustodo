use crossterm::event::{self, KeyCode};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub enum Event<I> {
    Input(I),
    Tick,
}

/// A small event handler that wrap crossterm input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
#[allow(dead_code)]
pub struct Events {
    rx: mpsc::Receiver<Event<KeyCode>>,
    input_handle: thread::JoinHandle<()>,
    tick_handle: thread::JoinHandle<()>,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub exit_key: KeyCode,
    pub tick_rate: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            exit_key: KeyCode::Char('q'),
            tick_rate: Duration::from_millis(250),
        }
    }
}

impl Events {
    pub fn new() -> Self {
        Self::with_config(Config::default())
    }

    pub fn with_config(config: Config) -> Self {
        let (tx, rx) = mpsc::channel();
        let input_handle = {
            let tx = tx.clone();
            thread::spawn(move || loop {
                if let event::Event::Key(key_event) = event::read().unwrap() {
                    if let Err(evt) = tx.send(Event::Input(key_event.code)) {
                        eprintln!("{}", evt);
                        return;
                    }
                    if key_event.code == config.exit_key {
                        return;
                    }
                }
            })
        };
        let tick_handle = {
            let tx = tx.clone();
            thread::spawn(move || loop {
                tx.send(Event::Tick).unwrap();
                thread::sleep(config.tick_rate);
            })
        };
        Self {
            rx,
            input_handle,
            tick_handle,
        }
    }

    pub fn next(&self) -> Result<Event<KeyCode>, mpsc::RecvError> {
        self.rx.recv()
    }
}
