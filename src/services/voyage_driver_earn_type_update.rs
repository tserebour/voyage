use actix_web::{ 
    // guard::Options, 
    post, web::{Data,Json}, HttpResponse
};

use crate::models::update_earn_type_model;
// use crate::utils::helper_functions::hash_password;

use crate::AppState;

#[post("/voyage/drivers/update_earn_type")]
async fn update_earn_type_to_database(state: Data<AppState>, location_data: Json<update_earn_type_model::EarnTypeUpdate>) -> Result<HttpResponse, actix_web::Error>{
    update_earn_type_model::update_earn_type_to_database(state,location_data).await


}