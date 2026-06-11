use egui::{Align, Button, Color32, Layout, RichText, Ui};

use crate::ui::{
    core::{PuwumpUi, View},
    sizes::SizeSheet,
    theme::Theme,
};

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
            .corner_radius(self.sizes.corner_radius),
        )
        .clicked()
    }

    pub fn button_full_width(&mut self, ui: &mut Ui, button_height: f32, color: Color32, title: &str) -> bool {
        let available_width = ui.available_width() - self.sizes.margin * 2.0;
        self.button(ui, available_width, button_height, color, title)
    }

    pub fn get_title(&self) -> &str {
        match self.view {
            View::AddPlan => "Add Plan",
            View::AddExercise => "Add Exercise",
            View::Default => "Puwump - Home",
            View::Workout => "Workout",
            View::AddIngredient => "Add ingredient",
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

    pub fn calc_button_height(&self, ui: &Ui, cnt: u8) -> f32 {
        let height = ui.available_height();
        let spacing = self.spacing(ui);
        (height - self.sizes.margin * 2.0 - spacing * (cnt - 1) as f32) / cnt as f32
    }

    pub fn spacing(&self, ui: &Ui) -> f32 {
        self.sizes.spc_mlt * ui.available_height()
    }

    /// Creates a small red round delete button with a X
    pub fn delete_button(&self, ui: &mut Ui) -> bool {
        let mut clicked = false;
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if ui
                .add(
                    Button::new(
                        RichText::new("X")
                            .color(self.theme.white)
                            .size(12.0),
                    )
                    .fill(self.theme.red)
                    .corner_radius(self.sizes.corner_radius),
                )
                .clicked()
            {
                clicked = true;
            }
        });
        clicked
    }
}

pub fn text_field(ui: &mut Ui, theme: &Theme, sizes: &SizeSheet, add_contents: impl FnOnce(&mut Ui)) {
    egui::Frame::NONE
        .fill(theme.text_field)
        .corner_radius(sizes.corner_radius)
        .inner_margin(egui::Margin::same(sizes.margin as i8))
        .show(ui, |ui| {
            add_contents(ui);
        });
}
