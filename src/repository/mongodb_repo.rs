use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::extjson::de::Error,
    results::InsertOneResult,
    Client, Collection,
};
use crate::models::user_model::User;

pub struct MongoRepo {
    user_col: Collection<User>
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };  

        let client = Client::with_uri_str(uri).await.ok().expect("Error connected to client");
        let db = client.database("Populace"); 
        let user_col: Collection<User> = db.collection("User");
        MongoRepo { user_col }
    }

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            email: new_user.email
        };

        let user = self
            .user_col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating user");
        Ok(user)
    }
}