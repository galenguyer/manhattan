use dotenv;
use reqwest::blocking::Client;
use serde_json::Value;
use std::env;

fn main() {
    dotenv::dotenv().ok();
    let keycloak_token = env::var("KEYCLOAK_TOKEN").expect("KEYCLOAK_TOKEN not set");

    let client = Client::new();
    let drink_response = client.get("https://drink.csh.rit.edu/drinks").bearer_auth(keycloak_token).send().unwrap();

    if drink_response.status().is_success() {
    } else {
        let response_status = drink_response.status();
        let response_text = drink_response.text().unwrap();
        let error = match serde_json::from_str::<Value>(&response_text) {
            Ok(v) => v["error"].as_str().unwrap().to_owned(),
            Err(_) => "error deserializing response".to_owned(),
        };
        println!("Whoops, drink returned a {} ({})", response_status, error);
    }
}
