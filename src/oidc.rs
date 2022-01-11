use reqwest::blocking::Client;
use serde_json::Value;
use std::collections::HashMap;

pub fn get_token(grant_uri: &str, client_id: &str, client_secret: &str) -> Result<String, String> {
    let client = Client::new();
    let mut params = HashMap::new();
    params.insert("grant_type", "client_credentials");
    match client
        .post(grant_uri)
        .basic_auth(client_id, Some(client_secret))
        .form(&params)
        .send()
    {
        Ok(response) => match serde_json::from_str::<Value>(&response.text().unwrap()) {
            Ok(value) => Ok(value["access_token"].to_string().replace("\"", "")),
            Err(e) => Err(format!("{:?}", e)),
        },
        Err(e) => Err(format!("{:?}", e)),
    }
}
