// Copyright (c) 2019 RoccoDev
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub mod config;

use std::os::raw::c_char;

#[link(kind = "dylib", name = "server")]
extern {
    pub fn server_load();
    pub fn player_connect(player_name: *mut c_char) -> *mut c_char;
    pub fn server_unload();
}

pub fn panic(reason: std::io::Error) {
    unsafe { server_unload(); }
    panic!(reason);
}