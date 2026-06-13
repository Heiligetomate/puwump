use egui::{RichText, Ui};
use uuid::Uuid;

use crate::{
    db::Db,
    errors::{PuwumpError, Result},
    models::{Plan, PlanExerciseDetail},
    ui::core::PuwumpUi,
};

pub struct PlanHandler {
    selected: Option<Plan>,
    plans: Option<Vec<Plan>>,
    plan_ex: Option<Vec<PlanExerciseDetail>>,
}

impl Default for PlanHandler {
    fn default() -> Self {
        Self {
            plans: None,
            plan_ex: None,
            selected: None,
        }
    }
}

impl PlanHandler {
    fn search(&self, query: &str) -> Option<Vec<&str>> {
        let mut results = Vec::new();
        if self.plan_ex.is_none() {
            return None;
        }
        for ex in self.plan_ex.as_ref().unwrap().iter() {
            let ex_name = ex.exercise.name.as_str();
            if ex_name.contains(query) {
                results.push(ex_name);
            }
        }
        return Some(results);
    }

    fn update_plans(&mut self, db: &Db) -> Result<()> {
        let plans = db.get_all_plans()?;
        self.plans = Some(plans);

        Ok(())
    }

    fn update_exercises(&mut self, db: &Db, id: Option<Uuid>) -> Result<()> {
        if id.is_some() {
            let exercises = db.get_plan_exercises(id.unwrap())?;
            self.plan_ex = Some(exercises);
        }

        Ok(())
    }

    fn update_selected(&mut self, id: Uuid) -> Result<()> {
        if self.plans.is_none() {
            return Err(PuwumpError::PlanNotFound);
        }

        for plan in self.plans.as_ref().unwrap() {
            if plan.id == id {
                self.selected = Some(plan.clone());
                return Ok(());
            }
        }

        Err(PuwumpError::PlanNotFound)
    }

    fn update(&mut self, db: &Db, id: Option<Uuid>) -> Result<()> {
        self.update_plans(db)?;
        self.update_exercises(db, id)?;

        Ok(())
    }
}

impl PuwumpUi {
    pub fn manage_view(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let width = ui.available_width();
            let height = ui.available_height();
            let half_width = width / 2.0 - self.sizes.margin * 2.0;

            ui.add_space(self.sizes.margin);
            ui.vertical(|ui| {
                ui.set_width(half_width);
                if ui.button("update plans").clicked() {
                    self.plan_hndl
                        .update_plans(&self.db)
                        .unwrap();
                }
            });
            ui.add_space(self.sizes.margin);
            ui.separator();
            ui.add_space(self.sizes.margin);

            ui.vertical(|ui| {
                ui.set_width(half_width);
                if let Some(plans) = &self.plan_hndl.plans {
                    for plan in plans {
                        ui.label(RichText::new(plan.name.as_str()).size(12.0));
                    }
                }
            });
        });
    }
}
