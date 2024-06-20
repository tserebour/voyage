struct Server {
    sessions: Arc<Mutex<Vec<Addr<MyWebSocket>>>>,
}

impl Server {
    fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl Handler<Connect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) {
        self.sessions.lock().unwrap().push(msg.addr);
    }
}

impl Handler<Disconnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) {
        self.sessions.lock().unwrap().retain(|addr| addr != &msg.addr);
    }
}

impl Handler<UpdateLocation> for Server {
    type Result = ();

    fn handle(&mut self, msg: UpdateLocation, _: &mut Self::Context) {
        for session in self.sessions.lock().unwrap().iter() {
            session.do_send(ws::Message::Text(msg.location.clone()));
        }
    }
}
