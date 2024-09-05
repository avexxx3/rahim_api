use crate::{
    models::profile_model::Profile,
    repository::{firebase_repo::FirebaseRepo, mongodb_repo::MongoRepo},
};
use actix_web::{
    cookie::Cookie,
    get, post,
    web::{Data, Json},
    HttpRequest, HttpResponse,
};

#[post("/create_profile")]
pub async fn create_profile(
    db: Data<MongoRepo>,
    firebase: Data<FirebaseRepo>,
    new_profile: Json<Profile>,
    request: HttpRequest,
) -> HttpResponse {
    let mut cookie = Cookie::new("", "");
    let mut email = "".to_string();

    match firebase.fetch_email(request).await {
        Ok(response) => {
            email = response.1;
            cookie = response.0;
        }
        Err(response) => return response,
    }

    let new_profile = Profile {
        id: None,
        email: email,
        public: new_profile.clone().public,
        about_oneself: new_profile.clone().about_oneself,
        bio_data: new_profile.clone().bio_data,
        hobbies: new_profile.clone().hobbies,
        whereabouts: new_profile.clone().whereabouts,
        qualifications: new_profile.clone().qualifications,
        appearance: new_profile.clone().appearance,
        age_range_min: new_profile.clone().age_range_min,
        age_range_max: new_profile.clone().age_range_max,
    };

    db.manage_profile(new_profile, cookie).await
}

#[get("/get_profiles")]
pub async fn get_profiles(
    db: Data<MongoRepo>,
    firebase: Data<FirebaseRepo>,
    request: HttpRequest,
) -> HttpResponse {
    let mut cookie = Cookie::new("", "");

    match firebase.fetch_email(request).await {
        Ok(response) => {
            cookie = response.0;
        }
        Err(response) => return response,
    }

    match db.get_profiles().await {
        Ok(response) => HttpResponse::Ok().cookie(cookie).json(response),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
