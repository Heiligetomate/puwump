use std::time::Instant;

use uuid::Uuid;

use crate::{
    db::Db,
    errors::Result,
    models::{Plan, PlanExerciseDetail},
};

const DEFAULT_REST_SECS: u32 = 30;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    /// Doing the current exercise
    Exercise,
    /// Resting before next exercise
    Resting,
    /// All exercises have been completed
    Done,
}

pub struct WorkoutHandler {
    pub plans: Vec<Plan>,
    pub selected: Option<Plan>,
    pub exercises: Vec<PlanExerciseDetail>,
    pub current_index: usize,

    pub phase: Phase,
    pub rest_secs: u32,
    pub remaining_secs: f32,
    pub last_tick: Option<Instant>,
}

impl Default for WorkoutHandler {
    fn default() -> Self {
        Self {
            plans: Vec::new(),
            selected: None,
            exercises: Vec::new(),
            current_index: 0,
            phase: Phase::Exercise,
            rest_secs: DEFAULT_REST_SECS,
            remaining_secs: 0.0,
            last_tick: None,
        }
    }
}

impl WorkoutHandler {
    pub fn update_plans(&mut self, db: &Db) -> Result<()> {
        self.plans = db.get_all_plans()?;
        Ok(())
    }

    /// Selects a plan by id
    /// Get its exercises by id
    /// Resets to the first exercise
    pub fn select_plan(&mut self, db: &Db, id: Uuid) -> Result<()> {
        let plan = db.get_plan(id)?;
        let exercises = db.get_plan_exercises(id)?;
        self.selected = Some(plan);
        self.exercises = exercises;
        self.restart_workout();
        Ok(())
    }

    pub fn current_exercise(&self) -> Option<&PlanExerciseDetail> {
        self.exercises.get(self.current_index)
    }

    pub fn total_exercises(&self) -> usize {
        self.exercises.len()
    }

    pub fn is_last_exercise(&self) -> bool {
        self.current_index + 1 >= self.exercises.len()
    }

    pub fn restart_workout(&mut self) {
        self.current_index = 0;
        self.phase = if self.exercises.is_empty() { Phase::Done } else { Phase::Exercise };
        self.remaining_secs = 0.0;
        self.last_tick = None;
    }

    pub fn finish_exercise(&mut self) {
        if self.phase != Phase::Exercise {
            return;
        }
        if self.is_last_exercise() {
            self.phase = Phase::Done;
            return;
        }
        self.phase = Phase::Resting;
        self.remaining_secs = self.rest_secs as f32;
        self.last_tick = Some(Instant::now());
    }

    pub fn skip_rest(&mut self) {
        if self.phase != Phase::Resting {
            return;
        }
        self.advance_to_next_exercise();
    }

    fn advance_to_next_exercise(&mut self) {
        self.current_index += 1;
        self.phase = Phase::Exercise;
        self.remaining_secs = 0.0;
        self.last_tick = None;
    }

    pub fn tick(&mut self) {
        if self.phase != Phase::Resting {
            return;
        }
        let now = Instant::now();
        let elapsed = self
            .last_tick
            .map(|t| now.duration_since(t).as_secs_f32())
            .unwrap_or(0.0);
        self.last_tick = Some(now);

        self.remaining_secs = (self.remaining_secs - elapsed).max(0.0);

        if self.remaining_secs <= 0.0 {
            self.advance_to_next_exercise();
        }
    }

    pub fn remaining_whole_secs(&self) -> u32 {
        self.remaining_secs.ceil() as u32
    }
}
