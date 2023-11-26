use tracing;
use tracing_subscriber;
use shared::config::config;
use shared::adapters::persistence::postgres::create_pool;

use crate::routing::router::router;
use crate::app_state::AppState;

pub async fn serve() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    
    let pg_pool = match create_pool().await {
        Ok(pool) => pool,
        Err(err) => panic!("Error connecting to db {err:?}")
    };

    let app_state = AppState {
        db: pg_pool,
    };

    let router = router(app_state).await;

    axum::Server::bind(&config().WEB_ADDR.parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
