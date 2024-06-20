use actix_web::{ 
    // guard::Options, 
    post, web::{Data,Json}, HttpResponse, Responder
};

use crate::models::voyage_driver_sign_up_model;
use crate::utils::helper_functions::hash_password;

use crate::AppState;

#[post("/voyage/drivers/create")]
async fn voyage_create_driver(state:Data<AppState>, body: Json<voyage_driver_sign_up_model::VoyageDriver>) -> impl Responder {

    match sqlx::query_as::<_, voyage_driver_sign_up_model::VoyageDriver>(
        "INSERT INTO voyage_drivers (fullname, email, password, license_number, vehicle_information) 
         VALUES ($1, $2, $3, $4, $5)
         RETURNING id, fullname, email, password, license_number, vehicle_information, rating",
    )
    .bind(body.fullname.to_string())
    .bind(body.email.to_string())
    .bind(hash_password(&body.password).unwrap().clone())
    .bind(body.license_number.clone())
    .bind(body.vehicle_information.clone())
    
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

