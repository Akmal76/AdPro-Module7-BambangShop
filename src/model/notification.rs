use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]

pub struct Notification {
    pub product_title: String,
    pub product_type: String,
    pub product_url: String,
    pub subscribe_name: String,
    pub status: String
}