
pub mod models{

    // voyage models|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||\

    pub mod voyage_models{

    use serde::{Serialize, Deserialize};
    use sqlx::{self, FromRow};






        #[derive(Serialize,Deserialize, Clone,FromRow)]
        pub struct VoyageUser{
            id: Option<i32>,
            username: String,
            email: String,
            password: String,

        }

        #[derive(Serialize,Deserialize, Clone,FromRow)]
        pub struct VoyageDriver{
            id: Option<i32>,
            fullname: String,
            email: String,
            password: String,
        }



        



        #[derive(Debug)]
        #[derive(Serialize,Deserialize, Clone,FromRow)]
        struct VoyageUserLoginCredentials {
            email: String,
            password: String,
        }

        #[derive(Debug)]
        #[derive(Serialize,Deserialize, Clone,FromRow)]
        struct VoyageDriverLoginCredentials {
            email: String,
            password: String,
        }

       
        

    }





    // bra_fie models||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||


    pub mod bra_fie_models{

        use serde::{Serialize, Deserialize};
        use sqlx::{self, FromRow};



        #[derive(Debug)]
        #[derive(Serialize,Deserialize, Clone,FromRow)]
        pub struct BraFieUser{
            id: Option<i32>,
            fullname: String,
            email: String,
            phone: String,
            password: String,
                
        }


        #[derive(Debug)]
        #[derive(Serialize,Deserialize, Clone,FromRow)]
        struct BraFieLoginCredentials {
            email: String,
            password: String,
        }

    }
    
}