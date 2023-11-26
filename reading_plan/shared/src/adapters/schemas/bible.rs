use sqlx::FromRow;
use chrono::prelude::*;
use crate::domain::value_objects::ID;

#[derive(Debug, FromRow)]
pub struct BibleBookInfoSchema {
	pub id: ID,
    pub no: i32,
	pub name: String,
    pub short: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}