mod domain;
mod db;

use domain::{book::Book, person::Person, bookperson::BookPerson, publisher::Publisher};
use rusqlite::{Connection, Result, Statement, ToSql};

fn main() -> Result<()> {
    let conn = Connection::open("book.db")?;

    let _res_create_person = create_table(
        &conn, 
        "Person", 
        vec![
            "id INTEGER PRIMARY KEY",
            "first_name TEXT NOT NULL",
            "middle_name TEXT",
            "last_name TEXT NOT NULL"
        ]
    );

    let _res_create_publisher = create_table(
        &conn,
        "Publisher",
        vec![
            "id INTEGER PRIMARY KEY",
            "name TEXT UNIQUE NOT NULL"
        ]
    );

    let _res_create_book = create_table(
        &conn,
        "Book",
        vec![
            "id INTEGER PRIMARY KEY",
            "title TEXT NOT NULL",
            "subtitle TEXT",
            "year_published INTEGER",
            "year_translated INTEGER",
            "publisher_id INTEGER",
            "FOREIGN KEY (publisher_id) REFERENCES Publisher(id)"
        ]
    );

    let _res_create_bookauthor = create_table(
        &conn, 
        "BookAuthor", 
        vec![
            "book_id INTEGER",
            "author_id INTEGER",
            "FOREIGN KEY (book_id) REFERENCES Book(id)",
            "FOREIGN KEY (author_id) REFERENCES Person(id)",
            "PRIMARY KEY (book_id, author_id)"
        ]
    );

    let _res_create_booktranslator = create_table(
        &conn,
        "BookTranslator",
        vec![
            "book_id INTEGER",
            "translator_id INTEGER",
            "FOREIGN KEY (book_id) REFERENCES Book(id)",
            "FOREIGN KEY (translator_id) REFERENCES Person(id)",
            "PRIMARY KEY (book_id, translator_id)"
        ]
    );

    let publishers = vec![
        Publisher { id: 1, name: "Ilmamaa".to_string() },
        Publisher { id: 2, name: "Tartu Ülikooli kirjastus".to_string() },
        Publisher { id: 3, name: "Vagabund".to_string() },
    ];

    for publisher in publishers {
        let _res_add_publisher = add_publisher(&conn, publisher);
    }

    let persons = vec![
        Person { id: 1, first_name: "Gottlob".to_string(), middle_name: "".to_string(), last_name: "Frege".to_string() },
        Person { id: 2, first_name: "Thomas".to_string(), middle_name: "S.".to_string(), last_name: "Kuhn".to_string() },
        Person { id: 3, first_name: "Peter".to_string(), middle_name: "L.".to_string(), last_name: "Berger".to_string() },
        Person { id: 4, first_name: "Thomas".to_string(), middle_name: "".to_string(), last_name: "Luckmann".to_string() },
        Person { id: 5, first_name: "Katre".to_string(), middle_name: "".to_string(), last_name: "Pärn".to_string() },
        Person { id: 6, first_name: "Mirjam".to_string(), middle_name: "".to_string(), last_name: "Parve".to_string() },
        Person { id: 7, first_name: "Ragne".to_string(), middle_name: "".to_string(), last_name: "Schults".to_string() },
        Person { id: 8, first_name: "Ruth".to_string(), middle_name: "".to_string(), last_name: "Lias".to_string() },
        Person { id: 9, first_name: "Piret".to_string(), middle_name: "".to_string(), last_name: "Kuusk".to_string() },
        Person { id: 10, first_name: "Oswald".to_string(), middle_name: "".to_string(), last_name: "Spengler".to_string() },
        Person { id: 11, first_name: "Mati".to_string(), middle_name: "".to_string(), last_name: "Sirkel".to_string() },
        Person { id: 12, first_name: "Katre".to_string(), middle_name: "".to_string(), last_name: "Ligi".to_string() },
        Person { id: 13, first_name: "Arthur".to_string(), middle_name: "".to_string(), last_name: "Schopenhauer".to_string() },
        Person { id: 14, first_name: "Toomas".to_string(), middle_name: "".to_string(), last_name: "Rosin".to_string() },
        Person { id: 15, first_name: "Leo".to_string(), middle_name: "".to_string(), last_name: "Tolstoi".to_string() },
        Person { id: 16, first_name: "Andri".to_string(), middle_name: "".to_string(), last_name: "Ksenofontov".to_string() },
        Person { id: 17, first_name: "Aristoteles".to_string(), middle_name: "".to_string(), last_name: "".to_string() },
        Person { id: 18, first_name: "Anne".to_string(), middle_name: "".to_string(), last_name: "Lill".to_string() },
        Person { id: 19, first_name: "Voltaire".to_string(), middle_name: "".to_string(), last_name: "".to_string() },
        Person { id: 20, first_name: "Katre".to_string(), middle_name: "".to_string(), last_name: "Talviste".to_string() },
        Person { id: 21, first_name: "Rene".to_string(), middle_name: "".to_string(), last_name: "Descartes".to_string() },
        Person { id: 22, first_name: "Andres".to_string(), middle_name: "".to_string(), last_name: "Raudsepp".to_string() },
        Person { id: 23, first_name: "Ezra".to_string(), middle_name: "".to_string(), last_name: "Pound".to_string() },
        Person { id: 24, first_name: "Urmas".to_string(), middle_name: "".to_string(), last_name: "Tõnisson".to_string() },
        Person { id: 25, first_name: "Udo".to_string(), middle_name: "".to_string(), last_name: "Uibo".to_string() },

    ];

    for person in persons {
        let _res_add_person = add_person(&conn, person);
    }

    let books = vec![
        Book { id: 1, title: "Lugemise Aabits".to_string(), subtitle: "".to_string(), year_published: 1951, year_translated: 2000, publisher_id: 3 },
        Book { id: 2, title: "Hinge Tundmused".to_string(), subtitle: "".to_string(), year_published: 1649, year_translated: 2014, publisher_id: 1 },
        Book { id: 3, title: "Traktaat Tolerantsusest".to_string(), subtitle: "".to_string(), year_published: 2000, year_translated: 2013, publisher_id: 1 },
        Book { id: 4, title: "Nikomachose Eetika".to_string(), subtitle: "".to_string(), year_published: 1890, year_translated: 2007, publisher_id: 1 },
        Book { id: 5, title: "Mis on Kunst?".to_string(), subtitle: "".to_string(), year_published: 1951, year_translated: 2014, publisher_id: 1 },
        Book { id: 6, title: "Maailm kui Tahe ja Kujutlus".to_string(), subtitle: "II Köide mis sisaldab täiendusi esimese köite neljale raamatule".to_string(), year_published: 1991, year_translated: 2018, publisher_id: 1 },
        Book { id: 7, title: "Maailm kui Tahe ja Kujutlus".to_string(), subtitle: "I köide neli raamatut ning Kanti filosoofia kriitikat sisaldav lisa".to_string(), year_published: 1991, year_translated: 2018, publisher_id: 1 },
        Book { id: 8, title: "Õhtumaa Allakäik".to_string(), subtitle: "I Köide kuju ja tegelikkus".to_string(), year_published: 1922, year_translated: 2012, publisher_id: 1 },
        Book { id: 9, title: "Õhtumaa Allakäik".to_string(), subtitle: "II köide maailma-ajaloolised perspektiivid".to_string(), year_published: 1922, year_translated: 2012, publisher_id: 1 },
        Book { id: 10, title: "Teadusrevolutsioonide Struktuur".to_string(), subtitle: "".to_string(), year_published: 1962, year_translated: 2003, publisher_id: 1 },
        Book { id: 11, title: "Tegelikkuse Sotsiaalne Ülesehitus".to_string(), subtitle: "Teadmussotsioloogiline uurimus".to_string(), year_published: 1991, year_translated: 2018, publisher_id: 1 },
        Book { id: 12, title: "Aritmeetika Alused".to_string(), subtitle: "Loogilis-matemaatiline uurimus arvu mõistest".to_string(), year_published: 1884, year_translated: 2014, publisher_id: 2 },
    ];

    for book in books {
        let _res_add_book = add_book(&conn, book);
    }

    let bookauthors = vec![
        BookPerson { book_id: 1, person_id: 23 },
        BookPerson { book_id: 2, person_id: 21 },
        BookPerson { book_id: 3, person_id: 19 },
        BookPerson { book_id: 4, person_id: 17 },
        BookPerson { book_id: 5, person_id: 15 },
        BookPerson { book_id: 6, person_id: 13 },
        BookPerson { book_id: 7, person_id: 13 },
        BookPerson { book_id: 8, person_id: 10 },
        BookPerson { book_id: 9, person_id: 10 },
        BookPerson { book_id: 10, person_id: 2 },
        BookPerson { book_id: 11, person_id: 3 },
        BookPerson { book_id: 12, person_id: 1 },
    ];

    for ba in bookauthors {
        let _res_add_ba = add_book_and_author(&conn, ba);
    }

    let booktranslators = vec![
        BookPerson { book_id: 12, person_id: 9 },
        BookPerson { book_id: 11, person_id: 5 },
        BookPerson { book_id: 11, person_id: 6 },
        BookPerson { book_id: 11, person_id: 7 },
        BookPerson { book_id: 10, person_id: 8 },
        BookPerson { book_id: 9, person_id: 11 },
        BookPerson { book_id: 9, person_id: 12 },
        BookPerson { book_id: 8, person_id: 11 },
        BookPerson { book_id: 8, person_id: 12 },
        BookPerson { book_id: 7, person_id: 14 },
        BookPerson { book_id: 6, person_id: 14 },
        BookPerson { book_id: 5, person_id: 16 },
        BookPerson { book_id: 4, person_id: 18 },
        BookPerson { book_id: 3, person_id: 20 },
        BookPerson { book_id: 2, person_id: 22 },
        BookPerson { book_id: 1, person_id: 24 },
        BookPerson { book_id: 1, person_id: 25 },
    ];

    for bt in booktranslators {
        println!("Trying to add {:?}", bt);
        let _res_add_bt = add_book_and_translator(&conn, bt);
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

fn add_person(conn: &Connection, person: Person) -> Result<()> {
    insert_command(
        conn,
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

fn get_persons(conn: &Connection) -> rusqlite::Result<Vec<Person>> {
    let mut persons = select_command(
        conn, 
        "Person", 
        vec![
            "id", 
            "first_name", 
            "middle_name",
            "last_name"
        ])?;
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

fn add_book(conn: &Connection, book: Book) -> Result<()> {
    insert_command(
        conn,
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

fn get_books(conn: &Connection) -> rusqlite::Result<Vec<Book>> {
    let mut books = select_command(
        conn, 
        "Book", vec![
            "id",
            "title",
            "subtitle",
            "year_published",
            "year_translated",
            "publisher_id"
        ])?;
    let mut book_iter = books.query_map([], |row| {
        Ok(Book {
            id: row.get(0)?,
            title: row.get(1)?,
            subtitle: row.get(2)?,
            year_published: row.get(4)?,
            year_translated: row.get(5)?,
            publisher_id: row.get(6)?,
        })
    })?;

    let return_vec: Vec<Book> = book_iter.collect::<Result<_, _>>()?;
    Ok(return_vec)
}

fn add_book_and_author(conn: &Connection, book_author: BookPerson) -> Result<()> {
    insert_command(
        conn, 
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

fn add_book_and_translator(conn: &Connection, book_translator: BookPerson) -> Result<()> {
    print!("INSIDE ADD_BOOK_AND_TRANSLATOR");
    insert_command(
        conn,
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
                year_published: row.get(4)?,
                year_translated: row.get(5)?,
                publisher_id: row.get(6)?
            },
            Person {
                id: row.get(7)?,
                first_name: row.get(8)?,
                middle_name: row.get(9)?,
                last_name: row.get(10)?
            },
            Publisher {
                id: row.get(11)?,
                name: row.get(12)?,
            }
        ))
    })?;

    let return_vec: Vec<(Book, Person, Publisher)> = book_persons_iter.collect::<Result<_, _>>()?;
    Ok(return_vec)
}
