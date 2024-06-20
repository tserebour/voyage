use serde::{Serialize, Deserialize};
    use sqlx::{self, FromRow};
    // use chrono::NaiveDateTime;






    


       
        

        #[derive(Deserialize, Serialize, Debug, Clone,FromRow)]
        pub struct RideRequest {
            pub id: Option<i32>,
            pub user_id: i32,
            pub pickup_address: String,
            pub pickup_latitude: f64,
            pub pickup_longitude: f64,
            pub dropoff_address: String,
            pub dropoff_latitude: f64,
            pub dropoff_longitude: f64,
            pub ride_type_id: i32,
            pub estimated_fare: Option<f64>,
            pub requested_at: Option<String>,
            pub status: String,
            pub car_id: Option<i32>,
        }