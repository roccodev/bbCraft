mod connection;
mod io;
mod packet;
mod handshake;

use std::net::TcpListener;
use std::thread;

pub fn listen(port: u16) {
    let listener = TcpListener::bind(("0.0.0.0", port));

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