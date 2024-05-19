use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, Pool};
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;
pub mod handlers;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn index() -> &'static str {
    "Welcome to NecroProde!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(index))
            .configure(handlers::init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}