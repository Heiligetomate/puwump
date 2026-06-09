use std::{fs::remove_file, path::PathBuf};

use rusqlite::Connection;

use crate::{
    errors::{PuwumpError, Result},
    util::{create_dirs_to_path, get_full_db_path},
};

pub const DB_LOCATION: &str = "~/.local/share/puwump/puwump.db";
pub const SQL_INIT: &str = include_str!("../../init.sql");

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
        self.con.execute_batch(SQL_INIT)?;

        Ok(())
    }

    pub fn delete(self) -> Result<()> {
        remove_file(&self.path).map_err(|_| PuwumpError::DbRemoval)?;

        Ok(())
    }

    pub fn reset(self) -> Result<Self> {
        self.delete()?;
        let db = Self::init()?;
        db.create()?;
        Ok(db)
    }
}
