use std::env;
extern crate dotenv;
use actix_web::{
    cookie::{Cookie, CookieBuilder},
    web::Data,
    HttpRequest, HttpResponse,
};
use dotenv::dotenv;

use crate::models::{firebase_model::CredentialsRequest, user_model::User};
use firebase_auth_sdk::FireAuth;

use super::mongodb_repo::MongoRepo;

pub struct FirebaseRepo {
    auth: FireAuth,
}

impl FirebaseRepo {
    pub async fn init() -> Self {
        dotenv().ok();

        let firebase_api_key = match env::var("FIREBASEWEBAPI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };

        let auth = FireAuth::new(firebase_api_key.clone());

        FirebaseRepo { auth }
    }

    pub async fn sign_in(&self, credentials: CredentialsRequest) -> HttpResponse {
        match self
            .auth
            .sign_in_email(&credentials.email, &credentials.password, true)
            .await
        {
            Ok(response) => {
                println!("A");
                return HttpResponse::Ok().json(response)}
                ,
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    }

    pub async fn sign_up(
        &self,
        credentials: CredentialsRequest,
        db: Data<MongoRepo>,
    ) -> HttpResponse {
        match self
            .auth
            .sign_up_email(&credentials.email, &credentials.password, true)
            .await
        {
            Ok(response) => {
                match db
                    .initalize_user(User {
                        id: None,
                        email: credentials.email,
                    })
                    .await
                {
                    Ok(_) => HttpResponse::Ok().json(response),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                }
            }
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    }

    pub async fn fetch_email(
        &self,
        request: HttpRequest,
    ) -> Result<(Cookie, String), HttpResponse> {
        let mut session_id = FirebaseRepo::extract_session_id(request.clone());

        if session_id.is_empty() {
            return Err(HttpResponse::InternalServerError().body("Failed to extract session ID"));
        }

        let mut email = self.verify_session_id(&session_id).await;

        if email == "null" {
            let refresh_id = FirebaseRepo::extract_refresh_id(request.clone());

            if refresh_id == "null" {
                return Err(
                    HttpResponse::InternalServerError().body("Failed to extract refresh ID")
                );
            }

            let refresh_response = self.refresh_session_id(refresh_id).await;

            if refresh_response == "null" {
                return Err(
                    HttpResponse::InternalServerError().body("Failed to refresh session ID")
                );
            }

            session_id = refresh_response.clone();
            email = self.verify_session_id(&session_id).await
        }

        let cookie = CookieBuilder::new("session_id".to_string(), session_id).finish();

        return Ok((cookie, email));
    }

    async fn verify_session_id(&self, session_id: &String) -> String {
        let user_info = self.auth.get_user_info(&session_id).await;

        match user_info {
            Ok(user) => user.email,
            Err(_) => "null".to_string(),
        }
    }

    async fn refresh_session_id(&self, refresh_id: String) -> String {
        let a = self.auth.refresh_id_token(refresh_id.as_str()).await;

        match a {
            Ok(response) => response.id_token,
            Err(_) => "null".to_string(),
        }
    }

    fn extract_session_id(request: HttpRequest) -> String {
        let session_id_request = request.cookie("session_id");

        match session_id_request {
            Some(_) => return session_id_request.unwrap().value().to_string(),
            None => return "null".to_string(),
        };
    }

    fn extract_refresh_id(request: HttpRequest) -> String {
        let refresh_id_request = request.cookie("refresh_id");

        match refresh_id_request {
            Some(_) => return refresh_id_request.unwrap().value().to_string(),
            None => return "null".to_string(),
        };
    }
}
