use dotenv;
use reqwest::blocking::Client;
use serde_json::Value;
use std::env;
mod drink;

fn main() {
    dotenv::dotenv().ok();
    let keycloak_token = env::var("KEYCLOAK_TOKEN").expect("KEYCLOAK_TOKEN not set");
    let drink_url = env::var("DRINK_URL").unwrap_or("https://drink.csh.rit.edu/drinks".to_string());
    let mut prev_response: Option<drink::Response> = None;
    let client = Client::new();
    loop {
        let drink_response = client
            .get(&drink_url)
            .bearer_auth(&keycloak_token)
            .send()
            .unwrap();

        if drink_response.status().is_success() {
            let response_text = drink_response.text().unwrap();
            let response: drink::Response = serde_json::from_str(&response_text).unwrap();
            match prev_response {
                Some(_) => {
                    let machines = response.clone().machines;
                    println!("{:?}", machines);
                    prev_response = Some(response);
                }
                None => {
                    prev_response = Some(response);
                },
            }
        } else {
            let response_status = drink_response.status();
            let response_text = drink_response.text().unwrap();
            let error = match serde_json::from_str::<Value>(&response_text) {
                Ok(v) => v["error"].as_str().unwrap().to_owned(),
                Err(_) => "error deserializing response".to_owned(),
            };
            println!("Whoops, drink returned a {} ({})", response_status, error);
        }

        std::thread::sleep(std::time::Duration::from_secs(5))
    }
}
