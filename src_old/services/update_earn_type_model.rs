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
pub struct EarnTypeUpdate {
    pub driver_id: i32,
    pub earn_type: Option<i32>,
   
}


pub async fn update_earn_type_to_database(state: Data<AppState>,body: Json<EarnTypeUpdate>,) -> Result<HttpResponse, actix_web::Error>{
    match sqlx::query_as::<_,()>(
        "UPDATE voyage_drivers
        SET earn_type = $1
        WHERE id = $2
        RETURNING  id;",
    )
    .bind(body.earn_type)
    .bind(body.driver_id)
    
    .fetch_one(&state.db)
    .await
    {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(err) => {
            eprintln!("Failed to update Earning type: {:?}", err);
            
            Err(actix_web::error::ErrorInternalServerError(err.to_string()))
            
        }
    }
}