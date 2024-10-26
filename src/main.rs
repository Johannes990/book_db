mod book;
mod author;
mod publisher;
mod bookauthor;

use book::Book;
use author::Author;
use publisher::Publisher;
use bookauthor::BookAuthor;

use rusqlite::{Connection, Result, Statement, ToSql};


fn main() -> Result<()> {
    let conn = Connection::open("book.db")?;

    let publishers = get_publishers(&conn);

    let books = vec![
        Book {
            id: 1,
            title: "Teadusrevolutsioonide Struktuur".to_string(),
            subtitle: "".to_string(),
            translator: "Ruth Lias".to_string(),
            year_published: 1962,
            year_translated: 2003,
            publisher_id: 1
        },
        Book {
            id: 2,
            title: "Aritmeetika Alused".to_string(),
            subtitle: "Loogilis-matemaatiline uurimus arvu mõistest".to_string(),
            translator: "Piret Kuusk".to_string(),
            year_published: 1884,
            year_translated: 2014,
            publisher_id: 2
        },
        Book {
            id: 3,
            title: "Tegelikkuse Sotsiaalne Ülesehitus".to_string(),
            subtitle: "Teadmussotsioloogiline uurimus".to_string(),
            translator: "Katre Pärn, Mirjam Parve, Ragne Schults".to_string(),
            year_published: 1991,
            year_translated: 2018,
            publisher_id: 1
        }
    ];

    for book in books {
        let _res_add_book = add_book(&conn, book)?;
    }

    if let Ok(publisher) = publishers {
        println!("Found publisher: {:?}", publisher);
    }

    Ok(())
}

fn create_table(
    conn: &Connection, 
    table: &str, 
    columns: Vec<&str>
) -> Result<()> {
    let columns_str = columns.join(", ");
    let sql = format!("CREATE TABLE IF NOT EXISTS {} ({})", table, columns_str);

    conn.execute(&sql, [])?;

    Ok(())
}

fn insert_command(
    conn: &Connection,
    table: &str,
    columns: Vec<&str>,
    values: Vec<&dyn ToSql>,
) -> Result<()> {
    let columns_str = columns.join(", ");
    let placeholders = (0..columns.len()).map(|_| "?").collect::<Vec<_>>().join(", ");
    let sql = format!("INSERT INTO {} ({}) VALUES ({})", table, columns_str, placeholders);

    conn.execute(&sql, values.as_slice())?;

    Ok(())
}

fn select_command<'conn>(
    conn: &'conn Connection,
    table: &str,
    columns: Vec<&str>
) -> rusqlite::Result<Statement<'conn>> {
    let columns_str = columns.join(", ");
    let sql = format!("SELECT {} FROM {}", columns_str, table);

    let res = conn.prepare(&sql)?;

    Ok(res)
}

fn add_publisher(conn: &Connection, publisher: Publisher) -> Result<()> {
    insert_command(
        conn,
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

fn get_publishers(conn: &Connection) -> rusqlite::Result<Vec<Publisher>> {
    let mut publishers = select_command(
        conn, 
        "Publisher", 
        vec![
            "id", 
            "name"
        ])?;
    let publisher_iter = publishers.query_map([], |row| {
        Ok(Publisher {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    let return_vec: Vec<Publisher> = publisher_iter.collect::<Result<_, _>>()?;
    Ok(return_vec)
}

fn add_author(conn: &Connection, author: Author) -> Result<()> {
    insert_command(
        conn,
        "Author",
        vec![
            "id", 
            "first_name", 
            "last_name"
        ],
        vec![
            &author.id as &dyn ToSql, 
            &author.first_name, 
            &author.last_name
        ]
    )
}

fn get_authors(conn: &Connection) -> rusqlite::Result<Vec<Author>> {
    let mut authors = select_command(
        conn, 
        "Author", 
        vec![
            "id", 
            "first_name", 
            "last_name"
        ])?;
    let author_iter = authors.query_map([], |row| {
        Ok(Author {
            id: row.get(0)?,
            first_name: row.get(1)?,
            last_name: row.get(2)?,
        })
    })?;

    let return_vec: Vec<Author> = author_iter.collect::<Result<_, _>>()?;
    Ok(return_vec)
}

fn add_book(conn: &Connection, book: Book) -> Result<()> {
    insert_command(
        conn,
        "Book",
        vec![
            "id", 
            "title", 
            "subtitle", 
            "translator", 
            "year_published", 
            "year_translated", 
            "publisher_id"
        ],
        vec![
            &book.id as &dyn ToSql, 
            &book.title, 
            &book.subtitle,
            &book.translator,
            &book.year_published,
            &book.year_translated,
            &book.publisher_id as &dyn ToSql
        ]
    )
}

fn get_books(conn: &Connection) -> rusqlite::Result<Vec<Book>> {
    let mut books = select_command(
        conn, 
        "Book", vec![
            "id",
            "title",
            "subtitle",
            "translator",
            "year_published",
            "year_translated",
            "publisher_id"
        ])?;
    let mut book_iter = books.query_map([], |row| {
        Ok(Book {
            id: row.get(0)?,
            title: row.get(1)?,
            subtitle: row.get(2)?,
            translator: row.get(3)?,
            year_published: row.get(4)?,
            year_translated: row.get(5)?,
            publisher_id: row.get(6)?,
        })
    })?;

    let return_vec: Vec<Book> = book_iter.collect::<Result<_, _>>()?;
    Ok(return_vec)
}

fn add_book_and_author(conn: &Connection, book: Book, author: Author) -> Result<()> {
    insert_command(
        conn, 
        "BookAuthor", 
        vec![
            "book_id", 
            "author_id"
        ], 
        vec![
            &book.id as &dyn ToSql,
            &author.id as &dyn ToSql
        ]
    )
}

fn get_books_with_authors(conn: &Connection) -> rusqlite::Result<Vec<BookAuthor>> {
    let mut book_authors = select_command(
        conn, 
        "BookAuthor",
        vec![
            "book_id",
            "author_id"
        ])?;
    let book_authors_iter = book_authors.query_map([], |row| {
        Ok(BookAuthor {
            book_id: row.get(0)?,
            author_id: row.get(1)?
        })
    })?;

    let return_vec: Vec<BookAuthor> = book_authors_iter.collect::<Result<_, _>>()?;
    Ok(return_vec)
}
