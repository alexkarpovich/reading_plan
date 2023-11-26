use sqlx::Type;
use serde::{Deserialize, Serialize};

#[derive(Debug, Type, Clone, Copy, Deserialize, Serialize)]
pub struct ID(pub i32);