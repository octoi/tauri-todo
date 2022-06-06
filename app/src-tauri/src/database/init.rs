use rusqlite::{Connection, Result};
use std::path::Path;

pub fn init_database<P: AsRef<Path>>(database_path: P) -> Result<()> {
    let conn = Connection::open(database_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS user (
      id integer primary key,
      email text not null unique,
      password text not null,
      name text not null
    )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
      id integer primary key,
      title text not null unique,
      resolved integer not null,
      user integer not null
    )",
        [],
    )?;

    Ok(())
}
