use crate::{
    db::Db,
    errors::Result,
    models::{CardAdd, card_compatible::CardCrud, core::Model},
};

use uuid::Uuid;

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
            amount_gr: row.get(2)?,
        })
    }
}

impl CardAdd for Ingredient {
    fn title(&self) -> &str {
        &self.name
    }

    fn body(&self) -> Option<&str> {
        None
    }

    fn key(&self) -> Uuid {
        self.id
    }
}

impl CardCrud for Ingredient {
    fn get_all(db: &Db) -> Result<Vec<Self>> {
        db.get_all_ingredients()
    }

    fn insert(db: &Db, values: &[super::card_compatible::InputField]) -> Result<()> {
        let name = values[0].value.as_str();
        db.insert_ingredient(name)?;

        Ok(())
    }

    fn delete(db: &Db, id: Uuid) -> Result<()> {
        db.remove_ingredient(id)
    }

    fn name() -> &'static str {
        "ingredient"
    }
}
