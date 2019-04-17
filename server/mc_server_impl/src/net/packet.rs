use std::net::TcpStream;
use std::io::{Read, Write};
use std::fs::read;
use mc_varint::{VarIntRead, VarIntWrite};
use byteorder::{ReadBytesExt, BigEndian, WriteBytesExt};
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

    pub fn write(&mut self, connection: &mut Connection) -> Result<(), std::io::Error> {
        let mut stream = &connection.stream;
        stream.write_var_i32(self.info.length as i32)?;

        if self.info.id == 0 {
            stream.write(&[0u8]);
        }
        else {
            stream.write_var_i32(self.info.id)?;
        }

        let mut bytes = &mut self.bytes;
        stream.write_all(&*bytes)
    }
}