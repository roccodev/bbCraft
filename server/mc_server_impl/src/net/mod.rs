// Copyright (c) 2019 RoccoDev
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::net::TcpListener;
use std::sync::Arc;
use std::thread;

use crate::Server;
use crate::util::ssl::SslInfo;

mod connection;
mod packet;
mod handshake;
mod encryption;

pub fn listen(server: &mut Server, cert: &'static SslInfo) {
    let listener = TcpListener::bind((server.config.ip_addr.as_str(),
                                      server.config.port));

    let online = server.config.online_mode;

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
                    connection::Connection::new(stream.unwrap(), cert, online).listen();
                });
            }
        },
        Err(err) => crate::api::panic(err)
    }
}

/// Attempts to stop the server.
pub fn stop(listener: &TcpListener) {
    println!("Stopping server...");
    unsafe { crate::api::server_unload(); }
    
    println!("Shutting down TCP listener...");
    drop(listener);

    print!("Exited succesfully.");
    std::process::exit(0);
}
