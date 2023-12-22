use std::sync::Arc;
use axum::{
    routing::get,
    extract::{Path, Query},
    Router, Extension, response::{Html, IntoResponse}, 
};
use askama::Template;
use serde::Deserialize;

use shared::{domain::{value_objects::ID, entities::bible::BibleExcerpt}, app::usecases::bible::GetExcerpt};
use shared::app::services::bible_service::BibleService;

use crate::app_state::AppState;


pub async fn router(state: AppState) -> Router {
    Router::new()
        .route("/:reference", get(get_excerpt))
        .with_state(state)
}

#[derive(Deserialize)]
struct TrQuery {
    tr_id: ID,
}

#[derive(Template)]
#[template(path = "exerpt.html")]
struct ExerptTemplate { 
    exerpt: Arc<BibleExcerpt>,
}

async fn get_excerpt(
    tr_query: Query<TrQuery>,
    Path(reference): Path<String>,
    Extension(bible_service): Extension<Arc<BibleService>>,
) -> impl IntoResponse {

    let exerpt = bible_service.get_excerpt(tr_query.tr_id, &reference).await.unwrap();
    let exerpt_tmpl = ExerptTemplate{exerpt: exerpt};

    Html(exerpt_tmpl.render().unwrap())
}