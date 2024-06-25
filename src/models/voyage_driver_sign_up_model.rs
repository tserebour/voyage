use serde::{Serialize, Deserialize};
use sqlx::{self, FromRow};

#[derive(Deserialize, Serialize, Debug, Clone,FromRow)]
        pub struct VoyageDriver {
            pub id: Option<i32>,
            pub fullname: String,
            pub email: String,
            pub password: String,
            pub license_number: Option<String>,
            pub vehicle_information: Option<String>,
            pub rating: Option<String>,
            pub earn_type: Option<i32>,
            pub is_online: bool,
            pub current_latitude: Option<f64>,
            pub current_longitude: Option<f64>,
        }