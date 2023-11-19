use std::env;
use std::sync::Arc;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn create_pool() -> Result<Arc<Pool<Postgres>>, sqlx::Error> {

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL was not provided.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await?;

    Ok(Arc::new(pool))
}