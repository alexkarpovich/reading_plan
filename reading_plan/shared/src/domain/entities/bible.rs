use std::sync::Arc;
use chrono::prelude::*;
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct BibleBook {
    pub id: ID,
    pub no: u8,
    pub name: String,
    pub short: String,
    pub chapters: Arc<Vec<BibleChapter>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}