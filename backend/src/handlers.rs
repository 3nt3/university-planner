use crate::errors::ServiceError;
use crate::schema::users::dsl::*;
use crate::{models::User, Pool};

use actix_web::{delete, get, post, web, Error, HttpResponse, Responder};
use diesel::RunQueryDsl;
use serde::{Deserialize, Serialize};

use derive_more::{Display, Error};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub name: String,
    pub email: String,
}

#[get("/users")]
pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
    let all_users = web::block(move || get_all_users(db)).await;
    match all_users {
        Ok(Ok(u)) => Ok(HttpResponse::Ok().json(u)),
        _ => Err(ServiceError::InternalServerError),
    }
}

fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    let items = users.load::<User>(&mut conn)?;
    Ok(items)
}

#[get("/users/{id}")]
pub async fn get_user_by_id() -> impl Responder {
    format!("hello from get user by id")
}

#[post("/users")]
pub async fn add_user() -> impl Responder {
    format!("hello from add user")
}

#[delete("/users/{id}")]
pub async fn delete_user() -> impl Responder {
    format!("hello from delete user")
}
