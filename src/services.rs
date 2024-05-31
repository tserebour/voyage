use actix_web::{get, 
    post, 
    web::{Data,Json}, 
    HttpResponse, 
    Responder
};


use sqlx::{self, FromRow};


use serde::{Serialize, Deserialize};

use crate::AppState;




#[derive(Serialize,Deserialize, Clone,FromRow)]
pub struct VoyageUser{
    id: Option<i32>,
    username: String,
    email: String,
    password: String,

}

#[derive(Serialize,Deserialize, Clone,FromRow)]
pub struct VoyageDriver{
    id: Option<i32>,
    fullname: String,
    email: String,
    password: String,
}



#[derive(Debug)]
#[derive(Serialize,Deserialize, Clone,FromRow)]
pub struct BraFieUser{
    id: Option<i32>,
    fullname: String,
    email: String,
    phone: String,
    password: String,
        
}




#[derive(Debug)]
#[derive(Serialize,Deserialize, Clone,FromRow)]
struct VoyageUserLoginCredentials {
    email: String,
    password: String,
}

#[derive(Debug)]
#[derive(Serialize,Deserialize, Clone,FromRow)]
struct VoyageDriverLoginCredentials {
    email: String,
    password: String,
}

#[derive(Debug)]
#[derive(Serialize,Deserialize, Clone,FromRow)]
struct BraFieLoginCredentials {
    email: String,
    password: String,
}





#[get("/")]
async fn index() -> impl Responder {
    format!("login route")
}

#[post("/voyage/users/create")]
async fn create_user(state:Data<AppState>, body: Json<VoyageUser>) -> impl Responder {

    match sqlx::query_as::<_,VoyageUser>(
        "INSERT INTO users (username, email, password) VALUES ($1, $2, $3) RETURNING id,username, email, password",
    )
    .bind(body.username.to_string())
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










#[post("/voyage/users/login")]
async fn sign_in(state: Data<AppState>, credentials: Json<VoyageDriverLoginCredentials>) -> Result<HttpResponse, actix_web::Error> {
let email = credentials.email.clone();
let password = credentials.password.clone();

// 1. Validate email and password (optional)
// You can add logic here to validate email format or password length

let user_result = sqlx::query_as::<_, VoyageUser>(
    "SELECT * FROM users WHERE username = $1;",
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
async fn bra_fie_sign_in(state: Data<AppState>, credentials: Json<BraFieLoginCredentials>) -> Result<HttpResponse, actix_web::Error> {
    let username = credentials.email.clone();
    let password = credentials.password.clone();

    // 1. Validate email and password (optional)
    // You can add logic here to validate email format or password length

    let user_result = sqlx::query_as::<_, BraFieUser>(
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
async fn bra_fie_create_user(state:Data<AppState>, body: Json<BraFieUser>) -> impl Responder {

    match sqlx::query_as::<_,BraFieUser>(
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
async fn voyage_create_driver(state:Data<AppState>, body: Json<VoyageDriver>) -> impl Responder {

    match sqlx::query_as::<_,VoyageDriver>(
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
async fn voyage_driver_sign_in(state: Data<AppState>, credentials: Json<VoyageDriverLoginCredentials>) -> Result<HttpResponse, actix_web::Error> {
    
    let email = credentials.email.clone();
    let password = credentials.password.clone();

    // 1. Validate email and password (optional)
    // You can add logic here to validate email format or password length

    let user_result = sqlx::query_as::<_, VoyageDriver>(
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
