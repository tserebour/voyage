// use dotenv::dotenv;
// use std::env;
// use dotenv::dotenv;
// mod services;
// use std::io::{self, Read};


// use actix_cors::Cors;
use std::time::Instant;
use actix_web::web::Data;
use actix_web::{
    // dev::Server, 
    // http, 
    
    
    web,
    App, 
    HttpServer,
    HttpRequest,
    Responder,
    HttpResponse,
    Error
    
};

use sqlx::postgres::any::driver;
use sqlx::postgres::PgPoolOptions;
use sqlx::{pool, Pool, Postgres};

use actix::prelude::*;
use actix_web_actors::ws;

mod services;
mod models;
mod utils;
mod actors;

use actors::server::WebSocketManager;
use actors::my_web_socket::WebSocketSession;



use actors::db_listener;




pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // dotenv().ok(); // Load the .env file

    



    // let database_url = var("DATABASE_URL").expect("DATABASE_URL not set");
    // let database_url: &str = "postgres://actix:actix@voyage-postgres-db/actix";
    let database_url: &str = "postgres://actix:actix@172.21.0.2/actix";


    

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    let ws_manager = WebSocketManager::new().start();

    let notification_service = db_listener::NotificationService::new(&database_url, ws_manager.clone()).await;
    let addr = notification_service.start();
    println!("Connected to {:#?}", addr);

    // let server = ride_request::RideRequestServer {
    //     pool: pool.clone(),
    //     drivers: Vec::new(),
    // }
    // .start();

        

    let port = 8080;

    println!("Port: {port}");

    // HttpServer::new(move || {
       
    //     App::new()
    //         .app_data(Data::new(AppState{db: pool.clone()}))
            
    //         // .wrap(cors)
    //         // .service(services::index)
    //         // .service(services::voyage_user_sign_in)
    //         // .service(services::voyage_create_user)
    //         // .service(services::voyage_create_driver)
    //         // .service(services::voyage_driver_sign_in)
    //         // .service(services::create_ride_request)
    //         // .service(services::add_driver_location_to_database)
    //         // .service(services::update_earn_type_to_database)

    //         // .service(services::bra_fie_sign_in)
    //         // .service(services::bra_fie_create_user)
    //         // .service(websocket::start_driver_location_server)


    //         .configure(services::config)
    //         .route("/ws/", web::get().to(ws_index))
            

            





            

    // })
    // .bind(("0.0.0.0", port))? 
    // .run()
    // .await


    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ws_manager.clone()))
            // .app_data(web::Data::new(server.clone()))
            .app_data(Data::new(AppState{db: pool.clone()}))
            
            .route("/ws/drivers/{id}/drivers_location_update", web::get().to(ws_drivers_location_update))
            // .route("/ws/drivers/{id}/ride_request", web::get().to(ws_ride_request))
            .configure(services::config)
    })
    .bind(("0.0.0.0",port))?
    .run()
    .await
}

// async fn websocket_handler(req: HttpRequest, stream: web::Payload, data: web::Data<Addr<Server>>) -> actix_web::Result<impl Responder> {
//     println!("eiie");

    

//     ws::start(
//         MyWebSocket::new(data.get_ref().clone()), 
//         &req, 
//         stream
//     )
// }





/// Define a WebSocket actor
/// 
/// 


/// WebSocket route
async fn ws_drivers_location_update(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<Addr<WebSocketManager>>,
) -> impl Responder {


    let driver_id = req.match_info().get("id").unwrap().parse::<usize>().unwrap();

    let addr = data.get_ref().clone();
    ws::start(WebSocketSession { id: driver_id, hb: Instant::now(), addr }, &req, stream)
}





// async fn ws_ride_request(
//     req: HttpRequest,
//     stream: web::Payload,
//     srv: web::Data<Addr<ride_request::RideRequestServer>>,
// ) -> Result<HttpResponse, Error> {
    // let driver_id = req.match_info().get("id").unwrap().parse::<i32>().unwrap();
//     // let driver_id = 1;
//     println!("{:?}", driver_id);
//     let res = ws::start(ride_request::DriverSession::new(driver_id, srv.get_ref().clone()), &req, stream);
//     res
// }


