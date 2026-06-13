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
        let width = self.sizes.width;
        let height = self.sizes.height;
        let margin = self.sizes.margin;

        let inner_margin = (width * 0.01) as i8;
        let left_width = width * 0.4;
        let list_width = width * 0.55 - margin * 2.0;

        ui.add_space(height * 0.05);

        let available_height = ui.available_height();

        ui.horizontal(|ui| {
            ui.add_space(margin);

            ui.vertical(|ui| {
                ui.set_width(left_width);

                if ui
                    .add_sized([left_width, height * 0.07], egui::Button::new("Update Plans"))
                    .clicked()
                {
                    self.plan_hndl
                        .update_plans(&self.db)
                        .unwrap();
                }
            });

            ui.add_space(margin);
            ui.separator();
            ui.add_space(margin);

            if let Some(id) = self.card_list(ui, list_width, available_height, inner_margin) {
                self.plan_hndl
                    .update(&self.db, Some(id))
                    .unwrap();
            }
        });
    }

    pub fn card_list(&self, ui: &mut Ui, list_width: f32, available_height: f32, inner_margin: i8) -> Option<Uuid> {
        let margin = self.sizes.margin / 3.0;
        let mut to_delete = None;
        ui.vertical(|ui| {
            ui.set_width(list_width);
            ui.set_min_height(available_height);
            egui::ScrollArea::vertical()
                .max_width(list_width - margin)
                .max_height(available_height)
                .show(ui, |ui| {
                    if let Some(plans) = &self.plan_hndl.plans {
                        for plan in plans.iter() {
                            if self.add_plan_card(ui, plan, list_width, margin, inner_margin) {
                                to_delete = Some(plan.id);
                            }
                            ui.add_space(margin * 0.5);
                        }
                        ui.add_space(margin);
                    }
                });
        });
        to_delete
    }

    pub fn add_plan_card(&self, ui: &mut Ui, plan: &Plan, list_width: f32, margin: f32, inner_margin: i8) -> bool {
        let mut deleted = false;
        egui::Frame::NONE
            .fill(self.theme.text_field)
            .corner_radius(self.sizes.corner_radius)
            .inner_margin(egui::Margin::same(inner_margin))
            .show(ui, |ui| {
                ui.set_width(list_width - margin * 2.0);
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(&plan.name)
                            .color(self.theme.title)
                            .strong()
                            .size(20.0),
                    );
                    if self.delete_button(ui) {
                        deleted = true
                    }
                });
                ui.separator();
                ui.add(
                    egui::Label::new(
                        RichText::new(&plan.description)
                            .color(self.theme.fg)
                            .weak()
                            .size(16.0),
                    )
                    .wrap(),
                );
            });
        deleted
    }
}
