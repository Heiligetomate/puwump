use uuid::Uuid;

use crate::models::core::Model;

#[derive(Debug)]
pub struct Ingredient {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug)]
pub struct Meal {
    pub name: String,
    pub description: String,
    pub calories: u32,
}

#[derive(Debug)]
pub struct MealIngredientDetail {
    pub ingredient: Ingredient,
    pub amount_gr: u32,
}

impl Model for Ingredient {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        let id: String = row.get(1)?;
        Ok(Self {
            name: row.get(0)?,
            id: Uuid::parse_str(&id).map_err(|_| rusqlite::Error::InvalidQuery)?,
        })
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

impl Model for MealIngredientDetail {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        let id: String = row.get(1)?;
        Ok(Self {
            ingredient: Ingredient {
                name: row.get(0)?,
                id: Uuid::parse_str(&id).map_err(|_| rusqlite::Error::InvalidQuery)?,
            },
            amount_gr: row.get(1)?,
        })
    }
}
