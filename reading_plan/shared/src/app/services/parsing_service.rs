use std::sync::Arc;
use std::collections::HashMap;

use regex::Regex;
use async_trait::async_trait;

use crate::domain::entities::bible::{Ref, ExcerptRef};
use crate::app::errors::AppError;
use crate::app::usecases::parsing::ParseReference;

#[derive(Clone)]
pub struct ParseReferenceImpl {}

impl ParseReferenceImpl {
    pub fn new() -> Self {
        Self{}
    }

    fn clean_ref(line: &str) -> String {
        let re = Regex::new(r"[ _]{2,}").unwrap();
        let reduced_line = re.replace_all(line, " ").into_owned();

        String::from(reduced_line.trim())
    }
}

#[async_trait]
impl ParseReference for ParseReferenceImpl {
    async fn execute(&self, reference: &str) -> Result<Arc<ExcerptRef>, AppError> {
        let re = Regex::new(
            r"^(?<s_bk>\d*[ _]?[^\W\d_]+)(?:[ _]?(?<s_ch>\d+)(?:[ _]?:[ _]?(?<s_vers>\d+))?)?(?:[ _]?-[ _]?(?<e_bk>\d*[ _]?[^\W\d_]+)?(?:[ _]?(?<e_ch>\d+)(?:[ _]?:[ _]?(?<e_vers>\d+))?)?)?$"
        ).unwrap();

        let clean_ref = ParseReferenceImpl::clean_ref(reference);

        tracing::info!("Clean reference line: '{clean_ref}'");

        let Some(caps) = re.captures(&clean_ref) else { 
            return Err(AppError::ParseReferenceError);    
        };

        let dict: HashMap<&str, &str> = re
            .capture_names()
            .flatten()
            .filter_map(|n| Some((n, caps.name(n)?.as_str())))
            .collect();

        tracing::debug!("{dict:?}");

        let mut s_ref = Ref::default();
        let mut e_ref = Ref::default();

        if let Some(s_book) = dict.get(&"s_bk") {
            s_ref.book = Some(String::from(*s_book));
        }

        if let Some(s_ch) = dict.get(&"s_ch") {
            s_ref.chapter = Some(s_ch.parse::<u8>().unwrap());
        }

        if let Some(s_vers) = dict.get(&"s_vers") {
            s_ref.vers = Some(s_vers.parse::<u8>().unwrap());
        }

        if let Some(e_book) = dict.get(&"e_bk") {
            e_ref.book = Some(String::from(*e_book));
        }

        if let Some(e_ch) = dict.get(&"e_ch") {
            e_ref.chapter = Some(e_ch.parse::<u8>().unwrap());
        }

        if let Some(e_vers) = dict.get(&"e_vers") {
            e_ref.vers = Some(e_vers.parse::<u8>().unwrap());
        }
        
        Ok(Arc::new((s_ref, e_ref)))
    }
}
