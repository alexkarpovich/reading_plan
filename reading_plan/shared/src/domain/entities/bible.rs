use chrono;
use std::sync::Arc;

use crate::domain::value_objects::ID;


#[derive(Debug)]
pub struct RefRange {
    pub start: ID,
    pub end: ID,
}

#[derive(Debug)]
pub struct BibleVerse {
    pub id: ID,
    pub no: u8,
    pub content: String,
}

#[derive(Debug)]
pub struct BibleChapter {
    pub id: ID,
    pub no: u8,
    pub name: String,
    pub verses: Arc<Vec<BibleVerse>>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug)]
pub struct BibleBook {
    pub id: ID,
    pub no: u8,
    pub name: String,
    pub short: String,
    pub chapters: Arc<Vec<BibleChapter>>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug)]
pub struct BibleFragment {
    // translation: ....,
    pub books: Arc<Vec<BibleBook>>,
}

#[derive(Debug)]
pub struct BibleBookInfo {
    pub id: ID,
    pub no: u8,
    pub name: String,
    pub short: String,
    // pub chapters_count: u8,
    pub is_new_testament: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}