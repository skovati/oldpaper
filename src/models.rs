use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Account {
    name: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct FeedResponse {
    pub name: String,
    pub feeds: Vec<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Subscription {
    pub id: usize,
    pub url: String,
}
