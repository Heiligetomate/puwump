mod db;
mod errors;
mod handlers;
mod models;
mod ui;
mod util;
mod values;

use crate::{db::Db, errors::Result, ui::core::PuwumpUi};
use values::*;

#[allow(unused)]
fn generate_exercise_examples(db: &Db) -> Result<()> {
    for (name, instr) in EXERCISE_EXAMPLE_VALUES {
        db.insert_exercise(name, instr)?;
    }

    Ok(())
}

#[allow(unused)]
fn generate_plan_examples(db: &Db) -> Result<()> {
    for (name, descr, est_min) in PLAN_EXAMPLE_VALUES {
        db.insert_plan(name, descr, est_min)?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let db = &Db::init()?.reset()?;
    generate_plan_examples(&db)?;
    generate_exercise_examples(&db)?;
    db.insert_plan_exercise(db.get_all_plans()?[0].id, db.get_all_exercises()?[0].id, 3)?;
    db.insert_plan_exercise(db.get_all_plans()?[0].id, db.get_all_exercises()?[1].id, 3)?;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };
    eframe::run_native("puwump", options, Box::new(|cc| Ok(Box::new(PuwumpUi::new(cc)?))))?;
    Ok(())
}
