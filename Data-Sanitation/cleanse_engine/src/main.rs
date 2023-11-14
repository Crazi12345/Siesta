use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use reqwest;
use std::fs;      
use serde::{Deserialize, Serialize};
mod client;
use client::ping;
#[derive(Serialize, Deserialize)]
struct Creds {
    api_key: String
}
fn read_config() -> Creds {
    let data = fs::read_to_string("credentials.json")
        .expect("Unable to read file");
    let creds: Creds = serde_json::from_str(&data)
        .expect("Unable to parse JSON");
    creds
}
async fn get_calendar_events() -> reqwest::Result<String>{
    let creds = read_config();
    ping().await;
    let api_key = creds.api_key;
    let calendar_id = "4cd37135fc423abf9738ad1535bb9c8691e74cbdaa6973225cde3225e97a4085@group.calendar.google.com";
    let url = format!("https://www.googleapis.com/calendar/v3/calendars/{}/events?key={}",calendar_id,api_key);
    let response = reqwest::get(&url).await?.text().await?;
    //println!("{}",response);
    Ok(response)
}

async fn get_all_calendars() -> reqwest::Result<String>{
    let creds = read_config();
    let api_key = creds.api_key;
    let calendar_id = "4cd37135fc423abf9738ad1535bb9c8691e74cbdaa6973225cde3225e97a4085@group.calendar.google.com";
    let url = format!("https://www.googleapis.com/calendar/v3/users/me/calendarList?key={}",api_key);
    let response = reqwest::get(&url).await?.text().await?;
    println!("{}",response);
    Ok(response)
}
async fn events() -> impl Responder {
    match get_calendar_events().await {
        Ok(data) => HttpResponse::Ok().body(data),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn list() -> impl Responder {
    match get_all_calendars().await {
        Ok(data) => HttpResponse::Ok().body(data),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/events", web::get().to(events))
            .route("/list",web::get().to(list))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
