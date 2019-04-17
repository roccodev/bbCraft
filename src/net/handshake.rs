use crate::net::packet::{Packet, PacketHandler};
use crate::net::connection::{Connection, State};

use std::io::{Read, Seek};
use byteorder::{BigEndian, ReadBytesExt};

pub struct HandshakePacket<'a> {
    pub packet: &'a Packet
}

impl<'a> PacketHandler for HandshakePacket<'a> {
    fn handle(&self, connection: &mut Connection) {
        let mut read_slice = &*(self.packet.bytes);
        let mut placeholder = vec![0u8; read_slice.len() - 2];
        read_slice.read(&mut placeholder);
        println!("{:?}", &placeholder);
        let next_state = read_slice.read_i16::<BigEndian>().unwrap();
        connection.state = if next_state == 1 { State::STATUS } else { State::LOGIN };
        println!("{:?}", connection);
    }
}