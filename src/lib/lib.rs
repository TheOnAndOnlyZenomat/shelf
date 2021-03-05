#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod db;

use crossterm::{cursor, queue, style::Print, terminal, QueueableCommand};
use rusqlite::{params, Connection, NO_PARAMS};
use std::convert::TryInto;
use std::io::{stdout, Write};

pub enum Mode {
    Title,
    Author,
    Id,
}

pub struct Book {
    id: u8,
    name: String,
    author: String,
}

pub fn length_of_longest_from_book(list: &Vec<Book>, mode: Mode) -> usize {
    match mode {
        Mode::Title => {
            let mut longest = list[0].name.to_string().len();
            for item in list {
                if item.name.to_string().len() > longest {
                    longest = item.name.to_string().len()
                }
            }
            longest
        }
        Mode::Author => {
            let mut longest = list[0].author.to_string().len();
            for item in list {
                if item.author.to_string().len() > longest {
                    longest = item.author.to_string().len()
                }
            }
            longest
        }
        Mode::Id => {
            let mut longest = list[0].id.to_string().len();
            for item in list {
                if item.id.to_string().len() > longest {
                    longest = item.id.to_string().len()
                }
            }
            longest
        }
    }
}

pub fn render(conn: &Connection, books: Vec<Book>) {
    let mut stdout = stdout();
    let rows = db::get_rows_in_db(conn);
    let row_length: u16 = rows.to_string().len().try_into().unwrap_or(100);
    let spacing_value: u16 = 2;
    let id_spacing: u16 = 4 + row_length + spacing_value;
    let longest_title_length: u16 = length_of_longest_from_book(&books, Mode::Title)
        .try_into()
        .unwrap();
    let title_spacing: u16 = 7 + longest_title_length + spacing_value;
    queue!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0),
    )
    .expect("Could not queue renderpart");
    for book in books {
        stdout
            .queue(Print(format!("ID: {}", book.id)))
            .expect("Could not queue renderpart");
        stdout
            .queue(cursor::MoveToColumn(id_spacing + 1))
            .expect("Could not queue renderpart");
        stdout
            .queue(Print(format!("Title: {}", book.name)))
            .expect("Could not queue renderpart");
        stdout
            .queue(cursor::MoveToColumn(id_spacing + title_spacing + 1))
            .expect("Could not queue renderpart");
        stdout
            .queue(Print(format!("Author: {}", book.author)))
            .expect("Could not queue renderpart");
        stdout
            .queue(Print("\n"))
            .expect("Could not queue renderpart");
    }

    stdout.flush().expect("Could not flush queue to screen");
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
            match db::search_depends(conn, mode, search) {
                Ok(books) => {
                    render(conn, books);
                }
                Err(e) => println!("ERROR: {}", e),
            }
        }
        None => match db::show_all(conn) {
            Ok(books) => {
                render(conn, books);
            }
            Err(e) => println!("ERROR: {}", e),
        },
    };
}
