use std::future::Future;
use std::pin::Pin;

use actix_web::dev::Payload;
use actix_web::{web, FromRequest, HttpRequest};
use serde::Deserialize;

use crate::data_source::PageParams;
use crate::http_server::web_error::WebError;

const MAX_SIZE: usize = 500;
const DEFAULT_SIZE: usize = 50;

#[derive(Deserialize)]
struct QueryParams {
    offset: Option<usize>,
    size: Option<usize>,
}

impl QueryParams {
    fn offset(&self) -> usize {
        self.offset.unwrap_or(0)
    }

    fn size(&self) -> usize {
        self.size.unwrap_or(DEFAULT_SIZE)
    }
}

impl FromRequest for PageParams {
    type Error = WebError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let query = req.query_string().to_string();

        Box::pin(async move {
            let params =
                web::Query::<QueryParams>::from_query(&query).map_err(|_| WebError::ParseError)?;

            Ok(PageParams::new(params.size(), params.offset()))
        })
    }
}
