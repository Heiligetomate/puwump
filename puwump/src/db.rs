use std::{fs::remove_file, path::PathBuf};

use rusqlite::Connection;

use crate::{
    errors::{PuwumpError, Result},
    util::{create_dirs_to_path, get_full_db_path},
};

pub const DB_LOCATION: &str = "~/.local/share/puwump/puwump.db";

pub struct Db {
    path: PathBuf,
    con: Connection,
}

impl Db {
    pub fn init() -> Result<Self> {
        let full: PathBuf = get_full_db_path()?;
        create_dirs_to_path(&full)?;

        let con = Connection::open(&full)?;
        Ok(Self { path: full, con })
    }

    pub fn create(&self) -> Result<()> {
        let _ = self.con.execute(
            "CREATE TABLE IF NOT EXISTS person (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            data BLOB
            )",
            (),
        )?;

        Ok(())
    }

    pub fn delete(self) -> Result<()> {
        remove_file(&self.path).map_err(|_| PuwumpError::DbRemoval)?;

        Ok(())
    }
}
