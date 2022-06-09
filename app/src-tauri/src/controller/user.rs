use crate::database::user::{get_user, login, sign_up};
use crate::init_database;

#[tauri::command]
pub fn sign_up_user(email: String, password: String) -> Result<i32, String> {
  let db = init_database()?;
  match sign_up(&db, email.clone(), password) {
    Ok(_) => {}
    Err(err) => return Err(String::from(err)),
  }

  match get_user(&db, email) {
    Ok(user) => return Ok(user.id),
    Err(err) => return Err(String::from(err)),
  }
}

#[tauri::command]
pub fn login_user(email: String, password: String) -> Result<i32, String> {
  let db = init_database()?;
  match login(&db, email.clone(), password) {
    Ok(_) => {}
    Err(err) => return Err(String::from(err)),
  }

  match get_user(&db, email) {
    Ok(user) => return Ok(user.id),
    Err(err) => return Err(String::from(err)),
  }
}
