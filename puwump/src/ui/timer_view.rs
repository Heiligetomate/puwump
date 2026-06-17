use egui::{Align2, FontId, Label, RichText, Ui};

use crate::{
    handlers::{Phase, WorkoutHandler},
    models::{CardAdd, PlanExerciseDetail},
    ui::core::PuwumpUi,
};

impl PuwumpUi {
    pub fn workout_view(&mut self, ui: &mut Ui) {
        let mut handler = std::mem::take(&mut self.workout_hndl);
        self.workout_view_inner(ui, &mut handler);
        self.workout_hndl = handler;
    }

    fn workout_view_inner(&mut self, ui: &mut Ui, handler: &mut WorkoutHandler) {
        let width = self.sizes.width;
        let height = self.sizes.height;
        let margin = self.sizes.margin;

        ui.add_space(height * 0.05);

        ui.vertical_centered(|ui| {
            ui.set_width(width * 0.6);

            self.workout_plan_picker(ui, width * 0.6, handler);
            ui.add_space(margin);

            if handler.selected.is_none() {
                ui.label(RichText::new("Pick a plan to start your workout").color(self.theme.fg));
                return;
            }

            if handler.phase == Phase::Done {
                // TODO: handle empty plans
                self.workout_finished(ui, handler);
                return;
            }

            handler.tick();
            if handler.phase == Phase::Resting {
                ui.ctx().request_repaint();
            }

            if handler.phase == Phase::Done {
                self.workout_finished(ui, handler);
                return;
            }

            self.workout_progress(ui, handler);
            ui.add_space(margin);
            self.workout_exercise_card(ui, width * 0.6, height, handler);
            ui.add_space(margin * 1.5);

            match handler.phase {
                Phase::Exercise => {
                    self.workout_exercise_controls(ui, width * 0.6, height, handler);
                }
                Phase::Resting => {
                    self.workout_rest_display(ui, height, handler);
                    ui.add_space(margin * 1.5);
                    self.workout_rest_controls(ui, width * 0.6, height, handler);
                }
                Phase::Done => unreachable!("handled above"),
            }
        });
    }

    fn workout_plan_picker(&mut self, ui: &mut Ui, width: f32, handler: &mut WorkoutHandler) {
        if let Err(err) = handler.update_plans(&self.db) {
            println!("failed to update plans: {err}");
        }

        let selected_text = handler
            .selected
            .as_ref()
            .map(|p| p.title())
            .unwrap_or("select a plan");

        let orig = ui.spacing().interact_size.y;
        ui.spacing_mut().interact_size.y = 40.0;
        self.set_dropdown_rounding(ui);

        egui::ComboBox::from_id_salt("workout_plan_selector")
            .selected_text(
                RichText::new(selected_text)
                    .color(self.theme.fg)
                    .size(16.0),
            )
            .width(width)
            .show_ui(ui, |ui| {
                self.style_dropdown_menu(ui);
                self.plan_options(ui, handler);
            });

        self.reset_dropdown_rounding(ui);
        ui.spacing_mut().interact_size.y = orig;
    }

    fn style_dropdown_menu(&self, ui: &mut Ui) {
        let widgets = &mut ui.style_mut().visuals.widgets;
        widgets.inactive.bg_fill = self.theme.text_field;
        widgets.hovered.bg_fill = self.theme.header_bg;
    }

    fn plan_options(&self, ui: &mut Ui, handler: &mut WorkoutHandler) {
        let before = handler
            .selected
            .as_ref()
            .map(|p| p.key());
        let mut new_selected = before;

        for plan in &handler.plans {
            let is_selected = new_selected == Some(plan.key());
            let clicked = ui
                .selectable_label(
                    is_selected,
                    RichText::new(plan.title())
                        .color(self.theme.fg)
                        .size(20.0),
                )
                .clicked();

            if clicked {
                new_selected = Some(plan.key());
            }
        }

        if new_selected != before {
            if let Some(id) = new_selected {
                if let Err(err) = handler.select_plan(&self.db, id) {
                    println!("failed to select plan: {err}");
                }
            }
        }
    }

    fn workout_progress(&self, ui: &mut Ui, handler: &WorkoutHandler) {
        ui.label(
            RichText::new(format!("Exercise {} / {}", handler.current_index + 1, handler.total_exercises()))
                .color(self.theme.fg)
                .size(14.0),
        );
    }

