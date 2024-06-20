
use actix_web::{ 
    // guard::Options, 
    post, web::{Data,Json}, HttpResponse
};

use crate::models::voyage_user_sign_up_model;
use crate::models::voyage_users_login_credentials_model;

use crate::utils::helper_functions::verify_password;

use crate::AppState;
#[post("/voyage/users/login")]
async fn voyage_user_sign_in(state: Data<AppState>, credentials: Json<voyage_users_login_credentials_model::VoyageDriverLoginCredentials>) -> Result<HttpResponse, actix_web::Error> {
    let email = credentials.email.clone();
    let plain_password = credentials.password.clone();
    
    // 1. Validate email and password (optional)
    // You can add logic here to validate email format or password length
    
    let user_result = sqlx::query_as::<_, voyage_user_sign_up_model::VoyageUser>(
        "SELECT * FROM voyage_users WHERE email = $1;",
    )
    .bind(email)
    .fetch_one(&state.db)
    .await; // Use await? to propagate potential errors
    
    
    match user_result {
        Ok(user) => {
            let verify_password_result = verify_password(&plain_password, &user.password);
    
            match verify_password_result{
    
                Ok(true) => {
    
    
                    let id = user.id.unwrap();
                    
    
                    let user_last_login_update = sqlx::query_as::<_, voyage_user_sign_up_model::VoyageUser>(
                        "UPDATE voyage_users SET last_login_at = CURRENT_TIMESTAMP WHERE id = $1 
                        RETURNING id, fullname, email, password, phone_number, account_created_at, last_login_at;",
                    )
                    .bind(id)
                    .fetch_one(&state.db)
                    .await; // Use await? 
    
    
                
            
                    match user_last_login_update{
                        Ok(_) =>{
                            Ok(HttpResponse::Ok().json(&user))
                        },
                        Err(err) => {
                            println!("Error updating last login time: {}", err);
                            return Err(actix_web::error::ErrorInternalServerError(err.to_string()));
                        }
                    }
            
                    
                },
                Ok(false) => {
                    return Err(actix_web::error::ErrorInternalServerError("Invalid email or password"));
    
                },
    
                Err(err) => {
                    println!("Error verifying password: {}", err);
                    return Err(actix_web::error::ErrorInternalServerError(err.to_string()));
                }
    
    
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
    
    