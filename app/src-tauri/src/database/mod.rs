mod init;
mod todo;
mod user;

use rusqlite::Connection;

pub use init::connect_database;

pub fn init_database() -> Option<Connection> {
    match connect_database("todo.db") {
        Ok(conn) => Some(conn),
        Err(err) => {
            eprintln!("{}", err);
            None
        }
    }
}
