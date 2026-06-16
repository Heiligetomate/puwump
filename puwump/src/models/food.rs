use crate::{
    db::Db,
    errors::{PuwumpError, Result},
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
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub calories: u32,
}

#[derive(Debug)]
pub struct MealIngredientDetail {
    pub id: Uuid,
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
        let id: String = row.get(0)?;
        Ok(Self {
            id: Uuid::parse_str(&id).map_err(|_| rusqlite::Error::InvalidQuery)?,
            name: row.get(1)?,
            calories: row.get(2)?,
            description: row.get(3)?,
        })
    }
}

impl Model for MealIngredientDetail {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        let ing_id: String = row.get(1)?;
        let id: String = row.get(0)?;
        Ok(Self {
            id: Uuid::parse_str(&id).map_err(|_| rusqlite::Error::InvalidQuery)?,
            ingredient: Ingredient {
                name: row.get(0)?,
                id: Uuid::parse_str(&ing_id).map_err(|_| rusqlite::Error::InvalidQuery)?,
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

impl CardAdd for Meal {
    fn key(&self) -> Uuid {
        self.id
    }

    fn body(&self) -> Option<&str> {
        Some(&self.description)
    }

    fn title(&self) -> &str {
        &self.name
    }
}

impl CardCrud for Meal {
    fn name() -> &'static str {
        "meal"
    }

    fn insert(db: &Db, values: &[super::card_compatible::InputField]) -> Result<()> {
        db.insert_meal(
            &values[0].value,
            &values[1].value,
            values[2]
                .value
                .parse()
                .map_err(|_| PuwumpError::InputFieldIntParse("calories should be a number"))?,
        )
    }

    fn delete(db: &Db, id: Uuid) -> Result<()> {
        db.remove_meal(id)
    }

    fn get_all(db: &Db) -> Result<Vec<Self>> {
        db.get_all_meals()
    }
}
