use rusqlite::{Connection, Result, Error, Statement, ToSql};
use ratatui::widgets::Row;
use std::collections::HashMap;
use std::fmt;


#[derive(Debug)]
pub enum DBError {
    ConnectionCreationError(String),
    TableAlreadyExists(String),
    TableDoesNotExist(String),
    ColumnDoesNotExist(String),
    SqlError(Error),
}

impl fmt::Display for DBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DBError::ConnectionCreationError(name) => write!(f, "Can't open connection to database '{}'", name),
            DBError::TableAlreadyExists(table) => write!(f, "Table '{}' already exists", table),
            DBError::TableDoesNotExist(table) => write!(f, "Table '{}' does not exist", table),
            DBError::ColumnDoesNotExist(column) => write!(f, "Column '{}' does not exist", column),
            Self::SqlError(e) => write!(f, "SQL Error: {}", e),
        }
    }
}

impl From<Error> for DBError {
    fn from(err: Error) -> DBError {
        DBError::SqlError(err)
    }
}

pub struct DB {
    db_name: String,
    db_conn: Connection,
    db_tab_col_map: HashMap<String, Vec<String>>,
}

impl DB {
    pub fn new(name: String) -> Result<Self, DBError> {
        let db_string = name.to_owned() + ".db";
        let conn = Connection::open(&db_string).map_err(|text| DBError::ConnectionCreationError(text.to_string()))?;
        Ok(
            Self {
                db_name: name, 
                db_conn: conn, 
                db_tab_col_map: HashMap::new() 
        })
    }

    pub fn get_db_name(&self) -> String {
        self.db_name.clone()
    }

    pub fn get_table_list(&self) -> Result<Vec<String>> {
        let mut statement = self.db_conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name;")?;
        let table_iter = statement.query_map([], |row| {
            let table_name: String = row.get(0)?;
            Ok(table_name)
        })?;

        let mut rows = Vec::new();
        for table_name in table_iter {
            rows.push(table_name?);
        }

        Ok(rows)
    }

    pub fn get_table_columns(&self, table_name: &str) -> Result<Vec<(String, String)>, DBError> {
        let mut statement = self.db_conn.prepare(&format!(
            "PRAGMA table_info({})",
            table_name
        ))?;
        let column_iter = statement.query_map([], |row| {
            let column_name = row.get(1)?;
            let column_type = row.get(2)?;
            Ok((column_name, column_type))
        })?;
        let mut columns = Vec::new();
        for column in column_iter {
            columns.push(column?);
        }

        Ok(columns)
    }

    pub fn create_table(&mut self, table_name: String, columns: Vec<String>, constraints: Vec<String>) -> Result<(), DBError> {
        let _ = self.check_table_does_not_exist(table_name.clone());

        let col_names = self.col_names_from_sql(&columns);
        self.db_tab_col_map.insert(table_name.clone(), col_names);

        let columns_str = columns.join(", ");
        let constraints_str = constraints.join(", ");
        let table_contents = columns_str + ", " + &constraints_str;
        let sql = format!("CREATE TABLE IF NOT EXISTS {} ({})", table_name, table_contents);
        self.db_conn.execute(&sql, [])?;
        Ok(())
    }

    pub fn insert_statement(&mut self, table_name: String, columns: Vec<String>, values: Vec<&dyn ToSql>) -> Result<(), DBError> {
        let _ = self.check_table_exists(&table_name);
        let table_cols = self.db_tab_col_map.get(&table_name).unwrap();
        let _ = self.check_cols_match_existing(table_cols, &columns);

        let col_str = columns.join(", ");
        let placeholders = (0..columns.len()).map(|_| "?").collect::<Vec<_>>().join(", ");
        let sql = format!("INSERT INTO {} ({}) VALUES ({})", table_name, col_str, placeholders);
        self.db_conn.execute(&sql, values.as_slice())?;

        Ok(())
    }

    pub fn select_statement(&self, table_name: &String, cols: &Vec<String>) -> Result<Statement> {
        let _ = self.check_table_exists(&table_name);
        let table_cols = self.db_tab_col_map.get(table_name).unwrap();
        let _ = self.check_cols_match_existing(table_cols, cols);

        let col_str = cols.join(", ");
        let sql = format!("SELECT {} FROM {}", col_str, table_name);
        let res = self.db_conn.prepare(&sql)?;

        Ok(res)
    }

    fn check_tab_col_map_contains_table(&self, table_name: &String) -> bool {
        self.db_tab_col_map.contains_key(table_name)
    }

    fn check_table_does_not_exist(&self, table_name: String) -> Result<(), DBError> {
        if !self.check_tab_col_map_contains_table(&table_name) {
            Ok(())
        } else {
            Err(DBError::TableAlreadyExists(table_name))
        }
    }

    fn check_table_exists(&self, table_name: &String) -> Result<(), DBError> {
        if self.check_tab_col_map_contains_table(table_name) {
            Ok(())
        } else {
            Err(DBError::TableDoesNotExist(table_name.to_string()))
        }
    }

    fn check_cols_match_existing(&self, existing_cols: &Vec<String>, cols: &Vec<String>) -> Result<(), DBError> {
        for col in cols {
            if !existing_cols.contains(col) {
                return Err(DBError::ColumnDoesNotExist(col.to_string()));
            }
        }
        Ok(())
    }

    fn col_names_from_sql<'a>(&self, sql_str: &Vec<String>) -> Vec<String> {
        let mut col_names = Vec::new();
        for col_str in sql_str {
            let col_parts: Vec<_> = col_str.split(' ').collect();
            let col_name = col_parts[0];
            println!("Received col name {}", col_name);
            col_names.push(col_name.to_string());
        }
        col_names
    }
}



