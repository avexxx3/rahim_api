use crate::{
    models::profile_model::{
        Appearance, BioData, Family, Profile, ProfileRequest, Qualifications, Whereabouts,
    },
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
    new_profile: Json<ProfileRequest>,
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
        bio_data: BioData {
            name: new_profile.clone().name,
            is_male: new_profile.is_male,
            nationality: new_profile.clone().nationality,
            native_language: new_profile.clone().native_language,
            addictions: new_profile.clone().addictions,
        },
        hobbies: new_profile.clone().hobbies,
        whereabouts: Whereabouts {
            country: new_profile.clone().country,
            city: new_profile.clone().city,
            living_arrangement: new_profile.clone().living_arrangement,
        },
        qualifications: Qualifications {
            qualificiation: new_profile.clone().qualificiation,
            university: new_profile.clone().university,
            occupation: new_profile.clone().occupation,
            monthly_income: new_profile.clone().monthly_income,
        },
        appearance: Appearance {
            height: new_profile.clone().height,
            weight: new_profile.clone().weight,
            skin_color: new_profile.clone().skin_color,
            fit: new_profile.clone().fit,
        },
        age_range_min: new_profile.clone().age_range_min,
        age_range_max: new_profile.clone().age_range_max,
        family: Family {
            divorced: new_profile.clone().divorced,
            children: new_profile.clone().children,
            parents: new_profile.clone().parents,
            siblings: new_profile.clone().siblings,
        },
    };

    db.manage_profile(new_profile, cookie).await
}

#[get("/get_profile")]
pub async fn get_profile(
    db: Data<MongoRepo>,
    firebase: Data<FirebaseRepo>,
    request: HttpRequest,
    email: String,
) -> HttpResponse {
    let mut cookie = Cookie::new("", "");

    match firebase.fetch_email(request).await {
        Ok(response) => {
            cookie = response.0;
        }
        Err(response) => return response,
    }

    match db.get_profile(email).await {
        Ok(response) => HttpResponse::Ok().cookie(cookie).json(response),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
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
