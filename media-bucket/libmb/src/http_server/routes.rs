use crate::http_server::web_error::WebError;
use actix_files::{Files, NamedFile};
use actix_web::dev::{fn_service, ServiceRequest, ServiceResponse};
use actix_web::http::header::ACCEPT;
use actix_web::{web, HttpResponse, Responder, Scope};
use std::path::PathBuf;
use std::sync::Arc;

mod buckets;
mod content;
mod groups;
mod media;
mod posts;
mod tags;

async fn not_found() -> impl Responder {
    HttpResponse::from_error(WebError::EndpointNotFound)
}

pub fn routes_with_static(file_root: PathBuf, index_file: String) -> Scope {
    let index_file = Arc::new(index_file);
    let file_root = Arc::new(file_root);

    web::scope("")
        .service(web::scope("/api").service(routes()))
        .service(
            Files::new("", file_root.as_os_str())
                .index_file(index_file.as_str())
                .prefer_utf8(true)
                .default_handler(fn_service(move |req: ServiceRequest| {
                    let index_file = index_file.clone();
                    let file_root = file_root.clone();

                    async move {
                        if let Some(accept) = req.headers().get(ACCEPT) {
                            if accept
                                .to_str()
                                .ok()
                                .and_then(|s| s.split(',').find(|s| s == &"text/html"))
                                .is_none()
                            {
                                return Ok(ServiceResponse::new(
                                    req.into_parts().0,
                                    HttpResponse::from_error(WebError::EndpointNotFound),
                                ));
                            }
                        }

                        let (req, _) = req.into_parts();
                        let file =
                            NamedFile::open_async(format!("{}/{index_file}", file_root.display()))
                                .await?;
                        let res = file.into_response(&req);
                        Ok(ServiceResponse::new(req, res))
                    }
                })),
        )
}

pub fn routes() -> Scope {
    web::scope("")
        .service(doc())
        .service(
            web::scope("/buckets")
                .service(buckets::index)
                .service(buckets::check_auth)
                .service(buckets::auth)
                .service(buckets::show)
                .service(
                    web::scope("/{bucket_id}")
                        .service(buckets::bucket_details)
                        .service(
                            web::scope("/media")
                                .service(media::file)
                                .service(media::show),
                        )
                        .service(
                            web::scope("/posts")
                                .service(posts::graph)
                                .service(posts::index)
                                .service(posts::index_playlist)
                                .service(posts::store)
                                .service(posts::index_items)
                                .service(posts::store_tags)
                                .service(posts::show_item)
                                .service(posts::show)
                                .service(posts::show_playlist)
                                .service(posts::show_tags)
                                .service(posts::delete)
                                .service(posts::update),
                        )
                        .service(
                            web::scope("/tag-groups")
                                .service(groups::index)
                                .service(groups::show)
                                .service(groups::store),
                        )
                        .service(
                            web::scope("/tags")
                                .service(tags::index)
                                .service(tags::show)
                                .service(tags::delete)
                                .service(tags::store)
                                .service(tags::update),
                        )
                        .service(
                            web::scope("/content")
                                .app_data(web::PayloadConfig::new(1000 * 1000 * 1000 * 1000)) // 1 TB
                                .service(content::store),
                        ),
                ),
        )
        .default_service(web::route().to(not_found))
}

#[cfg(feature = "http-server-spec")]
fn doc() -> Scope {
    use utoipa::OpenApi;
    use utoipa_swagger_ui::SwaggerUi;

    #[derive(OpenApi)]
    #[openapi(
        paths(
            posts::graph,
            posts::index,
            posts::index,
            posts::store,
            posts::index_items,
            posts::store_tags,
            posts::show_item,
            posts::show,
            posts::show_tags,
            posts::delete,
            posts::update,
            media::file,
            media::show,
            buckets::bucket_details,
            buckets::index,
            buckets::check_auth,
            buckets::auth,
            buckets::show,
            groups::index,
            groups::show,
            groups::store,
            tags::index,
            tags::show,
            tags::delete,
            tags::store,
            tags::update,
            content::store,
        ),
        components(schemas(
            crate::model::ImportBatch,
            crate::model::Dimensions,
            crate::model::MediaMetadata,
            crate::model::Media,
            crate::model::Content,
            crate::model::UploadMetadata,
            crate::model::Post,
            crate::model::PostItem,
            crate::model::TagGroup,
            crate::model::Tag,
            crate::model::SearchTag,
            crate::model::TagDetail,
            crate::model::PostDetail,
            crate::model::SearchPost,
            crate::model::SearchPostItem,
            crate::model::CreateFullPostItem,
            crate::model::CreateFullPost,
            crate::model::PostSearchQueryOrder,
            crate::model::PostSearchQuery,
            crate::model::GraphValue,
            crate::model::GraphPoint,
            crate::model::Graph,
            crate::model::GraphDiscriminator,
            crate::model::GraphSelect,
            crate::model::PostGraphQuery,
            crate::model::BucketDetails,
            crate::http_models::BucketInfo,
            crate::http_models::AuthRequest,
            crate::http_models::AuthResponse,
            crate::http_models::ErrorResponse,
            crate::http_models::CreateFullPostResponse,
            crate::http_models::CreateTagGroupRequest,
            crate::http_models::CreateTagRequest,
            crate::http_models::UpdateTagRequest,
            crate::http_models::UpdatePostRequest,
            crate::http_models::UpdatePostRequest,
        ))
    )]
    struct ApiDoc;

    let openapi = ApiDoc::openapi();

    web::scope("docs")
        .service(web::redirect("/ui", "/docs/ui/index.html"))
        .service(
            SwaggerUi::new("ui/{_:.*}")
                .url("openapi.json", openapi)
                .config(utoipa_swagger_ui::Config::new(["/docs/openapi.json"])),
        )
}

#[cfg(not(feature = "http-server-spec"))]
fn doc() -> Scope {
    web::scope("")
}
