use std::fmt;

pub enum NavigationError {
    RowInsertFormNavigation(String),
    RowDeleteFormNavigation(String),
    #[allow(dead_code)]
    NewTableFormNavigation(String),
}

impl fmt::Display for NavigationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RowInsertFormNavigation(msg) => {
                write!(f, "row insert form navigation error: {}", msg)
            }
            Self::RowDeleteFormNavigation(msg) => {
                write!(f, "row delete form navigation error: {}", msg)
            }
            Self::NewTableFormNavigation(msg) => {
                write!(f, "new table form navigation error: {}", msg)
            }
        }
    }
}

impl fmt::Debug for NavigationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for NavigationError {}
