use reqwest::blocking::Client;
use serde_json::json;
use crate::delta;

pub fn send_changes(webhook_uri: &str, changes: &[delta::Change]) {
    let mut change_string = String::from("Changes detected!\n\n");
    for change in changes {
        change_string.push_str(&format!("{}", change));
        change_string.push('\n')
    }
    println!("{}", change_string);

    let post_data = json!({
        "text": change_string
    });

    let client = Client::new();
    match client.post(webhook_uri).header(reqwest::header::CONTENT_TYPE, "application/json").body(post_data.to_string()).send() {
        Ok(_) => {
            println!("Updates sent to Slack!")
        },
        Err(err) => {
            println!("Slack message failed: {}", err)
        }
    }
}
