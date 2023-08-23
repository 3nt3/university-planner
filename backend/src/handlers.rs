use actix_web::{delete, get, post, Responder};

#[get("/users")]
pub async fn get_users() -> impl Responder {
    format!("hello from get users")
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
