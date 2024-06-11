
pub mod models{

    // voyage models|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||\

    pub mod voyage_models{

    use serde::{Serialize, Deserialize};
    use sqlx::{self, FromRow};
    use chrono::NaiveDateTime;






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
        }


    #[derive(Debug)]
        #[derive(Serialize,Deserialize, Clone,FromRow)]
        pub struct VoyageUserLoginCredentials {
            pub email: String,
            pub password: String,
        }

        #[derive(Serialize,Deserialize, Clone,FromRow)]
        pub struct VoyageDriver{
            pub id: Option<i32>,
            pub fullname: String,
            pub email: String,
            pub password: String,
        }



        



        

        #[derive(Debug)]
        #[derive(Serialize,Deserialize, Clone,FromRow)]
        pub struct VoyageDriverLoginCredentials {
            pub email: String,
            pub password: String,
        }

    
        

    }





    // bra_fie models||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||


    pub mod bra_fie_models{

        use serde::{Serialize, Deserialize};
        use sqlx::{self, FromRow};



        #[derive(Debug)]
        #[derive(Serialize,Deserialize, Clone,FromRow)]
        pub struct BraFieUser{
            pub id: Option<i32>,
            pub fullname: String,
            pub email: String,
            pub phone: String,
            pub password: String,
                
        }


        #[derive(Debug)]
        #[derive(Serialize,Deserialize, Clone,FromRow)]
        pub struct BraFieLoginCredentials {
            pub email: String,
            pub password: String,
        }

    }
    
}