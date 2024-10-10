use actix_web::{get, post, web, HttpResponse, Responder};
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};
use serde_json::json;

use crate::models::model_users::{ModelUsers, StructHandlerUsers};

#[post("/services/users/newuser")]
pub async fn new_user(
    pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
    item: web::Json<StructHandlerUsers>,
) -> impl Responder {
    let mut conn = pool.get().expect("Error to connect");
    match web::block(move || ModelUsers::new_user(&mut conn, &item)).await {
        Ok(data) => {
            let data = data.unwrap();
            HttpResponse::Ok().json(json!(data))
        }
        Err(err) => HttpResponse::Ok().body(err.to_string()),
    }
}

#[get("/services/users/getusers")]
pub async fn get_users(
    pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    let mut conn = pool.get().expect("Connection error");

    match web::block(move || ModelUsers::get_users(&mut conn)).await {
        Ok(data) => {
            let data = data.unwrap();
            HttpResponse::Ok().json(json!(data))
        }

        Err(err) => HttpResponse::Ok().body(err.to_string()),
    }
}
