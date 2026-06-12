use eframe::CreationContext;
use egui::{Button, RichText, Ui};

use crate::{
    db::Db,
    errors::Result,
    models::{AddTaskHandler, Exercise, Ingredient},
    ui::{sizes::SizeSheet, theme::Theme},
};

pub enum View {
    Default,
    AddExercise,
    AddIngredient,
    AddPlan,
    Workout,
}

pub struct PuwumpUi {
    pub view: View,
    pub theme: Theme,
    pub sizes: SizeSheet,
    pub exercise_hndl: AddTaskHandler<Exercise>,
    pub ingredient_hdnl: AddTaskHandler<Ingredient>,
    pub db: Db,
}

impl PuwumpUi {
    pub fn new(cc: &CreationContext) -> Result<Self> {
        Ok(Self {
            view: View::Default,
            theme: Theme::default(),
            sizes: SizeSheet::new(cc),
            exercise_hndl: AddTaskHandler::default(),
            ingredient_hdnl: AddTaskHandler::default(),
            db: Db::init()?,
        })
    }
}

impl eframe::App for PuwumpUi {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        let full_rect = ui.available_rect_before_wrap();
        let full_width = full_rect.width();
        let full_height = full_rect.height();
        let header_height = full_height * 0.09;

        self.sizes.update(ui);

        let header_rect = egui::Rect::from_min_size(full_rect.min, egui::vec2(full_width, header_height));
        let content_rect = egui::Rect::from_min_max(egui::pos2(full_rect.min.x, full_rect.min.y + header_height), full_rect.max);

        ui.scope_builder(egui::UiBuilder::new().max_rect(header_rect), |ui| {
            self.header(ui, full_width, full_height);
        });
        ui.scope_builder(egui::UiBuilder::new().max_rect(content_rect), |ui| match self.view {
            View::Default => self.home_view(ui),
            View::AddExercise => self.add_exercise_view(ui),
            View::AddPlan => self.add_plan_view(ui),
            View::Workout => self.work_out_view(ui),
            View::AddIngredient => self.add_ingredient_view(ui),
        });
    }
}

impl PuwumpUi {
    fn add_plan_view(&mut self, _: &mut Ui) {}

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
            .text(rect.center(), egui::Align2::CENTER_CENTER, self.get_title(), egui::FontId::proportional(font_size), self.theme.title);

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
}
