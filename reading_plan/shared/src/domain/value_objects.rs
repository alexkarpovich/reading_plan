use std::cmp::PartialEq;
use sqlx::Type;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, Hash, PartialEq, Type, Clone, Copy, Deserialize, Serialize)]
pub struct ID(pub i32);
