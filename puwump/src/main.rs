pub mod db;
pub mod errors;
pub mod models;
// mod ui;
pub mod util;

use crate::{db::Db, errors::Result};

#[rustfmt::skip]
static EXAMPLE_VALUES: [(&str, &str); 20] = [
    ("Push-up", "Start in a plank position with hands shoulder-width apart. Lower your chest to the floor, then push back up."),
    ("Pull-up", "Hang from a bar with palms facing away. Pull yourself up until your chin is above the bar, then lower slowly."),
    ("Squat", "Stand with feet shoulder-width apart. Lower your hips until thighs are parallel to the floor, then stand back up."),
    ("Deadlift", "Stand with feet hip-width apart, barbell over feet. Hinge at hips, grip the bar, and lift by driving through your legs."),
    ("Plank", "Hold a push-up position with arms straight or on forearms. Keep your body in a straight line from head to heels."),
    ("Lunge", "Step forward with one leg and lower your back knee toward the floor. Push back up and repeat on the other side."),
    ("Dumbbell Row", "Place one hand and knee on a bench. Pull a dumbbell up to your hip, keeping your elbow close to your body."),
    ("Overhead Press", "Hold dumbbells at shoulder height. Press them straight up overhead until arms are fully extended, then lower."),
    ("Bicep Curl", "Hold dumbbells at your sides with palms facing forward. Curl the weights up to your shoulders, then lower slowly."),
    ("Tricep Dip", "Place hands on a bench behind you. Lower your body by bending your elbows, then push back up."),
    ("Burpee", "Drop into a squat, kick feet back into a plank, do a push-up, jump feet forward, then jump up with arms overhead."),
    ("Mountain Climber", "Start in a plank position. Alternate driving your knees toward your chest as fast as possible."),
    ("Glute Bridge", "Lie on your back with knees bent. Drive your hips up by squeezing your glutes, hold briefly, then lower."),
    ("Leg Raise", "Lie flat on your back with legs straight. Raise them to 90 degrees, then lower slowly without touching the floor."),
    ("Lateral Raise", "Hold dumbbells at your sides. Raise them out to the sides until shoulder height, then lower slowly."),
    ("Calf Raise", "Stand with feet hip-width apart. Rise up onto your toes as high as possible, hold briefly, then lower."),
    ("Russian Twist", "Sit with knees bent and feet raised. Rotate your torso side to side, touching the floor on each side."),
    ("Face Pull", "Attach a rope to a cable machine at face height. Pull the rope toward your face, flaring your elbows out."),
    ("Hip Thrust", "Sit against a bench with a barbell across your hips. Drive your hips up until your body is straight, then lower."),
    ("Box Jump", "Stand in front of a box. Bend your knees and jump onto the box, land softly, then step back down."),
];

#[allow(unused)]
fn generate_examples(db: &Db) -> Result<()> {
    for (name, instr) in EXAMPLE_VALUES {
        db.new_exercise(name, instr)?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let db = Db::init()?.create()?;
    // db.insert_ingredient("fuß")?;
    // db.insert_meal("fußsuppe", "leicht käsig", 2)?;
    db.insert_meal_ingredient("fußsuppe", "fuß", 3)?;
    // generate_examples(&db)?;
    // let options = eframe::NativeOptions {
    //     viewport: egui::ViewportBuilder::default(),
    //     ..Default::default()
    // };
    // eframe::run_native("puwump", options, Box::new(|cc| Ok(Box::new(PuwumpUi::new(cc)?))))?;
    Ok(())
}
