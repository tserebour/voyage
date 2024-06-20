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