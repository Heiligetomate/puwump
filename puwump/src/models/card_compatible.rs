use uuid::Uuid;

use crate::{db::Db, errors::Result, models::core::Model};

pub trait CardAdd: Model {
    fn title(&self) -> &str;
    fn body(&self) -> Option<&str>;
    fn key(&self) -> Uuid;
}

pub trait CardCrud: CardAdd + Model + Sized {
    fn get_all(db: &Db) -> Result<Vec<Self>>;
    fn insert(db: &Db, values: &[InputField]) -> Result<()>;
    fn delete(db: &Db, id: Uuid) -> Result<()>;
    fn name() -> &'static str;
}

pub trait CardInputs {
    fn get_fields(&self) -> &[InputField];
    fn get_fields_mut(&mut self) -> Vec<&mut InputField>;
    fn is_empty(&self) -> bool;
    fn clear(&mut self);
}

pub struct InputField {
    pub value: String,
    pub line_count: u8,
    pub hint_text: String,
}

impl InputField {
    pub fn new(line_count: u8, hint_text: &str) -> Self {
        Self {
            value: String::new(),
            line_count,
            hint_text: hint_text.to_owned(),
        }
    }

    pub fn title() -> Self {
        Self::new(2, "name")
    }

    pub fn body(hint: &str) -> Self {
        Self::new(7, hint)
    }

    pub fn extra(hint: &str) -> Self {
        Self::new(1, hint)
    }
}

pub struct IngredientInputs {
    fields: Vec<InputField>,
}

impl IngredientInputs {
    pub fn new() -> Self {
        Self { fields: vec![InputField::title()] }
    }
}

pub struct ExerciseInputs {
    fields: Vec<InputField>,
}

pub struct PlanInputs {
    fields: Vec<InputField>,
}

impl ExerciseInputs {
    pub fn new() -> Self {
        Self {
            fields: vec![InputField::title(), InputField::body("instructions")],
        }
    }
}

impl PlanInputs {
    pub fn new() -> Self {
        Self {
            fields: vec![InputField::title(), InputField::body("description"), InputField::extra("estimated minutes")],
        }
    }
}

impl CardInputs for PlanInputs {
    fn get_fields(&self) -> &[InputField] {
        &self.fields
    }

    fn get_fields_mut(&mut self) -> Vec<&mut InputField> {
        self.fields.iter_mut().collect()
    }

    fn is_empty(&self) -> bool {
        for field in self.fields.iter() {
            if field.value.is_empty() {
                return true;
            }
        }

        false
    }

    fn clear(&mut self) {
        for field in self.fields.iter_mut() {
            field.value.clear();
        }
    }
}

impl CardInputs for ExerciseInputs {
    fn get_fields(&self) -> &[InputField] {
        &self.fields
    }

    fn get_fields_mut(&mut self) -> Vec<&mut InputField> {
        self.fields.iter_mut().collect()
    }

    fn is_empty(&self) -> bool {
        for field in self.fields.iter() {
            if field.value.is_empty() {
                return true;
            }
        }

        false
    }

    fn clear(&mut self) {
        for field in self.fields.iter_mut() {
            field.value.clear();
        }
    }
}

impl CardInputs for IngredientInputs {
    fn get_fields(&self) -> &[InputField] {
        &self.fields
    }

    fn get_fields_mut(&mut self) -> Vec<&mut InputField> {
        self.fields.iter_mut().collect()
    }

    fn is_empty(&self) -> bool {
        for field in self.fields.iter() {
            if field.value.is_empty() {
                return true;
            }
        }

        false
    }

    fn clear(&mut self) {
        for field in self.fields.iter_mut() {
            field.value.clear();
        }
    }
}
