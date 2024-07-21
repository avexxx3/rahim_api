use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{doc, extjson::de::Error},
    results::InsertOneResult,
    Client, Collection,
};
use crate::models::{session_model::Session, user_model::User};

pub struct MongoRepo {
    user_col: Collection<User>,
    session_col: Collection<Session>
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };  

        let client = Client::with_uri_str(uri).await.ok().expect("Error connected to client");
        let db = client.database("rustDB"); 
        let user_col: Collection<User> = db.collection("User");
        let session_col: Collection<Session> = db.collection("Session");
        MongoRepo { user_col , session_col}
    }

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };

        let user = self
            .user_col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    pub async fn upload_session_id(&self, session_id:String) -> Result<InsertOneResult, Error> {
        let new_doc = Session {
            id: None,
            session_id: session_id
        };

        let user = self
            .session_col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error uploading session id");
        Ok(user)
    }



    pub async fn verify_session_id(&self, session_id:String) -> bool {
        match self.get_session_id(session_id).await {
            Ok(_) => true,
            Err(_) => false
        }
    }

    async fn get_session_id(&self, session_id:String) -> Result<Session, Error> {
        let filter = doc! {"session_id": session_id};

        let session = self
            .session_col.find_one(filter, None)
            .await
            .ok()
            .expect("Error creating user");

        Ok(session.unwrap())
    }
}

