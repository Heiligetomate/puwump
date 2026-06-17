use uuid::Uuid;

use crate::{
    db::Db,
    errors::{PuwumpError, Result},
    models::{CardAdd, Plan, PlanExerciseDetail},
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
        Ok(Self { sel_data: None, selected: None, data: plans })
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
