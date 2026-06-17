use uuid::Uuid;

use crate::{
    db::Db,
    errors::Result,
    models::{Meal, MealIngredientDetail},
};

#[derive(Default)]
pub struct EatHandler {
    pub meals: Vec<Meal>,
    pub selected: Option<Meal>,
    pub ingredients: Vec<MealIngredientDetail>,
}

impl EatHandler {
    pub fn update_meals(&mut self, db: &Db) -> Result<()> {
        self.meals = db.get_all_meals()?;
        Ok(())
    }

    pub fn select_meal(&mut self, db: &Db, id: Uuid) -> Result<()> {
        let meal = db.get_meal(id)?;
        let ingredients = db.get_meal_ingredients(id)?;
        self.selected = Some(meal);
        self.ingredients = ingredients;
        Ok(())
    }
}
