use rusqlite::{Connection, Result};
use std::path::Path;

pub fn connect_database<P: AsRef<Path>>(database_path: P) -> Result<Connection> {
    let conn = Connection::open(database_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
      id integer primary key,
      data text
    )",
        [],
    )?;

    Ok(conn)
}
