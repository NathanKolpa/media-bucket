use std::future::Future;
use std::pin::Pin;

use actix_web::dev::Payload;
use actix_web::{web, FromRequest, HttpRequest};
use serde::Deserialize;

use crate::http_server::web_error::WebError;
use crate::model::{PostSearchQuery, PostSearchQueryOrder};

#[derive(Deserialize)]
struct PostSearchParams {
    tags: Option<String>,
    text: Option<String>,
    order: Option<String>,
    source: Option<String>,
    seed: Option<f32>,
}

impl FromRequest for PostSearchQuery {
    type Error = WebError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let query = req.query_string().to_string();

        Box::pin(async move {
            let query = web::Query::<PostSearchParams>::from_query(&query)
                .map_err(|_| WebError::ParseError)?;

            let mut tags = None;

            if let Some(ids) = query.tags.as_deref() {
                let str_ids = ids.split(',');
                let mut ids: Vec<u64> = Vec::new();

                for str_id in str_ids {
                    ids.push(str_id.parse().map_err(|_| WebError::ParseError)?);
                }

                tags = Some(ids);
            }

            let seed = query.seed
                .ok_or(WebError::ParseError);

            let order = match query.order.as_deref() {
                Some("newest") | None => Ok(PostSearchQueryOrder::Newest),
                Some("oldest") => Ok(PostSearchQueryOrder::Oldest),
                Some("random") => Ok(PostSearchQueryOrder::Random(seed?)),
                Some("relevant") => Ok(PostSearchQueryOrder::Relevant),
                _ => Err(WebError::ParseError),
            }?;

            Ok(PostSearchQuery {
                tags,
                text: query.text.clone(),
                source: query.source.clone(),
                order,
            })
        })
    }
}
