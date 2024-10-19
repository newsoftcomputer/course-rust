use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
extern crate diesel;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use dotenvy::dotenv;
use std::env;

extern crate tera;
use tera::Context as TeraContext;
use tera::Tera;

// imports
mod models;
mod schema;
mod services;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Database conection
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be");
    let connection = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(connection).expect("Pools error");

    HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(echo)
            .service(services::service_users::get_users)
            .service(services::service_users::new_user)
            .app_data(web::Data::new(pool.clone()))
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
