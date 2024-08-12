use std::env;
extern crate dotenv;
use actix_web::{cookie::Cookie, HttpResponse};
use dotenv::dotenv;

use crate::models::{profile_model::Profile, user_model::User};
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult},
    Client, Collection,
};

pub struct MongoRepo {
    user_col: Collection<User>,
    profile_col: Collection<Profile>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };

        let client = Client::with_uri_str(uri)
            .await
            .ok()
            .expect("Error connected to client");
        let db = client.database("Populace");
        let user_col: Collection<User> = db.collection("User");
        let profile_col: Collection<Profile> = db.collection("Profile");
        MongoRepo {
            user_col,
            profile_col,
        }
    }

    pub async fn initalize_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            email: new_user.email,
        };

        let user = self
            .user_col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    pub async fn manage_profile(&self, new_profile: Profile, cookie: Cookie<'_>) -> HttpResponse {
        self.delete_user(&new_profile.clone().email).await;

        match self.create_profile(new_profile).await {
            Ok(response) => HttpResponse::Ok().cookie(cookie).json(response),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),  
        }
    }

    pub async fn delete_user(&self, email: &String) {
        let filter = doc! {"email": email};
        let user_detail = self
            .profile_col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting user");
    }

    pub async fn create_profile(&self, new_profile: Profile) -> Result<InsertOneResult, Error>   {
        let user = self
            .profile_col
            .insert_one(new_profile, None)
            .await
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    pub async fn get_profiles(&self) -> Result<Vec<Profile>, Error> {
        let filter = doc! {"public": true};

        let mut cursors = self
            .profile_col
            .find(filter, None)
            .await
            .ok()
            .expect("Error getting list of users");
        let mut users: Vec<Profile> = Vec::new();
        while let Some(user) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            users.push(user)
        }

        Ok(users)
    }
}
