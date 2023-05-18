use actix_web::Scope;

pub mod api;
pub mod auth;
pub mod web;

pub fn routes() -> Scope {
    actix_web::web::scope("")
        .service(api::routes())
        .service(web::routes())
}
