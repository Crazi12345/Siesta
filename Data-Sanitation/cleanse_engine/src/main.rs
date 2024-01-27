use poem::{listener::TcpListener, web::Json, EndpointExt, Server, IntoResponse};
use poem_openapi::{payload::PlainText, OpenApi, OpenApiService};
use reqwest;
use serde::{Deserialize, Serialize};
use std::fs;

mod client;
use client::ping;

#[derive(Serialize, Deserialize)]
struct Creds {
    api_key: String,
}

fn read_config() -> Creds {
    let data = fs::read_to_string("credentials.json").expect("Unable to read file");
    serde_json::from_str(&data).expect("Unable to parse JSON")
}

struct Api;

#[OpenApi]
    impl Api {
    #[oai(path = "/get_calendar_events", method = "get")]
    async fn get_calendar_events(&self) -> Result<PlainText<String>, Error> {
        // Function implementation...
        let creds = read_config();
        ping().await;
        let api_key = creds.api_key;
        let calendar_id = "your_calendar_id@group.calendar.google.com";
        let url = format!("https://www.googleapis.com/calendar/v3/calendars/{}/events?key={}", calendar_id, api_key);

        let response = reqwest::get(&url).await.map_err(poem::Error::from)?.text().await.map_err(poem::Error::from)?;
        Ok(PlainText(response))
    }

    #[oai(path = "/get_all_calendars", method = "get")]
    async fn get_all_calendars(&self) -> Result<PlainText<String>, Error> {
        // Function implementation...
        let creds = read_config();
        ping().await;
        let api_key = creds.api_key;
        let calendar_id = "your_calendar_id@group.calendar.google.com";
        let url = format!("https://www.googleapis.com/calendar/v3/calendars/{}/events?key={}", calendar_id, api_key);

        let response = reqwest::get(&url).await.map_err(poem::Error::from)?.text().await.map_err(poem::Error::from)?;
        Ok(PlainText(response))
    }
}



#[tokio::main]
async fn main() -> poem::Result<()> {
    let api_service = OpenApiService::new(Api, "My API", "1.0").server("http://127.0.0.1:8080");
    let ui = api_service.swagger_ui();
    let app = route().nest("/", api_service).nest("/docs", ui);
    Server::new(TcpListener::bind("127.0.0.1:8080")).run(app).await?;

    Ok(())
}

