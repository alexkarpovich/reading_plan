use axum::{
    routing::get,
    extract::Query,
    Json, Router, Extension, response::IntoResponse, 
};
use serde_json::{json, Value};
use serde::Deserialize;

use shared::domain::value_objects::ID;
use shared::app::usecases::bible::ListBooks;
use shared::app::services::bible_service::BibleService;

use crate::app_state::AppState;


pub async fn router(state: AppState) -> Router {
    Router::new()
        .route("/books", get(list_books)).with_state(state)
}

#[derive(Deserialize)]
struct TrQuery {
    tr_id: ID,
}

async fn list_books(
    tr_query: Query<TrQuery>,
    Extension(bible_service): Extension<BibleService>,
) -> impl IntoResponse {

    let books = bible_service
        .list_books(tr_query.tr_id)
        .await
        .unwrap();

    Json(
        books.iter()
            .map(|x| json!({
                "id": x.id,
                "no": x.no,
                "name": x.name.to_owned(),
                "short": x.short.to_owned(),
                "created_at": x.created_at,
                "updated_at": x.updated_at,
            }))
            .collect::<Vec<Value>>()
    )
}