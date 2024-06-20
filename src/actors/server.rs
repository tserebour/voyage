
use std::collections::HashMap;
use actix::prelude::*;
use crate::actors::my_web_socket::{
    WebSocketSession,
    Connect,
    Disconnect,
    BroadcastMessage,
};

pub struct WebSocketManager {
    sessions: HashMap<usize, Addr<WebSocketSession>>,
    counter: usize,
}

impl WebSocketManager {
    pub fn new() -> Self {
        WebSocketManager {
            sessions: HashMap::new(),
            counter: 0,
        }
    }
}

impl Actor for WebSocketManager {
    type Context = Context<Self>;
}

impl Handler<Connect> for WebSocketManager {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
        self.counter += 1;
        self.sessions.insert(self.counter, msg.addr);
    }
}

impl Handler<Disconnect> for WebSocketManager {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        self.sessions.remove(&msg.id);
    }
}

impl Handler<BroadcastMessage> for WebSocketManager {
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, _: &mut Context<Self>) {
        for session in self.sessions.values() {
            session.do_send(BroadcastMessage(msg.0.clone()));
        }
    }
}

