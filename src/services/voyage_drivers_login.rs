use actix_web::{ 
    // guard::Options, 
    post, web::{Data,Json}, HttpResponse,
};

use crate::models::voyage_users_login_credentials_model;
use crate::models::voyage_driver_sign_up_model;
use crate::utils::helper_functions::verify_password;

use crate::AppState;

#[post("/voyage/drivers/login")]
async fn voyage_driver_sign_in(state: Data<AppState>, credentials: Json<voyage_users_login_credentials_model::VoyageDriverLoginCredentials>) -> Result<HttpResponse, actix_web::Error> {
    
    let email = credentials.email.clone();
    let password = credentials.password.clone();



    // 1. Validate email and password (optional)
    // You can add logic here to validate email format or password length

    let user_result = sqlx::query_as::<_, voyage_driver_sign_up_model::VoyageDriver>(
    "SELECT * FROM voyage_drivers WHERE email = $1;",
    )
    .bind(email)
    .fetch_one(&state.db)
    .await;

    // Use await? to propagate potential errors


    match user_result {
    Ok(user) => {

        let verify_password_result = verify_password(&password, &user.password);

        match verify_password_result{
            Ok(true) =>{
                Ok(HttpResponse::Ok().json(&user))

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

