use std::future::Future;
use std::pin::Pin;

use actix_web::dev::Payload;
use actix_web::{web, FromRequest, HttpRequest};
use serde::Deserialize;

use crate::http_server::instance::{InstanceDataSource, Session};
use crate::http_server::web_error::WebError;

#[derive(Deserialize)]
struct QueryParams {
    #[serde(rename = "bucket")]
    bucket_id: Option<u64>,
    token: Option<String>,
}

impl FromRequest for Session {
    type Error = WebError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let instances = req
            .app_data::<web::Data<InstanceDataSource>>()
            .unwrap()
            .clone();

        let params = web::Query::<QueryParams>::from_query(req.query_string())
            .map_err(|e| WebError::ParseError);

        let bucket_id = req
            .match_info().get("bucket_id")
            .ok_or(WebError::MissingBucketId)
            .and_then(|id| id.parse().map_err(|_| WebError::ParseError));

        let token = req
            .headers()
            .get("Authorization")
            .map(|h| {
                h.to_str()
                    .map_err(|_| WebError::ParseError)
                    .map(|s| s.to_string())
            })
            .or_else(|| {
                params.as_ref().ok().map(|p| {
                    p.token
                        .as_ref()
                        .ok_or(WebError::MissingAuthToken)
                        .map(|s| s.clone())
                })
            })
            .ok_or(WebError::MissingAuthToken);

        Box::pin(async move {
            let bucket_id = bucket_id?;
            let token = token??;

            let instance = instances
                .get_by_id(bucket_id)
                .ok_or(WebError::InstanceNotFound)?;

            let session = instance
                .get_session_by_token(token)
                .ok_or(WebError::InvalidAuthToken)?;

            Ok(session)
        })
    }
}
