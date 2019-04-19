use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub ip_addr: String,
    pub port: u16
}
