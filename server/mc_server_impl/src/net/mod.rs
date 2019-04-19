// Copyright (c) 2019 RoccoDev
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod connection;
mod io;
mod packet;
mod handshake;

use std::net::TcpListener;
use std::thread;
use crate::Server;
use std::sync::Arc;

pub fn listen(server: &mut Server) {
    let listener = TcpListener::bind((server.config.ip_addr.as_str(),
                                      server.config.port));
    match listener {
        Ok(listener) => {
            for stream in listener.incoming() {
                thread::spawn(move || {
                    connection::Connection::new(stream.unwrap()).listen();
                });
            }
        },
        Err(err) => panic!(err)
    }


}