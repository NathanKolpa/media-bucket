use std::net::IpAddr;
use std::ops::Deref;
use std::time::Duration;

use crate::http_models::{AuthRequest, AuthResponse, BucketInfo};
use actix_web::web::Data;
use actix_web::{get, post, web, HttpRequest, Responder};
use log::info;
use tokio::time::sleep;

use crate::http_server::instance::{InstanceDataSource, ServerBucketInstance, Session};
use crate::http_server::web_error::WebError;
use crate::model::BucketDetails;

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

#[cfg_attr(feature = "http-server-spec", utoipa::path)]
#[get("")]
pub async fn index(buckets: Data<InstanceDataSource>) -> impl Responder {
    let instances: Vec<BucketInfo> = buckets
        .all_sorted()
        .into_iter()
        .filter(|b| !b.hidden())
        .map(|i| BucketInfo::from(i.deref()))
        .collect();

    web::Json(instances)
}

#[cfg_attr(feature = "http-server-spec", utoipa::path)]
#[get("/{id}")]
pub async fn show(
    id: web::Path<u64>,
    buckets: Data<InstanceDataSource>,
) -> Result<impl Responder, WebError> {
    let instance = buckets.get_by_id(*id).ok_or(WebError::ResourceNotFound)?;

    Ok(web::Json(BucketInfo::from(instance.deref())))
}

#[cfg_attr(feature = "http-server-spec", utoipa::path)]
#[post("/gc")]
pub async fn gc(session: Session) -> Result<impl Responder, WebError> {
    info!("Running manual gc");
    let rows_affected = session.bucket().data_source().cross().gc().await?;
    info!("Gc affected {rows_affected} row(s)");

    Ok(web::Json(rows_affected))
}

#[cfg_attr(feature = "http-server-spec", utoipa::path)]
#[get("/details")]
pub async fn bucket_details(session: Session) -> Result<impl Responder, WebError> {
    let total_file_size = session
        .bucket()
        .data_source()
        .media()
        .get_total_size()
        .await?;

    let file_count = session.bucket().data_source().media().get_count().await?;

    Ok(web::Json(BucketDetails {
        total_file_size,
        file_count,
        sessions_created: session.instance().sessions_created(),
    }))
}

#[cfg_attr(feature = "http-server-spec", utoipa::path)]
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
        .connection_info()
        .realip_remote_addr()
        .expect("Cannot find the client ip address")
        .parse::<IpAddr>()
        .expect("Cannot parse ip address");

    let login_result = instance.login(req_body.password.as_deref(), ip).await;

    match &login_result {
        Ok(_) => info!("Successful login from {ip} for {instance}"),
        Err(e) => {
            info!("Failed attempt to login from {ip} for {instance}: {e}");
            sleep(Duration::from_millis(2000)).await
        }
    }

    let new_login = login_result?;

    Ok(web::Json(AuthResponse {
        token: new_login.token,
        share_token: new_login.share_token,
        active_tokens: instance.sessions_created(),
        now: new_login.now,
        lifetime: new_login.lifetime.num_seconds() as u64,
    }))
}

#[cfg_attr(feature = "http-server-spec", utoipa::path)]
#[get("/check-auth")]
pub async fn check_auth(bucket: Session) -> impl Responder {
    web::Json(())
}
