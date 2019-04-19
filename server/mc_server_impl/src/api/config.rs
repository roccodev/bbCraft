// Copyright (c) 2019 RoccoDev
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub ip_addr: String,
    pub port: u16
}
