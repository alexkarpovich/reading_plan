use std::sync::Arc;
use axum::{Router, Extension};

use shared::app::services::bible_service::BibleService;
use shared::adapters::gateways::parse_gateway::ParsingGateway;
use shared::adapters::gateways::postgres::bible_gateway::PostgresBibleGateway;

use crate::app_state::AppState;
use crate::routing::bible_router;


pub async fn router(app_state: AppState) -> Router {
    let bible_service = BibleService::new(
        Arc::new(PostgresBibleGateway::new(app_state.db.clone())),
        Arc::new(ParsingGateway{}),
    );

    Router::new()
        .nest("/bible", bible_router::router(app_state).await)
        .layer(Extension(bible_service))
}
