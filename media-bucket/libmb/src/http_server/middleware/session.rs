use std::future::Future;
use std::net::IpAddr;
use std::pin::Pin;

use actix_web::dev::Payload;
use actix_web::{web, FromRequest, HttpRequest};
use serde::Deserialize;

use crate::http_server::instance::{InstanceDataSource, Session};
use crate::http_server::web_error::WebError;

#[derive(Deserialize)]
#[cfg_attr(feature = "http-server-spec", derive(utoipa::IntoParams))]
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
            .match_info()
            .get("bucket_id")
            .and_then(|id| id.parse().ok());

        let token = req
            .headers()
            .get("Authorization")
            .and_then(|h| {
                h.to_str()
                    .map(|s| (s.to_string(), false))
                    .ok()
            })
            .or_else(|| {
                bucket_id.and_then(|id| {
                    req.cookie(&format!("bucket_{id}"))
                        .map(|e| (e.value().to_string(), false))
                })
            })
            .or_else(|| {
                params.as_ref().ok().and_then(|p| {
                    p.token
                        .as_ref()
                        .map(|s| (s.clone(), true))
                })
            });

        let ip = req
            .connection_info()
            .realip_remote_addr()
            .ok_or(WebError::InstanceNotFound)
            .and_then(|ip| ip.parse::<IpAddr>().map_err(|_| WebError::ParseError));

        let method = req.method().clone();

        Box::pin(async move {
            let bucket_id = bucket_id.ok_or(WebError::MissingBucketId)?;
            let (token, token_from_insecure_location) = token.ok_or(WebError::MissingAuthToken)?;
            let ip = ip?;

            let instance = instances
                .get_by_id(bucket_id)
                .ok_or(WebError::InstanceNotFound)?;

            if instance.is_bfu() {
                return Err(WebError::BeforeFirstUnlock);
            }

            let session = instance
                .authorize_token(&token, ip)
                .ok_or(WebError::InvalidAuthToken)?;

            if session.read_only() && !method.is_safe() {
                return Err(WebError::ReadOnlyToken);
            }

            if token_from_insecure_location && !session.read_only() {
                return Err(WebError::InsecureAuthToken);
            }

            Ok(session)
        })
    }
}
