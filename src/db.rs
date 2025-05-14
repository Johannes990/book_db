use rusqlite::{Connection, Result, Error, Statement, ToSql, types::ValueRef};
use std::collections::HashMap;
use std::fmt;
use crate::column::column_info::ColumnInfo;
use crate::row::row_info::RowInfo;


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
    pub db_conn: Connection,
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

    pub fn is_table_view(&self, table_name: &str) -> Result<bool> {
        let mut statement = self.db_conn.prepare(
            "SELECT type FROM sqlite_master WHERE name = ?"
        )?;
        let table_type: String = statement.query_row([table_name], |row| row.get(0))?;
        Ok(table_type == "view")
    }

    pub fn get_table_row_count(&self, table_name: &str) -> Result<u64> {
        let query = format!("SELECT COUNT(*) FROM {}", table_name);
        let mut statement = self.db_conn.prepare(&query)?;
        let count: u64 = statement.query_row([], |row| row.get(0))?;
        Ok(count)
    }

    pub fn get_table_rows(&self, table_name: &str) -> Result<Vec<RowInfo>, DBError> {
        let query = format!("SELECT * FROM {}", table_name);
        let mut statement = self.db_conn.prepare(&query)?;
        let column_count = statement.column_count();
        let rows = statement.query_map([], |row| {
            let mut values = Vec::new();
            for i in 0..column_count {
                let value = match row.get_ref(i)? {
                    ValueRef::Null => "NULL".to_string(),
                    ValueRef::Integer(v) => v.to_string(),
                    ValueRef::Real(v) => v.to_string(),
                    ValueRef::Text(v) => String::from_utf8_lossy(v).to_string(),
                    ValueRef::Blob(_) => "[BLOB]".to_string(),
                };
                values.push(value);
            }
            Ok(RowInfo {
                values
            })
        })?.collect::<Result<Vec<_>, _>>()?;

        Ok(rows)
    }

    pub fn get_table_columns(&self, table_name: &str) -> Result<Vec<ColumnInfo>, DBError> {
        let mut statement = self.db_conn.prepare(&format!(
            "PRAGMA table_info({})",
            table_name
        ))?;
        let mut columns = statement.query_map([], |row| {
            Ok(ColumnInfo { 
                name: row.get(1)?,
                col_type: row.get(2)?,
                is_pk: row.get::<_, i32>(5)? != 0, // checks if column has PK constraint
                is_fk: false,
                references_table: None,
                is_unique: false,
                is_not_null: row.get::<_, i32>(3)? != 0, // checks if column has NOT NULL constraint
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        
        // foreign key constraints
        let mut fk_statement = self.db_conn.prepare(&format!("
            PRAGMA foreign_key_list({})",
            table_name
        ))?;

        let foreign_keys: Vec<(String, String)> = fk_statement
            .query_map([], |row| {
                let from_col: String = row.get(3)?;
                let ref_table: String = row.get(2)?;
                Ok((from_col, ref_table))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        for col in &mut columns {
            if let Some((_, ref_table)) = foreign_keys
                .iter()
                .find(|(col_name, _)| col_name == &col.name) {
                col.is_fk = true;
                col.references_table = Some(ref_table.clone());
            }
        }

        // unique constraints
        let mut unique_statement = self.db_conn.prepare(&format!(
            "PRAGMA index_list({})",
            table_name
        ))?;

        let unique_indexes: Vec<String> = unique_statement
            .query_map([], |row| {
                let is_unique: i32 = row.get(2)?;
                if is_unique == 1 {
                    row.get(1) // column 1 contains index name
                } else {
                    Ok(String::new())
                }
            })?
            .filter_map(Result::ok)
            .filter(|s| !s.is_empty())
            .collect();

        for idx in unique_indexes {
            let mut index_info_statement = self.db_conn.prepare(&format!(
                "PRAGMA index_info({})",
                idx
            ))?;

            let unique_columns: Vec<String> = index_info_statement
                .query_map([], |row| row.get(2))?
                .collect::<Result<Vec<_>, _>>()?;

            for col in &mut columns {
                if unique_columns.contains(&col.name) {
                    col.is_unique = true;
                }
            }
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
