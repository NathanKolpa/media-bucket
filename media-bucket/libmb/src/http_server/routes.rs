use actix_web::{web, HttpResponse, Responder, Scope};
use crate::http_server::web_error::WebError;

mod buckets;
mod content;
mod media;
mod posts;
mod tags;

async fn not_found() -> impl Responder {
    HttpResponse::from_error(WebError::EndpointNotFound)
}

pub fn routes() -> Scope {
    web::scope("")
        .service(
            web::scope("/buckets")
                .service(buckets::index)
                .service(buckets::check_auth)
                .service(buckets::auth)
                .service(buckets::logout)
                .service(buckets::show)

                .service(web::scope("/{bucket_id}")
                    .service(
                        web::scope("/media")
                            .service(media::file)
                            .service(media::show),
                    )
                    .service(
                        web::scope("/posts")
                            .service(posts::index)
                            .service(posts::store)
                            .service(posts::index_items)
                            .service(posts::store_tags)
                            .service(posts::show_item)
                            .service(posts::show)
                            .service(posts::delete)
                            .service(posts::update),
                    )
                    .service(
                        web::scope("/tags")
                            .service(tags::index)
                            .service(tags::delete)
                            .service(tags::store),
                    )
                    .service(
                        web::scope("/content")
                            .app_data(web::PayloadConfig::new(1000 * 1000 * 1000 * 100)) // 100 GB
                            .service(content::store),
                    )
                )
        )
        .default_service(web::route().to(not_found))
}