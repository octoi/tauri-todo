use rusqlite::{Connection, Result};
use std::path::Path;

pub fn connect_database<P: AsRef<Path>>(database_path: P) -> Result<Connection> {
    let conn = Connection::open(database_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS user (
      id integer primary key,
      email text not null unique,
      name text not null,
      password text not null
    )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
      id integer primary key,
      user text not null,
      title text not null unique,
      resolved text not null
    )",
        [],
    )?;

    Ok(conn)
}
