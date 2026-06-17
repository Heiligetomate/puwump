use uuid::Uuid;

use crate::{
    db::Db,
    errors::{PuwumpError, Result},
    models::{CardAdd, Plan, PlanExerciseDetail},
    ui::ButtonTheme,
};

pub trait EditHandler {
    type Model: CardAdd + PartialEq;
    type SelModel: CardAdd;

    fn get_selected(&self) -> Option<&Self::Model>;
    fn get_sel_data(&self) -> Result<&Vec<Self::SelModel>>;
    fn set_selected(&mut self, model: Option<Self::Model>);
    fn get_data(&self) -> &Vec<Self::Model>;
    fn update(&mut self, db: &Db) -> Result<()>;
    fn update_sel(&mut self, db: &Db, id: Uuid) -> Result<()>;
    fn updated_sel_data(&mut self, db: &Db) -> Result<()>;
    fn insert_handler_model(&self, db: &Db, id: Uuid) -> Result<()>;

    fn card_buttons() -> &'static [ButtonTheme];
    fn handle_buttons(&mut self, results: Vec<(Uuid, Vec<bool>)>, db: &Db) -> Result<()>;
}

#[derive(Default)]
pub struct PlanEditHandler {
    selected: Option<Plan>,
    data: Vec<Plan>,
    sel_data: Option<Vec<PlanExerciseDetail>>,
}

impl EditHandler for PlanEditHandler {
    type Model = Plan;
    type SelModel = PlanExerciseDetail;

    fn insert_handler_model(&self, db: &Db, id: Uuid) -> Result<()> {
        let selected = self
            .get_selected()
            .ok_or(PuwumpError::SelectedDataNotFound)?
            .id;
        db.insert_plan_exercise(selected, id, 1)
    }

    fn card_buttons() -> &'static [ButtonTheme] {
        const BUTTONS: [ButtonTheme; 5] = [ButtonTheme::delete(), ButtonTheme::move_up(), ButtonTheme::move_down(), ButtonTheme::plus(), ButtonTheme::minus()];
        &BUTTONS
    }

    fn handle_buttons(&mut self, results: Vec<(Uuid, Vec<bool>)>, db: &Db) -> Result<()> {
        for (id, clicked) in results {
            if clicked[0] {
                db.remove_plan_exercise(id)?;
                self.updated_sel_data(db)?;
            } else if clicked[1] {
                let _ = db.move_plan_exercise(id, -1);
                self.updated_sel_data(db)?;
            } else if clicked[2] {
                let _ = db.move_plan_exercise(id, 1);
                self.updated_sel_data(db)?;
            } else if clicked[3] {
                db.incr_plan_exercise(id)?;
                self.updated_sel_data(db)?;
            } else if clicked[4] {
                db.decr_plan_exercise(id)?;
                self.updated_sel_data(db)?;
            }
        }

        Ok(())
    }

    fn update(&mut self, db: &Db) -> Result<()> {
        self.data = db.get_all_plans()?;

        Ok(())
    }

    fn get_data(&self) -> &Vec<Self::Model> {
        &self.data
    }

    fn update_sel(&mut self, db: &Db, id: Uuid) -> Result<()> {
        let new_plan = db.get_plan(id)?;
        self.selected = Some(new_plan);

        Ok(())
    }

    fn get_selected(&self) -> Option<&Self::Model> {
        self.selected.as_ref()
    }

    fn get_sel_data(&self) -> Result<&Vec<Self::SelModel>> {
        Ok(self
            .sel_data
            .as_ref()
            .ok_or(PuwumpError::SelectedDataNotFound)?)
    }

    fn updated_sel_data(&mut self, db: &Db) -> Result<()> {
        if let Some(sel) = &self.selected {
            let plans = db.get_plan_exercises(sel.id)?;
            self.sel_data = Some(plans);
            return Ok(());
        }

        Err(PuwumpError::SelectedDataNotFound)
    }

    fn set_selected(&mut self, model: Option<Self::Model>) {
        self.selected = model;
    }
}

impl PlanEditHandler {
    pub fn new(db: &Db) -> Result<Self> {
        let plans = db.get_all_plans()?;
        Ok(Self {
            sel_data: None,
            selected: None,
            data: plans,
        })
    }
}

// impl PlanEditHandler {
//     fn update_plans(&mut self, db: &Db) -> Result<()> {
//         let plans = db.get_all_plans()?;
//         self.data = Some(plans);
//
//         Ok(())
//     }
//
//     fn update_exercises(&mut self, db: &Db) -> Result<()> {
//         if let Some(plan) = &self.selected {
//             let exercises = db.get_plan_exercises(plan.id)?;
//             self.sel_data = Some(exercises);
//         }
//
//         Ok(())
//     }
//
//     fn update(&mut self, db: &Db) -> Result<()> {
//         self.update_plans(db)?;
//         self.update_exercises(db)?;
//
//         Ok(())
//     }
// }
