mod init;
pub mod todo;
pub mod user;

use rusqlite::Connection;

pub use init::connect_database;

pub fn init_database() -> Result<Connection, String> {
    match connect_database("todo.db") {
        Ok(conn) => Ok(conn),
        Err(err) => {
            eprintln!("{}", err);
            Err(String::from("Failed to connect database"))
        }
    }
}
