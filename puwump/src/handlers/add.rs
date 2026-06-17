use crate::models::{CardAdd, CardInputs, Exercise, ExerciseInputs, Ingredient, IngredientInputs, Meal, MealInputs, Model, Plan, PlanInputs};

pub struct AddTaskHandler<A: Model + CardAdd, I: CardInputs> {
    pub input_fields: I,
    pub data: Vec<A>,
    pub status: Option<std::result::Result<(), String>>,
}

impl Default for AddTaskHandler<Ingredient, IngredientInputs> {
    fn default() -> Self {
        Self {
            input_fields: IngredientInputs::new(),
            data: Vec::new(),
            status: None,
        }
    }
}

impl Default for AddTaskHandler<Exercise, ExerciseInputs> {
    fn default() -> Self {
        Self {
            input_fields: ExerciseInputs::new(),
            data: Vec::new(),
            status: None,
        }
    }
}

impl Default for AddTaskHandler<Plan, PlanInputs> {
    fn default() -> Self {
        Self {
            input_fields: PlanInputs::new(),
            data: Vec::new(),
            status: None,
        }
    }
}

impl Default for AddTaskHandler<Meal, MealInputs> {
    fn default() -> Self {
        Self {
            input_fields: MealInputs::new(),
            data: Vec::new(),
            status: None,
        }
    }
}

impl<A, I> AddTaskHandler<A, I>
where
    A: CardAdd,
    I: CardInputs,
{
    pub fn set_err(&mut self, msg: &str) {
        self.status = Some(Err(msg.to_owned()));
    }
}
