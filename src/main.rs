mod api; 
mod models;
mod repository;

use std::env;
extern crate dotenv;

use actix_web::{web::Data, App, HttpServer};
use api::{firebase_api::{sign_in, sign_up}, user_api::create_user};
use firebase_auth_sdk::FireAuth;
use repository::mongodb_repo::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    let firebase_api_key = match env::var("FIREBASEWEBAPI") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable"),
    };

    let auth_service = FireAuth::new(firebase_api_key.clone());
    let auth_data = Data::new(auth_service);

    HttpServer::new(move || {
            App::new()
                .app_data(db_data.clone())
                .app_data(auth_data.clone())
                .service(sign_in)
                .service(sign_up)
                .service(create_user)
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}