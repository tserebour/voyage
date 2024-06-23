use actix_web::{ 
    // guard::Options, 
    post, web::{Data,Json}, HttpResponse,
};

use crate::AppState;


use serde::{Serialize, Deserialize};
    use sqlx::{self, FromRow};


#[derive(Deserialize, Serialize, Debug, Clone,FromRow)]
struct UpdateDriverStatus {
    is_online: bool,
    driver_id: i32,

}

#[post("/voyage/driver/update_is_online")]
async fn update_driver_status(
    state: Data<AppState>,body: Json<UpdateDriverStatus>
) -> HttpResponse {
    match sqlx::query_as::<_,()>(
        "UPDATE voyage_drivers
        SET is_online = $1
        WHERE id = $2
        RETURNING  id;",
    )
    .bind(body.is_online)
    .bind(body.driver_id)
    
    .fetch_one(&state.db)
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            eprintln!("Failed to update Earning type: {:?}", err);
            
            HttpResponse::InternalServerError().finish()
            
        }
    }
}