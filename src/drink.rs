use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    pub machines: Vec<Machine>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Machine {
    pub display_name: String,
    pub id: u32,
    pub is_online: bool,
    pub name: String,
    pub slots: Vec<Slot>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Slot {
    pub number: u32,
    pub active: bool,
    pub empty: bool,
    pub item: Item,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub price: u32,
}
