
use actix_web::{ 
    // guard::Options, 
    post, web::{Data,Json}, HttpResponse, Responder
};

use crate::models::brafie_models;
// use crate::utils::helper_functions::verify_password;

use crate::AppState;

#[post("/bra_fie/users/create")]
async fn bra_fie_create_user(state:Data<AppState>, body: Json<bra_fie_models::BraFieUser>) -> impl Responder {

    match sqlx::query_as::<_,bra_fie_models::BraFieUser>(
        "INSERT INTO bra_fie_users (fullname, email, phone, password) VALUES ($1, $2,$3, $4) RETURNING id,fullname, email,phone, password",
    )
    .bind(body.fullname.to_string())
    .bind(body.email.to_string())
    .bind(body.phone.to_string())
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
