// Copyright (c) 2019 RoccoDev
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::io::{self, Write};
use std::net::TcpListener;
use std::sync::Arc;

pub fn accept_user_input(listener: &TcpListener) {
    loop {
        print!("> ");
        // stdout only flushes on newlines,
        // and the above 'print' macro does not include one.
        io::stdout().flush();

        let mut buf = String::new();
        io::stdin().read_line(&mut buf);

        match buf.trim() {
            "stop" => {
                crate::net::stop(listener);
            },
            (input) => {
                println!("{}: command not found.", input);
            }
        }

    }
}
