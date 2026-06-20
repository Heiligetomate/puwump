mod db;
mod errors;
mod handlers;
mod models;
mod ui;
mod util;
mod values;

use crate::{db::Db, errors::Result, ui::PuwumpUi};
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

#[allow(unused)]
fn generate_ingredient_examples(db: &Db) -> Result<()> {
    for name in INGREDIENT_EXAMPLE_VALUES {
        db.insert_ingredient(name)?;
    }

    Ok(())
}

#[allow(unused)]
fn generate_meal_examples(db: &Db) -> Result<()> {
    for (name, calories, description) in MEAL_EXAMPLE_VALUES {
        db.insert_meal(name, description, calories)?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };
    eframe::run_native("puwump", options, Box::new(|cc| Ok(Box::new(PuwumpUi::new(cc)?))))?;
    Ok(())
}
