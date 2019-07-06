extern crate bit_utils;
extern crate byteorder;
#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate mc_varint;
extern crate num_bigint;
extern crate openssl;
extern crate rand;
extern crate regex;
extern crate reqwest;
extern crate rustc_serialize;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate sha1;
extern crate toml;

use crate::api::config::Configuration;
use crate::util::ssl::SslInfo;

pub mod net;
pub mod api;
pub mod util;
pub mod cli;

lazy_static! {
    static ref KEY: SslInfo = {
        util::ssl::generate_der_cert()
    };
}

pub struct Server {
    pub config: Configuration
}

impl Server {
    pub fn new() -> Server {
        use std::fs::File;
        use std::path::Path;
        use std::io::prelude::*;
        use std::env;

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
    net::listen(&mut server, &KEY);

}

