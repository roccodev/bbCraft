// Copyright (c) 2019 RoccoDev
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::string::FromUtf8Error;

use openssl::hash::MessageDigest;
use openssl::nid::Nid;
use openssl::pkey::{PKey, Private};
use openssl::rsa::Rsa;
use openssl::x509::{X509, X509Name};

pub struct SslInfo {
    pub pub_key: Vec<u8>,
    pub prv_key: Rsa<Private>
}

pub fn generate_der_cert() -> SslInfo {
    let rsa = Rsa::generate(1024).unwrap();

    SslInfo {
        pub_key: (&rsa).public_key_to_der().unwrap(),
        prv_key: rsa
    }
}