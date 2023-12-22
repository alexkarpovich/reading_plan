use std::sync::Arc;
use axum::{
    routing::get,
    extract::{Path, Query},
    Json, Router, Extension, response::IntoResponse, 
};
use serde::Deserialize;

use shared::{domain::value_objects::ID, app::usecases::bible::GetExcerpt};
use shared::app::usecases::bible::ListBooks;
use shared::app::services::bible_service::BibleService;

use crate::app_state::AppState;
use crate::schemas::user::AuthenticatedUser;


pub async fn router(state: AppState) -> Router {
    Router::new()
        .route("/books", get(list_books))
        .route("/:reference", get(get_excerpt))
        .with_state(state)
}

#[derive(Deserialize)]
struct TrQuery {
    tr_id: ID,
}

async fn list_books(
    tr_query: Query<TrQuery>,
    Extension(bible_service): Extension<Arc<BibleService>>,
) -> impl IntoResponse {

    let books = bible_service
        .list_books(tr_query.tr_id)
        .await
        .unwrap();

    Json(books)
}


async fn get_excerpt(
    tr_query: Query<TrQuery>,
    Path(reference): Path<String>,
    Extension(bible_service): Extension<Arc<BibleService>>,
    user: AuthenticatedUser,
) -> impl IntoResponse {

    tracing::info!("{:?}", user);

    let frg = bible_service.get_excerpt(tr_query.tr_id, &reference).await.unwrap();

    Json(frg)
}