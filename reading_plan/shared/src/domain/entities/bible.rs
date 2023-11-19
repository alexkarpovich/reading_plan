use chrono::prelude::*;
use crate::domain::value_objects::ID;

#[derive(Debug)]
pub struct BibleBook {
    pub id: ID,
    pub lang_code: String,
    pub name: String,
    pub short: String,
}

#[derive(Debug)]
pub struct BibleRef {
    pub id: ID,
    pub book_id: ID,
    pub chapter: i32,
    pub verse: i32,
}

#[derive(Debug)]
pub struct RefRange<'a> {
    pub start: &'a ID,
    pub end: &'a ID,
}


#[derive(Debug)]
pub struct BibleTranslation {
    pub id: ID,
    pub ref_id: ID,
    pub lang_code: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct BibleVerse {
    pub ref_id: ID,
    pub content: String,
}