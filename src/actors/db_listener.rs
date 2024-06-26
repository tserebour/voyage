use actix::prelude::*;

use sqlx::postgres::{PgListener, PgPoolOptions};
use sqlx::{PgPool, Pool, Postgres};


use super::server::WebSocketManager;
use super::my_web_socket::BroadcastMessage;
use geo::{Point, HaversineDistance}; 

use crate::models::voyage_driver_sign_up_model::VoyageDriver;



#[derive(Message)]
#[rtype(result = "()")]
struct LocationUpdateNotification(String);


#[derive(Message)]
#[rtype(result = "()")]
struct RideRequestNotification(String);

pub struct NotificationService {
    pool: PgPool,
    ws_manager: Addr<WebSocketManager>,
}

impl NotificationService {
    pub async fn new(database_url: &str, ws_manager: Addr<WebSocketManager> ) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await
            .expect("Could not connect to the database");
        
        NotificationService { pool,  ws_manager }
    }


    async fn listen_for_notifications(pool: PgPool, addr: Addr<Self>, channels: Vec<&str>) {
        let mut listener = PgListener::connect_with(&pool)
            .await
            .expect("Failed to create listener");

        listener.listen("driver_location").await.expect("Failed to listen on channel");

        for channel in &channels {
            listener.listen(channel).await.expect(&format!("Failed to listen on channel {}", channel));
        }

        while let Ok(notification) = listener.recv().await {
            let payload = notification.payload();
            println!("Received notification: {:?}", payload);
            
            match notification.channel() {
                "driver_location" => {
                    addr.do_send(LocationUpdateNotification(payload.to_string()));
                }
                "ride_requests_channel" => {
                    addr.do_send(RideRequestNotification(payload.to_string()));
                }
                // Add other channels and their corresponding message types here
                _ => {
                    println!("Received notification from unknown channel: {:?}", notification.channel());
                }
            }
        }
    }
}

impl Actor for NotificationService {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let pool = self.pool.clone();
        let addr = ctx.address();

    let channels: Vec<&str> = vec!["ride_requests_channel", "driver_location"];


        // Spawn a new task to listen for notifications
        ctx.spawn(async move {
            NotificationService::listen_for_notifications(pool, addr, channels).await;
        }.into_actor(self));
    }
}

impl Handler<LocationUpdateNotification> for NotificationService {
    type Result = ();

    fn handle(&mut self, msg: LocationUpdateNotification, _ctx: &mut Self::Context) {
        // Handle the notification message
        println!("Handling notification: {}", msg.0);
        self.ws_manager.do_send(BroadcastMessage(msg.0));
        // Notify the WebSocket actor or other parts of the system
    }
}


impl Handler<RideRequestNotification> for NotificationService {
    type Result = ();

    fn handle(&mut self, msg: RideRequestNotification, _ctx: &mut Self::Context) {
        // Handle the notification message
        println!("Handling notification: {}", msg.0);
        self.ws_manager.do_send(BroadcastMessage(msg.0));
        // Notify the WebSocket actor or other parts of the system
    }
}





async fn find_nearby_drivers(
    pool: &Pool<Postgres>,
    user_latitude: f64,
    user_longitude: f64,
    search_radius_meters: f64,
) -> Result<Vec<VoyageDriver>, sqlx::Error> {
    

    

   let drivers: Vec<VoyageDriver> = match  sqlx::query_as(
        "SELECT * FROM drivers;"
    ).fetch_all(pool).await {
        Ok(drivers) => {
            drivers
        },

        Err(_) => {
            Vec::new()
        },
        
    };


    let mut nearby_drivers = Vec::new();
    let user_location = Point::new(user_longitude, user_latitude);

    for driver in drivers.iter() {
        let driver_location = Point::new(driver.current_longitude.unwrap(), driver.current_latitude.unwrap());

        let distance = user_location.haversine_distance(&driver_location);

        if distance <= search_radius_meters {
            nearby_drivers.push(driver.clone());
        }
    }

    Ok(nearby_drivers)
}
