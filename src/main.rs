// add the modules (import modules)
mod api;
mod models;
mod repository;

//use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
//modify imports below
//use actix_web::{web::Data, App, HttpServer};
//use api::user_api::create_user;
use api::user_api::{create_user, get_user, update_user, delete_user, get_all_users};
use repository::mongodb_repo::MongoRepo;

#[get("/")]
async fn welcome() -> impl Responder {
    HttpResponse::Ok().json("Hello from Rust and mongoDB")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(welcome)
            .service(create_user)
            .service(get_user)
            .service(update_user)
            .service(delete_user)
            .service(get_all_users)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
