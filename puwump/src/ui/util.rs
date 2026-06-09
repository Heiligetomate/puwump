use egui::{Color32, RichText, Ui};

use crate::ui::core::{PuwumpUi, View};

impl PuwumpUi {
    pub fn button(&mut self, ui: &mut Ui, available_width: f32, button_height: f32, color: Color32, title: &str) -> bool {
        ui.add_sized(
            [available_width, button_height],
            egui::Button::new(
                RichText::new(title)
                    .color(self.theme.fg)
                    .strong(),
            )
            .fill(color)
            .corner_radius(self.theme.corner_radius),
        )
        .clicked()
    }

    pub fn get_title(&self) -> &str {
        match self.view {
            View::AddPlan => "Add Plan",
            View::AddExercise => "Add Exercise",
            View::Default => "Puwump - Home",
            View::Workout => "Workout",
        }
    }

    pub fn no_text_box_hover(&self, ui: &mut Ui) {
        ui.visuals_mut()
            .widgets
            .active
            .bg_stroke = egui::Stroke::NONE;

        ui.visuals_mut().selection.stroke = egui::Stroke::NONE;
        ui.visuals_mut()
            .widgets
            .hovered
            .bg_stroke = egui::Stroke::NONE;
    }
}

pub fn text_field(ui: &mut Ui, inner_margin: i8, add_contents: impl FnOnce(&mut Ui)) {
    egui::Frame::NONE
        .fill(Color32::from_rgb(60, 56, 54))
        .corner_radius(12.0)
        .inner_margin(egui::Margin {
            left: inner_margin,
            right: inner_margin,
            top: inner_margin,
            bottom: inner_margin,
        })
        .show(ui, |ui| {
            add_contents(ui);
        });
}
