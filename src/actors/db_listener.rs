use actix::prelude::*;
use sqlx::postgres::{PgListener, PgPoolOptions};
use sqlx::PgPool;

use super::server::WebSocketManager;
use super::my_web_socket::BroadcastMessage;



#[derive(Message)]
#[rtype(result = "()")]
struct DbNotification(String);

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

    async fn listen_for_notifications(pool: PgPool, addr: Addr<Self>) {
        let mut listener = PgListener::connect_with(&pool)
            .await
            .expect("Failed to create listener");

        listener.listen("driver_location").await.expect("Failed to listen on channel");

        while let Ok(notification) = listener.recv().await {
            println!("Received notification: {:?}", notification.payload());
            addr.do_send(DbNotification(notification.payload().to_string()));
        }
    }
}

impl Actor for NotificationService {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let pool = self.pool.clone();
        let addr = ctx.address();

        // Spawn a new task to listen for notifications
        ctx.spawn(async move {
            NotificationService::listen_for_notifications(pool, addr).await;
        }.into_actor(self));
    }
}

impl Handler<DbNotification> for NotificationService {
    type Result = ();

    fn handle(&mut self, msg: DbNotification, _ctx: &mut Self::Context) {
        // Handle the notification message
        println!("Handling notification: {}", msg.0);
        self.ws_manager.do_send(BroadcastMessage(msg.0));
        // Notify the WebSocket actor or other parts of the system
    }
}
