use std::fmt;

pub enum NavigationError {
    RowInsertFormNavigation(String),
    RowDeleteFormNavigation(String),
    NewTableFormNavigation(String),
}

impl fmt::Display for NavigationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error while navigating ");
        match self {
            Self::RowInsertFormNavigation(msg) => write!(f, "row insert form: {}", msg),
            Self::RowDeleteFormNavigation(msg) => write!(f, "row delete form: {}", msg),
            Self::NewTableFormNavigation(msg) => write!(f, "new table form: {}", msg),
        }
    }
}

impl fmt::Debug for NavigationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for NavigationError {}
