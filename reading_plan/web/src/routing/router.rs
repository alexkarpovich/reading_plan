use std::sync::Arc;
use axum::{Router, Extension};

use shared::adapters::factory::service_factory::ServiceFactory;

use crate::app_state::AppState;
use crate::routing::{api, bible_router};



pub async fn router(app_state: AppState) -> Router {
    let service_factory = Arc::new(
        ServiceFactory::new(app_state.db.clone())
    );

    Router::new()
        .nest("/api", api::router::router(app_state.clone()).await)
        .nest("/bible", bible_router::router(app_state.clone()).await)
        .layer(Extension(service_factory))
        
}
