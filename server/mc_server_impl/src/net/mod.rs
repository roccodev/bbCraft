// Copyright (c) 2019 RoccoDev
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod connection;
mod packet;
mod handshake;

use std::net::TcpListener;
use std::thread;
use std::sync::Arc;
use crate::Server;

pub fn listen(server: &mut Server) {
    let listener = TcpListener::bind((server.config.ip_addr.as_str(),
                                      server.config.port));
    match listener {
        Ok(listener) => {
            let ptr = Arc::new(listener);
            {
            let for_cli = ptr.clone();
            
            let cli = thread::spawn(move || {
                crate::cli::accept_user_input(for_cli.as_ref());
            });
            }
            let listener = ptr.as_ref();
            
            for stream in listener.incoming() {
                thread::spawn(move || {
                    connection::Connection::new(stream.unwrap()).listen();
                });
            }
        },
        Err(err) => crate::api::panic(err)
    }
}

/// Attempts to stop the server.
pub fn stop(listener: &TcpListener) {
    println!("Stopping server...");
    crate::api::server_unload();
    
    println!("Shutting down TCP listener...");
    drop(listener);

    print!("Exited succesfully.");
    std::process::exit(0);
}