use crate::{models::user_model::User, repository::{firebase_repo::FirebaseRepo, mongodb_repo::MongoRepo}};
use actix_web::{
    cookie::Cookie, post, web::{Data, Json}, HttpRequest, HttpResponse
};              

#[post("/user")]
pub async fn create_profile(db: Data<MongoRepo>, firebase: Data<FirebaseRepo>, new_user: Json<User>, request: HttpRequest) -> HttpResponse {
    let mut cookie = Cookie::new("", "");
    let mut email = "".to_string();

    match firebase.fetch_email(request).await {
        Ok(response) => {
            email = response.1;
            cookie = response.0;
        },
        Err(response) => return response,
    }

    todo!();
}

