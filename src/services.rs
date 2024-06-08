use actix_web::{get, 
    post, 
    web::{Data,Json}, 
    HttpResponse, 
    Responder
};




use crate::AppState;
mod models;

use models::models::voyage_models;
use models::models::bra_fie_models;












#[get("/")]
async fn index() -> impl Responder {
    format!("login route")
}

#[post("/voyage/users/create")]
async fn voyage_create_user(state:Data<AppState>, body: Json<voyage_models::VoyageUser>) -> impl Responder {

    match sqlx::query_as::<_,voyage_models::VoyageUser>(
        "INSERT INTO voyage_users (fullname, email, password) VALUES ($1, $2, $3) RETURNING id,fullname, email, password",
    )
    .bind(body.fullname.to_string())
    .bind(body.email.to_string())
    .bind(body.password.to_string())

    .fetch_one(&state.db)
    .await


    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) =>{
        println!("Error creating user: {}", err); 
        HttpResponse::InternalServerError().finish()
    }


    }
}










#[post("/voyage/users/login")]
async fn voyage_user_sign_in(state: Data<AppState>, credentials: Json<voyage_models::VoyageDriverLoginCredentials>) -> Result<HttpResponse, actix_web::Error> {
let email = credentials.email.clone();
let password = credentials.password.clone();

// 1. Validate email and password (optional)
// You can add logic here to validate email format or password length

let user_result = sqlx::query_as::<_, voyage_models::VoyageUser>(
    "SELECT * FROM voyage_users WHERE email = $1;",
)
.bind(email)
.fetch_one(&state.db)
.await; // Use await? to propagate potential errors


match user_result {
    Ok(user) => {
    if user.password != password {
        Ok(HttpResponse::Ok().json("Invalid Username and Password"))
        
    }else {

        Ok(HttpResponse::Ok().json(&user))

    }
},
    Err(err) => {
        // Convert sqlx::Error to actix_web::Error
        let actix_err = actix_web::error::ErrorInternalServerError(err.to_string());
        return Err(actix_err);
    }
}




// println!("Success");
// Ok(HttpResponse::Ok().json(user_result.unwrap().clone()))
}


#[post("/bra_fie/users/login")]
async fn bra_fie_sign_in(state: Data<AppState>, credentials: Json<bra_fie_models::BraFieLoginCredentials>) -> Result<HttpResponse, actix_web::Error> {
    let username = credentials.email.clone();
    let password = credentials.password.clone();

    // 1. Validate email and password (optional)
    // You can add logic here to validate email format or password length

    let user_result = sqlx::query_as::<_, bra_fie_models::BraFieUser>(
        "SELECT * FROM bra_fie_users WHERE email = $1;",
    )
    .bind(username)
    .fetch_one(&state.db)
    .await; // Use await? to propagate potential errors


    match user_result {
    Ok(user) => {
        if user.password != password {
            Ok(HttpResponse::Ok().json("Invalid Username and Password"))
            
        }else {

            Ok(HttpResponse::Ok().json(&user))

    }
    },
    Err(err) => {
        // Convert sqlx::Error to actix_web::Error
        let actix_err = actix_web::error::ErrorInternalServerError(err.to_string());
        return Err(actix_err);
    }
    }




// println!("Success");
// Ok(HttpResponse::Ok().json(user_result.unwrap().clone()))
}


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


#[post("/voyage/drivers/create")]
async fn voyage_create_driver(state:Data<AppState>, body: Json<voyage_models::VoyageDriver>) -> impl Responder {

    match sqlx::query_as::<_,voyage_models::VoyageDriver>(
        "INSERT INTO drivers_users (fullname, email, password) VALUES ($1, $2, $3) RETURNING id,fullname, email, password",
    )
    .bind(body.fullname.to_string())
    .bind(body.email.to_string())
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


#[post("/voyage/drivers/login")]
async fn voyage_driver_sign_in(state: Data<AppState>, credentials: Json<voyage_models::VoyageDriverLoginCredentials>) -> Result<HttpResponse, actix_web::Error> {
    
    let email = credentials.email.clone();
    let password = credentials.password.clone();

    // 1. Validate email and password (optional)
    // You can add logic here to validate email format or password length

    let user_result = sqlx::query_as::<_, voyage_models::VoyageDriver>(
    "SELECT * FROM drivers_users WHERE email = $1;",
    )
    .bind(email)
    .fetch_one(&state.db)
    .await;

    // Use await? to propagate potential errors


    match user_result {
    Ok(user) => {
    if user.password != password {
        Ok(HttpResponse::Ok().json("Invalid Credentials"))
        
    }else {

        Ok(HttpResponse::Ok().json(&user))

    }
    },
    Err(err) => {
    // Convert sqlx::Error to actix_web::Error
        let actix_err = actix_web::error::ErrorInternalServerError(err.to_string());
        return Err(actix_err);
    }
}




    // println!("Success");
    // Ok(HttpResponse::Ok().json(user_result.unwrap().clone()))
}
