
use actix_web::{post, web::{Data, Json}, HttpResponse};

use crate::AppState;
use crate::models::update_location_model::UpdateDriverLocation;

#[post("/voyage/driver/update_location")]
pub async fn update_driver_location(
    state: Data<AppState>,
    body: Json<UpdateDriverLocation>,
) -> HttpResponse {
    // println!("{}: {}", body.current_latitude, body.current_longitude);
    match sqlx::query(
        "UPDATE voyage_drivers
        SET current_latitude = $1, current_longitude = $2
        WHERE id = $3
        RETURNING id;",
    )
    .bind(body.current_latitude)
    .bind(body.current_longitude)
    .bind(body.id)
    .fetch_one(&state.db)
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            eprintln!("Failed to update driver location: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}