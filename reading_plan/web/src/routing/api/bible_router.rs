use std::sync::Arc;
use axum::{
    routing::get,
    extract::{Path, Query},
    Json, Router, Extension, response::IntoResponse, 
};
use serde::Deserialize;

use shared::{domain::value_objects::ID, app::factory::AbstractServiceFactory, adapters::factory::service_factory::ServiceFactory};

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
    Extension(service_factory): Extension<Arc<ServiceFactory>>,
) -> impl IntoResponse {
    let list_books_service = service_factory.new_list_books_service();

    let books = list_books_service.execute(tr_query.tr_id).await.unwrap();

    Json(books)
}


async fn get_excerpt(
    tr_query: Query<TrQuery>,
    Path(reference): Path<String>,
    Extension(service_factory): Extension<Arc<ServiceFactory>>,
    user: AuthenticatedUser,
) -> impl IntoResponse {

    tracing::info!("{:?}", user);
    let get_excerpt_service = service_factory.new_get_excerpt_service();

    let excerpt = get_excerpt_service.execute(tr_query.tr_id, &reference).await.unwrap();

    Json(excerpt)
}