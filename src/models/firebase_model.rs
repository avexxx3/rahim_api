use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CredentialsRequest{
    pub name: String,
    pub password: String
}

#[derive(Serialize)]
pub struct Response{
    pub message:String
}