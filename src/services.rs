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
pub struct User{
id: Option<i32>,
fullname: String,
email: String,
password: String,

}





#[get("/")]
async fn index() -> impl Responder {
format!("login route")
}

#[post("/create")]
async fn create_user(state:Data<AppState>, body: Json<User>) -> impl Responder {

    match sqlx::query_as::<_,User>(
        "INSERT INTO bra_fie_users (username, email, password) VALUES ($1, $2, $3) RETURNING id,username, email, password",
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
struct LoginCredentials {
    username: String,
    password: String,
}

#[derive(Debug)]
#[derive(Serialize,Deserialize, Clone,FromRow)]
struct BraFieLoginCredentials {
username: String,
password: String,
}








#[post("/login")]
async fn sign_in(state: Data<AppState>, credentials: Json<LoginCredentials>) -> Result<HttpResponse, actix_web::Error> {
let username = credentials.username.clone();
let password = credentials.password.clone();

// 1. Validate email and password (optional)
// You can add logic here to validate email format or password length

let user_result = sqlx::query_as::<_, User>(
"SELECT * FROM users WHERE username = $1;",
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


#[post("/bra_fie_login")]
async fn bra_fie_sign_in(state: Data<AppState>, credentials: Json<LoginCredentials>) -> Result<HttpResponse, actix_web::Error> {
let username = credentials.username.clone();
let password = credentials.password.clone();

// 1. Validate email and password (optional)
// You can add logic here to validate email format or password length

let user_result = sqlx::query_as::<_, User>(
"SELECT * FROM bra_fie_users WHERE username = $1;",
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


#[post("/bra_fie_create")]
async fn bra_fie_create_user(state:Data<AppState>, body: Json<BraFieUser>) -> impl Responder {

    match sqlx::query_as::<_,User>(
        "INSERT INTO bra_fie_users (fullname, email, phone, password) VALUES ($1, $2,$3 $4) RETURNING id,fullname, email,phone, password",
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
