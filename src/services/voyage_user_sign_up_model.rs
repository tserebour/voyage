use serde::{Serialize, Deserialize};
use sqlx::{self, FromRow};
    // use chrono::NaiveDateTime;


#[derive(Serialize, Deserialize, Clone, FromRow)]
    pub struct VoyageUser {
        pub id: Option<i32>,
        pub fullname: String,
        pub email: String,
        pub password: String,
        pub phone_number: String,
        pub account_created_at: Option<String>,
        pub last_login_at: Option<String>,
        
    }