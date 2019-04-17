use std::net::{TcpStream, Shutdown};
use std::io::Read;
use std::fs::read;
use super::io::read::MinecraftReader;
use mc_varint::{VarIntRead};
use byteorder::{ReadBytesExt, BigEndian};
use super::packet::{PacketInfo, Packet};
use crate::net::handshake::HandshakePacket;
use crate::net::packet::PacketHandler;

#[derive(Debug)]
pub enum State {
    HANDSHAKE, LOGIN, STATUS
}

#[derive(Debug)]
pub struct Connection {
    pub stream: TcpStream,
    pub state: State,
    pub alive: bool
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream,
            state: State::HANDSHAKE,
            alive: true
        }
    }

    pub fn disconnect(&mut self) {
        self.stream.shutdown(Shutdown::Both);
        self.alive = false;

        std::thread::yield_now();
    }

    pub fn listen(&mut self) {
        println!("Got a new connection :)");
        loop {
            if !self.alive {
                println!("Disconnected!");
                break;
            }

            let packet = Packet::new(&mut self.stream);
            match packet {
                Ok(packet) => {
                    println!("Received packet with ID {} and length {}. Contents: {:?}",
                             packet.info.id, packet.info.length, packet.bytes);

                    match packet.info.id {
                        0x0 => {
                            if let State::HANDSHAKE = &self.state {
                                HandshakePacket {packet: &packet}.handle(self);
                            }
                        }
                        _ => {}
                    }

                }
                Err(error) => panic!(error)
            }

        }
    }
}