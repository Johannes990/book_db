use rusqlite::{Connection, Result, Error, Statement, ToSql};
use std::collections::{HashMap, HashSet};
use std::fmt::{self, write};

#[derive(Debug)]
pub enum DBError {
    TableAlreadyExists(String),
    TableDoesNotExist(String),
    ColumnDoesNotExist(String),
    SqlError(Error),
}

impl fmt::Display for DBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
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

pub struct DB<'life> {
    pub db_name: &'life str,
    db_conn: Connection,
    db_table_column_map: HashMap<&'life str, Vec<&'life str>>,
}

impl<'life> DB<'life> {
    pub fn new(&mut self, name: &'life str) {
        self.db_name = name;
        self.db_table_column_map = HashMap::new();
    }

    pub fn create_conn(&mut self) -> Result<()> {
        let db_string = self.db_name.to_owned() + ".db";
        self.db_conn = Connection::open(db_string)?;
        Ok(())
    }

    pub fn create_table(&mut self, table_name: &'life str, columns: Vec<&'life str>) -> Result<(), DBError> {
        self.check_table_exists(table_name);

        let col_names = self.col_names_from_sql(&columns);
        self.db_table_column_map.insert(table_name, col_names);

        let columns_str = columns.join(", ");
        let sql = format!("CREATE TABLE IF NOT EXISTS {} ({})", table_name, columns_str);
        self.db_conn.execute(&sql, [])?;
        Ok(())
    }

    pub fn insert_row(&mut self, table_name: &'life str, columns: Vec<&'life str>, values: Vec<&dyn ToSql>) -> Result<(), DBError> {
        self.check_table_exists(table_name);
        let table_cols = self.db_table_column_map.get(table_name).unwrap();
        self.check_insertion_columns_exist(table_cols, &columns);

        Ok(())
    }

    fn check_table_exists(&self, table_name: &'life str) -> Result<(), DBError> {
        if self.db_table_column_map.contains_key(table_name) {
            Ok(())
        } else {
            Err(DBError::TableDoesNotExist(table_name.to_string()))
        }
    }

    fn check_insertion_columns_exist(&self, expected_cols: &Vec<&'life str>, cols: &Vec<&'life str>) -> Result<(), DBError> {
        for col in cols {
            if !expected_cols.contains(col) {
                return Err(DBError::ColumnDoesNotExist(col.to_string()));
            }
        }
        Ok(())
    }

    fn col_names_from_sql<'a>(&self, sql_str: &Vec<&'a str>) -> Vec<&'a str> {
        let mut col_names = Vec::new();
        for col_str in sql_str {
            let col_parts: Vec<_> = col_str.split(' ').collect();
            let col_name = col_parts[0];
            println!("Received col name {}", col_name);
            col_names.push(col_name);
        }
        col_names
    }
}



