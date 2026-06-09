use std::{error::Error, fmt::Display};

pub type Result<T> = std::result::Result<T, PuwumpError>;

#[derive(Debug, Clone)]
pub enum PuwumpError {
    HomeNotFound,
    Rusqlite(String),
    PathCreation,
    DbRemoval,
}

impl Display for PuwumpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rusqlite(e) => write!(f, "rusqlite error: {e}"),
            Self::HomeNotFound => write!(f, "Home env not found"),
            Self::PathCreation => write!(f, "error while creating a path"),
            Self::DbRemoval => write!(f, "error while delting the db file"),
        }
    }
}

impl Error for PuwumpError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl From<rusqlite::Error> for PuwumpError {
    fn from(value: rusqlite::Error) -> Self {
        Self::Rusqlite(value.to_string())
    }
}
