use crate::database::init_database;
use crate::database::todo;

#[tauri::command]
pub fn create_todo(data: String) -> Result<(), String> {
    let db = init_database()?;
    todo::create(&db, data)?;
    Ok(())
}

#[tauri::command]
pub fn read_todos() -> Result<Vec<todo::Todo>, String> {
    let db = init_database()?;
    let todos = todo::read(&db)?;
    Ok(todos)
}

#[tauri::command]
pub fn update_todo(id: i32, data: String) -> Result<(), String> {
    let db = init_database()?;
    todo::update(&db, id, data)?;
    Ok(())
}

#[tauri::command]
pub fn delete_todo(id: i32) -> Result<(), String> {
    let db = init_database()?;
    todo::delete(&db, id)?;
    Ok(())
}
