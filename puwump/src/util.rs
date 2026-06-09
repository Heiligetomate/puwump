use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

use crate::{
    db::DB_LOCATION,
    errors::{PuwumpError, Result},
};

pub fn get_home_dir() -> Result<String> {
    std::env::var("HOME").map_err(|_| PuwumpError::HomeNotFound)
}

pub fn get_full_db_path() -> Result<PathBuf> {
    let raw = DB_LOCATION.replacen("~", &get_home_dir()?, 1);
    Ok(PathBuf::from(raw))
}

pub fn create_dirs_to_path(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        create_dir_all(parent).map_err(|_| PuwumpError::PathCreation)?;
    }

    Ok(())
}
