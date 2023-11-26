use std::sync::Arc;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use crate::config::config;


pub async fn create_pool() -> Result<Arc<Pool<Postgres>>, sqlx::Error> {

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config().DATABASE_URL).await?;

    Ok(Arc::new(pool))
}