use std::sync::Arc;
use axum::{
    routing::get,
    extract::{Path, Query},
    Router, Extension, response::{Html, IntoResponse}, 
};
use askama::Template;
use serde::Deserialize;

use shared::{domain::{value_objects::ID, entities::bible::BibleExcerpt}, adapters::factory::service_factory::ServiceFactory, app::factory::AbstractServiceFactory};

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
    excerpt: Arc<BibleExcerpt>,
}


async fn get_excerpt(
    Path(reference): Path<String>,
    tr_query: Query<TrQuery>,
    Extension(service_factory): Extension<Arc<ServiceFactory>>,
) -> impl IntoResponse {
    let get_excerpt_service = service_factory.new_get_excerpt_service();
    let excerpt = get_excerpt_service.execute(tr_query.tr_id, &reference).await.unwrap();
    let exerpt_tmpl = ExerptTemplate{excerpt};

    Html(exerpt_tmpl.render().unwrap())
}