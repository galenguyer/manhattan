use dotenv;
use reqwest::blocking::Client;
use serde_json::Value;
use std::env;
mod delta;
mod drink;

fn main() {
    dotenv::dotenv().ok();
    let keycloak_token = env::var("KEYCLOAK_TOKEN").expect("KEYCLOAK_TOKEN not set");
    let drink_url = env::var("DRINK_URL").unwrap_or("https://drink.csh.rit.edu/drinks".to_string());
    let mut prev_response: Option<drink::Response> = None;
    let client = Client::new();
    loop {
        match client.get(&drink_url).bearer_auth(&keycloak_token).send() {
            Ok(drink_response) => {
                if drink_response.status().is_success() {
                    let response_text = drink_response.text().unwrap();
                    let response: drink::Response = serde_json::from_str(&response_text).unwrap();
                    match prev_response {
                        Some(r) => {
                            let changes = diff(&r, &response);
                            match changes {
                                Some(changes) => {
                                    for change in changes {
                                        println!("{}", change)
                                    }
                                }
                                None => {}
                            }
                        }
                        None => {}
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

        std::thread::sleep(std::time::Duration::from_secs(5))
    }
}

fn diff(previous: &drink::Response, current: &drink::Response) -> Option<Vec<delta::Change>> {
    let mut changes: Vec<delta::Change> = vec![];
    for (pm, cm) in previous.machines.iter().zip(current.machines.iter()) {
        for (ps, cs) in pm.slots.iter().zip(cm.slots.iter()) {
            // Slot emptiness changed
            if ps.empty != cs.empty {
                // Slot was empty but isn't anymore
                if ps.empty == true && cs.empty == false {
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

    if changes.len() > 0 {
        return Some(changes);
    } else {
        return None;
    }
}
