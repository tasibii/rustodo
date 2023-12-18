pub use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Command {
    Add {
        text: String,
        duration: f32,
    },
    List,
    Update {
        id: i64,
        text: String,
        duration: f32,
    },
    Delete {
        id: i64,
    },
    Done {
        id: i64,
    },
    Undone {
        id: i64,
    },
    Report {
        id: i64,
        detail: String,
    },
    GetReport {
        id: i64,
    }
}
