use actix_web::{ 
    // guard::Options, 
    post, web::{Data,Json}, HttpResponse
};

use crate::models::driver_location_update_model;
// use crate::utils::helper_functions::verify_password;

use crate::AppState;

#[post("/voyage/drivers/location_update_create")]
async fn add_driver_location_to_database(state: Data<AppState>, location_data: Json<driver_location_update_model::DriverLocationUpdate>) -> Result<HttpResponse, actix_web::Error>{
    driver_location_update_model::add_driver_location_to_database(state,location_data).await

   
}