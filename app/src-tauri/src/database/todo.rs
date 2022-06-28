use rusqlite::Connection;
use serde::Serialize;

#[derive(Serialize)]
pub struct Todo {
    pub id: i32,
    pub data: String,
}

pub fn create(db: &Connection, data: String) -> Result<(), String> {
    match db.execute("INSERT INTO todo (data) VALUES (?1)", &[&data]) {
        Ok(_) => return Ok(()),
        Err(_) => return Err(String::from("Failed to save data")),
    }
}

pub fn read(db: &Connection) -> Result<Vec<Todo>, String> {
    let mut todo_vec: Vec<Todo> = Vec::new();

    let mut sql_query = match db.prepare("SELECT * FROM todo") {
        Ok(query) => query,
        Err(_) => return Err(String::from("Failed to load todos")),
    };

    let todo_iter = match sql_query.query_map([], |row| {
        Ok(Todo {
            id: row.get(0)?,
            data: row.get(1)?,
        })
    }) {
        Ok(todo_iter) => todo_iter,
        Err(_) => return Err(String::from("Failed to load todos")),
    };

    for todo in todo_iter {
        match todo {
            Ok(todo_data) => todo_vec.push(todo_data),
            Err(_) => continue,
        }
    }

    Ok(todo_vec)
}

pub fn update(db: &Connection, id: i32, data: String) -> Result<(), String> {
    let id = format!("{}", id);

    match db.execute("UPDATE todo SET data=(?1) WHERE id=(?2)", &[&data, &id]) {
        Ok(_) => return Ok(()),
        Err(_) => return Err(String::from("Failed to update todo")),
    };
}

pub fn delete(db: &Connection, id: i32) -> Result<(), String> {
    let id = format!("{}", id);

    match db.execute("DELETE FROM todo WHERE id=(?1)", &[&id]) {
        Ok(_) => return Ok(()),
        Err(_) => return Err(String::from("Failed to delete todo")),
    };
}
