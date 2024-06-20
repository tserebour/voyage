
use actix_web::{ 
    // guard::Options, 
    post, web::{Data,Json}, HttpResponse, Responder
};

use crate::models::voyage_user_sign_up_model;
use crate::utils::helper_functions::hash_password;

use crate::AppState;
#[post("/voyage/users/create")]
async fn voyage_create_user(state: Data<AppState>,body: Json<voyage_user_sign_up_model::VoyageUser>,) -> impl Responder {


    
    // let now = chrono::Utc::now().naive_utc();
    
    let user = voyage_user_sign_up_model::VoyageUser {
        id: None,
        fullname: body.fullname.clone(),
        email: body.email.clone(),
        password: hash_password(&body.password).unwrap().clone(),
        phone_number: body.phone_number.clone(),
        account_created_at: None,
        last_login_at: None,
        
    };

    match sqlx::query_as::<_, voyage_user_sign_up_model::VoyageUser>(
        "INSERT INTO voyage_users (fullname, email, password, phone_number,)
        VALUES ($1, $2, $3, $4, $5,)
        RETURNING id, fullname, email, password, phone_number, account_created_at, last_login_at",
    )
    .bind(&user.fullname)
    .bind(&user.email)
    .bind(&user.password)
    .bind(&user.phone_number)
    
    
    .fetch_one(&state.db)
    .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => {
            println!("Error creating user: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}



