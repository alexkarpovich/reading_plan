use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::{app::{factory::AbstractServiceFactory, usecases::{bible::{ListBooks, GetExcerpt}, parsing::ParseReference, translation::ListTranslations}, services::{bible_service::{ListBooksImpl, GetExcerptImpl}, parsing_service::ParseReferenceImpl, translation_service::ListTranslationsImpl}}, adapters::gateways::postgres::{bible_gateway::PostgresBibleGateway, translation_gateway::PostgresTranslationGateway}};

#[derive(Clone)]
pub struct ServiceFactory {
    db: Arc<Pool<Postgres>>
}

impl ServiceFactory {
    pub fn new(db: Arc<Pool<Postgres>>) -> Self {
        Self{db: db}
    }
}

impl AbstractServiceFactory for ServiceFactory {
    fn new_list_books_service(&self) -> Arc<dyn ListBooks> {
        Arc::new(
            ListBooksImpl::new(
                Arc::new(PostgresBibleGateway::new(self.db.clone())),
            )
        )
    }

    fn new_get_excerpt_service(&self) -> Arc<dyn GetExcerpt> {
        Arc::new(
            GetExcerptImpl::new(
                Arc::new(PostgresBibleGateway::new(self.db.clone())),
                self.new_parse_reference_service(),
            )
        )
    }

    fn new_parse_reference_service(&self) -> Arc<dyn ParseReference> {
        Arc::new(
            ParseReferenceImpl::new()
        )
    }

    fn new_list_translations_service(&self) -> Arc<dyn ListTranslations> {
        Arc::new(
            ListTranslationsImpl::new(
                Arc::new(PostgresTranslationGateway::new(self.db.clone())),
            )
        )
    }
}
