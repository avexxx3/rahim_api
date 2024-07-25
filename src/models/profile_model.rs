use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    pub name: String,
    pub gender: Gender
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Temp {

}

#[derive(Debug, Serialize, Deserialize)]

pub enum Gender {
    Male,
    Female,
    Femboy
}