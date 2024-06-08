// use dotenv::dotenv;
// use std::env;
use dotenv::dotenv;
mod services;
// use std::io::{self, Read};

// use actix_cors::Cors;
use actix_web::{
    // dev::Server, 
    // http, 
    web::Data, 
    App, 
    HttpServer
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok(); // Load the .env file

    



    // let database_url = var("DATABASE_URL").expect("DATABASE_URL not set");
    // let database_url: &str = "postgres://actix:actix@voyage-postgres-db/actix";
    let database_url: &str = "postgres://actix:actix@172.21.0.2/actix";


    

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    let port = 8080;

    println!("Port: {port}");

    HttpServer::new(move || {
       
        App::new()
            .app_data(Data::new(AppState{db: pool.clone()}))
            // .wrap(cors)
            .service(services::index)
            // .service(services::sign_in)
            // .service(services::create_user)
            // .service(services::bra_fie_sign_in)
            // .service(services::bra_fie_create_user)
            .service(services::voyage_create_driver)
            .service(services::voyage_driver_sign_in)





            

    })
    .bind(("0.0.0.0", port))? 
    .run()
    .await
}
