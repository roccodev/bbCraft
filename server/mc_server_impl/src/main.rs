// Copyright (c) 2019 RoccoDev
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

extern crate bit_utils;
extern crate byteorder;
extern crate mc_varint;
extern crate toml;
extern crate serde;
extern crate serde_derive;
extern crate libc;

use crate::api::config::Configuration;

pub mod net;
pub mod cli;
pub mod api;

pub struct Server {
    pub config: Configuration
}

impl Server {
    pub fn new() -> Server {
        use std::fs::File;
        use std::path::Path;
        use std::io::prelude::*;

        // Get the config file
        let cfg_path = Path::new("config.toml");
        let mut cfg_file = File::open(&cfg_path).unwrap();
        let mut config_str = String::new();
        cfg_file.read_to_string(&mut config_str).unwrap();

        let config: Configuration = toml::from_str(config_str.as_str()).unwrap();

        Server {config}
    }
}

fn main() {

    let mut server = Server::new();

    unsafe {
        api::server_load();
    }

    // Start the server
    net::listen(&mut server);

}
