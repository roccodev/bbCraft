// Copyright (c) 2019 RoccoDev
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::io::{Read, Write};
use std::net::TcpStream;

use mc_varint::{VarIntRead, VarIntWrite};
use openssl::symm::Cipher;

use crate::net::connection::Connection;

pub struct PacketInfo {
    pub length: usize,
    pub id: i32
}

pub struct Packet {
    pub info: PacketInfo,
    pub bytes: Vec<u8>
}

pub trait PacketHandler {
    fn handle(&self, connection: &mut Connection);
}

impl Packet {
    pub fn new(stream: &mut TcpStream) -> Result<Packet, std::io::Error> {
        let length = stream.read_var_i32()? as usize;
        let id = stream.read_var_i32()?;

        let info = PacketInfo {length, id};

        let mut buffer = vec![0u8; length - 1];
        let bytes_read = stream.read(&mut buffer)?;

        Ok(Packet {
            info,
            bytes: buffer
        })
    }

    pub fn new_from_data(packet_id: i32, data: Vec<u8>) -> Packet {
        let size = data.len() + 1;
        let info = PacketInfo {length: size, id: packet_id};

        Packet { info, bytes: data }
    }

    fn get_bytes(&mut self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes = vec![0u8; self.info.length];
        bytes.write_var_i32(self.info.length as i32)?;

        if self.info.id == 0 {
            bytes.write(&[0u8]);
        }
        else {
            bytes.write_var_i32(self.info.id)?;
        }

        bytes.write_all(&*self.bytes)?;

        Ok(bytes)
    }

    pub fn write(&mut self, connection: &mut Connection) -> Result<(), std::io::Error> {
        let mut stream = &connection.stream;
        let bytes = Packet::get_bytes(self)?;
        stream.write_all(&*bytes)
    }

    pub fn write_enc(&mut self, connection: &mut Connection, enc_key: Vec<u8>) -> Result<(), std::io::Error> {
        let cipher = Cipher::aes_128_cfb8();
        let secret = &enc_key.as_slice()[0..16];

        let bytes = Packet::get_bytes(self)?;

        let encrypt =
            openssl::symm::encrypt(cipher, secret, Some(secret), bytes.as_slice());
        let data = encrypt.unwrap();

        let mut stream = &connection.stream;
        stream.write_all(data.as_slice())
    }
}