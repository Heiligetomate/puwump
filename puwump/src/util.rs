use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use rusqlite::Statement;
use uuid::Uuid;

use crate::errors::{PuwumpError, Result};

pub fn get_full_db_path() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("io", "puwump", "puwump").ok_or(PuwumpError::HomeNotFound)?;

    Ok(proj_dirs
        .data_local_dir()
        .join("puwump.db"))
}

pub fn create_dirs_to_path(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        create_dir_all(parent).map_err(|_| PuwumpError::PathCreation)?;
    }

    Ok(())
}

pub fn ids_from_statement(mut stmt: Statement) -> Result<Vec<Uuid>> {
    let ids = stmt
        .query_map([], |row| row.get(0))?
        .collect::<rusqlite::Result<Vec<String>>>()?;

    ids.iter()
        .map(|id| Uuid::parse_str(id).map_err(|_| PuwumpError::UuidParse))
        .collect()
}
