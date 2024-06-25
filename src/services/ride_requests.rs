

use actix_web::{ 
    // guard::Options, 
    post, web::{Data,Json}, HttpResponse
};

use crate::models::ride_request_model;
// use crate::utils::helper_functions::verify_password;

use crate::AppState;

#[post("/voyage/ride_requests")]
async fn create_ride_request(state: Data<AppState>, ride_request: Json<ride_request_model::RideRequest>) -> Result<HttpResponse, actix_web::Error> {
    println!("{:?}", ride_request);
    let result = sqlx::query_as::<_, ride_request_model::RideRequest>(
        
        "INSERT INTO ride_requests (
            user_id,
            pickup_address,
            pickup_latitude,
            pickup_longitude,
            dropoff_address,
            dropoff_latitude,
            dropoff_longitude,
            ride_type_id
            
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, user_id, pickup_address, pickup_latitude, pickup_longitude, dropoff_address, dropoff_latitude, dropoff_longitude, ride_type_id, estimated_fare, requested_at, status;"
        )
        .bind(ride_request.user_id)
        .bind(ride_request.pickup_address.clone())
        .bind(ride_request.pickup_latitude)
        .bind(ride_request.pickup_longitude)
        .bind(ride_request.dropoff_address.clone())
        .bind(ride_request.dropoff_latitude)
        .bind(ride_request.dropoff_longitude)
        .bind(ride_request.ride_type_id)   
         

        .fetch_one(&state.db)
        .await;

    match result {
        Ok(ride_request) => Ok(HttpResponse::Ok().json(ride_request)),
        Err(err) => {
            println!("Error creating ride request: {}", err);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

