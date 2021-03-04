use rusqlite::{params, Connection, Result};

pub fn initialize_db() -> Result<Connection> {
    let db_path = String::from("shelf.db");
    let conn = Connection::open(&db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS books (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  author          TEXT NOT NULL
                  )",
        params![],
    )?;
    Ok(conn)
}

pub fn close_db(conn: Connection) {
    conn.close().expect("DB connection could not be closed");
}
