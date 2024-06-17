// use actix::fut::ok;
use actix_web::{get, 
    // guard::Options, 
    post, web::{Data,Json}, HttpResponse, Responder
};




use crate::AppState;
mod models;
mod voyage_user_sign_up_model;
mod driver_location_update_model;
mod update_earn_type_model;

mod helper_functions;
mod voyage_driver_sign_up_model;


// use voyage_user_sign_up_model::VoyageUser;
use models::models::voyage_models;
use models::models::bra_fie_models;
use helper_functions::helpers_functions::{hash_password,verify_password};

// use chrono::NaiveDateTime;














#[get("/")]
async fn index() -> impl Responder {
    

    format!("login route")
}

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










#[post("/voyage/users/login")]
async fn voyage_user_sign_in(state: Data<AppState>, credentials: Json<voyage_models::VoyageDriverLoginCredentials>) -> Result<HttpResponse, actix_web::Error> {
let email = credentials.email.clone();
let plain_password = credentials.password.clone();

// 1. Validate email and password (optional)
// You can add logic here to validate email format or password length

let user_result = sqlx::query_as::<_, voyage_user_sign_up_model::VoyageUser>(
    "SELECT * FROM voyage_users WHERE email = $1;",
)
.bind(email)
.fetch_one(&state.db)
.await; // Use await? to propagate potential errors


match user_result {
    Ok(user) => {
        let verify_password_result = verify_password(&plain_password, &user.password);

        match verify_password_result{

            Ok(true) => {


                let id = user.id.unwrap();
                

                let user_last_login_update = sqlx::query_as::<_, voyage_user_sign_up_model::VoyageUser>(
                    "UPDATE voyage_users SET last_login_at = CURRENT_TIMESTAMP WHERE id = $1 
                    RETURNING id, fullname, email, password, phone_number, account_created_at, last_login_at;",
                )
                .bind(id)
                .fetch_one(&state.db)
                .await; // Use await? 


            
        
                match user_last_login_update{
                    Ok(_) =>{
                        Ok(HttpResponse::Ok().json(&user))
                    },
                    Err(err) => {
                        println!("Error updating last login time: {}", err);
                        return Err(actix_web::error::ErrorInternalServerError(err.to_string()));
                    }
                }
        
                
            },
            Ok(false) => {
                return Err(actix_web::error::ErrorInternalServerError("Invalid email or password"));

            },

            Err(err) => {
                println!("Error verifying password: {}", err);
                return Err(actix_web::error::ErrorInternalServerError(err.to_string()));
            }


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



#[post("/ride_requests")]
async fn create_ride_request(state: Data<AppState>, ride_request: Json<voyage_models::RideRequest>) -> Result<HttpResponse, actix_web::Error> {
    let result = sqlx::query_as::<_, voyage_models::RideRequest>(
        
        "INSERT INTO ride_requests (
            user_id,
            pickup_address,
            pickup_latitude,
            pickup_longitude,
            dropoff_address,
            dropoff_latitude,
            dropoff_longitude,
            ride_type_id,
            car_id
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING id, user_id, pickup_address, pickup_latitude, pickup_longitude, dropoff_address, dropoff_latitude, dropoff_longitude, ride_type_id, estimated_fare, requested_at, status,car_id;"
        )
        .bind(ride_request.user_id.clone())
        .bind(ride_request.pickup_address.clone())
        .bind(ride_request.pickup_latitude.clone())
        .bind(ride_request.pickup_longitude.clone())
        .bind(ride_request.dropoff_address.clone())
        .bind(ride_request.dropoff_latitude.clone())
        .bind(ride_request.dropoff_longitude.clone())
        .bind(ride_request.ride_type_id.clone())   
        .bind(ride_request.car_id.clone())    

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
async fn voyage_create_driver(state:Data<AppState>, body: Json<voyage_driver_sign_up_model::VoyageDriver>) -> impl Responder {

    match sqlx::query_as::<_, voyage_driver_sign_up_model::VoyageDriver>(
        "INSERT INTO voyage_drivers (fullname, email, password, license_number, vehicle_information) 
         VALUES ($1, $2, $3, $4, $5)
         RETURNING id, fullname, email, password, license_number, vehicle_information, rating",
    )
    .bind(body.fullname.to_string())
    .bind(body.email.to_string())
    .bind(hash_password(&body.password).unwrap().clone())
    .bind(body.license_number.clone())
    .bind(body.vehicle_information.clone())
    
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

    let user_result = sqlx::query_as::<_, voyage_driver_sign_up_model::VoyageDriver>(
    "SELECT * FROM voyage_drivers WHERE email = $1;",
    )
    .bind(email)
    .fetch_one(&state.db)
    .await;

    // Use await? to propagate potential errors


    match user_result {
    Ok(user) => {

        let verify_password_result = verify_password(&password, &user.password);

        match verify_password_result{
            Ok(true) =>{
                Ok(HttpResponse::Ok().json(&user))

            },
            Ok(false) => {
                return Err(actix_web::error::ErrorInternalServerError("Invalid email or password"));
            },
            Err(err) => {
                println!("Error verifying password: {}", err);
                return Err(actix_web::error::ErrorInternalServerError(err.to_string()));
            }
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




#[post("/voyage/drivers/location_update_create")]
async fn add_driver_location_to_database(state: Data<AppState>, location_data: Json<driver_location_update_model::DriverLocationUpdate>) -> Result<HttpResponse, actix_web::Error>{
    driver_location_update_model::add_driver_location_to_database(state,location_data).await

   
}

#[post("/voyage/drivers/update_earn_type")]
async fn update_earn_type_to_database(state: Data<AppState>, location_data: Json<update_earn_type_model::EarnTypeUpdate>) -> Result<HttpResponse, actix_web::Error>{
    update_earn_type_model::update_earn_type_to_database(state,location_data).await


}

