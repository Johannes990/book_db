use crate::errors::{
    backend::{BackendError, DBError},
    navigation::NavigationError,
};
use std::fmt;

pub enum AppError {
    Backend(BackendError),
    Navigation(NavigationError),
    InvalidHandle(String),
    Io(std::io::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Backend(err) => write!(f, "Backend error: {}", err),
            Self::Navigation(err) => write!(f, "Navigation error: {}", err),
            Self::InvalidHandle(hnd) => write!(f, "Invalid handle to: {}", hnd),
            Self::Io(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for AppError {}

impl From<BackendError> for AppError {
    fn from(err: BackendError) -> Self {
        Self::Backend(err)
    }
}

impl From<NavigationError> for AppError {
    fn from(err: NavigationError) -> Self {
        Self::Navigation(err)
    }
}

impl From<DBError> for AppError {
    fn from(err: DBError) -> Self {
        Self::Backend(BackendError::DB(err))
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
