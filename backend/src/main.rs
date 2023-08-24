// dependencies
use actix_web::{dev::ServiceRequest, web::Data, App, Error, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use tracing::{event, instrument, Level};
use tracing_subscriber::EnvFilter;

use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;

// module declaration
mod auth;
mod errors;
mod handlers;
mod models;
mod schema;

// globals (?)
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[instrument]
async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.as_ref().clone())
        .unwrap_or_else(Default::default);
    match auth::validate_token(credentials.token()).await {
        Ok(res) => {
            if res == true {
                Ok(req)
            } else {
                Err((AuthenticationError::new(config).into(), req))
            }
        }
        Err(why) => {
            event!(Level::ERROR, "Error validating token: {}", why);
            Err((AuthenticationError::new(config).into(), req))
        }
    }
}

#[tokio::main]
#[instrument]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    // std::env::set_var("RUST_LOG", "actix_web=debug");

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // database stuff
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    event!(Level::DEBUG, "database_url: {}", database_url);

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to postgres pool create pool.");
    event!(Level::DEBUG, "Connected to database!");

    Ok(HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator);
        App::new()
            .wrap(auth)
            .app_data(Data::new(pool.clone()))
            .service(handlers::get_users)
            .service(handlers::get_user_by_id)
            .service(handlers::add_user)
            .service(handlers::delete_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?)
}
