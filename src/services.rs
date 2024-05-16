use actix_web::{get, 
                post, 
                web::{Data,Json,Path}, 
                HttpResponse, 
                Responder
            };

            
use sqlx::{self, error, FromRow};


use serde::{Serialize, Deserialize};

use crate::AppState;


#[derive(Serialize,Deserialize, Clone,FromRow)]
pub struct User{
    id: Option<i32>,
    username: String,
    email: String,
    password: String,
    // mobile_number: String,
    // plate_number: String,
    // license_number: String,
    // station_name: String,
}



#[get("/")]
async fn index() -> impl Responder {
    format!("login route")
}

#[post("/create")]
async fn create_user(state:Data<AppState>, body: Json<User>) -> impl Responder {
    
    match sqlx::query_as::<_,User>(
        "INSERT INTO users (username, email, password) VALUES ($1, $2, $3) RETURNING id,username, email, password",
    )
    .bind(body.username.to_string())
    .bind(body.email.to_string())
    .bind(body.password.to_string())
   
    .fetch_one(&state.db)
    .await
      {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) =>{
            println!("Error creating user: {}", err);  // Log the actual error
            HttpResponse::InternalServerError().finish()  
        }

        
    }
}




#[derive(Debug)]
#[derive(Serialize,Deserialize, Clone,FromRow)]
struct LoginCredentials {
    username: String,
    password: String,
}




// #[post("/login")]

// async fn sign_in(state:Data<AppState>, credentials: Json<LoginCredentials>) -> impl Responder {
//     let username = credentials.username.clone();
//     let password = credentials.password.clone();

//     // 1. Validate email and password (optional)
//     // You can add logic here to validate email format or password length

//     let user = sqlx::query_as::<_, User>(
//         "SELECT * FROM users WHERE username = $1;",
//     )
//     .bind(username)
//     .fetch_one(&state.db)
//     .await;

    

//     // 2. Verify password (replace with your password hashing logic)
//     // This example assumes password is stored in plain text (not recommended)
//     // if user().unwrap().clone().password != password {
//     //     return Err(actix_web::error::ErrorUnauthorized("Invalid email or password"));
//     // }


//     let person:User = user.unwrap();

//     match user {
//         Ok(_) => {
//             // Successful login
//             println!("Success");
//             return Ok(HttpResponse::Ok().json(person));
            
//         } // Do something specific on success (optional)
//         Err(err) => {

//     return Err(HttpResponse::InternalServerError().body("Error fetching user"));

//             // Handle different error types (e.g., database errors vs. data errors)
//             // match err {
//             //     sqlx::Error::Database(inner) => {
//             //         println!("Database error: {}", inner);
//             //         Err(HttpResponse::InternalServerError().body("Error fetching user"));
//             //     },
//             //     sqlx::Error::RowNotFound => {
//             //         println!("User not found");
//             //         Err(HttpResponse::InternalServerError().body("Error fetching user"));
//             //     },
//             //     _ => {
//             //         println!("Unexpected error: {}", err);
//             //         Err(HttpResponse::InternalServerError().body("Error fetching user"));
//             //     }, // Handle other errors generically
//             // }


            
            
//         }

//     }


    

    
// }



#[post("/login")]
async fn sign_in(state: Data<AppState>, credentials: Json<LoginCredentials>) -> Result<HttpResponse, actix_web::Error> {
    let username = credentials.username.clone();
    let password = credentials.password.clone();

    // 1. Validate email and password (optional)
    // You can add logic here to validate email format or password length

    let user_result = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE username = $1;",
    )
    .bind(username)
    .fetch_one(&state.db)
    .await; // Use await? to propagate potential errors


    match user_result {
        Ok(user) => {
            if user.password != password {
                Ok(HttpResponse::Ok().json("Invalid Username and Password"))
                
            }else {

                Ok(HttpResponse::Ok().json(&user))

            }
        },
        Err(err) => {
            // Convert sqlx::Error to actix_web::Error
            let actix_err = actix_web::error::ErrorInternalServerError(err.to_string());
            return Err(actix_err);
        }
    }


    
   
    // println!("Success");
    // Ok(HttpResponse::Ok().json(user_result.unwrap().clone()))
}
