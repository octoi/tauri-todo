use rusqlite::Connection;

pub struct Todo {
    id: i32,
    user: i32,
    title: String,
    resolved: bool,
}

pub fn saveTodo(db: &Connection, title: String, uid: i32) -> Result<(), &'static str> {
    let user_id = format!("{}", uid);
    let resolved = format!("{}", false);

    match db.execute(
        "INSERT INTO todo (user, title, resolved) VALUES (?1, ?2, ?3)",
        &[&user_id, &title, &resolved],
    ) {
        Ok(_) => return Ok(()),
        Err(_) => {
            return Err("Failed to add todo");
        }
    }
}

pub fn read_all_todo(db: &Connection) -> Result<(), &'static str> {
    let mut sql_query = match db.prepare("SELECT * FROM todo") {
        Ok(query) => query,
        Err(_) => return Err("Failed to query todo"),
    };

    let todo_iter = match sql_query.query_map([], |row| {
        Ok(Todo {
            id: row.get(0)?,
            user: row.get(1)?,
            title: row.get(2)?,
            resolved: row.get(3)?,
        })
    }) {
        Ok(todo_iter) => todo_iter,
        Err(_) => return Err("Failed to query todo"),
    };

    let mut todo_vec: Vec<Todo> = Vec::new();

    for todo in todo_iter {
        match todo {
            Ok(todo) => todo_vec.push(todo),
            Err(_) => continue,
        }
    }

    Ok(())
}

pub fn update_todo(db: &Connection, id: i32, title: String, resolved: bool) -> Result<(), &'static str> {
    let id = format!("{}", id);
    let resolved = format!("{}", resolved);

    match db.execute("UPDATE todo SET title=(?1) resolved=(?2) WHERE id=(?3)", &[&title, &resolved, &id]) {
        Ok(_) => return Ok(()),
        Err(_) => return Err("Failed to update todo")
    };
}

pub fn delete_todo(db: &Connection, id: i32) -> Result<(), &'static str> {
    let id = format!("{}", id);

    match db.execute("DELETE FROM notes WHERE id=(?1)", &[&id]) {
        Ok(_) => return Ok(()),
        Err(_) => return Err("Failed to delete todo")
    };
}