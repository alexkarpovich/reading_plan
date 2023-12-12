use chrono;
use serde::Serialize;

use crate::domain::value_objects::ID;


#[derive(Debug)]
pub struct Ref {
    pub book: Option<String>,
    pub chapter: Option<u8>,
    pub vers: Option<u8>,
}

impl Default for Ref {
    fn default() -> Self {
        Self {
            book: None,
            chapter: None,
            vers: None,
        }
    }
}

pub type ExcerptRef = (Ref, Ref);

#[derive(Debug, Serialize)]
pub struct BibleContent {
    pub id: ID,
    pub chapter: u8,
	pub vers: u8,
	pub text: String,
}

#[derive(Debug, Serialize)]
pub struct BibleContentWithBook {
    pub id: ID,
    pub book_id: ID,
    pub chapter: u8,
	pub vers: u8,
	pub text: String,
}

#[derive(Debug, Serialize)]
pub struct BibleBook {
    pub id: ID,
    pub no: u8,
    pub key: String,
    pub name: String,
    pub content: Vec<BibleContent>,
    pub is_new_testament: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct BibleExcerpt {
    // translation: ....,
    pub books: Vec<BibleBook>,
}

#[derive(Debug, Serialize)]
pub struct BibleBookInfo {
    pub id: ID,
    pub no: u8,
    pub key: String,
    pub name: String,
    pub is_new_testament: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}