// Copyright (c) 2019 RoccoDev
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::net::packet::{Packet, PacketHandler};
use crate::net::connection::{Connection, State};

use std::io::{Read, Seek, Write};
use mc_varint::VarIntWrite;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use crate::api::player_connect;
use std::ffi::{CStr, CString};

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
        read_slice.read_to_string(&mut player_name);

        unsafe {
            let result = player_connect(CString::new(player_name).unwrap().into_raw());
            let result = CString::from_raw(result);
            let result = result.to_str().unwrap();

            DisconnectPacket::new(String::from(result)).packet.write(connection).unwrap();
        }
    }
}

pub struct DisconnectPacket {
    pub packet: Packet
}

impl DisconnectPacket {
    fn new(reason: String) -> DisconnectPacket {
        let mut data = vec![];
        let str_bytes = reason.as_bytes();
        let size = data.write_var_i32(str_bytes.len() as i32);
        data.write_all(&str_bytes);

        let packet = Packet::new_from_data(0x0, data);

        DisconnectPacket { packet }
    }
}