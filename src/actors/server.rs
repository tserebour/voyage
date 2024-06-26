
use std::collections::HashMap;
use actix::prelude::*;
use crate::actors::my_web_socket::{
    WebSocketSession,
    Connect,
    Disconnect,
    BroadcastMessage,
};

pub struct WebSocketManager {
    driver_sessions: HashMap<usize, Addr<WebSocketSession>>,
    user_sessions: HashMap<usize, Addr<WebSocketSession>>

}

impl WebSocketManager {
    pub fn new() -> Self {
        WebSocketManager {
            driver_sessions: HashMap::new(),
            user_sessions: HashMap::new(),
            
            
        }
    }
}

impl Actor for WebSocketManager {
    type Context = Context<Self>;
}

impl Handler<Connect> for WebSocketManager {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
        let user_id = msg.id;
        self.driver_sessions.insert(user_id, msg.addr);
        println!("{:#?} connected",self.driver_sessions);
    }
}

impl Handler<Disconnect> for WebSocketManager {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        self.driver_sessions.remove(&msg.id);
        println!("{} connected",self.driver_sessions.len());
    }
}

impl Handler<BroadcastMessage> for WebSocketManager {
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, _: &mut Context<Self>) {
        for session in self.driver_sessions.values() {
            session.do_send(BroadcastMessage(msg.0.clone()));
        }
    }
}

