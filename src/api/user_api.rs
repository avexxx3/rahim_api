use crate::{
    models::profile_model::{
        Appearance, BioData, Family, Profile, ProfileRequest, Qualifications, Whereabouts,
    },
    repository::{firebase_repo::FirebaseRepo, mongodb_repo::MongoRepo},
};
use actix_web::{
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
    let mut session_id: Option<String> = None;
    let mut email: Option<String> = None;

    match firebase.fetch_email(request).await {
        Ok(response) => {
            session_id.replace(response.0);
            email.replace(response.1);
        }
        Err(response) => return response,
    }

    let new_profile = Profile {
        id: None,
        email: email.unwrap(),
        public: new_profile.clone().public,
        about_oneself: new_profile.clone().about_oneself,
        bio_data: BioData {
            name: new_profile.clone().name,
            age: new_profile.clone().age,
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
        family: Family {
            divorced: new_profile.clone().divorced,
            children: new_profile.clone().children,
            parents: new_profile.clone().parents,
            siblings: new_profile.clone().siblings,
        },
    };

    db.manage_profile(new_profile, session_id.unwrap()).await
}

#[get("/get_profile")]
pub async fn get_profile(
    db: Data<MongoRepo>,
    firebase: Data<FirebaseRepo>,
    request: HttpRequest,
) -> HttpResponse {
    let mut session_id: Option<String> = None;
    let mut email: Option<String> = None;

    match firebase.fetch_email(request).await {
        Ok(response) => {
            session_id.replace(response.0);
            email.replace(response.1);
        }
        Err(response) => return response,
    }


    match db.get_profile(email.unwrap()).await {
        Ok(response) => HttpResponse::Ok().append_header(("Set-Cookie", session_id.unwrap())).json(response),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/get_profiles")]
pub async fn get_profiles(
    db: Data<MongoRepo>,
    firebase: Data<FirebaseRepo>,
    request: HttpRequest,
) -> HttpResponse {
    let mut session_id: Option<String> = None;

    match firebase.fetch_email(request).await {
        Ok(response) => {
            session_id.replace(response.0);
        }
        Err(response) => return response,
    }

    match db.get_profiles().await {
        Ok(response) => HttpResponse::Ok().append_header(("Set-Cookie", session_id.unwrap())).json(response),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
