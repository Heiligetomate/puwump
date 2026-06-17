use uuid::Uuid;

use crate::{
    db::Db,
    errors::{PuwumpError, Result},
    handlers::EditHandler,
    models::{Exercise, Plan, PlanExerciseDetail},
    ui::ButtonTheme,
};

#[derive(Default)]
pub struct PlanEditHandler {
    selectable: Vec<Exercise>,
    selected: Option<Plan>,
    pub data: Vec<Plan>,
    sel_data: Option<Vec<PlanExerciseDetail>>,
}

impl PlanEditHandler {
    pub fn new(db: &Db) -> Result<Self> {
        let plans = db.get_all_plans()?;
        let exercises = db.get_all_exercises()?;
        Ok(Self {
            data: plans,
            selectable: exercises,
            sel_data: None,
            selected: None,
        })
    }
}

impl EditHandler for PlanEditHandler {
    type Model = Plan;
    type SelModel = PlanExerciseDetail;
    type Selectable = Exercise;

    fn sel_is_none(&self) -> bool {
        self.selected.is_none()
    }

    fn get_selectable(&self) -> &Vec<Self::Selectable> {
        &self.selectable
    }

    fn update_selectable(&mut self, db: &Db) -> Result<()> {
        let new_selectable = db.get_all_exercises()?;
        self.selectable = new_selectable;

        Ok(())
    }

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
}
