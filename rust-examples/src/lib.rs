// Copyright (c) 2019 RoccoDev
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
extern "C" fn server_load() {
    println!("Server loaded!");
}

#[no_mangle]
extern "C" fn server_unload() {
    println!("Server unloaded!");
}

#[no_mangle]
extern "C" fn player_connect(uuid: *mut c_char, player: *mut c_char) -> *mut c_char {
	// `uuid` == ptr::null_mut() when the server is in offline mode.
    unsafe {
        let player = CString::from_raw(player);
        let player = player.to_str().unwrap();
        println!("Player {} joined.", player);

        let json = format!(r#"{{"text": "Hi {}"}}"#, player);

        let cstr = CString::new(json).unwrap();
        cstr.into_raw()
    }
}
