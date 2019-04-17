use std::net::{TcpStream, Shutdown};
use std::io::Read;
use std::fs::read;
use super::io::read::MinecraftReader;
use mc_varint::{VarIntRead};
use byteorder::{ReadBytesExt, BigEndian};
use super::packet::{PacketInfo, Packet};
use crate::net::handshake::{HandshakePacket, LoginPacket};
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
        loop {
            if !self.alive {
                break;
            }

            let packet = Packet::new(&mut self.stream);
            match packet {
                Ok(packet) => {

                    match packet.info.id {
                        0x0 => {
                            match self.state {
                                State::HANDSHAKE => {
                                    HandshakePacket {packet: &packet}.handle(self);
                                },
                                State::LOGIN => {
                                    LoginPacket {packet: &packet}.handle(self);
                                },
                                _ => {}
                            }
                        }
                        _ => {}
                    }

                }
                Err(error) => {
                    self.disconnect();
                }
            }

        }
    }
}