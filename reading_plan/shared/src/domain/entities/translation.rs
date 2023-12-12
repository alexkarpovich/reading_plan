use crate::domain::value_objects::ID;

#[derive(Debug)]
pub struct Lang {
    pub id: ID,
    pub code: String,
    pub iso_name: String,
    pub native_name: String,
}

#[derive(Debug)]
pub struct Translation {
    pub id: ID,
    pub lang_id: ID,
    pub name: String,
    pub description: String,
}