use std::fs;                                                                                          
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Creds {
    auth_uri: String,
    client_id: String,
    client_secret: String,
    redirect_uris: [String;1],
    token_uri: String,
    project_id: String
}
fn read_config() -> Creds {
    let data = fs::read_to_string("credentials.json")
        .expect("Unable to read file");
    let creds: Creds = serde_json::from_str(&data)
        .expect("Unable to parse JSON");
    creds
}

fn main() {
   let test:Creds = read_config();
   println!("{}",test.client_id);
}
