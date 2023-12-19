use actix::Addr;
use actix_web::{error, web, HttpResponse, Result}; 
use serde_derive::Deserialize;
use std::fs;

use crate::backend::websocket::WebSocketServer;
use crate::controllers::task::{TaskController, TaskError};

#[derive(Deserialize)]
pub struct DateRequest {
    date: String
}

#[derive(Deserialize)]
pub struct TaskRequest {
    name: String
}

#[derive(Debug)]
struct MyError {
    name: &'static str,
}

#[derive(Clone)]

pub struct AppState {
   pub server_addr: Addr<WebSocketServer>,
}

pub async fn index() -> HttpResponse {
    let html_content = fs::read_to_string("static/index.html")
        .unwrap_or_else(|_| String::from("Error reading HTML file"));

    HttpResponse::Ok().content_type("text/html").body(html_content)
}

pub async fn tasks() -> HttpResponse {
    let html_content = fs::read_to_string("static/tasks.html")
        .unwrap_or_else(|_| String::from("Error reading HTML file"));

    HttpResponse::Ok().content_type("text/html").body(html_content)
}

pub async fn test() -> HttpResponse {
    let html_content = fs::read_to_string("static/test.html")
        .unwrap_or_else(|_| String::from("Error reading HTML file"));

    HttpResponse::Ok().content_type("text/html").body(html_content)
}

pub async fn get_tasks() -> Result<HttpResponse> {
    let tasks = TaskController::get_tasks();
    let json_string = serde_json::to_string(&tasks).expect("Failed to serialize");

    Ok(HttpResponse::Ok().content_type("application/json").body(json_string))
}

pub async fn get_date_tasks(request: web::Json<DateRequest>) -> Result<HttpResponse> {
    let date = &request.date;

    let tasks = TaskController::get_date_tasks(date.to_string());
    let json_string = serde_json::to_string(&tasks).expect("Failed to serialize");

    Ok(HttpResponse::Ok().content_type("application/json").body(json_string))
}


pub async fn start_task(request: web::Json<TaskRequest>) -> Result<HttpResponse> {
    let name = &request.name;

    let result: Result<(), TaskError> = TaskController::start_task(name);
    let json_string = format!("{{result: {}}}", match result {
        Err(_err) => "false", 
        Ok(_) => "true", 
    });

    Ok(HttpResponse::Ok().content_type("application/json").body(json_string))
}

pub async fn stop_task(request: web::Json<TaskRequest>) -> Result<HttpResponse> {
    let name = &request.name;

    let result: Result<(), TaskError> = TaskController::stop_task(name);
    match result {
        Err(_err) => {
            let result = Err(MyError { name: "task doesn't exist" });
            result.map_err(|err| error::ErrorBadRequest(err.name)) 
        }
        Ok(_) => Ok(HttpResponse::Ok().content_type("application/json").body("{}"))
    }


}
