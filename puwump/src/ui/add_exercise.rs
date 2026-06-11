use egui::Ui;

use crate::ui::core::PuwumpUi;

impl PuwumpUi {
    pub fn add_exercise_view(&mut self, ui: &mut Ui) {
        let mut handler = std::mem::take(&mut self.exercise_hndl);
        self.add_view(ui, &mut handler);
        self.exercise_hndl = handler;
    }
}
