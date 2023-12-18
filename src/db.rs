use chrono::prelude::*;
use rusqlite::{Connection, Result};
use colored::{Colorize, ColoredString};

pub struct Todo {
    pub id: i64,
    pub start_at: i64,
    pub duration: f32,
    pub title: String,
    pub detail: String,
    pub completed: bool,
}

impl Todo {
    pub fn add(db: &Connection, title: &str, duration: f32) -> Result<()> {
        db.execute(
            "INSERT INTO todo_list (title, detail, completed, start_at, duration) VALUES (?1, '', 0, ?2, ?3)",
            &[title, Utc::now().timestamp().to_string().as_str(), duration.to_string().as_str()],
        )?;
        Ok(())
    }

    pub fn delete(db: &Connection, id: i64) -> Result<()> {
        db.execute("DELETE FROM todo_list WHERE id = ?1", &[id.to_string().as_str()])?;
        Ok(())
    }

    pub fn get_all(db: &Connection) -> Result<()> {
        let query = "SELECT id, start_at, duration, title, completed FROM todo_list";
        let mut stmt = db.prepare(query)?;
    
        let todo_iter = stmt.query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                start_at: row.get(1)?,
                duration: row.get(2)?,
                title: row.get(3)?,
                detail: String::new(),
                completed: row.get(4)?,
            })
        })?;
    
        println!(
            "{}",
            "Color state display for certain cases: green indicating 'done', yellow for 'undone', and red for 'expired'"
                .blue()
        );
    
        for todo in todo_iter {
            let todo_temp = todo?;
            let completed = todo_temp.completed;
            let remaining_time =
                todo_temp.duration - (Utc::now().timestamp() - todo_temp.start_at) as f32 / 3600.0;
            let expired = remaining_time < 0.0;
    
            let (status, id_color, title_color): (ColoredString, fn(_) -> ColoredString, fn(_) -> ColoredString) = if completed {
                ("[x]".green(), Colorize::green, Colorize::green)
            } else if !completed && expired {
                ("[ ]".red(), Colorize::red, Colorize::red)
            } else {
                ("[ ]".yellow(), Colorize::yellow, Colorize::yellow)
            };

            let remaining_time_display = if completed {
                "".to_string()
            } else {
                format!("({:.3} h)", remaining_time)
            };
    
            println!(
                "{} {:0>3}: {} {}",
                status,
                id_color(todo_temp.id.to_string().as_str()),
                title_color(todo_temp.title.to_uppercase().as_str()),
                remaining_time_display
            );
        }
    
        Ok(())
    }

    pub fn toggle(db: &Connection, id: i64) -> Result<()> {
        db.execute(
            "UPDATE todo_list SET completed = 1 - completed WHERE id = ?1",
            &[id.to_string().as_str()],
        )?;
        Ok(())
    }

    pub fn update(db: &Connection, id: i64, title: &str, duration: f32) -> Result<()> {
        db.execute(
            "UPDATE todo_list SET title=?1, duration=?2 WHERE id = ?3",
            &[&title, duration.to_string().as_str(), id.to_string().as_str()],
        )?;
        Ok(())
    }

    pub fn report(db: &Connection, id: i64, detail: &str) -> Result<()> {
        db.execute(
            "UPDATE todo_list SET detail = ?1 WHERE id = ?2",
            &[detail, id.to_string().as_str()],
        )?;
        Ok(())
    }

    pub fn get_report(db: &Connection, id: i64) -> Result<()> {
        db.execute(
            "SELECT detail from todo_list WHERE id = ?1",
            &[id.to_string().as_str()],
        )?;
        Ok(())
    }
}

pub fn connect(path: &str) -> Result<Connection> {
    let conn = Connection::open(path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo_list (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            start_at INTEGER NOT NULL,
            duration INTEGER NOT NULL,
            title TEXT NOT NULL,
            detail TEXT,
            completed INTEGER NOT NULL
        )",
        [],
    )?;

    Ok(conn)
}
