use actix_web::{cookie::time::Date, web::Data};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow};
use chrono::{DateTime, Utc};


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User{
pub id: i32,
pub username: String,
pub email: String,
pub created_at: Option<DateTime<Utc>>
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest{
    pub username: Option<String>,
    pub email: Option<String>
}