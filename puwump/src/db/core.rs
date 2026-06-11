use std::{fs::remove_file, path::PathBuf};

use rusqlite::Connection;

use crate::{
    errors::{PuwumpError, Result},
    util::{create_dirs_to_path, get_full_db_path},
};

pub const DB_LOCATION: &str = "~/.local/share/puwump/puwump.db";
const SQL_INIT: &str = include_str!("../../../init.sql");

pub struct Db {
    path: PathBuf,
    pub con: Connection,
}

impl Db {
    pub fn init() -> Result<Self> {
        let full: PathBuf = get_full_db_path()?;
        create_dirs_to_path(&full)?;
        let con = Connection::open(&full)?;
        con.pragma_update(None, "foreign_keys", "on")?;

        Ok(Self { path: full, con })
    }

    pub fn create(self) -> Result<Self> {
        self.con.execute_batch(SQL_INIT)?;

        Ok(self)
    }

    pub fn delete(self) -> Result<()> {
        remove_file(&self.path).map_err(|_| PuwumpError::DbRemoval)?;

        Ok(())
    }

    pub fn reset(self) -> Result<Self> {
        self.delete()?;
        let db = Self::init()?;
        db.create()
    }

    pub fn map_sqlite_err(e: rusqlite::Error) -> PuwumpError {
        match e {
            rusqlite::Error::SqliteFailure(e, _) => match e.extended_code {
                // https://www.sqlite.org/rescode.html#constraint_primarykey
                // https://www.sqlite.org/rescode.html#constraint_unique
                1555 | 2067 => PuwumpError::UniqueViolation,
                // https://www.sqlite.org/rescode.html#constraint_foreignkey
                787 => PuwumpError::ForeignKeyViolation,
                _ => PuwumpError::Rusqlite(e.to_string()),
            },
            _ => PuwumpError::Rusqlite(e.to_string()),
        }
    }
}
