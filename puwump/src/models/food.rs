use crate::models::core::Model;

#[derive(Debug)]
pub struct Ingredient {
    pub name: String,
}

#[derive(Debug)]
pub struct Meal {
    pub name: String,
    pub description: String,
    pub calories: u32,
}

impl Model for Ingredient {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Self { name: row.get(0)? })
    }
}

impl Model for Meal {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Self {
            name: row.get(0)?,
            calories: row.get(1)?,
            description: row.get(2)?,
        })
    }
}
