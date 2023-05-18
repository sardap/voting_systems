use actix_web::{App, HttpServer};
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};

#[macro_use]
extern crate lazy_static;
extern crate diesel;

mod anti_plurality;
mod approval;
mod borda_count;
mod condorcet_method;
mod cumulative;
mod db;
mod elections;
mod majority_judgment;
mod models;
mod preferential_voting;
mod routes;
mod schema;
mod score;
mod single_non_transferable_vote;
mod single_party;
mod single_transferable_vote;
mod star;
mod three_two_one;
mod usual_judgment;

lazy_static! {
    static ref DATABASE_CONNECTION: String = std::env::var("DATABASE_CONNECTION").unwrap();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let manager = ConnectionManager::<PgConnection>::new(DATABASE_CONNECTION.to_string());
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .service(routes::routes())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
