mod api;
mod models;
mod repository;

extern crate dotenv;

use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use api::{
    firebase_api::{sign_in, sign_up, temp},
    user_api::{create_profile, get_profile, get_profiles},
};
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
            .wrap(Cors::permissive())
            .service(sign_in)
            .service(sign_up)
            .service(create_profile)
            .service(get_profiles)
            .service(get_profile)
    })
    .bind(("0.0.0.0", 10000))?
    .run()
    .await
}
