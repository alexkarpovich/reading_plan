use std::sync::Arc;
use axum::{
    routing::get,
    Json, Router, Extension, response::IntoResponse, 
};
use serde_json::{json, Value};

use shared::adapters::factory::service_factory::ServiceFactory;
use shared::app::factory::AbstractServiceFactory;

use crate::app_state::AppState;


pub async fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(list_translations)).with_state(state)
}

async fn list_translations(
    Extension(service_factory): Extension<Arc<ServiceFactory>>,
) -> impl IntoResponse {
    
    let list_translations = service_factory.new_list_translations_service();

    let books = list_translations.execute()
        .await
        .unwrap();

    Json(
        books.iter()
            .map(|x| json!({
                "id": x.id,
                "lang_id": x.lang_id,
                "name": x.name.to_owned(),
            }))
            .collect::<Vec<Value>>()
    )
}