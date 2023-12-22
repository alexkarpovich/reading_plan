use axum::{Router, http::Method};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

use crate::app_state::AppState;
use crate::routing::api::{bible_router, translation_router};



pub async fn router(app_state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        .nest("/bible", bible_router::router(app_state.clone()).await)
        .nest("/translations", translation_router::router(app_state.clone()).await)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}
