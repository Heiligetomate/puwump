use uuid::Uuid;

use crate::models::core::Model;

#[derive(Debug)]
pub struct Exercise {
    pub id: Uuid,
    pub name: String,
    pub instructions: String,
}

impl Model for Exercise {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        let id: String = row.get(0)?;
        Ok(Self {
            id: Uuid::parse_str(&id).map_err(|_| rusqlite::Error::InvalidQuery)?,
            instructions: row.get(1)?,
            name: row.get(2)?,
        })
    }
}
