use std::ops::Deref;

use crate::http_models::{AuthRequest, AuthResponse, BucketInfo};
use actix_web::web::Data;
use actix_web::{get, post, web, HttpRequest, Responder};
use log::info;

use crate::http_server::instance::{InstanceDataSource, ServerBucketInstance, Session};
use crate::http_server::web_error::WebError;

impl From<&ServerBucketInstance> for BucketInfo {
    fn from(value: &ServerBucketInstance) -> Self {
        Self {
            id: value.id(),
            name: value.name().to_string(),
            password_protected: value.password_protected(),
            encrypted: value.password_protected(),
        }
    }
}

#[get("")]
pub async fn index(buckets: Data<InstanceDataSource>) -> impl Responder {
    let instances: Vec<BucketInfo> = buckets
        .all_sorted()
        .into_iter()
        .map(|i| BucketInfo::from(i.deref()))
        .collect();

    web::Json(instances)
}

#[get("/{id}")]
pub async fn show(
    id: web::Path<u64>,
    buckets: Data<InstanceDataSource>,
) -> Result<impl Responder, WebError> {
    let instance = buckets.get_by_id(*id).ok_or(WebError::ResourceNotFound)?;

    Ok(web::Json(BucketInfo::from(instance.deref())))
}

#[post("/{id}/auth")]
pub async fn auth(
    id: web::Path<u64>,
    buckets: Data<InstanceDataSource>,
    req_body: web::Json<AuthRequest>,
    req: HttpRequest,
) -> Result<impl Responder, WebError> {
    let instance = buckets
        .get_by_id(id.into_inner())
        .ok_or(WebError::ResourceNotFound)?;

    let ip = req
        .peer_addr()
        .expect("Cannot find the client ip address")
        .ip();

    let login_result = instance.login(req_body.password.as_deref(), ip).await;

    match &login_result {
        Ok(_) => info!("Successful login from {ip} for {instance}"),
        Err(e) => info!("Failed attempt to login from {ip} for {instance}: {e}"),
    }

    let login_token = login_result?;

    Ok(web::Json(AuthResponse {
        token: login_token,
        active_tokens: 0,
    }))
}

#[post("/logout")]
pub async fn logout(session: Session) -> impl Responder {
    info!("User from {} logged out", session.ip());

    session.logout();
    web::Json(())
}

#[get("/check-auth")]
pub async fn check_auth(bucket: Session) -> impl Responder {
    web::Json(())
}
