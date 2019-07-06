// Copyright (C) 2019 RoccoDev
// Licensed under the MIT license.
// <https://opensource.org/licenses/MIT>

// Bench results:
// First hash: 152ms
// Second hash: 1ms
// Third hash: 0ms

use std::iter;

use regex::Regex;
use rustc_serialize::hex::ToHex;
use sha1::Sha1;

const LEADING_ZERO_REGEX: &str = r#"^0+"#;

pub fn calc_hash(shared_secret: &Vec<u8>, pub_key: &Vec<u8>) -> String {
    let mut hasher = Sha1::new();
    hasher.update(&[0u8; 20usize]);
    hasher.update(&shared_secret.as_slice()[0..16]);
    hasher.update(pub_key.as_slice());
    let mut hex = hasher.digest().bytes();

    let negative = (hex[0] & 0x80) == 0x80;

    let regex = Regex::new(LEADING_ZERO_REGEX).unwrap();

    if negative {
        two_complement(&mut hex);
        format!("-{}", regex.replace(hex.to_hex().as_str(), "").to_string())
    }
    else {
        regex.replace(hex.to_hex().as_str(), "").to_string()
    }
}

fn two_complement(bytes: &mut [u8; 20]) {
    let mut carry = true;
    for i in (0..bytes.len()).rev() {
        bytes[i] = !bytes[i] & 0xff;
        if carry {
            carry = bytes[i] == 0xff;
            bytes[i] = bytes[i] + 1;
        }
    }
}