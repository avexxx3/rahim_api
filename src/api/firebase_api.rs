use crate::{models::firebase_model::CredentialsRequest, repository::{firebase_repo::FirebaseRepo, mongodb_repo::MongoRepo}};
use actix_web::{
    post, web::{Data, Json}, HttpResponse
};

#[post("/signin")]
pub async fn sign_in(firebase: Data<FirebaseRepo>, new_user: Json<CredentialsRequest>) -> HttpResponse {
    let data = CredentialsRequest {
        email: new_user.email.clone(),
        password: new_user.password.clone()
    };

    return firebase.sign_in(data).await;    
}

#[post("/signup")]
pub async fn sign_up(db: Data<MongoRepo>, firebase: Data<FirebaseRepo>, new_user: Json<CredentialsRequest>) -> HttpResponse {
    let data = CredentialsRequest {
        email: new_user.email.clone(),
        password: new_user.password.clone()
    };


    return firebase.sign_up(data, db).await;    
}