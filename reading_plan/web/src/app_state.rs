use std::sync::Arc;
use sqlx::postgres::PgPool;


#[derive(Clone)]
pub struct AppState {
    pub db: Arc<PgPool>,
}
