use std::net::TcpStream;
use std::io::Read;
use std::fs::read;
use mc_varint::{VarIntRead};
use byteorder::{ReadBytesExt, BigEndian};
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

    pub fn new_from_data(packet_id: i32, data: &Vec<u8>) -> Result<Packet, std::io::Error> {

    }
}