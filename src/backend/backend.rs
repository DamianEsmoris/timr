use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use actix_files as fs;
use actix::Actor;
use std::env;

mod handelers;
mod websocket;

use crate::{Config, Query};
use handelers::*;
use websocket::{ws_route, start_task_handler, end_task_handeler};
use crate::backend::websocket::WebSocketServer;

#[actix_web::main]
pub async fn serve_server() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let query: Query = Query::new(&args);
    let c: Config = Config::new(&query, false);
    
    // Create shared state with the server address
    let server_addr = WebSocketServer::new().start();
    let app_state = AppState { server_addr: server_addr.clone() };

    // Start Actix system with the shared state
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Cors::default().allow_any_origin().allow_any_method())
            .service(web::resource("/").to(index))
            .service(web::resource("/tasks").to(tasks))
            .service(web::resource("/test").to(test))
            .service(fs::Files::new("/static", "static"))
            .service(web::resource("/get_tasks").route(web::post().to(get_tasks)))
            .service(web::resource("/get_date_tasks").route(web::post().to(get_date_tasks)))
            .service(web::resource("/stop_task").route(web::post().to(stop_task)))
            .service(web::resource("/start_task").route(web::post().to(start_task)))
            .service(web::resource("/ws/").to(ws_route))
            .service(web::resource("/start_task_notify").to(start_task_handler))
            .service(web::resource("/stop_task_notify").to(end_task_handeler))
    })
    .bind((c.server_ip, c.server_port))?
    .run()
    .await
}
