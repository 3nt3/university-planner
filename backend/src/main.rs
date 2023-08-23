// dependencies
use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use tracing::{event, instrument, Level};

// module declaration
mod errors;
mod handlers;
mod models;
mod schema;

// globals (?)
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[tokio::main]
#[instrument]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");

    tracing_subscriber::fmt::init();

    // database stuff
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    event!(Level::DEBUG, "database_url: {}", database_url);

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to postgres pool create pool.");
    event!(Level::DEBUG, "Connected to database!");

    HttpServer::new(|| {
        App::new()
            .service(handlers::get_users)
            .service(handlers::get_user_by_id)
            .service(handlers::add_user)
            .service(handlers::delete_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