    fn workout_exercise_card(&self, ui: &mut Ui, width: f32, height: f32, handler: &WorkoutHandler) {
        let Some(entry) = handler.current_exercise() else {
            return;
        };

        egui::Frame::NONE
            .fill(self.theme.text_field)
            .corner_radius(self.sizes.corner_radius)
            .inner_margin(egui::Margin::same((self.sizes.margin * 1.5) as i8))
            .show(ui, |ui| {
                ui.set_width(width - self.sizes.margin * 2.0);
                ui.vertical_centered(|ui| {
                    self.exercise_card_header(ui, handler.phase);
                    self.exercise_card_title(ui, entry.exercise.title(), height);
                    self.exercise_card_reps(ui, entry.reps);
                    self.exercise_card_body(ui, entry, handler.phase);
                });
            });
    }

    fn exercise_card_header(&self, ui: &mut Ui, phase: Phase) {
        let label = if phase == Phase::Resting { "Up next" } else { "Now" };
        ui.label(
            RichText::new(label)
                .color(self.theme.fg)
                .weak()
                .size(13.0),
        );
        ui.add_space(self.sizes.margin * 0.2);
    }

    fn exercise_card_title(&self, ui: &mut Ui, title: &str, height: f32) {
        ui.label(
            RichText::new(title)
                .color(self.theme.title)
                .strong()
                .size(height * 0.045),
        );
        ui.add_space(self.sizes.margin * 0.3);
    }

    fn exercise_card_reps(&self, ui: &mut Ui, reps: u16) {
        ui.label(
            RichText::new(format!("{reps} reps"))
                .color(self.theme.green)
                .size(16.0),
        );
    }

    fn exercise_card_body(&self, ui: &mut Ui, entry: &PlanExerciseDetail, phase: Phase) {
        if phase != Phase::Exercise {
            return;
        }
        let Some(body) = entry.exercise.body() else {
            return;
        };

        ui.add_space(self.sizes.margin * 0.5);
        ui.separator();
        ui.add_space(self.sizes.margin * 0.5);

        ui.add(
            Label::new(
                RichText::new(body)
                    .color(self.theme.fg)
                    .weak()
                    .size(15.0),
            )
            .wrap(),
        );
    }

    /// Displays the progress circle
    /// Displays some labels for understanding
    /// Displays the remaining seconds
    fn workout_rest_display(&self, ui: &mut Ui, height: f32, handler: &WorkoutHandler) {
        let remaining_secs = handler.remaining_whole_secs();
        let color = if remaining_secs <= 5 { self.theme.red } else { self.theme.blue };

        ui.label(
            RichText::new("Rest")
                .color(self.theme.fg)
                .weak()
                .size(14.0),
        );
        ui.add_space(self.sizes.margin * 0.3);

        let diameter = height * 0.28;
        let (rect, _) = ui.allocate_exact_size(egui::vec2(diameter, diameter), egui::Sense::empty());

        let painter = ui.painter();
        painter.circle_stroke(rect.center(), diameter / 2.0, egui::Stroke::new(4.0, self.theme.header_bg));

        let filled = if handler.rest_secs > 0 {
            (handler.remaining_secs / handler.rest_secs as f32).clamp(0.0, 1.0)
        } else {
            0.0
        };

        if filled > 0.0 {
            self.progress_circle(ui, diameter, rect, filled, color);
        }

        painter.text(rect.center(), Align2::CENTER_CENTER, format!("{remaining_secs}"), FontId::proportional(diameter * 0.35), color);
    }

    /// Button for finishing the current exercise and jump to the next step
    fn workout_exercise_controls(&mut self, ui: &mut Ui, width: f32, height: f32, handler: &mut WorkoutHandler) {
        let button_height = height * 0.08;
        let label = if handler.is_last_exercise() { "Finish workout" } else { "Finish exercise" };

        if self.button(ui, width, button_height, self.theme.green, label) {
            handler.finish_exercise();
        }
    }

    /// Skip button to skip the rest between exercises
    fn workout_rest_controls(&mut self, ui: &mut Ui, width: f32, height: f32, handler: &mut WorkoutHandler) {
        let button_height = height * 0.07;

        if self.button(ui, width, button_height, self.theme.header_bg, "Skip rest") {
            handler.skip_rest();
        }
    }

    /// Adds a label to show that the workout is finished
    /// Adds a button to restart the workout
    fn workout_finished(&mut self, ui: &mut Ui, handler: &mut WorkoutHandler) {
        ui.add_space(self.sizes.height * 0.05);
        ui.label(
            RichText::new("Workout complete!")
                .color(self.theme.green)
                .strong()
                .size(28.0),
        );
        ui.add_space(self.sizes.margin);

        if self.button(ui, self.sizes.width * 0.3, self.sizes.height * 0.07, self.theme.blue, "Restart") {
            handler.restart_workout();
        }
    }
}
