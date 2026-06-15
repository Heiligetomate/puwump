use egui::{RichText, Ui};
use uuid::Uuid;

use crate::{
    db::Db,
    errors::Result,
    models::{Plan, PlanExerciseDetail},
    ui::{core::PuwumpUi, theme::ButtonTheme},
};

pub struct PlanHandler {
    selected: Option<Plan>,
    plans: Option<Vec<Plan>>,
    plan_ex: Option<Vec<PlanExerciseDetail>>,
}

impl Default for PlanHandler {
    fn default() -> Self {
        Self { plans: None, plan_ex: None, selected: None }
    }
}

impl PlanHandler {
    fn update_plans(&mut self, db: &Db) -> Result<()> {
        let plans = db.get_all_plans()?;
        self.plans = Some(plans);

        Ok(())
    }

    fn update_exercises(&mut self, db: &Db) -> Result<()> {
        if let Some(plan) = &self.selected {
            let exercises = db.get_plan_exercises(plan.id)?;
            self.plan_ex = Some(exercises);
        }

        Ok(())
    }

    fn update(&mut self, db: &Db) -> Result<()> {
        self.update_plans(db)?;
        self.update_exercises(db)?;

        Ok(())
    }
}

impl PuwumpUi {
    pub fn edit_plan_view(&mut self, ui: &mut Ui) {
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
                let drop_down_height = height * 0.2;
                let list_height = height - drop_down_height - margin;

                let orig = ui.spacing().interact_size.y;
                self.plan_drop_down(ui, left_width);
                ui.spacing_mut().interact_size.y = orig;

                ui.add_space(margin);
                let exercises = { if let Some(exs) = &self.plan_hndl.plan_ex { exs } else { return } };
                let results = self.add_list(
                    ui,
                    left_width,
                    list_height,
                    inner_margin,
                    exercises,
                    &[ButtonTheme::delete(), ButtonTheme::move_up(), ButtonTheme::move_down(), ButtonTheme::plus(), ButtonTheme::minus()],
                );
                for (id, clicked) in results {
                    if clicked[0] {
                        self.db
                            .remove_plan_exercise(id)
                            .unwrap();
                        self.plan_hndl
                            .update_exercises(&self.db)
                            .unwrap();
                    } else if clicked[1] {
                        let _ = self.db.move_plan_exercise(id, -1);
                        self.plan_hndl
                            .update_exercises(&self.db)
                            .unwrap();
                    } else if clicked[2] {
                        let _ = self.db.move_plan_exercise(id, 1);
                        self.plan_hndl
                            .update_exercises(&self.db)
                            .unwrap();
                    } else if clicked[3] {
                        self.db.incr_plan_exercise(id).unwrap();
                        self.plan_hndl
                            .update_exercises(&self.db)
                            .unwrap();
                    } else if clicked[4] {
                        self.db.decr_plan_exercise(id).unwrap();
                        self.plan_hndl
                            .update_exercises(&self.db)
                            .unwrap();
                    }
                }
            });

            ui.add_space(margin);
            ui.separator();
            ui.add_space(margin);

            for (id, clicked) in self.add_list(ui, list_width, available_height, inner_margin, &self.exercise_hndl.data, &[ButtonTheme::add()]) {
                if clicked[0] {
                    if let Some(plan_id) = self
                        .plan_hndl
                        .selected
                        .as_ref()
                        .map(|p| p.id)
                    {
                        self.db
                            .insert_plan_exercise(plan_id, id, 1)
                            .unwrap();
                        self.plan_hndl.update(&self.db).unwrap();
                    }
                }
            }
        });
    }

    pub fn plan_drop_down(&mut self, ui: &mut Ui, width: f32) {
        let selected_text = self
            .plan_hndl
            .selected
            .as_ref()
            .map(|p| p.name.as_str())
            .unwrap_or("    select plan");
        ui.spacing_mut().interact_size.y = 40.0;

        self.set_dropdown_rounding(ui);

        egui::ComboBox::from_id_salt("plan_selector")
            .selected_text(
                RichText::new(selected_text)
                    .color(self.theme.fg)
                    .size(16.0),
            )
            .width(width)
            .show_ui(ui, |ui| {
                ui.style_mut()
                    .visuals
                    .widgets
                    .inactive
                    .bg_fill = self.theme.text_field;
                ui.style_mut()
                    .visuals
                    .widgets
                    .hovered
                    .bg_fill = self.theme.header_bg;

                self.plan_hndl
                    .update_plans(&self.db)
                    .unwrap();

                let plans = self
                    .plan_hndl
                    .plans
                    .clone()
                    .unwrap_or_default();

                let before: Option<Uuid> = self
                    .plan_hndl
                    .selected
                    .as_ref()
                    .map(|p| p.id);

                for plan in plans.iter() {
                    ui.selectable_value(
                        &mut self.plan_hndl.selected,
                        Some(plan.clone()),
                        RichText::new(&plan.name)
                            .color(self.theme.fg)
                            .size(20.0),
                    );
                }

                let after: Option<Uuid> = self
                    .plan_hndl
                    .selected
                    .as_ref()
                    .map(|p| p.id);
                if before != after {
                    self.plan_hndl
                        .update_exercises(&self.db)
                        .unwrap();
                }
            });
        self.reset_dropdown_rounding(ui);
    }

    pub fn set_dropdown_rounding(&self, ui: &mut Ui) {
        let rad = self.sizes.corner_radius as u8;
        let corner_radius = egui::CornerRadius::same(rad);

        ui.visuals_mut()
            .widgets
            .inactive
            .corner_radius = corner_radius;
        ui.visuals_mut()
            .widgets
            .active
            .corner_radius = corner_radius;
        ui.visuals_mut()
            .widgets
            .hovered
            .corner_radius = corner_radius;
        ui.visuals_mut()
            .widgets
            .open
            .corner_radius = corner_radius;

        ui.ctx().global_style_mut(|style| {
            style.visuals.menu_corner_radius = corner_radius;
        });
    }

    pub fn reset_dropdown_rounding(&self, ui: &mut Ui) {
        let default = egui::CornerRadius::default();

        ui.visuals_mut()
            .widgets
            .inactive
            .corner_radius = default;
        ui.visuals_mut()
            .widgets
            .active
            .corner_radius = default;
        ui.visuals_mut()
            .widgets
            .hovered
            .corner_radius = default;
        ui.visuals_mut()
            .widgets
            .open
            .corner_radius = default;
        ui.ctx().global_style_mut(|style| {
            style.visuals.menu_corner_radius = default;
        });
    }
}
