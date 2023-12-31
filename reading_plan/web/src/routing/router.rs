use std::sync::Arc;
use axum::{Router, Extension};

use shared::app::services::bible_service::BibleService;
use shared::adapters::gateways::postgres::bible_gateway::PostgresBibleGateway;
use shared::adapters::gateways::postgres::translation_gateway::PostgresTranslationGateway;
use shared::app::services::parsing_service::ParsingService;
use shared::app::services::translation_service::TranslationService;

use crate::app_state::AppState;
use crate::routing::{api, bible_router};



pub async fn router(app_state: AppState) -> Router {
    let parsing_service = Arc::new(
        ParsingService::new()
    );

    let bible_service = Arc::new(
        BibleService::new(
            Arc::new(PostgresBibleGateway::new(app_state.db.clone())),
            parsing_service.clone(),
        )
    );

    let translation_service = Arc::new(
        TranslationService::new(
            Arc::new(PostgresTranslationGateway::new(app_state.db.clone())),
        )
    );

    Router::new()
        .nest("/api", api::router::router(app_state.clone()).await)
        .nest("/bible", bible_router::router(app_state.clone()).await)
        .layer(Extension(bible_service))
        .layer(Extension(translation_service))
        .layer(Extension(parsing_service))
        
}
