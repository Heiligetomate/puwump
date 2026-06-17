use eframe::{CreationContext, Frame};
use egui::{Align2, Button, FontId, Rect, RichText, Ui, UiBuilder};

use crate::{
    db::Db,
    errors::Result,
    models::{
        AddTaskHandler, Exercise, Ingredient, Meal, Plan, PlanEditHandler,
        card_compatible::{ExerciseInputs, IngredientInputs, MealInputs, PlanInputs},
    },
    ui::{sizes::SizeSheet, theme::Theme},
};

pub enum View {
    Default,
    AddExercise,
    AddIngredient,
    AddPlan,
    AddMeal,
    EditPlan,
    Workout,
}

pub struct PuwumpUi {
    pub view: View,
    pub theme: Theme,
    pub sizes: SizeSheet,
    pub meal_hndl: AddTaskHandler<Meal, MealInputs>,
    pub exercise_hndl: AddTaskHandler<Exercise, ExerciseInputs>,
    pub ingredient_hdnl: AddTaskHandler<Ingredient, IngredientInputs>,
    pub plan_handler: AddTaskHandler<Plan, PlanInputs>,
    pub edit_plan_hndl: PlanEditHandler,
    pub db: Db,
}

impl PuwumpUi {
    pub fn new(cc: &CreationContext) -> Result<Self> {
        let db = Db::init()?;
        Ok(Self {
            view: View::Default,
            theme: Theme::default(),
            sizes: SizeSheet::new(cc),
            meal_hndl: AddTaskHandler::default(),
            exercise_hndl: AddTaskHandler::default(),
            ingredient_hdnl: AddTaskHandler::default(),
            plan_handler: AddTaskHandler::default(),
            edit_plan_hndl: PlanEditHandler::new(&db)?,
            db,
        })
    }
}

impl eframe::App for PuwumpUi {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        let full_rect = ui.available_rect_before_wrap();
        let full_width = full_rect.width();
        let full_height = full_rect.height();
        let header_height = full_height * 0.09;

        self.sizes.update(ui);

        let header_rect = Rect::from_min_size(full_rect.min, egui::vec2(full_width, header_height));
        let content_rect = Rect::from_min_max(egui::pos2(full_rect.min.x, full_rect.min.y + header_height), full_rect.max);

        ui.scope_builder(UiBuilder::new().max_rect(header_rect), |ui| {
            self.header(ui, full_width, full_height);
        });

        ui.scope_builder(egui::UiBuilder::new().max_rect(content_rect), |ui| match self.view {
            View::Default => self.home_view(ui),
            View::AddExercise => self.add_exercise_view(ui),
            View::EditPlan => self.edit_plan_view(ui),
            View::Workout => self.work_out_view(ui),
            View::AddIngredient => self.add_ingredient_view(ui),
            View::AddPlan => self.add_plan_view(ui),
            View::AddMeal => self.add_meal_view(ui),
        });
    }
}

impl PuwumpUi {
    fn edit_plan_view(&mut self, ui: &mut Ui) {
        let mut handler = std::mem::take(&mut self.edit_plan_hndl);
        self.edit_view(ui, &mut handler);
        self.edit_plan_hndl = handler;
    }

    fn add_meal_view(&mut self, ui: &mut Ui) {
        let mut handler = std::mem::take(&mut self.meal_hndl);
        self.add_view(ui, &mut handler);
        self.meal_hndl = handler;
    }

    fn add_plan_view(&mut self, ui: &mut Ui) {
        let mut handler = std::mem::take(&mut self.plan_handler);
        self.add_view(ui, &mut handler);
        self.plan_handler = handler;
    }

    fn work_out_view(&mut self, _: &mut Ui) {}

    pub fn header(&mut self, ui: &mut Ui, full_width: f32, full_height: f32) {
        let header_height = full_height * 0.09;
        let margin = self.sizes.margin;
        let button_height = header_height * 0.55;
        let button_width = full_width * 0.12;
        let font_size = header_height * 0.55;

        let rect = ui.available_rect_before_wrap();

        ui.painter()
            .rect_filled(rect, 0.0, self.theme.header_bg);

        ui.painter()
            .text(rect.center(), Align2::CENTER_CENTER, self.get_title(), FontId::proportional(font_size), self.theme.title);

        let button_rect = egui::Rect::from_min_size(rect.min + egui::vec2(margin, (header_height - button_height) / 2.0), egui::vec2(button_width, button_height));

        if ui
            .put(
                button_rect,
                Button::new(RichText::new("Home").color(self.theme.fg))
                    .fill(self.theme.text_field)
                    .corner_radius(self.sizes.corner_radius),
            )
            .clicked()
        {
            self.view = View::Default;
        }
    }

    pub fn add_ingredient_view(&mut self, ui: &mut Ui) {
        let mut handler = std::mem::take(&mut self.ingredient_hdnl);
        self.add_view(ui, &mut handler);
        self.ingredient_hdnl = handler;
    }

    pub fn add_exercise_view(&mut self, ui: &mut Ui) {
        let mut handler = std::mem::take(&mut self.exercise_hndl);
        self.add_view(ui, &mut handler);
        self.exercise_hndl = handler;
    }
}
