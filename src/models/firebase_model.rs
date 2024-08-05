use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CredentialsRequest{
    pub email: String,
    pub password: String
}

#[derive(Serialize)]
pub struct Response{
    pub message:String
}
