
use actix_web::{ 
    // guard::Options, 
    post, web::{Data,Json}, HttpResponse
};

use crate::models::brafie_models;
// use crate::utils::helper_functions::verify_password;

use crate::AppState;

#[post("/bra_fie/users/login")]
async fn bra_fie_sign_in(state: Data<AppState>, credentials: Json<brafie_models::BraFieLoginCredentials>) -> Result<HttpResponse, actix_web::Error> {
    let username = credentials.email.clone();
    let password = credentials.password.clone();

    // 1. Validate email and password (optional)
    // You can add logic here to validate email format or password length

    let user_result = sqlx::query_as::<_, brafie_models::BraFieUser>(
        "SELECT * FROM bra_fie_users WHERE email = $1;",
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

