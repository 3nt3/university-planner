use crate::errors::ServiceError;
use crate::models::NewUser;
use crate::schema::users::dsl::*;
use crate::{models::User, Pool};

use actix_web::{delete, get, post, web, HttpResponse, Responder};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};
use tracing::{event, instrument, Level};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub name: String,
    pub email: String,
}

#[instrument]
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

#[instrument]
#[get("/users/{user_id}")]
pub async fn get_user_by_id(
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = user_id.into_inner();
    let specific_user = web::block(move || db_get_user_by_id(db, user_id)).await;

    event!(Level::DEBUG, "specific_user: {:?}", specific_user);

    match specific_user {
        Ok(Ok(u)) => Ok(HttpResponse::Ok().json(u)),
        Ok(Err(diesel::result::Error::NotFound)) => Err(ServiceError::NotFound),
        _ => Err(ServiceError::InternalServerError),
    }
}

fn db_get_user_by_id(pool: web::Data<Pool>, user_id: i32) -> Result<User, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    users.find(user_id).get_result::<User>(&mut conn)
}

#[instrument]
#[post("/users")]
pub async fn add_user(
    db: web::Data<Pool>,
    new_user: web::Json<InputUser>,
) -> Result<HttpResponse, ServiceError> {
    let new_user = new_user.into_inner();
    let created_user = web::block(move || db_add_user(db, new_user)).await;

    match created_user {
        Ok(Ok(u)) => Ok(HttpResponse::Ok().json(u)),
        // this can be Err(err) or Ok(Err(err)) but we don't have to convert the different error
        // types
        _ => {
            event!(Level::ERROR, "error inserting new user: {:?}", created_user);
            Err(ServiceError::InternalServerError)
        }
    }
}

fn db_add_user(pool: web::Data<Pool>, new_user: InputUser) -> Result<User, diesel::result::Error> {
    let new_user = NewUser {
        name: &new_user.name,
        email: &new_user.email,
        created_at: chrono::Local::now().naive_local(),
    };

    let mut conn = pool.get().unwrap();
    diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut conn)?;

    // NOTE: this _could_ fail if there are multiple users created at the same time
    let inserted_user = users.order(id.desc()).first(&mut conn)?;

    Ok(inserted_user)
}

#[instrument]
#[delete("/users/{id}")]
pub async fn delete_user(
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = user_id.into_inner();

    match web::block(move || db_delete_user(db, user_id)).await {
        Ok(Ok(_)) => Ok(HttpResponse::NoContent().finish()),
        Ok(Err(diesel::result::Error::NotFound)) => Err(ServiceError::NotFound),
        deleted_user => {
            event!(Level::ERROR, "error deleting user: {:?}", deleted_user);
            Err(ServiceError::InternalServerError)
        }
    }
}

fn db_delete_user(pool: web::Data<Pool>, user_id: i32) -> Result<(), diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    diesel::delete(users.find(user_id)).execute(&mut conn)?;
    Ok(())
}
