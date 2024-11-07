mod domain;
mod db;
mod ui;
mod app;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    prelude::CrosstermBackend,
    Terminal,
};
use rusqlite::{
    Connection,
    Result,
    ToSql,
};
use domain::{
    book::Book,
    person::Person,
    bookperson::BookPerson,
    publisher::Publisher,
};
use db::{
    DB,
    DBError,
};
use ui::{
    colorscheme::ColorScheme,
    events::{handle_key_events, setup_keyboard_enchancements},
};
use app::App;

use std::io;

fn main() -> io::Result<()> {
    setup_keyboard_enchancements();
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let terminal_height = terminal.size()?.height;
    let terminal_width = terminal.size()?.width;
    let mut app = App::new(ColorScheme::Autumn, terminal_height, terminal_width);

    //let res = run(&mut terminal, &mut app);
    let res = app.run(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err)
    }

    Ok(())
}

fn add_publisher(db: &mut DB, publisher: Publisher) -> Result<(), DBError> {
    db.insert_statement(
        "Publisher",
        vec![
            "id", 
            "name"
        ],
        vec![
            &publisher.id as &dyn ToSql, 
            &publisher.name
        ]
    )
}

fn get_publishers(db: &DB) -> rusqlite::Result<Vec<Publisher>> {
    let cols = vec![
            "id", 
            "name"
        ];
    let mut publishers = db.select_statement("Publisher", &cols)?;
    let publisher_iter = publishers.query_map([], |row| {
        Ok(Publisher {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    let return_vec: Vec<Publisher> = publisher_iter.collect::<Result<_, _>>()?;
    Ok(return_vec)
}

fn add_person(db: &mut DB, person: Person) -> Result<(), DBError> {
    db.insert_statement(
        "Person",
        vec![
            "id", 
            "first_name", 
            "middle_name",
            "last_name"
        ],
        vec![
            &person.id as &dyn ToSql, 
            &person.first_name,
            &person.middle_name, 
            &person.last_name
        ]
    )
}

fn get_persons(db: &DB) -> rusqlite::Result<Vec<Person>> {
    let cols = vec![
        "id", 
        "first_name", 
        "middle_name",
        "last_name"
    ];
    let mut persons = db.select_statement("Person", &cols)?;
    let person_iter = persons.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            first_name: row.get(1)?,
            middle_name: row.get(2)?,
            last_name: row.get(3)?,
        })
    })?;

    let return_vec: Vec<Person> = person_iter.collect::<Result<_, _>>()?;
    Ok(return_vec)
}

fn add_book(db: &mut DB, book: Book) -> Result<(), DBError> {
    db.insert_statement(
        "Book",
        vec![
            "id", 
            "title", 
            "subtitle",  
            "year_published", 
            "year_translated", 
            "publisher_id"
        ],
        vec![
            &book.id as &dyn ToSql, 
            &book.title, 
            &book.subtitle,
            &book.year_published,
            &book.year_translated,
            &book.publisher_id as &dyn ToSql
        ]
    )
}

fn get_books(db: &DB) -> rusqlite::Result<Vec<Book>> {
    let cols = vec![
        "id",
        "title",
        "subtitle",
        "year_published",
        "year_translated",
        "publisher_id"
    ];
    let mut books = db.select_statement("Book", &cols)?;
    let mut book_iter = books.query_map([], |row| {
        Ok(Book {
            id: row.get(0)?,
            title: row.get(1)?,
            subtitle: row.get(2)?,
            year_published: row.get(3)?,
            year_translated: row.get(4)?,
            publisher_id: row.get(5)?,
        })
    })?;

    let return_vec: Vec<Book> = book_iter.collect::<Result<_, _>>()?;
    Ok(return_vec)
}

fn add_book_and_author(db: &mut DB, book_author: BookPerson) -> Result<(), DBError> {
    db.insert_statement(
        "BookAuthor", 
        vec![
            "book_id", 
            "author_id"
        ], 
        vec![
            &book_author.book_id as &dyn ToSql,
            &book_author.person_id as &dyn ToSql
        ]
    )
}

fn add_book_and_translator(db: &mut DB, book_translator: BookPerson) -> Result<(), DBError> {
    db.insert_statement(
        "BookTranslator",
        vec![
            "book_id",
            "translator_id",
        ],
        vec![
            &book_translator.book_id as &dyn ToSql,
            &book_translator.person_id as &dyn ToSql
        ]
    )
}

fn get_books_with_persons(conn: &Connection) -> rusqlite::Result<Vec<(Book, Person, Publisher)>> {
    let sql = "
        SELECT B.id, B.title, B.subtitle, B.year_published,
        B.year_translated, B.publisher_id, A.id,
        A.first_name, A.middle_name, A.last_name, P.id, P.name
        FROM Book AS B
        JOIN BookAuthor on B.id = BookAuthor.book_id
        JOIN Author AS A on A.id = BookAuthor.author_id
        JOIN Publisher AS P on P.id = B.publisher_id
    ";
    let mut stmt = conn.prepare(&sql)?;

    let book_persons_iter = stmt.query_map([], |row| {
        Ok((
            Book {
                id: row.get(0)?,
                title: row.get(1)?,
                subtitle: row.get(2)?,
                year_published: row.get(3)?,
                year_translated: row.get(4)?,
                publisher_id: row.get(5)?
            },
            Person {
                id: row.get(6)?,
                first_name: row.get(7)?,
                middle_name: row.get(7)?,
                last_name: row.get(9)?
            },
            Publisher {
                id: row.get(10)?,
                name: row.get(11)?,
            }
        ))
    })?;

    let return_vec: Vec<(Book, Person, Publisher)> = book_persons_iter.collect::<Result<_, _>>()?;
    Ok(return_vec)
}
