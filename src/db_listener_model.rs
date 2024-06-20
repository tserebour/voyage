use actix::prelude::*;
use sqlx::postgres::{PgListener, PgPool};
use std::sync::Arc;

struct DbListener {
    pool: Arc<PgPool>,
    server_addr: Addr<Server>,
}

impl DbListener {
    async fn listen_for_updates(&self) {
        let mut listener = PgListener::connect_with(&self.pool).await.expect("Failed to connect to the database");
        listener.listen("driver_location_updates").await.expect("Failed to listen for driver location updates");

        while let Ok(notification) = listener.recv().await {
            let payload = notification.payload();
            self.server_addr.do_send(UpdateLocation { location: payload.to_string() });
        }
    }
}

impl Actor for DbListener {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let pool = self.pool.clone();
        let server_addr = self.server_addr.clone();
        ctx.spawn(async move {
            DbListener { pool, server_addr }.listen_for_updates().await;
        });
    }
}
