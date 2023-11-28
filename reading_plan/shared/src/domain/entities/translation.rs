use crate::domain::value_objects::ID;

#[derive(Debug)]
pub struct Lang {
    pub id: ID,
    pub code: String,
    pub name: String,
}

#[derive(Debug)]
pub struct Translation {
    pub id: ID,
    pub lang_id: ID,
    pub name: String,
}