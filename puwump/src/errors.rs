use std::{error::Error, fmt::Display};

pub type Result<T> = std::result::Result<T, PuwumpError>;

#[derive(Debug, Clone)]
pub enum PuwumpError {
    HomeNotFound,
    Ui(String),
    Rusqlite(String),
    UniqueViolation,
    PathCreation,
    DbRemoval,
    UuidParse,
    RowNotFound,
    ForeignKeyViolation,
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
            Self::UniqueViolation => write!(f, "Exercise already exists"),
            Self::ForeignKeyViolation => write!(f, "Foreign key doesnt exist"),
        }
    }
}

impl Error for PuwumpError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl From<rusqlite::Error> for PuwumpError {
    fn from(e: rusqlite::Error) -> Self {
        match e {
            rusqlite::Error::SqliteFailure(err, _) if err.code == rusqlite::ErrorCode::ConstraintViolation => PuwumpError::UniqueViolation,
            _ => PuwumpError::Rusqlite(e.to_string()),
        }
    }
}

impl From<eframe::Error> for PuwumpError {
    fn from(value: eframe::Error) -> Self {
        Self::Ui(value.to_string())
    }
}
