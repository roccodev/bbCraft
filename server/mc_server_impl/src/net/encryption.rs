// Copyright (c) 2019 RoccoDev
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::HashMap;
use std::ffi::CString;
use std::io::{Cursor, Read, Write};

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use mc_varint::{VarIntRead, VarIntWrite};
use openssl::rsa::Padding;

use crate::api::player_connect;
use crate::net::connection::{Connection, PlayerIdentity, State};
use crate::net::packet::{Packet, PacketHandler};

pub struct EncryptionRequestPacket {
    pub packet: Packet
}

pub struct EncryptionResponsePacket {
    pub packet: Packet
}

impl EncryptionRequestPacket {
    pub fn new(key: &'static Vec<u8>, verify_token: i32) -> EncryptionRequestPacket {
        let mut data = vec![];
        let empty_str = [0u8; 20];
        data.write_var_i32(20i32);
        data.write_all(&empty_str);

        let bytes = key.as_slice();

        data.write_var_i32(bytes.len() as i32);
        data.write_all(&bytes);


        data.write_var_i32(4);
        data.write_all(&verify_token.to_ne_bytes());

        let packet = Packet::new_from_data(0x01, data);

        EncryptionRequestPacket {packet}
    }
}

impl PacketHandler for EncryptionResponsePacket {
    fn handle(&self, connection: &mut Connection) {
        let mut read_slice = &*(self.packet.bytes);
        let shared_len = read_slice.read_var_i32().unwrap();
        let mut shared_tkn = vec![0u8; shared_len as usize];
        read_slice.read(shared_tkn.as_mut_slice());

        let verify_len = read_slice.read_var_i32().unwrap();
        let mut verify_tkn = vec![0u8; verify_len as usize];
        read_slice.read(verify_tkn.as_mut_slice());

        let prv_key = &connection.key.prv_key;
        let mut verify_buf = vec![0u8; prv_key.size() as usize];
        let mut shared_buf = vec![0u8; prv_key.size() as usize];
        prv_key.private_decrypt(verify_tkn.as_slice(), verify_buf.as_mut_slice(), Padding::PKCS1);
        prv_key.private_decrypt(shared_tkn.as_slice(), shared_buf.as_mut_slice(), Padding::PKCS1);

        let verify_token = Cursor::new(verify_buf).read_i32::<LittleEndian>();
        if verify_token.unwrap() == connection.verify_token {
            let hash = crate::util::hash::calc_hash(&shared_buf, &connection.key.pub_key);

            let url = format!("https://sessionserver.mojang.com/session/minecraft/hasJoined?username={}&serverId={}",
                connection.player.name, hash);

            let mut res = reqwest::get(url.as_str()).unwrap();
            let mojang: PlayerIdentity = res.json().unwrap();

            connection.player = mojang.clone();

            connection.api_kick_enc(mojang.id, mojang.name, shared_buf);
        }
        else {
            connection.kick("Invalid verification token. Pirated version?".to_string());
        }
    }
}