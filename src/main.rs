use dotenv;
use std::env;

fn main() {
    dotenv::dotenv().ok();
    let _keycloak_token = env::var("KEYCLOAK_TOKEN").expect("KEYCLOAK_TOKEN not set");

    println!("Hello, world!");
}
