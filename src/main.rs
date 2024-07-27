mod api; 
mod models;
mod repository;

extern crate dotenv;

use actix_web::{web::Data, App, HttpServer};
use api::{firebase_api::{sign_in, sign_up}, user_api::{create_profile, get_profiles}};
use repository::{firebase_repo::FirebaseRepo, mongodb_repo::MongoRepo};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    let firebase = FirebaseRepo::init().await;
    let firebase_data = Data::new(firebase);

    HttpServer::new(move || {
            App::new()
                .app_data(db_data.clone())
                .app_data(firebase_data.clone())
                .service(sign_in)
                .service(sign_up)
                .service(create_profile)
                .service(get_profiles)
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
