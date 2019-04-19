// Copyright (c) 2019 RoccoDev
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use byteorder::{ReadBytesExt, NativeEndian};
use bit_utils::BitInformation;

pub trait MinecraftReader : ReadBytesExt {
    fn read_varint_unsigned(&mut self) -> u32 {
        let mut shift: u32 = 0;
        let mut decoded: u32 = 0;

        loop {
            match self.read_u32::<NativeEndian>() {
                Ok(uint) => {
                    decoded |= ((uint & 0b01111111) as u32) << shift;

                    if uint.has_most_signifigant_bit() {
                        shift += 7;
                    }
                    else {
                        return decoded;
                    }
                },
                Err(err) => panic!(err)
            }
        }
    }
}

impl MinecraftReader for std::net::TcpStream {}
impl MinecraftReader for std::io::Cursor<Vec<u8>> {}