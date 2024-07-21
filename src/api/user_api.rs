use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use actix_web::{
    post, web::{Data, Json}, HttpRequest, HttpResponse
};              
use firebase_auth_sdk::FireAuth;

#[post("/user")]
pub async fn create_user(db: Data<MongoRepo>,_firebase_auth: Data<FireAuth>, new_user: Json<User>, request: HttpRequest) -> HttpResponse {
    
    let session_id_request = request.cookie("session_id");
    match session_id_request {
        Some(_) => {},
        None => return HttpResponse::InternalServerError().body("No session ID was found"),
    };
    let session_id = session_id_request.unwrap();



    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };

    if !db.verify_session_id(session_id.value().to_string()).await {
        return HttpResponse::InternalServerError().body("Failed to verify session ID");
    }

    let user_detail = db.create_user(data).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
