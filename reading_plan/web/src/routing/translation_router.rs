use axum::{
    routing::get,
    Json, Router, Extension, response::IntoResponse, 
};
use serde_json::{json, Value};

use shared::app::usecases::translation::ListTranslations;
use shared::app::services::translation_service::TranslationService;

use crate::app_state::AppState;


pub async fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(list_translations)).with_state(state)
}

async fn list_translations(
    Extension(translation_service): Extension<TranslationService>,
) -> impl IntoResponse {

    let books = translation_service
        .list_translations()
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