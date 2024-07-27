use std::ops::Range;

use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    pub public: bool,
    pub bio_data: BioData,
    pub about_oneself: String,
    pub hobbies: String,
    pub whereabouts: Whereabouts,
    pub qualifications: Qualifications,
    pub appearance: Appearance,
    pub age_range: Range<i32>
}           

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BioData {
    pub name: String,
    pub is_male: bool,
    pub nationality: String,
    pub native_language: String,
    pub addictions: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Appearance {
    pub height: i32,
    pub weight: i32,
    pub skin_color: String,
    pub looksmaxxed: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Whereabouts {
    pub country: String,
    pub city: String,
    pub living_arrangement: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Qualifications {
    pub qualificiation: String,
    pub university: String,
    pub occupation: String,
    pub monthly_income_min: Range<i32>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Family {
    pub divorced: bool,
    pub children: i32,
    pub parents: bool,
    pub siblings: bool
}