mod db;
mod cmd;
use db::{connect, Todo};
use cmd::{Command, StructOpt};

fn main() {
    let db = connect("database.sqlite").expect("Error: failed to initialize database");

    let opt = Command::from_args();
    match opt {
        Command::Add { text, duration } => {
            let _ = Todo::add(&db, text.as_str(), duration).expect("Failed to add task");
        }
        Command::List => {
            let _ = Todo::get_all(&db).expect("Failed to get all");
        }
        Command::Update { id, text, duration } => {
            let _ = Todo::update(&db, id, text.as_str(), duration ).expect("Failed to update");
        }
        Command::Delete { id } => {
            let _ = Todo::delete( &db, id).expect("Failed to delete task");
        }
        Command::Done { id } => {
            let _ = Todo::toggle(&db, id).expect("Failed to done task");
        }
        Command::Undone { id } => {
            let _ = Todo::toggle(&db, id).expect("Failed to undone task");
        }
        Command::GetReport { id } => {
            let _ = Todo::get_report(&db, id).expect("Failed to get report");
        }
        Command::Report { id, detail } => {
            let _ = Todo::report(&db, id, detail.as_str()).expect("Failed to report");
        }
    }
}
