
use actix_web::{web, Error, HttpRequest, HttpResponse, error};
use actix::{Actor, StreamHandler, Addr, Handler, Message};
use actix_web_actors::ws;
use actix::prelude::*;
use serde_derive::{Serialize, Deserialize};

use crate::controllers::task::TaskController;

use super::handelers;
use handelers::AppState;

#[derive(Clone, Serialize, Deserialize)]
struct MyMsg{
    action: String,
    content: String,
}

// MyWs

pub struct MyWs {
    server: Addr<WebSocketServer>,
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.server.do_send(ServerMessage::ClientConnected(ctx.address()));
        let running_tasks = TaskController::get_running_tasks();
        let content = serde_json::to_string(&running_tasks[running_tasks.len()-1]).unwrap();


        let msg: MyMsg = MyMsg {
            action: "task".to_string(), 
            content
        };

        let json_message = serde_json::to_string(&msg).unwrap();
        ctx.text(json_message);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                ctx.text(text);
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

// WebSocketServer

pub struct WebSocketServer {
    pub(crate) clients: Vec<Addr<MyWs>>,
}

impl WebSocketServer {
    pub fn new() -> Self {
        WebSocketServer { clients: Vec::new() }
    }

    fn notify_clients(&self, message: MyMsg) {
        for client in &self.clients {
            client.do_send(ServerMessage::NotifyClients(message.clone()));
        }
    }
}

impl Actor for WebSocketServer {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        println!("WebSocketServer started");
    }
}

impl Handler<ServerMessage> for WebSocketServer {
    type Result = ();

    fn handle(&mut self, msg: ServerMessage, _: &mut Context<Self>) {
        match msg {
            ServerMessage::NotifyClients(message) => {
                self.notify_clients(message);
            }
            ServerMessage::ClientConnected(addr) => {
                println!("Client connected: {:?}", addr);
                self.clients.push(addr);
            }
        }
    }
}

// ServerMessage

pub enum ServerMessage {
    ClientConnected(Addr<MyWs>),
    NotifyClients(MyMsg),
}


impl Message for ServerMessage {
    type Result = ();
}

impl Handler<ServerMessage> for MyWs {
    type Result = ();

    fn handle(&mut self, msg: ServerMessage, ctx: &mut Self::Context) {
        match msg {
            ServerMessage::ClientConnected(addr) => {
                // Handle the client connection message if needed
                println!("Client connected: {:?}", addr);
            }
            ServerMessage::NotifyClients(msg) => {
                let json_message = serde_json::to_string(&msg).unwrap();
                ctx.text(json_message);
            }
        }
    }
}

pub async fn start_task_handler(data: web::Data<AppState>) -> HttpResponse {
    let running_tasks = TaskController::get_running_tasks();
    let content = serde_json::to_string(&running_tasks[running_tasks.len()-1]).unwrap();

    data.server_addr.do_send(ServerMessage::NotifyClients(MyMsg {
        action: "task_started".to_string(), 
        content
    }));
    HttpResponse::Ok().finish()
}

pub async fn end_task_handeler(data: web::Data<AppState>) -> HttpResponse {
    data.server_addr.do_send(ServerMessage::NotifyClients(MyMsg {
        action: "task_stopped".to_string(), 
        content: "{}".to_string()
    }));
    HttpResponse::Ok().finish()
}

pub async fn ws_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let server_addr = req.app_data::<web::Data<AppState>>()
        .map(|data| data.server_addr.clone())
        .ok_or_else(|| error::ErrorBadRequest("WebSocket server address not found"))?;

    let resp = ws::start(MyWs { server: server_addr.clone() }, &req, stream);

    match &resp {
        Ok(_) => println!("WebSocket connection established"),
        Err(err) => println!("WebSocket connection failed: {:?}", err),
    }

    resp
}
