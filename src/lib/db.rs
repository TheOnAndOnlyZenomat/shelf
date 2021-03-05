use crate::{Book, Mode};

use rusqlite::{params, Connection, Result, NO_PARAMS};

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

pub fn search_depends(
    conn: &Connection,
    mode: Mode,
    search: String,
) -> Result<Vec<Book>, rusqlite::Error> {
    let mut stmt;
    match mode {
        Mode::Title => stmt = conn.prepare("SELECT * FROM books WHERE name = ?;")?,
        Mode::Author => stmt = conn.prepare("SELECT * FROM books WHERE author = ?;")?,
        Mode::Id => stmt = conn.prepare("SELECT * FROM books WHERE id = ?;")?,
    };

    let mut rows = stmt.query(params![search])?;

    let mut books = Vec::new();
    while let Some(rows) = rows.next()? {
        let book = Book {
            id: rows.get(0).unwrap(), //safe to use unwrap, because there will always be an ID
            name: rows.get(1).unwrap_or("No ID title".to_string()),
            author: rows.get(2).unwrap_or("No ID author".to_string()),
        };
        books.push(book);
    }

    Ok(books)
}

pub fn show_all(conn: &Connection) -> Result<Vec<Book>, rusqlite::Error> {
    println!("Showing all\n");
    let mut stmt = conn.prepare("SELECT * FROM books ORDER BY name ASC;")?;

    let mut rows = stmt.query(NO_PARAMS)?;

    let mut books = Vec::new();
    while let Some(rows) = rows.next()? {
        let book = Book {
            id: rows.get(0).unwrap(), //safe to use unwrap, because there will always be an ID
            name: rows.get(1).unwrap_or("No ID found".to_string()),
            author: rows.get(2).unwrap_or("No ID found".to_string()),
        };
        books.push(book);
    }

    Ok(books)
}

pub fn get_rows_in_db(conn: &Connection) -> u8 {
    match conn.query_row(
        "SELECT * FROM books WHERE id = (SELECT MAX(id)  FROM books);",
        NO_PARAMS,
        |row| row.get(0),
    ) {
        Ok(row) => row,
        Err(e) if e.to_string() == "Query returned no rows".to_string() => 0,
        Err(e) => panic!("Error: {}", e),
    }
}

pub fn book_add(name: String, author: String, conn: &Connection) {
    let book = Book {
        id: get_rows_in_db(conn) + 1,
        name: name,
        author: author,
    };
    println!(
        "Adding the book \"{}\" with author \"{}\" with DB id of {}",
        book.name, book.author, book.id
    );
    conn.execute(
        "INSERT INTO books (id, name, author) VALUES (?1, ?2, ?3);",
        params![book.id, book.name, book.author],
    )
    .expect("Could not insert book");
}

pub fn book_remove(conn: &Connection, mode: Mode, searchterm: String) {
    let conn = conn;
    match mode {
        Mode::Title => conn
            .execute("DELETE FROM books WHERE name = ?;", params![searchterm])
            .expect("Could not delete item by name from DB"),
        Mode::Author => conn
            .execute("DELETE FROM books WHERE author = ?;", params![searchterm])
            .expect("Could not delete item by author from DB"),
        Mode::Id => conn
            .execute("DELETE FROM books WHERE id = ?;", params![searchterm])
            .expect("Could not delete item by id from DB"),
    };
}
