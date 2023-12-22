use chrono::Local;
use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct Todo {
    pub id: usize,
    pub task: String,
    pub date: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(db: &Connection, task: &str) -> Result<()> {
        db.execute(
            "INSERT INTO todos (task, date, completed) VALUES (?1, ?2, 0)",
            &[task, &Local::now().format("%HH:%MM  %d/%m/%Y").to_string()],
        )?;
        Ok(())
    }

    pub fn delete(db: &Connection, id: usize) -> Result<()> {
        db.execute(
            "DELETE FROM todos WHERE id = ?1",
            &[id.to_string().as_str()],
        )?;
        Ok(())
    }

    pub fn get(db: &Connection) -> Result<Vec<Todo>> {
        let query = "SELECT id, task, date, completed FROM todos";
        let mut stmt = db.prepare(query)?;

        let todo_iter = stmt.query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                task: row.get(1)?,
                date: row.get(2)?,
                completed: row.get(3)?,
            })
        })?;
        let mut list = vec![];
        for todo in todo_iter {
            list.push(todo.unwrap());
        }
        Ok(list)
    }

    pub fn toggle(db: &Connection, id: usize) -> Result<()> {
        db.execute(
            "UPDATE todos SET completed = 1 - completed WHERE id = ?1",
            &[id.to_string().as_str()],
        )?;
        Ok(())
    }

    pub fn update(db: &Connection, id: usize, task: &str) -> Result<()> {
        db.execute(
            "UPDATE todos SET task=?1 WHERE id = ?2",
            &[&task, id.to_string().as_str()],
        )?;
        Ok(())
    }
}

pub fn connect(path: &str) -> Result<Connection> {
    let conn = Connection::open(path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            task TEXT NOT NULL,
            date TEXT NOT NULL,
            completed INTEGER NOT NULL
        )",
        [],
    )?;

    Ok(conn)
}
