#![allow(unused_variables)]

pub mod db;

use rusqlite::{params, Connection, NO_PARAMS};

pub struct Book {
    id: u8,
    name: String,
    author: String,
}

pub fn get_rows_in_db(conn: &Connection) -> u8 {
    match conn.query_row("SELECT count(*) FROM books;", NO_PARAMS, |row| row.get(0)) {
        Ok(rows) => rows,
        Err(e) => panic!("ERROR: {}", e),
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

pub fn book_remove(name: String) {
    println!("Removing the book: {}", name);
}

pub fn display(conn: &Connection, searchparams: Option<(String, String)>) {
    let mode;
    let search;
    match searchparams {
        Some(s) => {
            mode = s.0;
            search = s.1;
            match search_depends(conn, mode, search) {
                Ok(books) => {
                    for book in books.iter() {
                        println!(
                            "ID: {}, Title: {}, Author: {}\n",
                            book.id, book.name, book.author
                        )
                    }
                }
                Err(e) => println!("ERROR: {}", e),
            }
        }
        None => match show_all(conn) {
            Ok(books) => {
                for book in books.iter() {
                    println!(
                        "ID: {}, Title: {}, Author: {}\n",
                        book.id, book.name, book.author
                    )
                }
            }
            Err(e) => println!("ERROR: {}", e),
        },
    };
}

pub fn search_depends(
    conn: &Connection,
    mode: String,
    search: String,
) -> Result<Vec<Book>, rusqlite::Error> {
    let mut stmt;
    match mode.as_str() {
        "title" => stmt = conn.prepare("SELECT * FROM books WHERE name = ?;")?,
        "author" => stmt = conn.prepare("SELECT * FROM books WHERE author = ?;")?,
        "id" => stmt = conn.prepare("SELECT * FROM books WHERE id = ?;")?,
        _ => stmt = conn.prepare("SELECT * FROM books ;")?,
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
    println!("Showing all");
    let mut stmt = conn.prepare("SELECT * FROM books;")?;

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
