
use serde::{Serialize, Deserialize};
use sqlx::{self, FromRow};
    // use chrono::NaiveDateTime;

#[derive(Debug)]
#[derive(Serialize,Deserialize, Clone,FromRow)]

pub struct VoyageUserLoginCredentials {
        pub email: String,
        pub password: String,
    }


    #[derive(Debug)]
        #[derive(Serialize,Deserialize, Clone,FromRow)]
        pub struct VoyageDriverLoginCredentials {
            pub email: String,
            pub password: String,
        }