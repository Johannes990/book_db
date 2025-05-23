mod domain;
mod db;
mod options;
mod ui;
mod app;
mod column;
mod table;
mod file_explorer;
mod row;
mod widgets;

use crossterm::{
    event::{KeyboardEnhancementFlags, PushKeyboardEnhancementFlags, PopKeyboardEnhancementFlags},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::{Backend, CrosstermBackend},
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
    events::handle_key_events,
};
use app::App;
use std::io;


fn main() -> io::Result<()> {
    let mut stdout = std::io::stdout();

    let _ = execute!(
        stdout,
        PushKeyboardEnhancementFlags(
            KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES |
            KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
        )
    );

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = setup_terminal(backend)?;
    let mut app = setup_app(&terminal, ColorScheme::Autumn)?;
    let res = app.run(&mut terminal);
    handle_errors(res);
    teardown_terminal(&mut terminal)?;

    let _ = execute!(stdout, PopKeyboardEnhancementFlags);

    Ok(())
}

fn setup_terminal<B>(mut backend: B) -> Result<Terminal<B>, io::Error>
where
    B: Backend + std::io::Write,
{
    enable_raw_mode()?;
    execute!(backend, EnterAlternateScreen)?;
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}

fn setup_app<B>(terminal: &Terminal<B>, color_scheme: ColorScheme) -> Result<App, io::Error> 
where 
    B: Backend,
{
    let terminal_height = terminal.size()?.height;
    let terminal_width = terminal.size()?.width;
    let app = App::new(color_scheme);

    Ok(app)
}

fn teardown_terminal<B>(terminal: &mut Terminal<B>) -> Result<(), io::Error>
where 
    B: Backend + std::io::Write,
{
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn handle_errors(res: io::Result<()>) {
    if let Err(err) = res {
        eprintln!("Error: {:?}", err)
    }
}

fn add_publisher(db: &mut DB, publisher: Publisher) -> Result<(), DBError> {
    db.insert_statement(
        "Publisher".to_string(),
        vec![
            "id".to_string(), 
            "name".to_string()
        ],
        vec![
            &publisher.id as &dyn ToSql, 
            &publisher.name
        ]
    )
}

fn get_publishers(db: &DB) -> rusqlite::Result<Vec<Publisher>> {
    let cols = vec![
            "id".to_string(), 
            "name".to_string()
        ];
    let mut publishers = db.select_statement(&"Publisher".to_string(), &cols)?;
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
        "Person".to_string(),
        vec![
            "id".to_string(), 
            "first_name".to_string(), 
            "middle_name".to_string(),
            "last_name".to_string()
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
        "id".to_string(), 
        "first_name".to_string(), 
        "middle_name".to_string(),
        "last_name".to_string()
    ];
    let mut persons = db.select_statement(&"Person".to_string(), &cols)?;
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
        "Book".to_string(),
        vec![
            "id".to_string(), 
            "title".to_string(), 
            "subtitle".to_string(),  
            "year_published".to_string(), 
            "year_translated".to_string(), 
            "publisher_id".to_string()
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
        "id".to_string(),
        "title".to_string(),
        "subtitle".to_string(),
        "year_published".to_string(),
        "year_translated".to_string(),
        "publisher_id.to_string()".to_string()
    ];
    let mut books = db.select_statement(&"Book".to_string(), &cols)?;
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
        "BookAuthor".to_string(), 
        vec![
            "book_id".to_string(), 
            "author_id".to_string()
        ], 
        vec![
            &book_author.book_id as &dyn ToSql,
            &book_author.person_id as &dyn ToSql
        ]
    )
}

fn add_book_and_translator(db: &mut DB, book_translator: BookPerson) -> Result<(), DBError> {
    db.insert_statement(
        "BookTranslator".to_string(),
        vec![
            "book_id".to_string(),
            "translator_id".to_string(),
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
