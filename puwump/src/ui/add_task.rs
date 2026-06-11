use crate::models::{Exercise, Ingredient, add_model::CardAdd, core::Model};

pub struct AddTaskHandler<A: Model + CardAdd> {
    pub data: Vec<A>,
    pub title_track: String,
    pub body_track: Option<String>,
    pub status: Option<std::result::Result<(), String>>,
}

impl Default for AddTaskHandler<Ingredient> {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            title_track: String::new(),
            body_track: None,
            status: None,
        }
    }
}

impl Default for AddTaskHandler<Exercise> {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            title_track: String::new(),
            body_track: Some(String::new()),
            status: None,
        }
    }
}

impl<A: Model + CardAdd> AddTaskHandler<A> {
    pub fn track_empty(&self) -> bool {
        self.title_track.is_empty()
            || self
                .body_track
                .as_ref()
                .is_some_and(|s| s.is_empty())
    }

    pub fn set_err(&mut self, msg: &str) {
        self.status = Some(Err(msg.to_owned()));
    }

    pub fn reset(&mut self) {
        self.title_track.clear();

        if let Some(body) = &mut self.body_track {
            body.clear();
        }
    }
}
