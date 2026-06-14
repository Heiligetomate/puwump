use uuid::Uuid;

use crate::models::core::Model;

#[derive(Debug, Clone, PartialEq)]
pub struct Plan {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub est_mins: u32,
}

impl Model for Plan {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        let id: String = row.get(0)?;
        Ok(Self {
            id: Uuid::parse_str(&id).map_err(|_| rusqlite::Error::InvalidQuery)?,
            name: row.get(1)?,
            description: row.get(2)?,
            est_mins: row.get(3)?,
        })
    }
}
