
use serde::{Deserialize, Serialize};
use crate::AppState;
use sqlx::{self, FromRow};
use actix_web::{
    // get, 
    // post, 
    web::{Data,Json},

// , Error, 
HttpResponse, 
// Responder
};

#[derive(Deserialize, Serialize, Debug, Clone,FromRow)]
pub struct DriverLocationUpdate {
    pub driver_id: Option<i32>,
    pub latitude: f64,
    pub longitude: f64,
    pub previous_latitude: Option<f64>,
    pub previous_longitude: Option<f64>,
    pub timestamp: Option<String>,
}

impl DriverLocationUpdate{
    


    
    


}



pub async fn add_driver_location_to_database(state: Data<AppState>,body: Json<DriverLocationUpdate>,) -> Result<HttpResponse, actix_web::Error>{
    match sqlx::query_as::<_, DriverLocationUpdate>(
        "INSERT INTO driver_locations (driver_id, latitude, longitude, previous_latitude, previous_longitude, timestamp)
        VALUES ($1, $2, $3, $4, $5,$6) RETURNING id, driver_id, latitude, longitude, previous_latitude, previous_longitude, timestamp;",
    )
    .bind(body.driver_id)
    .bind(body.latitude)
    .bind(body.longitude)
    .bind(body.previous_latitude)
    .bind(body.previous_longitude)
    .bind(body.timestamp.clone())

    .fetch_one(&state.db)
    .await
    {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(err) => {
            eprintln!("Failed to update driver location: {:?}", err);
            Err(actix_web::error::ErrorInternalServerError(err.to_string()))
            
        }
    }
}