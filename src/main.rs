use reqwest::blocking::Client;
use serde_json::Value;
use std::env;
mod delta;
mod drink;
mod oidc;
mod slack;

fn main() {
    dotenv::dotenv().ok();
    let drink_url =
        env::var("DRINK_URL").unwrap_or_else(|_| "https://drink.csh.rit.edu/drinks".to_owned());
    let slack_uri = env::var("SLACK_WEBHOOK_URI").expect("SLACK_WEBHOOK_URI not set");
    let mut prev_response: Option<drink::Response> = None;
    let client = Client::new();
    loop {
        // The SSO token expires every 5 minutes, and we fetch every 5 minutes... so there's no real
        // reason to implement any sort of caching or validity checking for it, we might as well
        // just fetch a new one each time
        match oidc::get_token(
            &env::var("OIDC_URI").unwrap_or_else(|_| {
                "https://sso.csh.rit.edu/auth/realms/csh/protocol/openid-connect/token".to_owned()
            }),
            &env::var("OIDC_CLIENT_ID").unwrap_or_else(|_| "manhattan".to_owned()),
            &env::var("OIDC_CLIENT_SECRET").expect("OIDC_CLIENT_SECRET not set"),
        ) {
            Ok(keycloak_token) => {
                match client.get(&drink_url).bearer_auth(&keycloak_token).send() {
                    Ok(drink_response) => {
                        if drink_response.status().is_success() {
                            let response_text = drink_response.text().unwrap();
                            let response: drink::Response =
                                serde_json::from_str(&response_text).unwrap();
                            if let Some(r) = prev_response {
                                let changes = diff(&r, &response);
                                if let Some(changes) = changes {
                                    slack::send_changes(&slack_uri, &changes)
                                } else {
                                    println!("No changes detected")
                                }
                            } else {
                                println!("Fetched initial state");
                            }
                            prev_response = Some(response);
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
                    Err(err) => {
                        println!("{}", err);
                    }
                }
            }
            Err(e) => println!("{}", e),
        }

        std::thread::sleep(std::time::Duration::from_secs(5 * 60))
    }
}

fn diff(previous: &drink::Response, current: &drink::Response) -> Option<Vec<delta::Change>> {
    let mut changes: Vec<delta::Change> = vec![];
    for (pm, cm) in previous.machines.iter().zip(current.machines.iter()) {
        for (ps, cs) in pm.slots.iter().zip(cm.slots.iter()) {
            // Slot emptiness changed
            if ps.empty != cs.empty {
                // Slot was empty but isn't anymore
                if ps.empty && !cs.empty {
                    changes.push(delta::Change {
                        change_type: delta::ChangeType::SlotNowFull,
                        previous_machine: pm.clone(),
                        previous_slot: ps.clone(),
                        current_machine: cm.clone(),
                        current_slot: cs.clone(),
                    })
                } else {
                    // Slot wasn't empty but now is
                    changes.push(delta::Change {
                        change_type: delta::ChangeType::SlotNowEmpty,
                        previous_machine: pm.clone(),
                        previous_slot: ps.clone(),
                        current_machine: cm.clone(),
                        current_slot: cs.clone(),
                    })
                }
            }
            // Item name changed
            if ps.item.name != cs.item.name {
                changes.push(delta::Change {
                    change_type: delta::ChangeType::ItemNameChanged,
                    previous_machine: pm.clone(),
                    previous_slot: ps.clone(),
                    current_machine: cm.clone(),
                    current_slot: cs.clone(),
                })
            }
            // Item price (but not item) changed
            if ps.item.price != cs.item.price && ps.item.id == cs.item.id {
                changes.push(delta::Change {
                    change_type: delta::ChangeType::ItemPriceChanged,
                    previous_machine: pm.clone(),
                    previous_slot: ps.clone(),
                    current_machine: cm.clone(),
                    current_slot: cs.clone(),
                })
            }
        }
    }

    if !changes.is_empty() {
        Some(changes)
    } else {
        None
    }
}
