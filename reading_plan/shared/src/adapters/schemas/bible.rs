use chrono;
use sqlx::FromRow;

use crate::domain::value_objects::ID;
use crate::domain::entities::bible::BibleBookInfo;

#[derive(Debug, FromRow)]
pub struct BibleBookInfoSchema {
	pub id: i32,
    pub no: i32,
	pub name: String,
    pub short: String,
    pub is_new_testament: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Into<BibleBookInfo> for BibleBookInfoSchema {
    fn into(self) -> BibleBookInfo {
        BibleBookInfo {
            id: ID(self.id),
            no: self.no as u8,
            name: self.name.to_owned(),
            short: self.short.to_owned(),
            is_new_testament: self.is_new_testament,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
