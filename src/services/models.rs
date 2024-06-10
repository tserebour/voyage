
pub mod models{

    // voyage models|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||\

    pub mod voyage_models{

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
        pub account_created_at: i32,
        pub last_login_at: Option<i32>,
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