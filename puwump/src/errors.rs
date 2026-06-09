use std::{error::Error, fmt::Display};

pub type Result<T> = std::result::Result<T, PuwumpError>;

#[derive(Debug, Clone)]
pub enum PuwumpError {
    HomeNotFound,
    Rusqlite(String),
    PathCreation,
    DbRemoval,
    UuidParse,
    RowNotFound,
    Ui(String),
}

impl Display for PuwumpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ui(e) => write!(f, "Ui error: {e}"),
            Self::Rusqlite(e) => write!(f, "rusqlite error: {e}"),
            Self::HomeNotFound => write!(f, "Home env not found"),
            Self::PathCreation => write!(f, "error while creating a path"),
            Self::DbRemoval => write!(f, "error while delting the db file"),
            Self::UuidParse => write!(f, "uuid parse error"),
            Self::RowNotFound => write!(f, "row not found"),
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

impl From<eframe::Error> for PuwumpError {
    fn from(value: eframe::Error) -> Self {
        Self::Ui(value.to_string())
    }
}
