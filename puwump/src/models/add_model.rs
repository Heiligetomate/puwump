use uuid::Uuid;

use crate::{
    db::Db,
    errors::Result,
    models::{Exercise, Ingredient, core::Model},
};

pub trait CardAdd: Model + Sized {
    fn title(&self) -> &str;
    fn body(&self) -> Option<&str>;
    fn key(&self) -> Uuid;
    fn get_all(db: &Db) -> Result<Vec<Self>>;
    fn insert(db: &Db, name: &str, body: Option<&str>) -> Result<()>;
    fn delete(db: &Db, id: Uuid) -> Result<()>;
}

impl CardAdd for Exercise {
    fn title(&self) -> &str {
        &self.name
    }

    fn body(&self) -> Option<&str> {
        Some(&self.instructions)
    }

    fn key(&self) -> Uuid {
        self.id
    }

    fn get_all(db: &Db) -> Result<Vec<Self>> {
        db.get_all_exercises()
    }

    fn insert(db: &Db, name: &str, body: Option<&str>) -> Result<()> {
        if body.is_none() {
            panic!("Exercise should always have a body");
        }

        db.insert_exercise(name, body.unwrap())
    }

    fn delete(db: &Db, id: Uuid) -> Result<()> {
        db.remove_exercise(id)
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

    fn get_all(db: &Db) -> Result<Vec<Self>> {
        db.get_all_ingredients()
    }

    fn insert(db: &Db, name: &str, body: Option<&str>) -> Result<()> {
        if body.is_some() {
            panic!("Ingredient should never have a body");
        }

        db.insert_ingredient(name)?;
        Ok(())
    }

    fn delete(db: &Db, id: Uuid) -> Result<()> {
        db.remove_ingredient(id)
    }
}
