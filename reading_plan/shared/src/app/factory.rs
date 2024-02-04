use std::sync::Arc;

use super::usecases::{
    bible::{ListBooks, GetExcerpt}, 
    parsing::ParseReference, 
    translation::ListTranslations
};

pub trait AbstractServiceFactory {
    fn new_list_books_service(&self) -> Arc<dyn ListBooks>;
    fn new_get_excerpt_service(&self) -> Arc<dyn GetExcerpt>;
    fn new_parse_reference_service(&self) -> Arc<dyn ParseReference>;
    fn new_list_translations_service(&self) -> Arc<dyn ListTranslations>;
}
