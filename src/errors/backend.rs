use rusqlite::Error;
use serde::{Deserialize, Serialize};
use sqlparser::parser::ParserError;
use std::fmt;

pub enum BackendError {
    DB(DBError),
}

impl fmt::Display for BackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DB(err) => write!(f, "database error: {}", err),
        }
    }
}

impl fmt::Debug for BackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for BackendError {}

impl From<DBError> for BackendError {
    fn from(err: DBError) -> Self {
        Self::DB(err)
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DBError {
    ConnectionCreationError(String),
    NoDBInMemory,
    NoTableInMemory,
    TableAlreadyExists(String),
    TableDoesNotExist(String),
    ColumnDoesNotExist(String),
    CannotAddRow(String),
    CannotDeleteRow(String),
    SqlError(String),
    ParseError(String),
}

impl fmt::Display for DBError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DBError::ConnectionCreationError(name) => {
                write!(f, "unable to create connection to database <{}>", name)
            }
            DBError::NoDBInMemory => write!(f, "no database loaded into memory"),
            DBError::NoTableInMemory => write!(f, "no table loaded into memory"),
            DBError::TableAlreadyExists(table) => write!(f, "table <{}> already exists", table),
            DBError::TableDoesNotExist(err) => write!(f, "table <{}> does not exist", err),
            DBError::ColumnDoesNotExist(column) => write!(f, "column <{}> does not exist", column),
            DBError::CannotAddRow(row) => write!(f, "unable to add row <{}>", row),
            DBError::CannotDeleteRow(row) => write!(f, "unable to delete row <{}>", row),
            Self::SqlError(e) => write!(f, "sql error: {}", e),
            DBError::ParseError(e) => write!(f, "error while parsing sql statement: {}", e),
        }
    }
}

impl fmt::Debug for DBError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<Error> for DBError {
    fn from(err: Error) -> DBError {
        DBError::SqlError(err.to_string())
    }
}

impl From<ParserError> for DBError {
    fn from(err: ParserError) -> DBError {
        DBError::ParseError(err.to_string())
    }
}
