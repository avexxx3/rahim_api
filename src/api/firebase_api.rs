use crate::{models::firebase_model::CredentialsRequest, repository::mongodb_repo::MongoRepo};
use actix_web::{
    post, web::{Data, Json}, HttpResponse
};
use firebase_auth_sdk::FireAuth;

#[post("/signin")]
pub async fn sign_in(db: Data<MongoRepo>, firebase_auth: Data<FireAuth>, new_user: Json<CredentialsRequest>) -> HttpResponse {
    match firebase_auth.sign_in_email(&new_user.name, &new_user.password, true).await {
        Ok(response) => {
            match db.upload_session_id(response.id_token.clone()).await {
                Ok(_) => HttpResponse::Ok().json(response.id_token),
                Err(err) => HttpResponse::InternalServerError().body(err.to_string())
            }
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[post("/signup")]
pub async fn sign_up(db: Data<MongoRepo>, firebase_auth: Data<FireAuth>, new_user: Json<CredentialsRequest>) -> HttpResponse {
    match firebase_auth.sign_up_email(&new_user.name, &new_user.password, true).await {
        Ok(response) => {
            match db.upload_session_id(response.id_token.clone()).await {
                Ok(_) => HttpResponse::Ok().json(response.id_token),
                Err(err) => HttpResponse::InternalServerError().body(err.to_string())
            }
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}
