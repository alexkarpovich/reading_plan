use sqlx::FromRow;

use crate::domain::value_objects::ID;
use crate::domain::entities::translation::Translation;

#[derive(Debug, FromRow)]
pub struct TranslationSchema {
	pub id: i32,
    pub lang_id: i32,
	pub name: String,
}

impl Into<Translation> for TranslationSchema {
    fn into(self) -> Translation {
        Translation {
            id: ID(self.id),
            lang_id: ID(self.lang_id),
            name: self.name.to_owned(),
        }
    }
}
