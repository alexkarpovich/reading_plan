use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use shared::domain::value_objects::ID;

#[derive(Deserialize)]
pub struct ListBooksRequest {
    pub tr_id: ID,
}

#[derive(Serialize)]
pub struct ListBooksInfoResponse {
    pub id: ID,
    pub no: u8,
    pub name: String,
    pub short: String,
    // pub chapters_count: u8,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

