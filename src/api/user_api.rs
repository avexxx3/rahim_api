use crate::{models::user_model::{User, UserRequest}, repository::mongodb_repo::MongoRepo};
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};              
use firebase_auth_sdk::FireAuth;

#[post("/user")]
pub async fn create_user(db: Data<MongoRepo>,firebase_auth: Data<FireAuth>, new_user: Json<UserRequest>) -> HttpResponse {
    let is_verified = firebase_auth.verify_id_token(&new_user.auth_token).await;

    match is_verified {
        Ok(_) => {},
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string())
    };

    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };

    let user_detail = db.create_user(data).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
