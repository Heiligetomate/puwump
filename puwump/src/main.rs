pub mod db;
pub mod errors;
pub mod models;
mod ui;
pub mod util;

use crate::{db::Db, errors::Result, ui::core::PuwumpUi};

#[rustfmt::skip]
static EXERCISE_EXAMPLE_VALUES: [(&str, &str); 20] = [
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

#[rustfmt::skip]
static PLAN_EXAMPLE_VALUES: [(&str, &str, u16); 20] = [ // Very sloppy values holy shit "Evening
                                                        // Wind Down" lmao
    ("Morning Stretch", "A gentle full-body stretching routine to improve mobility and wake up your muscles.", 15),
    ("Quick Cardio Blast", "A short high-intensity cardio session featuring jumping jacks, burpees, and mountain climbers.", 20),
    ("Upper Body Strength", "Focus on chest, shoulders, back, and arms using bodyweight or resistance training.", 45),
    ("Lower Body Workout", "Train quads, hamstrings, glutes, and calves with squats, lunges, and related exercises.", 50),
    ("Core Conditioning", "Strengthen your abs, obliques, and lower back with targeted core exercises.", 30),
    ("Full Body Circuit", "A balanced workout combining strength and cardio exercises for the entire body.", 40),
    ("Yoga Flow", "A relaxing sequence of yoga poses designed to improve flexibility and balance.", 35),
    ("Mobility Session", "Joint mobility drills and dynamic stretches to enhance range of motion.", 25),
    ("5K Run Training", "A structured running session focused on endurance and pacing for a 5K race.", 45),
    ("Sprint Intervals", "Alternate between high-speed sprints and recovery periods to build speed and stamina.", 30),
    ("Beginner Strength Plan", "An introductory strength workout suitable for those new to resistance training.", 40),
    ("Advanced Push Day", "A challenging workout emphasizing chest, shoulders, and triceps.", 60),
    ("Pull Day Routine", "Focus on back and biceps with pulling movements and accessory exercises.", 55),
    ("Leg Day Challenge", "An intense lower-body workout targeting strength and muscular endurance.", 65),
    ("Recovery Walk", "A light walking session to encourage active recovery and circulation.", 30),
    ("Meditation Break", "Guided breathing and mindfulness exercises to reduce stress and improve focus.", 10),
    ("Weekend Hike Prep", "Mobility, endurance, and strength work designed to prepare for a long hike.", 50),
    ("Desk Worker Reset", "Exercises and stretches to counteract prolonged sitting and improve posture.", 20),
    ("Evening Wind Down", "Gentle stretching and relaxation exercises to help you unwind before bed.", 15),
    ("Athletic Performance", "A mixed training session focused on power, agility, coordination, and conditioning.", 70),
];

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
