// Copyright (c) 2019 RoccoDev
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::ffi::CString;
use std::io::{Read, Write};

use byteorder::{BigEndian, ReadBytesExt};
use mc_varint::{VarIntRead, VarIntWrite};
use openssl::symm::Cipher;

use crate::api::player_connect;
use crate::net::connection::{Connection, State};
use crate::net::encryption::EncryptionRequestPacket;
use crate::net::packet::{Packet, PacketHandler};

pub struct HandshakePacket<'a> {
    pub packet: &'a Packet
}

impl<'a> PacketHandler for HandshakePacket<'a> {
    fn handle(&self, connection: &mut Connection) {
        let mut read_slice = &*(self.packet.bytes);
        let mut placeholder = vec![0u8; read_slice.len() - 2];
        read_slice.read(&mut placeholder);
        let next_state = read_slice.read_i16::<BigEndian>().unwrap();
        connection.state = if next_state == 1 { State::STATUS } else { State::LOGIN };
    }
}

pub struct LoginPacket<'a> {
    pub packet: &'a Packet
}

impl<'a> PacketHandler for LoginPacket<'a> {
    fn handle(&self, connection: &mut Connection) {
        let mut read_slice = &*(self.packet.bytes);
        let mut player_name = String::new();
        read_slice.read_var_i32(); // String length, we don't care
        read_slice.read_to_string(&mut player_name);

        connection.player.name = player_name.clone();

        if connection.is_online {
            EncryptionRequestPacket::new(&connection.key.pub_key, connection.verify_token)
                .packet.write(connection).unwrap();
        }
        else {
            connection.api_kick(String::new(), player_name);
        }
    }
}

pub struct DisconnectPacket {
    pub packet: Packet
}

impl DisconnectPacket {
    pub fn new(reason: String) -> DisconnectPacket {
        let mut data = vec![];
        let str_bytes = reason.as_bytes();
        let size = data.write_var_i32(str_bytes.len() as i32);
        data.write_all(&str_bytes);

        let packet = Packet::new_from_data(0x0, data);

        DisconnectPacket { packet }
    }
}