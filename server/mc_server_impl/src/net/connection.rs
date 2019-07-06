// Copyright (c) 2019 RoccoDev
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::ffi::CString;
use std::net::{Shutdown, TcpStream};

use crate::api::player_connect;
use crate::net::encryption::EncryptionResponsePacket;
use crate::net::handshake::{DisconnectPacket, HandshakePacket, LoginPacket};
use crate::net::packet::PacketHandler;
use crate::util::ssl::SslInfo;

use super::packet::Packet;

#[derive(Debug)]
pub enum State {
    HANDSHAKE, LOGIN, STATUS
}

#[derive(Deserialize, Clone)]
pub struct PlayerIdentity {
    pub name: String,
    pub id: String
}

pub struct Connection {
    pub stream: TcpStream,
    pub state: State,
    pub alive: bool,
    pub key: &'static SslInfo,
    pub player: PlayerIdentity,
    pub verify_token: i32,
    pub is_online: bool
}

impl Connection {
    pub fn new(stream: TcpStream, key: &'static SslInfo, is_online: bool) -> Connection {
        let verify_token: i32 = rand::random();
        Connection {
            stream,
            state: State::HANDSHAKE,
            alive: true,
            key,
            player: PlayerIdentity {
                name: String::new(),
                id: String::new()
            },
            verify_token,
            is_online
        }
    }

    pub fn disconnect(&mut self) {
        self.stream.shutdown(Shutdown::Both);
        self.alive = false;

        std::thread::yield_now();
    }

    pub fn kick(&mut self, reason: String) {
        DisconnectPacket::new(reason).packet.write(self).unwrap();
    }

    fn call_api(&mut self, uuid: String, name: String) -> String {
        unsafe {
            let uuid_ptr =
                if uuid.len() == 0 {std::ptr::null_mut()} else {CString::new(uuid).unwrap().into_raw()};

            let name_ptr =
                if name.len() == 0 {std::ptr::null_mut()} else {CString::new(name).unwrap().into_raw()};

            let result = player_connect(uuid_ptr, name_ptr);

            let result = CString::from_raw(result);
            let result = result.to_str().unwrap();

            String::from(result)
        }
    }

    pub fn api_kick(&mut self, uuid: String, name: String) {
        unsafe {
            let string = Connection::call_api(self, uuid, name);
            DisconnectPacket::new(string).packet.write(self).unwrap();
        }
    }

    pub fn api_kick_enc(&mut self, uuid: String, name: String, enc_key: Vec<u8>) {
        unsafe {
            let string = Connection::call_api(self, uuid, name);
            DisconnectPacket::new(string).packet.write_enc(self, enc_key).unwrap();
        }
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
                                    LoginPacket {packet: &packet}
                                        .handle(self);
                                },
                                _ => {}
                            }
                        },
                        0x01 => {
                            EncryptionResponsePacket {packet}.handle(self);
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