use eframe::CreationContext;
use egui::{Color32, RichText, Ui};

use crate::{
    db::Db,
    errors::Result,
    models::Exercise,
    ui::{add_exercise::AddExerciseForm, sizes::SizeSheet, theme::Theme},
};

pub enum View {
    Default,
    AddExercise,
    AddPlan,
    Workout,
}

pub struct PuwumpUi {
    pub view: View,
    pub theme: Theme,
    pub sizes: SizeSheet,
    pub add_exercise: AddExerciseForm,
    pub exercises: Vec<Exercise>,
    pub db: Db,
}

impl PuwumpUi {
    pub fn new(cc: &CreationContext) -> Result<Self> {
        Ok(Self {
            view: View::Default,
            theme: Theme::default(),
            sizes: SizeSheet::new(cc),
            add_exercise: AddExerciseForm::default(),
            exercises: Vec::new(),
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
        });
    }
}

impl PuwumpUi {
    fn add_plan_view(&mut self, _: &mut Ui) {}

    fn work_out_view(&mut self, _: &mut Ui) {}

    pub fn header(&mut self, ui: &mut Ui, full_width: f32, full_height: f32) {
        let header_height = full_height * 0.09;
        let margin = full_width * 0.02;
        let button_height = header_height * 0.55;
        let button_width = full_width * 0.12;
        let font_size = header_height * 0.55;

        let rect = ui.available_rect_before_wrap();

        ui.painter()
            .rect_filled(rect, 0.0, Color32::from_rgb(50, 48, 47));

        ui.painter()
            .text(rect.center(), egui::Align2::CENTER_CENTER, self.get_title(), egui::FontId::proportional(font_size), self.theme.title);

        let button_rect = egui::Rect::from_min_size(rect.min + egui::vec2(margin, (header_height - button_height) / 2.0), egui::vec2(button_width, button_height));

        let mut go_home = false;
        if ui
            .put(
                button_rect,
                egui::Button::new(RichText::new("Home").color(self.theme.fg))
                    .fill(Color32::from_rgb(60, 56, 54))
                    .corner_radius(self.sizes.corner_radius),
            )
            .clicked()
        {
            go_home = true;
        }

        if go_home {
            self.view = View::Default;
        }
    }
}
