extern crate shelf_lib;

use shelf_lib::Mode;

extern crate clap;
use clap::{App, Arg, SubCommand};

fn main() {
    let conn = shelf_lib::db::initialize_db().expect("Could not establish DB connection");

    let flag = App::new("Shelf")
        //.version("1.0")
        //.author("Kevin K. <kbknapp@gmail.com>")
        //.about("Does awesome things")
        .subcommand(
            SubCommand::with_name("search")
                .about("Search for books")
                .arg(
                    Arg::with_name("DEFAULT")
                        .help("Default searchterm is title. Is used if nothing specified")
                        .index(1),
                )
                .arg(
                    Arg::with_name("title")
                        .help("Title of the book, assumed as default when nothing specified")
                        .long("title")
                        .value_name("TITLE")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("author")
                        .help("Author of the book, optional")
                        .long("author")
                        .value_name("AUTHOR")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("id")
                        .help("ID of the book, optional")
                        .long("id")
                        .value_name("ID")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("Adds a book")
                .arg(
                    Arg::with_name("title")
                        .long("title")
                        .value_name("TITLE")
                        .help("Title of the book")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("author")
                        .long("author")
                        .value_name("AUTHOR")
                        .help("Author of the book")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("remove")
                .about("Removes a book, only specify one of the options")
                .arg(
                    Arg::with_name("title")
                        .long("title")
                        .value_name("TITLE")
                        .help("Title of the book")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("author")
                        .long("author")
                        .value_name("AUTHOR")
                        .help("Author of the book")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("id")
                        .long("id")
                        .value_name("ID")
                        .help("ID of the book")
                        .takes_value(true),
                ),
        )
        .get_matches();

    if let Some(flag) = flag.subcommand_matches("add") {
        shelf_lib::db::book_add(
            flag.value_of("title").expect("Title required").to_string(),
            flag.value_of("author")
                .expect("Author required")
                .to_string(),
            &conn,
        );
    } else if let Some(flag) = flag.subcommand_matches("remove") {
        if let Some(title) = flag.value_of("title") {
            shelf_lib::db::book_remove(&conn, Mode::Title, title.to_string());
        } else if let Some(author) = flag.value_of("author") {
            shelf_lib::db::book_remove(&conn, Mode::Author, author.to_string());
        } else if let Some(id) = flag.value_of("id") {
            shelf_lib::db::book_remove(&conn, Mode::Id, id.to_string());
        }
    } else if let Some(flag) = flag.subcommand_matches("search") {
        if let Some(title) = flag.value_of("DEFAULT") {
            shelf_lib::display(&conn, Mode::Title, title.to_string());
        } else if let Some(title) = flag.value_of("title") {
            shelf_lib::display(&conn, Mode::Title, title.to_string());
        } else if let Some(author) = flag.value_of("author") {
            shelf_lib::display(&conn, Mode::Author, author.to_string());
        } else if let Some(id) = flag.value_of("id") {
            shelf_lib::display(&conn, Mode::Id, id.to_string());
        } else {
            shelf_lib::render(
                &conn,
                shelf_lib::db::show_all(&conn).expect("Could not show all"),
            );
        }
    } else {
        shelf_lib::render(
            &conn,
            shelf_lib::db::show_all(&conn).expect("Could not show all"),
        );
    }
    shelf_lib::db::close_db(conn);
}
